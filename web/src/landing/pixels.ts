// Dot-matrix drawings for the front page (ADR 0015), in the manner of context.dev: a grid of
// dots where each cell is off, a hollow ring, a dim dot or a lit dot, redrawn from a scene
// function. Hand-written on one 2D canvas (no library): DPR capped at 2, frames capped at
// 30 a second, nothing drawn while off screen, and one still frame under reduced motion.

/** A cell's look: 0 empty, 1 hollow ring, 2 dim dot, 3 lit dot (the accent), 4 ink dot. */
export type Dot = 0 | 1 | 2 | 3 | 4;

export class Grid {
  cells: Uint8Array;
  constructor(
    readonly w: number,
    readonly h: number,
  ) {
    this.cells = new Uint8Array(w * h);
  }
  clear() {
    this.cells.fill(0);
  }
  get(x: number, y: number): Dot {
    return x < 0 || y < 0 || x >= this.w || y >= this.h ? 0 : (this.cells[y * this.w + x] as Dot);
  }
  set(x: number, y: number, v: Dot) {
    x = Math.round(x);
    y = Math.round(y);
    if (x >= 0 && y >= 0 && x < this.w && y < this.h) this.cells[y * this.w + x] = v;
  }
  /** A filled block. */
  fill(x: number, y: number, w: number, h: number, v: Dot) {
    for (let j = 0; j < h; j++) for (let i = 0; i < w; i++) this.set(x + i, y + j, v);
  }
  /** An outline. */
  box(x: number, y: number, w: number, h: number, v: Dot) {
    for (let i = 0; i < w; i++) {
      this.set(x + i, y, v);
      this.set(x + i, y + h - 1, v);
    }
    for (let j = 0; j < h; j++) {
      this.set(x, y + j, v);
      this.set(x + w - 1, y + j, v);
    }
  }
  hline(x: number, y: number, w: number, v: Dot) {
    for (let i = 0; i < w; i++) this.set(x + i, y, v);
  }
  vline(x: number, y: number, h: number, v: Dot) {
    for (let j = 0; j < h; j++) this.set(x, y + j, v);
  }
  /** A line of "text": short words of dots with gaps, seeded so it stays put. */
  words(x: number, y: number, w: number, v: Dot, seed = 1) {
    let i = 0;
    let s = seed;
    while (i < w) {
      s = (s * 9301 + 49297) % 233280;
      const len = 2 + (s % 5);
      this.hline(x + i, y, Math.min(len, w - i), v);
      i += len + 1;
    }
  }
}

/** A stable pseudo-random number in [0, 1) for a cell: the same everywhere, every frame. */
export function hash(x: number, y: number): number {
  const n = Math.sin(x * 127.1 + y * 311.7) * 43758.5453;
  return n - Math.floor(n);
}

/** Draws one frame: `t` is seconds since the scene started. */
export type Scene = (g: Grid, t: number) => void;

export type PixelOptions = {
  scene: Scene;
  /** The cell size in CSS pixels. */
  cell?: number;
  /** For a scene drawn to a fixed design: the grid's size in cells, scaled to fit. */
  cols?: number;
  rows?: number;
  /** Seconds into the scene to show when motion is reduced. */
  still?: number;
};

type Palette = { ring: string; dim: string; on: string; ink: string };

// Read once and shared by every drawing: each getComputedStyle forces a style pass over the
// whole page, and eight drawings doing it at mount made one long task on a slow phone.
let shared: { key: string; palette: Palette } | null = null;
function palette(el: Element): Palette {
  const key = `${matchMedia("(prefers-color-scheme: dark)").matches}`;
  if (shared?.key === key) return shared.palette;
  const css = getComputedStyle(el);
  const v = (name: string, fallback: string) => css.getPropertyValue(name).trim() || fallback;
  const read = { ring: v("--px-ring", "#c9d4ea"), dim: v("--px-dim", "#9db4de"), on: v("--px-on", "#2457d6"), ink: v("--px-ink", "#1f1733") };
  shared = { key, palette: read };
  return read;
}

