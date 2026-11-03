// The front page's dot-matrix scenes (ADR 0015, recoloured in ADR 0016). Each one argues a point of the architecture
// (docs/proposal.md): one kernel under any framework under any experience, agents that pass
// through rules, work that waits offline, classes walled apart. Fixed-size scenes declare their
// design grid in SIZE.
import { type Grid, hash, type Scene } from "./pixels.ts";

const frac = (n: number) => n - Math.floor(n);

/** Points along a polyline, `s` from 0 to 1. */
function along(points: [number, number][], s: number): [number, number] {
  const lengths = points.slice(1).map((p, i) => Math.abs(p[0] - points[i]![0]) + Math.abs(p[1] - points[i]![1]));
  let d = s * lengths.reduce((a, b) => a + b, 0);
  for (let i = 0; i < lengths.length; i++) {
    const len = lengths[i]!;
    if (d <= len || i === lengths.length - 1) {
      const [x0, y0] = points[i]!;
      const [x1, y1] = points[i + 1]!;
      const k = len ? Math.min(1, d / len) : 0;
      return [x0 + (x1 - x0) * k, y0 + (y1 - y0) * k];
    }
    d -= len;
  }
  return points[0]!;
}

function path(g: Grid, points: [number, number][], v: 1 | 2 | 3) {
  for (let i = 1; i < points.length; i++) {
    const [x0, y0] = points[i - 1]!;
    const [x1, y1] = points[i]!;
    if (x0 === x1) g.vline(x0, Math.min(y0, y1), Math.abs(y1 - y0) + 1, v);
    else g.hline(Math.min(x0, x1), y0, Math.abs(x1 - x0) + 1, v);
  }
}

/** The architecture: four experiences over four frameworks over one kernel. A configuration
 * lights up, and pulses climb from the kernel through the framework to the experience. */
export const ENGINE = { cols: 64, rows: 40 } as const;
const COLS = [2, 18, 34, 50];
export const engine: Scene = (g, t) => {
  const step = Math.floor(t / 2.6);
  const fw = step % 4;
  const ex = (step * 3 + 1) % 4;
  // Experiences (rows 1–12): a path, a grid, a studio list, a chat.
  COLS.forEach((x, i) => {
    const on = i === ex;
    g.box(x, 1, 12, 12, on ? 3 : 1);
    const ink = on ? 3 : 2;
    if (i === 0) [[3, 10], [5, 8], [7, 9], [8, 6], [6, 4], [8, 3]].forEach(([a, b]) => g.fill(x + a! - 1, b! - 1, 2, 2, ink));
    if (i === 1) for (let r = 0; r < 4; r++) for (let c = 0; c < 4; c++) g.fill(x + 2 + c * 2, 3 + r * 2, 1, 1, hash(c, r + step) > 0.4 ? ink : 1);
    if (i === 2) for (let r = 0; r < 4; r++) g.words(x + 2, 3 + r * 2, 8, ink, r + 3);
    if (i === 3) {
      g.box(x + 2, 3, 7, 3, ink);
      g.box(x + 3, 7, 7, 3, 1);
    }
  });
  // Frameworks (rows 17–22).
  COLS.forEach((x, i) => {
    const on = i === fw;
    g.box(x, 17, 12, 6, on ? 3 : 1);
    g.words(x + 2, 19, 8, on ? 3 : 2, i + 11);
    g.words(x + 2, 20, 5, 1, i + 7);
  });
  // The kernel (rows 28–38): six building blocks.
  g.box(1, 28, 62, 11, 4);
  for (let b = 0; b < 6; b++) {
    const x = 4 + b * 10;
    const lit = Math.floor(t * 2) % 6 === b;
    g.fill(x, 31, 7, 5, lit ? 3 : 2);
  }
  // Connectors and the configuration's pulses.
  COLS.forEach((x) => {
    g.vline(x + 6, 23, 5, 1);
    g.vline(x + 6, 13, 4, 1);
  });
  const route: [number, number][] = [
    [COLS[fw]! + 6, 28],
    [COLS[fw]! + 6, 23],
    [COLS[fw]! + 6, 15],
    [COLS[ex]! + 6, 15],
    [COLS[ex]! + 6, 13],
  ];
  path(g, route, 2);
  for (let p = 0; p < 4; p++) {
    const [px, py] = along(route, frac(t * 0.55 + p / 4));
    g.set(px, py, 3);
  }
};

/** The six building blocks, lit one after another. */
export const CARD = { cols: 36, rows: 22 } as const;
export const blocks: Scene = (g, t) => {
  const lit = Math.floor(t * 1.4) % 6;
  for (let i = 0; i < 6; i++) {
    const x = 2 + (i % 3) * 11;
    const y = 2 + Math.floor(i / 3) * 10;
    const on = i === lit;
    g.box(x, y, 10, 8, on ? 3 : 1);
    g.words(x + 2, y + 3, 6, on ? 3 : 2, i + 1);
    g.words(x + 2, y + 5, 4, 1, i + 5);
  }
};

