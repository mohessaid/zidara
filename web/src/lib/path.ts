// Where each stone sits on the map, as fractions of the map's width and height. The path
// winds in the reading direction, so it is mirrored for right-to-left languages.
export type Point = { x: number; y: number };

export function stonePositions(count: number, rtl: boolean): Point[] {
  return Array.from({ length: count }, (_, i) => {
    const along = count === 1 ? 0.5 : 0.14 + (0.72 * i) / (count - 1);
    const y = 0.5 + (i % 2 === 0 ? 0.16 : -0.16);
    return { x: rtl ? 1 - along : along, y };
  });
}