/** A Svelte action: `<canvas use:pixels={{ scene }} />`. The canvas fills its parent. */
export function pixels(canvas: HTMLCanvasElement, options: PixelOptions) {
  let opts = options;
  const ctx = canvas.getContext("2d");
  if (!ctx) return {};
  canvas.style.visibility = "hidden";
  const reduce = matchMedia("(prefers-reduced-motion: reduce)");
  const dark = matchMedia("(prefers-color-scheme: dark)");
  let colors: Palette | null = null;
  let grid = new Grid(1, 1);
  let cell = opts.cell ?? 10;
  let dpr = 1;
  let visible = false;
  let raf = 0;
  let dirty = true;
  let last = 0;
  const start = performance.now();
  let sprites: HTMLCanvasElement[] = [];

  // Each look is drawn once into a tiny sprite, then stamped: far cheaper than an arc per dot.
  function makeSprites() {
    colors ??= palette(canvas);
    const c0 = colors;
    const size = Math.max(2, Math.round(cell * dpr));
    const r = size * 0.32;
    sprites = [0, 1, 2, 3, 4].map((v) => {
      const s = document.createElement("canvas");
      s.width = s.height = size;
      const c = s.getContext("2d")!;
      c.beginPath();
      c.arc(size / 2, size / 2, v === 1 ? r * 0.9 : r, 0, Math.PI * 2);
      if (v === 1) {
        c.strokeStyle = c0.ring;
        c.lineWidth = Math.max(1, size * 0.09);
        c.stroke();
      } else if (v > 1) {
        c.fillStyle = v === 2 ? c0.dim : v === 3 ? c0.on : c0.ink;
        c.fill();
      }
      return s;
    });
  }

  function resize() {
    const box = canvas.parentElement?.getBoundingClientRect() ?? canvas.getBoundingClientRect();
    if (!box.width || !box.height) return;
    dpr = Math.min(2, devicePixelRatio || 1);
    if (opts.cols && opts.rows) cell = Math.min(box.width / opts.cols, box.height / opts.rows);
    else cell = opts.cell ?? 10;
    const cols = opts.cols ?? Math.ceil(box.width / cell);
    const rows = opts.rows ?? Math.ceil(box.height / cell);
    const w = opts.cols ? cols * cell : box.width;
    const h = opts.rows ? rows * cell : box.height;
    canvas.style.width = `${w}px`;
    canvas.style.height = `${h}px`;
    canvas.width = Math.round(w * dpr);
    canvas.height = Math.round(h * dpr);
    grid = new Grid(cols, rows);
    makeSprites();
    dirty = true;
    // Off screen, nothing is drawn: the first frame waits until the drawing comes into view, so
    // a page of drawings never adds up to one long task.
    if (visible) paint();
  }

  function paint() {
    dirty = false;
    draw(reduce.matches ? (opts.still ?? 2) : (performance.now() - start) / 1000);
    // Shown only once drawn at its final size: a canvas that appears never shifts the layout.
    canvas.style.visibility = "visible";
  }

  function draw(t: number) {
    grid.clear();
    opts.scene(grid, t);
    ctx!.clearRect(0, 0, canvas.width, canvas.height);
    const step = cell * dpr;
    for (let y = 0; y < grid.h; y++) {
      for (let x = 0; x < grid.w; x++) {
        const v = grid.cells[y * grid.w + x]!;
        if (v) ctx!.drawImage(sprites[v]!, Math.round(x * step), Math.round(y * step));
      }
    }
  }

  function loop(now: number) {
    raf = 0;
    if (!visible || reduce.matches) return;
    if (now - last >= 33) {
      last = now;
      draw((now - start) / 1000);
    }
    raf = requestAnimationFrame(loop);
  }
  const play = () => {
    if (!raf && visible && !reduce.matches) raf = requestAnimationFrame(loop);
  };

  const ro = new ResizeObserver(resize);
  const io = new IntersectionObserver(([entry]) => {
    visible = !!entry?.isIntersecting;
    if (visible && dirty) paint();
    play();
  });
  const recolor = () => {
    colors = palette(canvas);
    resize();
  };
  // Nothing runs before the page's words are painted.
  const idle = (globalThis as { requestIdleCallback?: (f: () => void, o?: { timeout: number }) => number }).requestIdleCallback;
  const begin = () => {
    ro.observe(canvas.parentElement ?? canvas);
    io.observe(canvas);
    dark.addEventListener("change", recolor);
    reduce.addEventListener("change", recolor);
  };
  if (idle) idle(begin, { timeout: 1200 });
  else setTimeout(begin, 200);

  return {
    update(next: PixelOptions) {
      opts = next;
      resize();
    },
    destroy() {
      if (raf) cancelAnimationFrame(raf);
      ro.disconnect();
      io.disconnect();
      dark.removeEventListener("change", recolor);
      reduce.removeEventListener("change", recolor);
    },
  };
}
