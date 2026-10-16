import { defineConfig } from "vitest/config";
import { svelte } from "@sveltejs/vite-plugin-svelte";
import { VitePWA } from "vite-plugin-pwa";

// The API runs on the Rust server; vite proxies it so the app and API share an origin.
const api = process.env.ZIDARA_API ?? "http://127.0.0.1:8080";

export default defineConfig({
  plugins: [
    svelte(),
    VitePWA({
      registerType: "autoUpdate",
      injectRegister: "script-defer",
      includeAssets: ["icon.svg"],
      manifest: {
        name: "Zidara",
        short_name: "Zidara",
        description: "طريق تعلّم يعرف كيف يكون التعليم · A learning path that knows how teaching works",
        lang: "ar",
        dir: "rtl",
        start_url: "/",
        display: "standalone",
        background_color: "#FFF8EC",
        theme_color: "#FFF8EC",
        icons: [
          { src: "icon-192.png", sizes: "192x192", type: "image/png" },
          { src: "icon-512.png", sizes: "512x512", type: "image/png" },
          { src: "icon-maskable-512.png", sizes: "512x512", type: "image/png", purpose: "maskable" },
        ],
      },
      workbox: {
        // The shell, the kernel's wasm and the fonts: everything needed to learn offline.
        // API data is not cached here; the IndexedDB log and outbox hold it.
        globPatterns: ["**/*.{js,css,html,wasm,woff2,svg,png}"],
        maximumFileSizeToCacheInBytes: 2 * 1024 * 1024,
        navigateFallback: "index.html",
        navigateFallbackDenylist: [/^\/api\//],
      },
    }),
  ],
  server: { proxy: { "/api": api } },
  preview: { proxy: { "/api": api } },
  build: { target: "es2022" },
  test: {
    include: ["tests/unit/**/*.test.ts"],
    environment: "node",
  },
});
