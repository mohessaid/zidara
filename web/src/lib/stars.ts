import type { Judgement } from "./kernel/types/Judgement.ts";

/** Stars come only from the framework's judgement band: the lowest band is 0 stars. */
export function stars(judgement: Judgement): { n: number; of: number } {
  const band = judgement.band;
  if (!band) return { n: 0, of: 0 };
  return { n: band.rank, of: band.of - 1 };
}
