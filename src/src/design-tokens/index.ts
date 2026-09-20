export * from './colors';
export * from './spacing';
export * from './typography';
export * from './radii';
export * from './shadows';
export * from './transitions';
export * from './z-index';

import { generateCSSVariables as generateColorVars } from './colors';
import { generateSpacingCSSVariables } from './spacing';
import { generateTypographyCSSVariables } from './typography';
import { generateRadiiCSSVariables } from './radii';
import { generateShadowsCSSVariables } from './shadows';
import { generateTransitionsCSSVariables } from './transitions';
import { generateZIndexCSSVariables } from './z-index';

export type ThemeMode = 'light' | 'dark' | 'highContrast';

export interface DesignTokens {
  colors: ReturnType<typeof generateColorVars>;
  spacing: ReturnType<typeof generateSpacingCSSVariables>;
  typography: ReturnType<typeof generateTypographyCSSVariables>;
  radii: ReturnType<typeof generateRadiiCSSVariables>;
  shadows: ReturnType<typeof generateShadowsCSSVariables>;
  transitions: ReturnType<typeof generateTransitionsCSSVariables>;
  zIndex: ReturnType<typeof generateZIndexCSSVariables>;
}

export function generateAllCSSVariables(mode: ThemeMode = 'dark'): Record<string, string> {
  return {
    ...generateColorVars(mode),
    ...generateSpacingCSSVariables(),
    ...generateTypographyCSSVariables(),
    ...generateRadiiCSSVariables(),
    ...generateShadowsCSSVariables(),
    ...generateTransitionsCSSVariables(),
    ...generateZIndexCSSVariables(),
  };
}

export function injectCSSVariables(mode: ThemeMode = 'dark', target: HTMLElement = document.documentElement): void {
  const vars = generateAllCSSVariables(mode);
  Object.entries(vars).forEach(([key, value]) => {
    target.style.setProperty(key, value);
  });
}

export function createThemeStyleElement(mode: ThemeMode = 'dark', id = 'design-tokens'): HTMLStyleElement {
  const existing = document.getElementById(id);
  if (existing) {
    existing.remove();
  }

  const style = document.createElement('style');
  style.id = id;
  style.type = 'text/css';

  const vars = generateAllCSSVariables(mode);
  const cssVars = Object.entries(vars)
    .map(([key, value]) => `  ${key}: ${value};`)
    .join('\n');

  style.textContent = `:root {\n${cssVars}\n}`;

  document.head.appendChild(style);
  return style;
}

export const defaultTheme: ThemeMode = 'dark';

export function getThemeFromStorage(): ThemeMode {
  if (typeof window === 'undefined') return defaultTheme;
  const stored = localStorage.getItem('oxide-theme') as ThemeMode | null;
  if (stored && ['light', 'dark', 'highContrast'].includes(stored)) {
    return stored;
  }
  if (window.matchMedia('(prefers-color-scheme: dark)').matches) {
    return 'dark';
  }
  return 'light';
}

export function setTheme(mode: ThemeMode): void {
  if (typeof window === 'undefined') return;
  localStorage.setItem('oxide-theme', mode);
  createThemeStyleElement(mode);
  document.documentElement.setAttribute('data-theme', mode);
}

export function initializeTheme(): ThemeMode {
  const mode = getThemeFromStorage();
  createThemeStyleElement(mode);
  document.documentElement.setAttribute('data-theme', mode);
  return mode;
}

export const themeMediaQueries = {
  light: '(prefers-color-scheme: light)',
  dark: '(prefers-color-scheme: dark)',
  highContrast: '(prefers-contrast: more)',
  reducedMotion: '(prefers-reduced-motion: reduce)',
} as const;

export function subscribeToThemeChanges(callback: (mode: ThemeMode) => void): () => void {
  if (typeof window === 'undefined') return () => {};

  const mediaQueryLists = Object.entries(themeMediaQueries).map(([mode, query]) => ({
    mode: mode as ThemeMode,
    mql: window.matchMedia(query),
  }));

  const handleChange = () => {
    const mode = getThemeFromStorage();
    callback(mode);
  };

  mediaQueryLists.forEach(({ mql }) => {
    mql.addEventListener('change', handleChange);
  });

  return () => {
    mediaQueryLists.forEach(({ mql }) => {
      mql.removeEventListener('change', handleChange);
    });
  };
}