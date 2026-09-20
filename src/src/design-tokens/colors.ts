export const colors = {
  base: {
    white: '#ffffff',
    black: '#000000',
    transparent: 'transparent',
  },

  zinc: {
    50: '#fafafa',
    100: '#f4f4f5',
    200: '#e4e4e7',
    300: '#d4d4d8',
    400: '#a1a1aa',
    500: '#71717a',
    600: '#52525b',
    700: '#3f3f46',
    800: '#27272a',
    900: '#18181b',
    950: '#09090b',
  },

  amber: {
    50: '#fffbeb',
    100: '#fef3c7',
    200: '#fde68a',
    300: '#fcd34d',
    400: '#fbbf24',
    500: '#f59e0b',
    600: '#d97706',
    700: '#b45309',
    800: '#92400e',
    900: '#78350f',
    950: '#451a03',
  },

  emerald: {
    50: '#ecfdf5',
    100: '#d1fae5',
    200: '#a7f3d0',
    300: '#6ee7b7',
    400: '#34d399',
    500: '#10b981',
    600: '#059669',
    700: '#047857',
    800: '#065f46',
    900: '#064e3b',
    950: '#022c22',
  },

  cyan: {
    50: '#ecfeff',
    100: '#cffafe',
    200: '#a5f3fc',
    300: '#67e8f9',
    400: '#22d3ee',
    500: '#06b6d4',
    600: '#0891b2',
    700: '#0e7490',
    800: '#155e75',
    900: '#164e63',
    950: '#083344',
  },

  rose: {
    50: '#fff1f2',
    100: '#ffe4e6',
    200: '#fecdd3',
    300: '#fda4af',
    400: '#fb7185',
    500: '#f43f5e',
    600: '#e11d48',
    700: '#be123c',
    800: '#9f1239',
    900: '#881337',
    950: '#4c0519',
  },

  orange: {
    50: '#fff7ed',
    100: '#ffedd5',
    200: '#fed7aa',
    300: '#fdba74',
    400: '#fb923c',
    500: '#f97316',
    600: '#ea580c',
    700: '#c2410c',
    800: '#9a3412',
    900: '#7c2d12',
    950: '#431407',
  },

  violet: {
    50: '#f5f3ff',
    100: '#ede9fe',
    200: '#ddd6fe',
    300: '#c4b5fd',
    400: '#a78bfa',
    500: '#8b5cf6',
    600: '#7c3aed',
    700: '#6d28d9',
    800: '#5b21b6',
    900: '#4c1d95',
    950: '#2e1065',
  },
} as const;

export type ColorScale = keyof typeof colors.zinc;
export type ColorName = keyof typeof colors;