/** A framework: an outcome graph whose next step opens only when the one before is met. */
export const frameworks: Scene = (g, t) => {
  const nodes: [number, number][] = [[4, 10], [14, 4], [14, 16], [25, 10], [32, 4], [32, 16]];
  const edges: [number, number][] = [[0, 1], [0, 2], [1, 3], [2, 3], [3, 4], [3, 5]];
  const reached = Math.floor(t * 1.1) % 8;
  edges.forEach(([a, b]) => {
    const [x0, y0] = nodes[a]!;
    const [x1, y1] = nodes[b]!;
    const mid = Math.round((x0 + x1) / 2);
    path(g, [[x0 + 1, y0 + 1], [mid, y0 + 1], [mid, y1 + 1], [x1 + 1, y1 + 1]], b <= reached ? 2 : 1);
  });
  nodes.forEach(([x, y], i) => {
    if (i < reached) g.fill(x, y, 3, 3, 3);
    else if (i === reached) g.box(x - 1, y - 1, 5, 5, 3);
    else g.box(x, y, 3, 3, 1);
  });
};

/** Experiences: the same truth as a phone path, a dashboard and a chat, one after another. */
export const experiences: Scene = (g, t) => {
  const on = Math.floor(t / 1.8) % 3;
  const v = (i: number) => (i === on ? 3 : 1);
  g.box(2, 3, 9, 16, v(0));
  [[5, 15], [7, 12], [5, 9], [7, 6]].forEach(([x, y], i) => g.fill(x!, y!, 2, 2, i < 2 ? (on === 0 ? 3 : 2) : 1));
  g.box(13, 3, 12, 10, v(1));
  for (let i = 0; i < 4; i++) g.fill(15 + i * 2, 11 - (i + 1) * 2 + (i % 2), 1, (i + 1) * 2 - (i % 2), on === 1 ? 3 : 2);
  g.hline(17, 14, 4, v(1));
  g.box(27, 3, 8, 5, v(2));
  g.words(28, 5, 6, on === 2 ? 3 : 2, 4);
  g.box(25, 10, 8, 5, 1);
  g.words(26, 12, 6, 2, 9);
  g.set(29, 8, v(2));
};

/** AI agents as actors: a proposal passes the rules; anything unsure stops at the teacher. */
export const agents: Scene = (g, t) => {
  g.box(1, 7, 8, 8, 2);
  g.fill(3, 9, 4, 4, 2);
  g.box(14, 6, 9, 10, 4);
  g.words(16, 9, 5, 2, 2);
  g.words(16, 12, 5, 2, 6);
  g.box(27, 7, 8, 8, 3);
  g.fill(30, 9, 2, 2, 3);
  g.fill(29, 12, 4, 2, 3);
  g.hline(9, 11, 5, 1);
  g.hline(23, 11, 4, 1);
  g.vline(18, 16, 4, 1);
  g.hline(18, 19, 13, 1);
  g.vline(30, 15, 4, 1);
  for (let p = 0; p < 3; p++) {
    const s = frac(t * 0.35 + p / 3);
    const unsure = Math.floor(t * 0.35 + p / 3) % 2 === 1;
    const route: [number, number][] = unsure
      ? [[9, 11], [18, 11], [18, 19], [30, 19], [30, 15]]
      : [[9, 11], [27, 11]];
    const [x, y] = along(route, s);
    g.set(x, y, 3);
  }
};

/** Offline first: work waits on the phone, then syncs, and the server checks it again. */
export const offline: Scene = (g, t) => {
  const cycle = frac(t / 6);
  const online = cycle > 0.55;
  g.box(2, 3, 9, 16, 4);
  g.box(25, 4, 9, 14, 4);
  for (let r = 0; r < 3; r++) g.hline(27, 7 + r * 4, 5, 2);
  for (let x = 12; x < 24; x += 2) g.set(x, 11, online ? 2 : 1);
  const queued = online ? Math.max(0, 5 - Math.floor((cycle - 0.55) * 30)) : Math.min(5, Math.floor(cycle * 12));
  for (let q = 0; q < queued; q++) g.fill(4, 16 - q * 2, 5, 1, 3);
  if (online) for (let p = 0; p < 2; p++) g.set(12 + frac(t * 0.9 + p / 2) * 12, 11, 3);
};

/** Classes walled apart by the database: each one's dots move but never cross. */
export const walls: Scene = (g, t) => {
  for (let c = 0; c < 3; c++) {
    const x0 = 1 + c * 12;
    g.box(x0, 3, 10, 16, 4);
    for (let d = 0; d < 4; d++) {
      const fx = frac(t * (0.13 + d * 0.04) + hash(c, d));
      const fy = frac(t * (0.09 + d * 0.05) + hash(d, c));
      const x = x0 + 1 + Math.round(Math.abs(fx * 2 - 1) * 7);
      const y = 4 + Math.round(Math.abs(fy * 2 - 1) * 13);
      g.set(x, y, c === 1 ? 3 : 2);
    }
  }
};
