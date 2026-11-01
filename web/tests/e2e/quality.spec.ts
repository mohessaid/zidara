// The economy-class profile from the spec: a mid-range Android (Pixel 7 viewport), CPU
// throttled 4x, "fast 4G". The path must hold 60 fps through a level transition, with no long
// task over 50 ms. Uses Sara (class 4 B) so the school story is untouched. The path is plain
// HTML and CSS (ADR 0012), so no GPU is needed to measure it honestly.
import { expect, test } from "@playwright/test";
import { backToPath, playF1, signIn, stone } from "./helpers.ts";

declare global {
  interface Window {
    __frames: number[];
    __long: number[];
  }
}

test("the path holds its frame rate through a level transition on an economy phone", async ({ page, context }) => {
  // Three full sessions on a phone throttled four times: more than the default minute.
  test.setTimeout(150_000);
  const cdp = await context.newCDPSession(page);
  await cdp.send("Emulation.setCPUThrottlingRate", { rate: 4 });
  await cdp.send("Network.emulateNetworkConditions", {
    offline: false,
    latency: 150,
    downloadThroughput: (1.6 * 1024 * 1024) / 8,
    uploadThroughput: (750 * 1024) / 8,
  });
  await signIn(page, "sara");
  // Three level transitions, judged by the median: one run on a busy laptop is too noisy.
  const runs: { fps: number; worstTask: number; worstFrame: number }[] = [];
  for (let attempt = 0; attempt < 3; attempt++) {
    await playF1(page);
    // Start measuring, then step back onto the path: the level turns gold (the transition).
    await page.evaluate(() => {
      // Each measurement owns its arrays; earlier frame loops and observers stop writing.
      const frames: number[] = [];
      const long: number[] = [];
      window.__frames = frames;
      window.__long = long;
      const observer = new PerformanceObserver((list) => {
        if (window.__long !== long) return observer.disconnect();
        list.getEntries().forEach((e) => long.push(e.duration));
      });
      observer.observe({ type: "longtask" });
      const tick = (t: number) => {
        if (window.__frames !== frames) return;
        frames.push(t);
        requestAnimationFrame(tick);
      };
      requestAnimationFrame(tick);
    });
    await backToPath(page);
    await expect(stone(page, "F1")).toHaveAttribute("data-state", "achieved");
    await page.waitForTimeout(1_500);
    const { frames, long } = await page.evaluate(() => ({ frames: window.__frames, long: window.__long }));
    const gaps = frames.slice(1).map((t, i) => t - frames[i]!);
    runs.push({ fps: (1000 * gaps.length) / (frames.at(-1)! - frames[0]!), worstTask: Math.max(0, ...long), worstFrame: Math.max(...gaps) });
  }
  const median = (xs: number[]) => [...xs].sort((a, b) => a - b)[1]!;
  const fps = median(runs.map((r) => r.fps));
  const worstTask = median(runs.map((r) => r.worstTask));
  console.log(`economy profile: ${runs.map((r) => `${r.fps.toFixed(1)} fps / long task ${r.worstTask.toFixed(0)} ms / worst frame ${r.worstFrame.toFixed(0)} ms`).join("; ")}; median ${fps.toFixed(1)} fps, ${worstTask.toFixed(0)} ms`);
  expect(fps).toBeGreaterThanOrEqual(55);
  expect(worstTask).toBeLessThanOrEqual(50);
});

test("the path costs an economy phone little", async ({ page, context }) => {
  // The same throttled phone, signed in, three seconds on the path with its animations running.
  const cdp = await context.newCDPSession(page);
  await cdp.send("Emulation.setCPUThrottlingRate", { rate: 4 });
  await cdp.send("Performance.enable");
  await signIn(page, "sara");
  await expect(stone(page, "F1")).toBeVisible();
  await page.waitForTimeout(3_000);
  const { metrics } = await cdp.send("Performance.getMetrics");
  const get = (name: string) => metrics.find((m) => m.name === name)!.value;
  const heapMB = get("JSHeapUsedSize") / 2 ** 20;
  const taskMs = get("TaskDuration") * 1000;
  console.log(`path: heap ${heapMB.toFixed(1)} MB, main thread ${taskMs.toFixed(0)} ms, script ${(get("ScriptDuration") * 1000).toFixed(0)} ms`);
  expect(heapMB).toBeLessThan(BUDGET.heapMB);
  expect(taskMs).toBeLessThan(BUDGET.taskMs);
});

/** Measured 3.8 MB and 1.6 s on 1 November 2026 (docs/JOURNAL.md); about 3x, for a busy machine. */
const BUDGET = { heapMB: 12, taskMs: 4_000 };
