<script lang="ts">
  // A course's badge in dots (ADR 0015): a pie in quarters for fractions, a path of levels for
  // the course about Zidara itself. Unknown courses get a star.
  import { bitmap } from "./dots.ts";

  let { lesson, size = 64 }: { lesson: string; size?: number } = $props();
  const ICONS: Record<string, { rows: string[]; colours: Record<string, string> }> = {
    "equivalent-fractions": {
      rows: [
        "....aaoo....",
        "..aaaaoooo..",
        ".aaaaa.oooo.",
        ".aaaaa.oooo.",
        "aaaaaa.ooooo",
        "aaaaaa.ooooo",
        "............",
        "aaaaaa.aaaaa",
        ".aaaaa.aaaa.",
        ".aaaaa.aaaa.",
        "..aaaa.aaa..",
        "....aa.a....",
      ],
      colours: { a: "#e8700a", o: "#f6d38f" },
    },
    "how-zidara-works": {
      rows: [
        "..........yy",
        ".........yyy",
        ".........yy.",
        "........r...",
        ".......r....",
        "....bb.r....",
        "...bbbb.....",
        "...bbbb.....",
        "..r.bb......",
        ".r..........",
        "gg..........",
        "gg..........",
      ],
      colours: { g: "#1d7a45", b: "#2450d3", y: "#f5b301", r: "#9fb3e6" },
    },
  };
  const STAR = {
    rows: ["....yy....", "....yy....", "yyyyyyyyyy", ".yyyyyyyy.", "..yyyyyy..", "..yy..yy..", ".yy....yy."],
    colours: { y: "#f5b301" },
  };
  const icon = $derived(ICONS[lesson] ?? STAR);
  const dots = $derived(bitmap(icon.rows, icon.colours));
  const w = $derived(icon.rows[0]!.length);
  const h = $derived(icon.rows.length);
</script>

<svg width={size} height={size} viewBox={`-0.5 -0.5 ${Math.max(w, h) + 1} ${Math.max(w, h) + 1}`} aria-hidden="true">
  {#each dots as d (`${d.x},${d.y}`)}<circle cx={d.x + 0.5} cy={d.y + 0.5} r="0.42" fill={d.c} />{/each}
</svg>
