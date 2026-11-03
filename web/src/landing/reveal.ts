// Two small actions for the landing page. Both do nothing under reduced motion, and nothing
// without IntersectionObserver: the page reads complete either way (girth-gameui: motion has a
// reason, and a reduced-motion visitor loses nothing).
const still = () => typeof IntersectionObserver === "undefined" || matchMedia("(prefers-reduced-motion: reduce)").matches;

/** Rises once into place the first time it is a quarter on screen. */
export function reveal(node: HTMLElement) {
  if (still()) return;
  node.dataset.reveal = "";
  const io = new IntersectionObserver(
    (entries) => {
      if (!entries.some((e) => e.isIntersecting)) return;
      node.dataset.reveal = "in";
      io.disconnect();
    },
    { threshold: 0.15 },
  );
  io.observe(node);
  return { destroy: () => io.disconnect() };
}

/** Counts the number in a figure such as "369" or "60 fps" up from zero, once, in 900 ms. */
export function countUp(node: HTMLElement) {
  if (still()) return;
  const text = node.textContent ?? "";
  const match = /\d+/.exec(text);
  if (!match) return;
  const target = Number(match[0]);
  const [before, after] = [text.slice(0, match.index), text.slice(match.index + match[0].length)];
  let frame = 0;
  const io = new IntersectionObserver((entries) => {
    if (!entries.some((e) => e.isIntersecting)) return;
    io.disconnect();
    const start = performance.now();
    const tick = (now: number) => {
      const p = Math.min(1, (now - start) / 900);
      node.textContent = `${before}${Math.round(target * (1 - (1 - p) ** 3))}${after}`;
      if (p < 1) frame = requestAnimationFrame(tick);
    };
    frame = requestAnimationFrame(tick);
  });
  io.observe(node);
  return {
    destroy: () => {
      io.disconnect();
      cancelAnimationFrame(frame);
      node.textContent = text;
    },
  };
}
