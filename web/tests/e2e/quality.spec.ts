// The economy-class profile from the spec: a mid-range Android (Pixel 7 viewport), CPU
// throttled 4x, "fast 4G". The Quest map must hold 60 fps through a level transition, with no
// long task over 50 ms. Uses Sara (class 4 B) so the school story is untouched.
import { expect, test } from "@playwright/test";
import { passQuizF1, signIn, stone } from "./helpers.ts";

// A real GPU (this machine's Intel HD 630), as a phone has. Headless Chromium otherwise renders
// WebGL in software (SwiftShader), which measured the CPU pretending to be a GPU: 2.4 s of
// native "(program)" time against ~100 ms of our code.
test.use({ launchOptions: { args: ["--use-angle=gl", "--ignore-gpu-blocklist", "--enable-gpu"] } });

declare global {
  interface Window {
    __frames: number[];
    __long: number[];
  }
}

test("the map holds its frame rate through a level transition on an economy phone", async ({ page, context }) => {
  const cdp = await context.newCDPSession(page);
  await cdp.send("Emulation.setCPUThrottlingRate", { rate: 4 });
  await cdp.send("Network.emulateNetworkConditions", {
    offline: false,
    latency: 150,
    downloadThroughput: (1.6 * 1024 * 1024) / 8,
    uploadThroughput: (750 * 1024) / 8,
  });
  const renderer = await page.evaluate(() => {
    const gl = document.createElement("canvas").getContext("webgl2");
    const ext = gl?.getExtension("WEBGL_debug_renderer_info");
    return ext ? String(gl!.getParameter(ext.UNMASKED_RENDERER_WEBGL)) : "none";
  });
  test.skip(renderer.includes("SwiftShader") || renderer === "none", `no GPU here (${renderer}); frame rate is not measured on software WebGL`);
  await signIn(page, "sara");
  // Three level transitions, judged by the median: one run on a busy laptop is too noisy.
  const runs: { fps: number; worstTask: number; worstFrame: number }[] = [];
  for (let attempt = 0; attempt < 3; attempt++) {
    await stone(page, "F1").click();
    await passQuizF1(page);
    await page.getByRole("button", { name: /رجوع إلى الطريق/ }).first().click();
    // Start measuring, then step back onto the map: the stone redraws (the transition).
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
    await page.getByRole("button", { name: /رجوع إلى الطريق/ }).first().click();
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
