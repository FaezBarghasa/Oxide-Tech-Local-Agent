export const spacing = {
  0: '0',
  0.5: '2px',
  1: '4px',
  1.5: '6px',
  2: '8px',
  2.5: '10px',
  3: '12px',
  3.5: '14px',
  4: '16px',
  5: '20px',
  6: '24px',
  7: '28px',
  8: '32px',
  9: '36px',
  10: '40px',
  11: '44px',
  12: '48px',
  14: '56px',
  16: '64px',
  20: '80px',
  24: '96px',
  28: '112px',
  32: '128px',
  36: '144px',
  40: '160px',
  44: '176px',
  48: '192px',
  52: '208px',
  56: '224px',
  60: '240px',
  64: '256px',
  72: '288px',
  80: '320px',
  96: '384px',
} as const;

export const semanticSpacing = {
  component: {
    none: spacing[0],
    xs: spacing[1],
    sm: spacing[2],
    md: spacing[3],
    lg: spacing[4],
    xl: spacing[6],
    '2xl': spacing[8],
  },
  layout: {
    none: spacing[0],
    xs: spacing[2],
    sm: spacing[4],
    md: spacing[6],
    lg: spacing[8],
    xl: spacing[12],
    '2xl': spacing[16],
  },
  inset: {
    none: spacing[0],
    xs: spacing[1],
    sm: spacing[2],
    md: spacing[3],
    lg: spacing[4],
    xl: spacing[6],
    '2xl': spacing[8],
  },
  stack: {
    none: spacing[0],
    xs: spacing[1],
    sm: spacing[2],
    md: spacing[3],
    lg: spacing[4],
    xl: spacing[6],
    '2xl': spacing[8],
  },
  inline: {
    none: spacing[0],
    xs: spacing[1],
    sm: spacing[2],
    md: spacing[3],
    lg: spacing[4],
    xl: spacing[6],
  },
  gap: {
    none: spacing[0],
    xs: spacing[1],
    sm: spacing[2],
    md: spacing[3],
    lg: spacing[4],
    xl: spacing[6],
    '2xl': spacing[8],
  },
} as const;

export type SpacingScale = keyof typeof spacing;
export type SemanticSpacing = typeof semanticSpacing;

export function generateSpacingCSSVariables(): Record<string, string> {
  const vars: Record<string, string> = {};

  Object.entries(spacing).forEach(([key, value]) => {
    vars[`--space-${key}`] = value;
  });

  Object.entries(semanticSpacing.component).forEach(([key, value]) => {
    vars[`--space-component-${key}`] = value;
  });
  Object.entries(semanticSpacing.layout).forEach(([key, value]) => {
    vars[`--space-layout-${key}`] = value;
  });
  Object.entries(semanticSpacing.inset).forEach(([key, value]) => {
    vars[`--space-inset-${key}`] = value;
  });
  Object.entries(semanticSpacing.stack).forEach(([key, value]) => {
    vars[`--space-stack-${key}`] = value;
  });
  Object.entries(semanticSpacing.inline).forEach(([key, value]) => {
    vars[`--space-inline-${key}`] = value;
  });
  Object.entries(semanticSpacing.gap).forEach(([key, value]) => {
    vars[`--space-gap-${key}`] = value;
  });

  return vars;
}