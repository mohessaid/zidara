// Where API calls go. The real build calls the server. The demo build (`VITE_ZIDARA_DEMO=browser`,
// `pnpm build:demo`) calls the demo school inside the page, so the prototype can be tried from a
// static link. Vite replaces the flag when building, so the real build carries none of the demo.
import wasmUrl from "./kernel/pkg/zidara_wasm_bg.wasm?url";

export const DEMO = import.meta.env.VITE_ZIDARA_DEMO === "browser";

let school: Promise<typeof fetch> | undefined;

async function openSchool(): Promise<typeof fetch> {
  const [{ DemoSchool }, { loadKernel }, fractions, concept] = await Promise.all([
    import("./demo/school.ts"),
    import("./kernel/kernel.ts"),
    import("../../../content/lessons/equivalent-fractions.json"),
    import("../../../content/lessons/how-zidara-works.json"),
  ]);
  await loadKernel(wasmUrl);
  return new DemoSchool([fractions.default, concept.default] as never, localStorage).fetch;
}

export const apiFetch: typeof fetch = DEMO
  ? async (input, init) => (await (school ??= openSchool()))(input, init)
  : (...args) => fetch(...args);
