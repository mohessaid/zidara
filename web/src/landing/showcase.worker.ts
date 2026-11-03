// The landing page's engine, off the main thread (ADR 0014): the kernel is pure computation, so
// a worker loads it and judges the showcase while the page stays responsive. On a phone slowed
// four times this was a 234 ms task on the main thread; here it blocks nothing.
import { loadKernel } from "../lib/kernel/kernel.ts";
import wasm from "../lib/kernel/pkg/zidara_wasm_bg.wasm?url";
import type { LessonPack } from "../lib/kernel/types/LessonPack.ts";
import { showcase } from "../lib/showcase.ts";
import pack from "../../../content/lessons/equivalent-fractions.json";

const ready = loadKernel(wasm);
const reply = (message: unknown) => (self as unknown as Worker).postMessage(message);

self.onmessage = async (event: MessageEvent<string>) => {
  try {
    await ready;
    reply({ ok: true, locale: event.data, views: showcase(pack as unknown as LessonPack, event.data) });
  } catch (error) {
    reply({ ok: false, error: String(error) });
  }
};
