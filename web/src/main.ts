// Baloo Bhaijaan 2 (SIL OFL): one rounded face for Arabic and Latin, so all three languages
// look like the same friendly game. Each weight loads only the scripts a page uses.
import "@fontsource/baloo-bhaijaan-2/500.css";
import "@fontsource/baloo-bhaijaan-2/700.css";
import "@fontsource/baloo-bhaijaan-2/800.css";
import "./app.css";
import { mount } from "svelte";
import App from "./App.svelte";

const target = document.getElementById("app")!;
target.replaceChildren();
mount(App, { target });
