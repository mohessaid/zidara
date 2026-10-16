import { defineConfig } from "vitest/config";
import { svelte } from "@sveltejs/vite-plugin-svelte";

// The API runs on the Rust server; vite proxies it so the app and API share an origin.
const api = process.env.ZIDARA_API ?? "http://127.0.0.1:8080";

export default defineConfig({
  plugins: [svelte()],
  server: { proxy: { "/api": api } },
  preview: { proxy: { "/api": api } },
  build: { target: "es2022" },
  test: {
    include: ["tests/unit/**/*.test.ts"],
    environment: "node",
  },
});
