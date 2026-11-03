// Dot-matrix art for the learner screens (ADR 0015): the same language as the front page, where
// every picture is a grid of dots. A picture is rows of characters, one per cell; the palette
// names each character's colour and "." is empty. Drawn as SVG circles: static, crisp at any size,
// no canvas and no frame loop, so the path stays at 60 frames a second on an economy phone.

export type Dot = { x: number; y: number; c: string };

export function bitmap(rows: readonly string[], palette: Readonly<Record<string, string>>): Dot[] {
  const dots: Dot[] = [];
  rows.forEach((row, y) => {
    [...row].forEach((ch, x) => {
      const c = palette[ch];
      if (c) dots.push({ x, y, c });
    });
  });
  return dots;
}

/** Bizra (بذرة, "seed"), the guide, as dots: a sprout on a seed with feet. Eyes and mouth are
 * drawn per mood on top of this body. */
export const BIZRA = [
  "....LL....RR....",
  "...LLLL..RRRR...",
  "....LLLssRRR....",
  ".......s........",
  ".....bbbbbb.....",
  "...bbbbbbbbbb...",
  "..bbbbbbbbbbbb..",
  ".bbbbbbbbbbbbbb.",
  ".bbbbbbbbbbbbbb.",
  "bbbbbbbbbbbbbbbb",
  "bbbbbbbbbbbbbbbb",
  "bbbbbbbbbbbbbbbb",
  "bbbbbllllllbbbbb",
  ".bbbllllllllbbb.",
  ".bbbllllllllbbb.",
  "..bbbllllllbbb..",
  "....bbbbbbbb....",
  "....ff....ff....",
] as const;

export const BIZRA_COLOURS = { L: "#7ed957", R: "#9be36f", s: "#2f8f3a", b: "#3fb950", l: "#b9f0a6", f: "#ff9600" } as const;

type Face = { eyes: [number, number, string][]; mouth: [number, number, string][]; arms?: [number, number][] };
const K = "#14121c";
const W = "#ffffff";
const P = "#ff8fa3";
const open: [number, number, string][] = [
  [3, 7, W], [4, 7, W], [3, 8, W], [4, 8, K],
  [11, 7, W], [12, 7, W], [11, 8, W], [12, 8, K],
];
const glad: [number, number, string][] = [[3, 8, K], [4, 7, K], [5, 8, K], [10, 8, K], [11, 7, K], [12, 8, K]];
const cheeks: [number, number, string][] = [[2, 10, P], [13, 10, P]];

export const FACES: Record<"idle" | "happy" | "cheer" | "think" | "encourage", Face> = {
  idle: { eyes: open, mouth: [...cheeks, [6, 10, K], [7, 11, K], [8, 11, K], [9, 10, K]] },
  happy: { eyes: glad, mouth: [...cheeks, [6, 10, K], [7, 11, K], [8, 11, K], [9, 10, K]] },
  cheer: { eyes: glad, mouth: [...cheeks, [6, 10, K], [7, 10, K], [8, 10, K], [9, 10, K], [7, 11, "#ff6b81"], [8, 11, "#ff6b81"]], arms: [[-1, 8], [-2, 7], [16, 8], [17, 7]] },
  think: {
    eyes: [[3, 7, W], [4, 7, K], [3, 8, W], [4, 8, W], [11, 7, W], [12, 7, K], [11, 8, W], [12, 8, W]],
    mouth: [...cheeks, [8, 11, K]],
  },
  encourage: { eyes: open, mouth: [...cheeks, [6, 11, K], [7, 11, K], [8, 11, K], [9, 11, K]] },
};

/** A date palm in dots: one grows for every outcome the framework says is achieved. The trunk
 * stands on column 6 of the last row. */
export const PALM = [
  "..GG.....GG..",
  ".GGGG...GGGG.",
  "GG..GG.GG..GG",
  "G....GgGG...G",
  "...GGgggGG...",
  "..G..dtd..G..",
  "......t......",
  "......tt.....",
  ".......t.....",
  ".......t.....",
  "......tt.....",
  "......t......",
  "......t......",
  ".....ttt.....",
] as const;
export const PALM_COLOURS = { G: "#2f9e5b", g: "#5fcf85", d: "#c0541f", t: "#9a6435" } as const;
