// Two faces, both SIL OFL and both drawn for Arabic and Latin (ADR 0016): Alexandria, heavy, for
// headings; Readex Pro, quiet, for reading. Each loads only the scripts a page uses.
import "@fontsource-variable/alexandria";
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
