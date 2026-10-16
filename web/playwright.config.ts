import { defineConfig, devices } from "@playwright/test";

// One school, one story: tests run in order on a fresh database.
export default defineConfig({
  testDir: "tests/e2e",
  fullyParallel: false,
  workers: 1,
  retries: 0,
  timeout: 60_000,
  reporter: [["list"]],
  use: { baseURL: "http://127.0.0.1:4173", ...devices["Pixel 7"], locale: "ar" },
  webServer: [
    { command: "bash ../scripts/e2e-server.sh", url: "http://127.0.0.1:18080/api/me", timeout: 300_000, reuseExistingServer: false },
    {
      command: "npx vite build && npx vite preview --port 4173 --strictPort --host 127.0.0.1",
      url: "http://127.0.0.1:4173",
      env: { ZIDARA_API: "http://127.0.0.1:18080" },
      timeout: 180_000,
      reuseExistingServer: false,
    },
  ],
});
