export const durations = {
  instant: '0ms',
  fastest: '50ms',
  fast: '100ms',
  normal: '150ms',
  slow: '200ms',
  slower: '300ms',
  slowest: '500ms',
} as const;

export const easings = {
  linear: 'linear',
  in: 'cubic-bezier(0.4, 0, 1, 1)',
  out: 'cubic-bezier(0, 0, 0.2, 1)',
  inOut: 'cubic-bezier(0.4, 0, 0.2, 1)',
  sharp: 'cubic-bezier(0.4, 0, 0.6, 1)',
  bounce: 'cubic-bezier(0.34, 1.56, 0.64, 1)',
  spring: 'cubic-bezier(0.175, 0.885, 0.32, 1.275)',
} as const;

export const semanticTransitions = {
  all: {
    fastest: `${durations.fastest} ${easings.out}`,
    fast: `${durations.fast} ${easings.out}`,
    normal: `${durations.normal} ${easings.out}`,
    slow: `${durations.slow} ${easings.out}`,
  },
  colors: {
    fastest: `${durations.fastest} ${easings.out}`,
    fast: `${durations.fast} ${easings.out}`,
    normal: `${durations.normal} ${easings.out}`,
  },
  transform: {
    fastest: `${durations.fastest} ${easings.spring}`,
    fast: `${durations.fast} ${easings.spring}`,
    normal: `${durations.normal} ${easings.spring}`,
    slow: `${durations.slow} ${easings.spring}`,
  },
  opacity: {
    fastest: `${durations.fastest} ${easings.out}`,
    fast: `${durations.fast} ${easings.out}`,
    normal: `${durations.normal} ${easings.out}`,
  },
  shadow: {
    fast: `${durations.fast} ${easings.out}`,
    normal: `${durations.normal} ${easings.out}`,
  },
  height: {
    normal: `${durations.normal} ${easings.inOut}`,
    slow: `${durations.slow} ${easings.inOut}`,
  },
  width: {
    normal: `${durations.normal} ${easings.inOut}`,
    slow: `${durations.slow} ${easings.inOut}`,
  },
  component: {
    button: `${durations.fast} ${easings.out}`,
    buttonActive: `${durations.fastest} ${easings.in}`,
    card: `${durations.normal} ${easings.out}`,
    dropdown: `${durations.fast} ${easings.out}`,
    modal: `${durations.normal} ${easings.out}`,
    modalOverlay: `${durations.fast} ${easings.out}`,
    popover: `${durations.fast} ${easings.out}`,
    tooltip: `${durations.fastest} ${easings.out}`,
    toast: `${durations.normal} ${easings.spring}`,
    tabs: `${durations.fast} ${easings.out}`,
    accordion: `${durations.normal} ${easings.inOut}`,
    input: `${durations.fast} ${easings.out}`,
    badge: `${durations.fastest} ${easings.out}`,
  },
} as const;

export type Duration = keyof typeof durations;
export type Easing = keyof typeof easings;
export type SemanticTransitions = typeof semanticTransitions;

export function generateTransitionsCSSVariables(): Record<string, string> {
  const vars: Record<string, string> = {};

  Object.entries(durations).forEach(([key, value]) => {
    vars[`--duration-${key}`] = value;
  });
  Object.entries(easings).forEach(([key, value]) => {
    vars[`--easing-${key}`] = value;
  });

  Object.entries(semanticTransitions.all).forEach(([key, value]) => {
    vars[`--transition-all-${key}`] = value;
  });
  Object.entries(semanticTransitions.colors).forEach(([key, value]) => {
    vars[`--transition-colors-${key}`] = value;
  });
  Object.entries(semanticTransitions.transform).forEach(([key, value]) => {
    vars[`--transition-transform-${key}`] = value;
  });
  Object.entries(semanticTransitions.opacity).forEach(([key, value]) => {
    vars[`--transition-opacity-${key}`] = value;
  });
  Object.entries(semanticTransitions.shadow).forEach(([key, value]) => {
    vars[`--transition-shadow-${key}`] = value;
  });
  Object.entries(semanticTransitions.height).forEach(([key, value]) => {
    vars[`--transition-height-${key}`] = value;
  });
  Object.entries(semanticTransitions.width).forEach(([key, value]) => {
    vars[`--transition-width-${key}`] = value;
  });

  Object.entries(semanticTransitions.component).forEach(([key, value]) => {
    vars[`--transition-component-${key}`] = value;
  });

  return vars;
}