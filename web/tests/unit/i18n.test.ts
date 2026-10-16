import { describe, expect, it } from "vitest";
import { STRINGS } from "../../src/lib/i18n/strings.ts";
import { direction, formatDate, isLocale, t } from "../../src/lib/i18n/i18n.ts";

describe("i18n", () => {
  it("has every key in every language, none empty", () => {
    const keys = Object.keys(STRINGS.en).sort();
    for (const locale of ["ar", "fr"] as const) {
      expect(Object.keys(STRINGS[locale]).sort()).toEqual(keys);
      for (const value of Object.values(STRINGS[locale])) expect(value.trim()).not.toBe("");
    }
  });

  it("keeps the same placeholders in every translation", () => {
    const vars = (s: string) => [...s.matchAll(/\{(\w+)\}/g)].map((m) => m[1]).sort();
    for (const [key, english] of Object.entries(STRINGS.en)) {
      for (const locale of ["ar", "fr"] as const) {
        expect(vars(STRINGS[locale][key as keyof typeof STRINGS.en]), `${locale}:${key}`).toEqual(vars(english));
      }
    }
  });

  it("fills placeholders and leaves unknown ones visible", () => {
    expect(t("en", "stars", { n: 2, of: 3 })).toBe("2 of 3 stars");
    expect(t("ar", "signin.as", { name: "أمينة" })).toBe("الدخول باسم أمينة");
    expect(t("fr", "stars", { n: 1 })).toBe("1 étoiles sur {of}");
  });

  it("knows direction and locales", () => {
    expect(direction("ar")).toBe("rtl");
    expect(direction("fr")).toBe("ltr");
    expect(isLocale("en")).toBe(true);
    expect(isLocale("de")).toBe(false);
    expect(isLocale(3)).toBe(false);
  });

  it("formats dates in the learner's language", () => {
    const monday = Date.UTC(2026, 9, 12, 12);
    expect(formatDate("en", monday)).toBe("Monday, October 12");
    expect(formatDate("fr", monday)).toBe("lundi 12 octobre");
    expect(formatDate("ar", monday)).toMatch(/الاثنين/);
  });
});
