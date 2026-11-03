// The app's state: who is signed in, which cohort, the kernel for it, and sync status.
// Everything the screens show is derived from the kernel's view intents.
import { Api, ApiError, type Me } from "./api.ts";
import { direction, isLocale, type Locale } from "./i18n/i18n.ts";
import { Kernel, loadKernel } from "./kernel/kernel.ts";
import type { Actor } from "./kernel/types/Actor.ts";
import type { Command } from "./kernel/types/Command.ts";
import type { Event } from "./kernel/types/Event.ts";
import type { LearnerView } from "./kernel/types/LearnerView.ts";
import type { LessonPack } from "./kernel/types/LessonPack.ts";
import { openStore, type Store } from "./store.ts";
import { act, sync, type SyncReport } from "./sync.ts";
import wasmUrl from "./kernel/pkg/zidara_wasm_bg.wasm?url";

type Cohort = Me["cohorts"][number];
export type SyncState = { online: boolean; pending: number; refused: number; busy: boolean };

class App {
  locale: Locale = $state(readLocale());
  me: Me | undefined = $state();
  cohort: Cohort | undefined = $state();
  pack: LessonPack | undefined = $state();
  /** Bumped whenever the kernel's log changes, so views recompute. */
  version = $state(0);
  sync: SyncState = $state({ online: navigator.onLine, pending: 0, refused: 0, busy: false });
  error: string | undefined = $state();
  ready = $state(false);

  store: Store | undefined;
  api: Api | undefined;
  kernel: Kernel | undefined;

  /** The wasm kernel (143 KB) is fetched once someone needs it: a returning learner at once,
   *  a new visitor after sign-in or two seconds after the sign-in screen settles. Fetching it
   *  during first paint put it on the critical path (Lighthouse LCP 3.9 s). */
  #kernelReady: Promise<void> | undefined;
  #kernel = () => (this.#kernelReady ??= loadKernel(wasmUrl));

  async start() {
    this.store = await openStore();
    const session = await this.store.session();
    if (session) {
      this.api = new Api(session.token);
      await this.#enter(session.cohort ?? session.cohorts[0]?.id).catch(() => this.signOut());
    } else if (location.hash === "#play" || location.hash === "#classes") {
      // Warm the kernel for the game only; the front page runs its own copy in a worker.
      setTimeout(() => void this.#kernel(), 2_000);
    }
    addEventListener("online", () => {
      this.sync.online = true;
      void this.syncNow();
    });
    addEventListener("offline", () => (this.sync.online = false));
    // Every 4 s while the page is on screen (so a live round reaches every phone quickly; an
    // empty sync is a few bytes), never while it is hidden; at once when it comes back.
    setInterval(() => document.visibilityState === "visible" && void this.syncNow(), 4_000);
    document.addEventListener("visibilitychange", () => document.visibilityState === "visible" && void this.syncNow());
    this.ready = true;
  }

  setLocale(locale: Locale) {
    this.locale = locale;
    localStorage.setItem("zidara.locale", locale);
    document.documentElement.lang = locale;
    document.documentElement.dir = direction(locale);
  }

  async signIn(user: string) {
    this.error = undefined;
    await this.#begin(await Api.signIn(user));
  }

  /** Starts learning without an account, on the course whose lesson is `lesson` (ADR 0012). */
  async guest(lesson: string) {
    this.error = undefined;
    await this.#begin(await Api.guest(), lesson);
  }

  async #begin(api: Api, lesson?: string) {
    this.api = api;
    const me = await api.me();
    const cohort = me.cohorts.find((c) => c.lesson_id === lesson) ?? me.cohorts[0];
    await this.store!.saveSession({ token: api.token, user: me.id, displayName: me.display_name, cohorts: me.cohorts, cohort: cohort?.id });
    await this.#enter(cohort?.id, me);
  }

  /** Moves to another of this person's classes (a guest's other course). */
  async switchCohort(id: string) {
    if (id === this.cohort?.id) return;
    const session = await this.store!.session();
    if (session) await this.store!.saveSession({ ...session, cohort: id });
    this.kernel = this.pack = undefined;
    await this.#enter(id, this.me);
  }

