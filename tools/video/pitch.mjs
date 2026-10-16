// The Solve pitch (silent, captions on screen): cards (pitch.html part a) → the recorded demo
// (pitch/demo/zidara-demo.mp4) → cards (part b). Output: pitch/solve/zidara-solve-pitch.mp4.
import { chromium } from "../../web/node_modules/@playwright/test/index.mjs";
import { spawn, execFileSync } from "node:child_process";
import { mkdirSync, mkdtempSync, writeFileSync } from "node:fs";
import { tmpdir } from "node:os";
import { join } from "node:path";
import { fileURLToPath } from "node:url";

const FPS = 25;
const here = fileURLToPath(new URL(".", import.meta.url));
const root = fileURLToPath(new URL("../../", import.meta.url));
const work = mkdtempSync(join(tmpdir(), "zidara-pitch-"));
mkdirSync(join(root, "pitch/solve"), { recursive: true });

const browser = await chromium.launch();
const page = await browser.newPage({ viewport: { width: 1920, height: 1080 } });
await page.goto(`file://${here}pitch.html`);
await page.evaluate(() => window.ready);
const parts = await page.evaluate(() => window.PARTS);
for (const [part, seconds] of Object.entries(parts)) {
  const file = join(work, `${part}.mp4`);
  const ffmpeg = spawn("ffmpeg", ["-y", "-loglevel", "error", "-f", "image2pipe", "-framerate", String(FPS), "-i", "-",
    "-c:v", "libx264", "-preset", "slow", "-crf", "20", "-pix_fmt", "yuv420p", "-r", String(FPS), file], { stdio: ["pipe", "inherit", "inherit"] });
  const done = new Promise((ok, fail) => ffmpeg.on("close", (c) => (c === 0 ? ok() : fail(new Error(`ffmpeg ${c}`)))));
  for (let frame = 0; frame < seconds * FPS; frame++) {
    await page.evaluate(([t, p]) => window.render(t, p), [frame / FPS, part]);
    if (!ffmpeg.stdin.write(await page.screenshot({ type: "png" }))) await new Promise((r) => ffmpeg.stdin.once("drain", r));
  }
  ffmpeg.stdin.end();
  await done;
}
await browser.close();
const demo = join(work, "demo.mp4");
execFileSync("ffmpeg", ["-y", "-loglevel", "error", "-i", join(root, "pitch/demo/zidara-demo.mp4"), "-vf", "fps=25,scale=1920:1080,setsar=1",
  "-c:v", "libx264", "-preset", "slow", "-crf", "20", "-pix_fmt", "yuv420p", demo]);
writeFileSync(join(work, "list.txt"), ["a.mp4", "demo.mp4", "b.mp4"].map((f) => `file '${join(work, f)}'`).join("\n"));
const out = join(root, "pitch/solve/zidara-solve-pitch.mp4");
execFileSync("ffmpeg", ["-y", "-loglevel", "error", "-f", "concat", "-safe", "0", "-i", join(work, "list.txt"), "-c", "copy", "-movflags", "+faststart", out]);
console.log(execFileSync("ffprobe", ["-v", "error", "-show_entries", "format=duration,size", "-of", "csv=p=0", out]).toString().trim());
