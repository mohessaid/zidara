// Renders tools/video/fractions.html to web/public/media/fractions-intro.<locale>.mp4:
// each frame is drawn at an exact time and piped to ffmpeg (H.264, 720p, 24 fps, no audio).
// Usage: node tools/video/render.mjs [ar fr en]
import { chromium } from "../../web/node_modules/@playwright/test/index.mjs";
import { spawn } from "node:child_process";
import { mkdirSync } from "node:fs";
import { fileURLToPath } from "node:url";

const FPS = 24;
const here = fileURLToPath(new URL(".", import.meta.url));
const out = fileURLToPath(new URL("../../web/public/media/", import.meta.url));
mkdirSync(out, { recursive: true });
const locales = process.argv.slice(2).length ? process.argv.slice(2) : ["ar", "fr", "en"];

const browser = await chromium.launch();
const page = await browser.newPage({ viewport: { width: 1280, height: 720 } });
await page.goto(`file://${here}fractions.html`);
await page.evaluate(() => window.fontsReady);
const duration = await page.evaluate(() => window.DURATION);
for (const locale of locales) {
  const file = `${out}fractions-intro.${locale}.mp4`;
  const ffmpeg = spawn("ffmpeg", ["-y", "-loglevel", "error", "-f", "image2pipe", "-framerate", String(FPS), "-i", "-",
    "-c:v", "libx264", "-preset", "slow", "-crf", "26", "-pix_fmt", "yuv420p", "-movflags", "+faststart", "-tune", "animation", file], { stdio: ["pipe", "inherit", "inherit"] });
  const done = new Promise((ok, fail) => ffmpeg.on("close", (code) => (code === 0 ? ok() : fail(new Error(`ffmpeg ${code}`)))));
  for (let frame = 0; frame < duration * FPS; frame++) {
    await page.evaluate(([t, l]) => window.render(t, l), [frame / FPS, locale]);
    const png = await page.screenshot({ type: "png" });
    if (!ffmpeg.stdin.write(png)) await new Promise((r) => ffmpeg.stdin.once("drain", r));
  }
  ffmpeg.stdin.end();
  await done;
  console.log(`rendered ${file}`);
}
await browser.close();
