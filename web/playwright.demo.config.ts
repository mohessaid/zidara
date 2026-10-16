import { defineConfig } from "@playwright/test";
import base from "./playwright.config.ts";

// The demo film: same servers as the E2E suite, one recorded story, real GPU for the map.
export default defineConfig({
  ...base,
  testDir: "tests/demo",
  timeout: 180_000,
  use: { ...base.use, launchOptions: { args: ["--use-angle=gl", "--ignore-gpu-blocklist", "--enable-gpu"] } },
});
