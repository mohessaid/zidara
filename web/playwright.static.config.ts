import { defineConfig, devices } from "@playwright/test";

// The demo build (`pnpm build:demo`): the whole school in the browser, served as static files.
export default defineConfig({
  testDir: "tests/static",
  workers: 1,
  retries: 0,
  timeout: 60_000,
  reporter: [["list"]],
  use: { baseURL: "http://127.0.0.1:4175", ...devices["Pixel 7"], locale: "ar" },
  webServer: {
    command: "npm run -s build:demo && npx vite preview --outDir dist-demo --port 4175 --strictPort --host 127.0.0.1",
    url: "http://127.0.0.1:4175",
    timeout: 180_000,
    reuseExistingServer: false,
  },
});
