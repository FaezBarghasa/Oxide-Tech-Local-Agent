export const zIndex = {
  hide: -1,
  base: 0,
  dropdown: 100,
  sticky: 200,
  fixed: 300,
  modalBackdrop: 400,
  modal: 500,
  popover: 600,
  tooltip: 700,
  toast: 800,
  commandPalette: 900,
  max: 9999,
} as const;

export const semanticZIndex = {
  layout: {
    background: zIndex.base,
    content: zIndex.base + 1,
    sidebar: zIndex.sticky,
    header: zIndex.sticky + 10,
    footer: zIndex.sticky,
  },
  overlay: {
    dropdown: zIndex.dropdown,
    popover: zIndex.popover,
    tooltip: zIndex.tooltip,
    toast: zIndex.toast,
    modalBackdrop: zIndex.modalBackdrop,
    modal: zIndex.modal,
    commandPalette: zIndex.commandPalette,
  },
  component: {
    button: zIndex.base,
    input: zIndex.base,
    card: zIndex.base,
    badge: zIndex.base + 1,
    avatar: zIndex.base + 1,
  },
  states: {
    focus: zIndex.tooltip + 10,
    hover: zIndex.base + 5,
    active: zIndex.base + 5,
    selected: zIndex.base + 2,
    disabled: zIndex.base,
    loading: zIndex.base + 3,
  },
} as const;

export type ZIndexScale = keyof typeof zIndex;
export type SemanticZIndex = typeof semanticZIndex;

export function generateZIndexCSSVariables(): Record<string, string> {
  const vars: Record<string, string> = {};

  Object.entries(zIndex).forEach(([key, value]) => {
    vars[`--z-${key}`] = String(value);
  });

  Object.entries(semanticZIndex.layout).forEach(([key, value]) => {
    vars[`--z-layout-${key}`] = String(value);
  });
  Object.entries(semanticZIndex.overlay).forEach(([key, value]) => {
    vars[`--z-overlay-${key}`] = String(value);
  });
  Object.entries(semanticZIndex.component).forEach(([key, value]) => {
    vars[`--z-component-${key}`] = String(value);
  });
  Object.entries(semanticZIndex.states).forEach(([key, value]) => {
    vars[`--z-state-${key}`] = String(value);
  });

  return vars;
}