export const semanticColors = {
  light: {
    bg: {
      base: colors.zinc[50],
      surface: colors.zinc[100],
      surfaceElevated: colors.base.white,
      surfaceHover: colors.zinc[200],
      surfaceActive: colors.zinc[300],
    },
    border: {
      subtle: colors.zinc[200],
      default: colors.zinc[300],
      focus: colors.amber[500],
      error: colors.rose[500],
      success: colors.emerald[500],
    },
    text: {
      primary: colors.zinc[950],
      secondary: colors.zinc[600],
      tertiary: colors.zinc[400],
      inverse: colors.base.white,
      link: colors.amber[600],
      linkHover: colors.amber[700],
    },
    accent: {
      primary: colors.amber[500],
      primaryHover: colors.amber[600],
      primaryActive: colors.amber[700],
      primarySubtle: colors.amber[100],
      secondary: colors.cyan[500],
      secondaryHover: colors.cyan[600],
      success: colors.emerald[500],
      successSubtle: colors.emerald[100],
      warning: colors.amber[500],
      warningSubtle: colors.amber[100],
      error: colors.rose[500],
      errorSubtle: colors.rose[100],
      info: colors.cyan[500],
      infoSubtle: colors.cyan[100],
    },
    overlay: {
      scrim: 'rgba(0, 0, 0, 0.4)',
      modal: 'rgba(0, 0, 0, 0.6)',
      tooltip: colors.zinc[900],
    },
    focusRing: colors.amber[500],
    selection: 'rgba(245, 158, 11, 0.3)',
  },

  dark: {
    bg: {
      base: colors.zinc[950],
      surface: colors.zinc[900],
      surfaceElevated: colors.zinc[800],
      surfaceHover: colors.zinc[800],
      surfaceActive: colors.zinc[700],
    },
    border: {
      subtle: 'rgba(255, 255, 255, 0.06)',
      default: 'rgba(255, 255, 255, 0.1)',
      focus: colors.amber[400],
      error: colors.rose[400],
      success: colors.emerald[400],
    },
    text: {
      primary: colors.zinc[50],
      secondary: colors.zinc[300],
      tertiary: colors.zinc[500],
      inverse: colors.zinc[950],
      link: colors.amber[400],
      linkHover: colors.amber[300],
    },
    accent: {
      primary: colors.amber[400],
      primaryHover: colors.amber[300],
      primaryActive: colors.amber[500],
      primarySubtle: 'rgba(245, 158, 11, 0.15)',
      secondary: colors.cyan[400],
      secondaryHover: colors.cyan[300],
      success: colors.emerald[400],
      successSubtle: 'rgba(16, 185, 129, 0.15)',
      warning: colors.amber[400],
      warningSubtle: 'rgba(245, 158, 11, 0.15)',
      error: colors.rose[400],
      errorSubtle: 'rgba(244, 63, 94, 0.15)',
      info: colors.cyan[400],
      infoSubtle: 'rgba(6, 182, 212, 0.15)',
    },
    overlay: {
      scrim: 'rgba(0, 0, 0, 0.6)',
      modal: 'rgba(0, 0, 0, 0.8)',
      tooltip: colors.zinc[50],
    },
    focusRing: colors.amber[400],
    selection: 'rgba(245, 158, 11, 0.25)',
  },

  highContrast: {
    bg: {
      base: '#000000',
      surface: '#0a0a0a',
      surfaceElevated: '#1a1a1a',
      surfaceHover: '#2a2a2a',
      surfaceActive: '#3a3a3a',
    },
    border: {
      subtle: '#333333',
      default: '#ffffff',
      focus: '#ffcc00',
      error: '#ff6b6b',
      success: '#6bff6b',
    },
    text: {
      primary: '#ffffff',
      secondary: '#cccccc',
      tertiary: '#999999',
      inverse: '#000000',
      link: '#ffcc00',
      linkHover: '#ffff00',
    },
    accent: {
      primary: '#ffcc00',
      primaryHover: '#ffff00',
      primaryActive: '#ffaa00',
      primarySubtle: 'rgba(255, 204, 0, 0.2)',
      secondary: '#00ffff',
      secondaryHover: '#66ffff',
      success: '#6bff6b',
      successSubtle: 'rgba(107, 255, 107, 0.2)',
      warning: '#ffcc00',
      warningSubtle: 'rgba(255, 204, 0, 0.2)',
      error: '#ff6b6b',
      errorSubtle: 'rgba(255, 107, 107, 0.2)',
      info: '#00ffff',
      infoSubtle: 'rgba(0, 255, 255, 0.2)',
    },
    overlay: {
      scrim: 'rgba(0, 0, 0, 0.8)',
      modal: 'rgba(0, 0, 0, 0.9)',
      tooltip: '#ffffff',
    },
    focusRing: '#ffcc00',
    selection: 'rgba(255, 204, 0, 0.4)',
  },
} as const;

export type ThemeMode = keyof typeof semanticColors;
export type SemanticColor = (typeof semanticColors)[ThemeMode];

export function getSemanticColors(mode: ThemeMode): SemanticColor {
  return semanticColors[mode];
}

export function generateCSSVariables(mode: ThemeMode = 'dark'): Record<string, string> {
  const c = semanticColors[mode];
  const vars: Record<string, string> = {};

  Object.entries(c.bg).forEach(([key, value]) => {
    vars[`--bg-${key}`] = value;
  });
  Object.entries(c.border).forEach(([key, value]) => {
    vars[`--border-${key}`] = value;
  });
  Object.entries(c.text).forEach(([key, value]) => {
    vars[`--text-${key}`] = value;
  });
  Object.entries(c.accent).forEach(([key, value]) => {
    vars[`--accent-${key}`] = value;
  });
  Object.entries(c.overlay).forEach(([key, value]) => {
    vars[`--overlay-${key}`] = value;
  });
  vars['--focus-ring'] = c.focusRing;
  vars['--selection'] = c.selection;

  return vars;
}