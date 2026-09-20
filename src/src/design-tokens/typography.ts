export const fontFamilies = {
  sans: "'Plus Jakarta Sans', system-ui, -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, sans-serif",
  mono: "'JetBrains Mono', ui-monospace, SFMono-Regular, Menlo, Monaco, Consolas, monospace",
  display: "'Plus Jakarta Sans', system-ui, -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, sans-serif",
} as const;

export const fontWeights = {
  light: 300,
  normal: 400,
  medium: 500,
  semibold: 600,
  bold: 700,
  extrabold: 800,
} as const;

export const fontSizes = {
  '2xs': '10px',
  xs: '11px',
  sm: '12px',
  base: '14px',
  lg: '16px',
  xl: '18px',
  '2xl': '20px',
  '3xl': '24px',
  '4xl': '30px',
  '5xl': '36px',
  '6xl': '48px',
} as const;

export const lineHeights = {
  none: 1,
  tight: 1.1,
  snug: 1.25,
  normal: 1.5,
  relaxed: 1.625,
  loose: 2,
} as const;

export const letterSpacings = {
  tighter: '-0.02em',
  tight: '-0.01em',
  normal: '0',
  wide: '0.01em',
  wider: '0.02em',
  widest: '0.04em',
  'widest-plus': '0.1em',
} as const;

export const textStyles = {
  display: {
    lg: { fontSize: fontSizes['5xl'], fontWeight: fontWeights.bold, lineHeight: lineHeights.tight, letterSpacing: letterSpacings.tighter },
    md: { fontSize: fontSizes['4xl'], fontWeight: fontWeights.bold, lineHeight: lineHeights.tight, letterSpacing: letterSpacings.tighter },
    sm: { fontSize: fontSizes['3xl'], fontWeight: fontWeights.bold, lineHeight: lineHeights.tight, letterSpacing: letterSpacings.tight },
  },
  heading: {
    h1: { fontSize: fontSizes['3xl'], fontWeight: fontWeights.bold, lineHeight: lineHeights.tight, letterSpacing: letterSpacings.tight },
    h2: { fontSize: fontSizes['2xl'], fontWeight: fontWeights.semibold, lineHeight: lineHeights.snug, letterSpacing: letterSpacings.tight },
    h3: { fontSize: fontSizes.xl, fontWeight: fontWeights.semibold, lineHeight: lineHeights.snug, letterSpacing: letterSpacings.normal },
    h4: { fontSize: fontSizes.lg, fontWeight: fontWeights.semibold, lineHeight: lineHeights.normal, letterSpacing: letterSpacings.normal },
  },
  body: {
    lg: { fontSize: fontSizes.lg, fontWeight: fontWeights.normal, lineHeight: lineHeights.relaxed, letterSpacing: letterSpacings.normal },
    md: { fontSize: fontSizes.base, fontWeight: fontWeights.normal, lineHeight: lineHeights.relaxed, letterSpacing: letterSpacings.normal },
    sm: { fontSize: fontSizes.sm, fontWeight: fontWeights.normal, lineHeight: lineHeights.normal, letterSpacing: letterSpacings.normal },
    xs: { fontSize: fontSizes.xs, fontWeight: fontWeights.normal, lineHeight: lineHeights.normal, letterSpacing: letterSpacings.normal },
  },
  label: {
    lg: { fontSize: fontSizes.sm, fontWeight: fontWeights.medium, lineHeight: lineHeights.normal, letterSpacing: letterSpacings.wide },
    md: { fontSize: fontSizes.xs, fontWeight: fontWeights.medium, lineHeight: lineHeights.normal, letterSpacing: letterSpacings.wider },
    sm: { fontSize: fontSizes['2xs'], fontWeight: fontWeights.medium, lineHeight: lineHeights.normal, letterSpacing: letterSpacings.widest },
  },
  mono: {
    lg: { fontSize: fontSizes.base, fontWeight: fontWeights.normal, lineHeight: lineHeights.normal, letterSpacing: letterSpacings.normal, fontFamily: fontFamilies.mono },
    md: { fontSize: fontSizes.sm, fontWeight: fontWeights.normal, lineHeight: lineHeights.normal, letterSpacing: letterSpacings.normal, fontFamily: fontFamilies.mono },
    sm: { fontSize: fontSizes.xs, fontWeight: fontWeights.normal, lineHeight: lineHeights.normal, letterSpacing: letterSpacings.normal, fontFamily: fontFamilies.mono },
    xs: { fontSize: fontSizes['2xs'], fontWeight: fontWeights.normal, lineHeight: lineHeights.normal, letterSpacing: letterSpacings.normal, fontFamily: fontFamilies.mono },
  },
  code: {
    inline: { fontSize: '0.9em', fontWeight: fontWeights.normal, lineHeight: lineHeights.normal, fontFamily: fontFamilies.mono },
    block: { fontSize: fontSizes.sm, fontWeight: fontWeights.normal, lineHeight: lineHeights.relaxed, fontFamily: fontFamilies.mono },
  },
} as const;

export type FontFamily = keyof typeof fontFamilies;
export type FontWeight = keyof typeof fontWeights;
export type FontSize = keyof typeof fontSizes;
export type LineHeight = keyof typeof lineHeights;
export type LetterSpacing = keyof typeof letterSpacings;
export type TextStyleCategory = keyof typeof textStyles;

export function generateTypographyCSSVariables(): Record<string, string> {
  const vars: Record<string, string> = {};

  Object.entries(fontFamilies).forEach(([key, value]) => {
    vars[`--font-family-${key}`] = value;
  });
  Object.entries(fontWeights).forEach(([key, value]) => {
    vars[`--font-weight-${key}`] = String(value);
  });
  Object.entries(fontSizes).forEach(([key, value]) => {
    vars[`--font-size-${key}`] = value;
  });
  Object.entries(lineHeights).forEach(([key, value]) => {
    vars[`--line-height-${key}`] = String(value);
  });
  Object.entries(letterSpacings).forEach(([key, value]) => {
    vars[`--letter-spacing-${key}`] = value;
  });

  Object.entries(textStyles).forEach(([category, styles]) => {
    Object.entries(styles).forEach(([variant, style]) => {
      const prefix = `--text-${category}-${variant}`;
      vars[`${prefix}-font-size`] = style.fontSize;
      vars[`${prefix}-font-weight`] = String(style.fontWeight);
      vars[`${prefix}-line-height`] = String(style.lineHeight);
      vars[`${prefix}-letter-spacing`] = style.letterSpacing;
      if (style.fontFamily) {
        vars[`${prefix}-font-family`] = style.fontFamily;
      }
    });
  });

  return vars;
}