  async signOut() {
    await this.store?.clearSession();
    this.me = this.cohort = this.pack = this.kernel = this.api = undefined;
  }

  get actor(): Actor | undefined {
    if (!this.me || !this.cohort) return undefined;
    return { kind: "human", user: this.me.id, role: this.cohort.role };
  }

  #events: { version: number; events: Event[] } | undefined;

  /** The kernel's log, read once per change: the screens that need it (XP, the streak) share
   *  one copy instead of each asking the kernel to serialise the whole log again. */
  events(): Event[] {
    const version = this.version;
    if (!this.kernel) return [];
    if (this.#events?.version !== version) this.#events = { version, events: this.kernel.events() };
    return this.#events.events;
  }

  learnerView(now = Date.now()): LearnerView | undefined {
    void this.version;
    return this.kernel && this.me ? this.kernel.learnerView(this.me.id, this.locale, now) : undefined;
  }

  /** Runs a command offline-first, then tries to sync. Returns the kernel's verdict. */
  async run(command: Command) {
    // A plain copy: commands built from a screen's state are proxies, which IndexedDB cannot store.
    const result = await act(this.store!, this.kernel!, this.cohort!.id, this.actor!, $state.snapshot(command) as Command);
    this.version++;
    await this.#countOutbox();
    await this.syncNow();
    return result;
  }

  #live: ReturnType<typeof setInterval> | undefined;

  /** While a live round runs, sync every 1.5 s instead of every 20 s, so the board and the
   *  phones keep pace with the teacher. */
  followLive(on: boolean) {
    if (on && !this.#live) this.#live = setInterval(() => void this.syncNow(), 1_500);
    if (!on && this.#live) {
      clearInterval(this.#live);
      this.#live = undefined;
    }
  }

  #queue: Promise<unknown> = Promise.resolve();

  /** Syncs now, after any sync already running: a request is never dropped. */
  syncNow(): Promise<SyncReport | undefined> {
    const next = this.#queue.then(() => this.#sync());
    this.#queue = next.catch(() => undefined);
    return next;
  }

  async #sync(): Promise<SyncReport | undefined> {
    if (!this.api || !this.kernel || !this.cohort) return;
    this.sync.busy = true;
    try {
      const report = await sync(this.store!, this.api, this.kernel, this.cohort.id);
      this.sync.online = true;
      if (report.changed) this.version++;
      return report;
    } catch (error) {
      if (error instanceof ApiError && error.status === 401) await this.signOut();
      else this.sync.online = false;
    } finally {
      this.sync.busy = false;
      await this.#countOutbox();
    }
  }

  async #enter(cohortId: string | undefined, me?: Me) {
    const store = this.store!;
    const session = await store.session();
    this.me = me ?? (await this.api!.me().catch(() => undefined)) ?? (session && { id: session.user, display_name: session.displayName, cohorts: session.cohorts as Cohort[] });
    this.cohort = this.me?.cohorts.find((c) => c.id === cohortId) ?? this.me?.cohorts[0];
    if (!this.cohort) return;
    const [pack] = await Promise.all([this.api!.pack(this.cohort.lesson_id, store), this.#kernel()]);
    this.pack = pack;
    this.kernel = new Kernel({ lesson: this.pack, framework: this.cohort.framework, tenant: this.cohort.tenant_id, cohort: this.cohort.id });
    // The saved log already holds offline work (act saves the kernel's whole log), so the
    // outbox is not replayed here: that would count every offline answer twice.
    this.kernel.load(await store.log(this.cohort.id));
    this.version++;
    await this.syncNow();
  }

  async #countOutbox() {
    if (!this.store || !this.cohort) return;
    this.sync.pending = (await this.store.pending(this.cohort.id)).length;
    this.sync.refused = (await this.store.refused(this.cohort.id)).length;
  }
}

function readLocale(): Locale {
  const saved = localStorage.getItem("zidara.locale");
  return isLocale(saved) ? saved : "en";
}

export const app = new App();
