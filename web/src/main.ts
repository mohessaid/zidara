// Baloo Bhaijaan 2 (SIL OFL): one rounded face for Arabic and Latin, so all three languages
// look like the same friendly game. Each weight loads only the scripts a page uses.
// One face for every screen, Arabic and Latin (ADR 0015).
import "@fontsource-variable/readex-pro";
import "./app.css";
import { mount } from "svelte";
import App from "./App.svelte";

// Offline comes after the first look: the service worker registers once the page has loaded and
// the browser is idle, so its start-up never competes with the first screen on a slow phone.
if ("serviceWorker" in navigator) {
  addEventListener("load", () => {
    const idle = (globalThis as { requestIdleCallback?: (f: () => void, o?: { timeout: number }) => number }).requestIdleCallback;
    const register = () => void navigator.serviceWorker.register("/sw.js", { scope: "/" });
    setTimeout(() => (idle ? idle(register, { timeout: 3000 }) : register()), 1500);
  });
}

const target = document.getElementById("app")!;
target.replaceChildren();
mount(App, { target });
