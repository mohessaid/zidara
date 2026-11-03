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
      // Registered by main.ts once the page is idle, not on load (ADR 0015).
      injectRegister: false,
      includeAssets: ["icon.svg"],
      manifest: {
        name: "Zidara",
        short_name: "Zidara",
        description: "The adaptable learning engine for the AI era · محرّك التعلّم القابل للتكيّف لعصر الذكاء الاصطناعي",
        lang: "en",
        dir: "ltr",
        // An installed app opens the game; the front page is for visitors (ADR 0014).
        start_url: "/#play",
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
        // The share image is for visitors, not for learning offline.
        // The face works offline in Arabic and Latin (53 KB); the other subsets load only if needed.
        globIgnores: ["**/og.png", "**/readex-pro-vietnamese-*.woff2", "**/readex-pro-latin-ext-*.woff2"],
        maximumFileSizeToCacheInBytes: 2 * 1024 * 1024,
        navigateFallback: "index.html",
        navigateFallbackDenylist: [/^\/api\//],
        // Lesson films are cached the first time they play, so they can be watched again offline.
        runtimeCaching: [
          {
            urlPattern: ({ url }) => url.pathname.startsWith("/media/"),
            handler: "CacheFirst",
            options: { cacheName: "zidara-media", rangeRequests: true, expiration: { maxEntries: 30 } },
          },
        ],
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
