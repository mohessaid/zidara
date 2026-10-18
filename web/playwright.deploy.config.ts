import { defineConfig, devices } from "@playwright/test";

// Checks a running deployment without changing its data: ZIDARA_URL=https://… npm run smoke
const baseURL = process.env.ZIDARA_URL;
if (!baseURL) throw new Error("set ZIDARA_URL to the deployment, e.g. https://zidara.example.org");

export default defineConfig({
  testDir: "tests/deploy",
  workers: 1,
  retries: 0,
  timeout: 60_000,
  reporter: [["list"]],
  use: { baseURL, ...devices["Pixel 7"], locale: "ar" },
});
