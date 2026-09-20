export const radii = {
  none: '0',
  xs: '2px',
  sm: '4px',
  md: '6px',
  lg: '8px',
  xl: '12px',
  '2xl': '16px',
  '3xl': '24px',
  full: '9999px',
} as const;

export const semanticRadii = {
  component: {
    none: radii.none,
    xs: radii.xs,
    sm: radii.sm,
    md: radii.md,
    lg: radii.lg,
    xl: radii.xl,
    '2xl': radii['2xl'],
    full: radii.full,
  },
  layout: {
    none: radii.none,
    sm: radii.sm,
    md: radii.md,
    lg: radii.lg,
    xl: radii.xl,
    '2xl': radii['2xl'],
  },
  button: {
    sm: radii.sm,
    md: radii.md,
    lg: radii.lg,
  },
  card: {
    sm: radii.md,
    md: radii.lg,
    lg: radii.xl,
  },
  input: {
    sm: radii.sm,
    md: radii.md,
    lg: radii.lg,
  },
  badge: radii.full,
  tooltip: radii.sm,
  modal: radii.xl,
  popover: radii.lg,
} as const;

export type RadiusScale = keyof typeof radii;
export type SemanticRadii = typeof semanticRadii;

export function generateRadiiCSSVariables(): Record<string, string> {
  const vars: Record<string, string> = {};

  Object.entries(radii).forEach(([key, value]) => {
    vars[`--radius-${key}`] = value;
  });

  Object.entries(semanticRadii.component).forEach(([key, value]) => {
    vars[`--radius-component-${key}`] = value;
  });
  Object.entries(semanticRadii.layout).forEach(([key, value]) => {
    vars[`--radius-layout-${key}`] = value;
  });
  Object.entries(semanticRadii.button).forEach(([key, value]) => {
    vars[`--radius-button-${key}`] = value;
  });
  Object.entries(semanticRadii.card).forEach(([key, value]) => {
    vars[`--radius-card-${key}`] = value;
  });
  Object.entries(semanticRadii.input).forEach(([key, value]) => {
    vars[`--radius-input-${key}`] = value;
  });
  vars['--radius-badge'] = semanticRadii.badge;
  vars['--radius-tooltip'] = semanticRadii.tooltip;
  vars['--radius-modal'] = semanticRadii.modal;
  vars['--radius-popover'] = semanticRadii.popover;

  return vars;
}