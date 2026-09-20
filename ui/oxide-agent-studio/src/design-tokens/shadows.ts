export const shadows = {
  none: 'none',
  xs: '0 1px 2px 0 rgba(0, 0, 0, 0.05)',
  sm: '0 1px 3px 0 rgba(0, 0, 0, 0.1), 0 1px 2px -1px rgba(0, 0, 0, 0.1)',
  md: '0 4px 6px -1px rgba(0, 0, 0, 0.1), 0 2px 4px -2px rgba(0, 0, 0, 0.1)',
  lg: '0 10px 15px -3px rgba(0, 0, 0, 0.1), 0 4px 6px -4px rgba(0, 0, 0, 0.1)',
  xl: '0 20px 25px -5px rgba(0, 0, 0, 0.1), 0 8px 10px -6px rgba(0, 0, 0, 0.1)',
  '2xl': '0 25px 50px -12px rgba(0, 0, 0, 0.25)',
  inner: 'inset 0 2px 4px 0 rgba(0, 0, 0, 0.05)',
  focus: '0 0 0 3px',
} as const;

export const semanticShadows = {
  elevation: {
    none: shadows.none,
    level0: shadows.none,
    level1: shadows.xs,
    level2: shadows.sm,
    level3: shadows.md,
    level4: shadows.lg,
    level5: shadows.xl,
  },
  component: {
    card: shadows.sm,
    cardHover: shadows.md,
    cardActive: shadows.lg,
    dropdown: shadows.lg,
    modal: shadows['2xl'],
    popover: shadows.lg,
    tooltip: shadows.md,
    toast: shadows.lg,
    button: shadows.none,
    buttonHover: shadows.xs,
    buttonActive: shadows.inner,
    input: shadows.none,
    inputFocus: shadows.focus,
  },
  overlay: {
    scrim: 'rgba(0, 0, 0, 0.4)',
    modal: 'rgba(0, 0, 0, 0.6)',
  },
  focusRing: {
    default: '0 0 0 2px',
    inset: 'inset 0 0 0 2px',
  },
} as const;

export type ShadowScale = keyof typeof shadows;
export type SemanticShadows = typeof semanticShadows;

export function generateShadowsCSSVariables(): Record<string, string> {
  const vars: Record<string, string> = {};

  Object.entries(shadows).forEach(([key, value]) => {
    vars[`--shadow-${key}`] = value;
  });

  Object.entries(semanticShadows.elevation).forEach(([key, value]) => {
    vars[`--shadow-elevation-${key}`] = value;
  });
  Object.entries(semanticShadows.component).forEach(([key, value]) => {
    vars[`--shadow-component-${key}`] = value;
  });
  Object.entries(semanticShadows.overlay).forEach(([key, value]) => {
    vars[`--shadow-overlay-${key}`] = value;
  });
  Object.entries(semanticShadows.focusRing).forEach(([key, value]) => {
    vars[`--shadow-focus-${key}`] = value;
  });

  return vars;
}