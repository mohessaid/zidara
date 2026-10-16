// Lighthouse on the production build (vite preview), mobile profile (Lighthouse's simulated
// mid-range phone: 4x CPU, slow 4G). Gate: performance >= 90, accessibility = 100.
// Usage: node scripts/lighthouse.mjs [url]   (start `npx vite preview --port 4173` first)
import { chromium } from "@playwright/test";
import lighthouse from "lighthouse";
import { launch } from "chrome-launcher";

const url = process.argv[2] ?? "http://127.0.0.1:4173/";
const chrome = await launch({ chromePath: chromium.executablePath(), chromeFlags: ["--headless=new", "--use-angle=gl", "--ignore-gpu-blocklist"] });
try {
  const result = await lighthouse(url, { port: chrome.port, onlyCategories: ["performance", "accessibility"], formFactor: "mobile", output: "json" });
  const { categories, audits } = result.lhr;
  const performance = Math.round(categories.performance.score * 100);
  const accessibility = Math.round(categories.accessibility.score * 100);
  const metric = (id) => audits[id].displayValue;
  console.log(`lighthouse ${url}: performance ${performance}, accessibility ${accessibility}`);
  console.log(`  FCP ${metric("first-contentful-paint")}, LCP ${metric("largest-contentful-paint")}, TBT ${metric("total-blocking-time")}, CLS ${metric("cumulative-layout-shift")}`);
  const failing = Object.values(audits).filter((a) => a.score !== null && a.score < 1 && categories.accessibility.auditRefs.some((r) => r.id === a.id));
  for (const a of failing) console.log(`  a11y: ${a.id}: ${a.title}`);
  if (performance < 90 || accessibility < 100) process.exitCode = 1;
} finally {
  await chrome.kill();
}
