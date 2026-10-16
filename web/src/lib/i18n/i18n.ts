import { LOCALES, STRINGS, type Key, type Locale } from "./strings.ts";

export { LOCALES, type Key, type Locale };

/** The text for `key` in `locale`, with `{placeholders}` filled in. */
export function t(locale: Locale, key: Key, vars: Record<string, string | number> = {}): string {
  const text: string = STRINGS[locale][key];
  return text.replace(/\{(\w+)\}/g, (whole, name: string) => (name in vars ? String(vars[name]) : whole));
}

export const direction = (locale: Locale): "rtl" | "ltr" => (locale === "ar" ? "rtl" : "ltr");

export function isLocale(value: unknown): value is Locale {
  return typeof value === "string" && (LOCALES as readonly string[]).includes(value);
}

/** Numbers and dates in the learner's language (Arabic uses Arabic-Indic digits by default). */
export function formatDate(locale: Locale, ms: number): string {
  return new Intl.DateTimeFormat(locale, { weekday: "long", day: "numeric", month: "long" }).format(ms);
}
