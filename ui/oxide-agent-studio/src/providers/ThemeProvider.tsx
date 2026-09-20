import React, { createContext, useContext, useEffect, useState, useCallback } from 'react';
import { initializeTheme, setTheme, subscribeToThemeChanges, ThemeMode } from '../design-tokens';

interface ThemeContextValue {
  theme: ThemeMode;
  setTheme: (mode: ThemeMode) => void;
  toggleTheme: () => void;
  resolvedTheme: ThemeMode;
}

const ThemeContext = createContext<ThemeContextValue | null>(null);

export function ThemeProvider({ children }: { children: React.ReactNode }) {
  const [theme, setThemeState] = useState<ThemeMode>('dark');
  const [resolvedTheme, setResolvedTheme] = useState<ThemeMode>('dark');
  const [mounted, setMounted] = useState(false);

  useEffect(() => {
    const initialTheme = initializeTheme();
    setThemeState(initialTheme);
    setResolvedTheme(initialTheme);
    setMounted(true);

    const unsubscribe = subscribeToThemeChanges((mode) => {
      setResolvedTheme(mode);
    });

    return unsubscribe;
  }, []);

  const setThemeMode = useCallback((mode: ThemeMode) => {
    setTheme(mode);
    setThemeState(mode);
  }, []);

  const toggleTheme = useCallback(() => {
    const modes: ThemeMode[] = ['light', 'dark', 'highContrast'];
    const currentIndex = modes.indexOf(theme);
    const nextIndex = (currentIndex + 1) % modes.length;
    setThemeMode(modes[nextIndex]);
  }, [theme, setThemeMode]);

  if (!mounted) {
    return (
      <ThemeContext.Provider value={{ theme: 'dark', setTheme: () => {}, toggleTheme: () => {}, resolvedTheme: 'dark' }}>
        {children}
      </ThemeContext.Provider>
    );
  }

  return (
    <ThemeContext.Provider value={{ theme, setTheme: setThemeMode, toggleTheme, resolvedTheme }}>
      {children}
    </ThemeContext.Provider>
  );
}

export function useTheme(): ThemeContextValue {
  const context = useContext(ThemeContext);
  if (!context) {
    throw new Error('useTheme must be used within a ThemeProvider');
  }
  return context;
}