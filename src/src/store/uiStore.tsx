import React, { createContext, useContext, useState, useCallback } from 'react';
import type { TabId } from '../types';
interface UIState {
  tab: TabId;
  setTab: (t: TabId) => void;
  sidebarCollapsed: boolean;
  toggleSidebar: () => void;
  paletteOpen: boolean;
  setPalette: (v: boolean) => void;
  deployModel: string | null;
  setDeployModel: (m: string | null) => void;
  mobileCompanionOpen: boolean;
  setMobileCompanionOpen: (v: boolean) => void;
  hitlOpen: boolean;
  setHitlOpen: (v: boolean) => void;
  hitlPendingCount: number;
  setHitlPendingCount: (n: number) => void;
  toasts: { id: number; msg: string }[];
  toast: (msg: string) => void;
}
const Ctx = createContext<UIState | null>(null);
let toastId = 1;
export function UIProvider({ children, initialTab }: { children: React.ReactNode; initialTab: TabId }) {
  const [tab, setTab] = useState<TabId>(initialTab);
  const [sidebarCollapsed, setCollapsed] = useState(false);
  const [paletteOpen, setPalette] = useState(false);
  const [deployModel, setDeployModel] = useState<string | null>(null);
  const [mobileCompanionOpen, setMobileCompanionOpen] = useState(false);
  const [hitlOpen, setHitlOpen] = useState(false);
  const [hitlPendingCount, setHitlPendingCount] = useState(2);
  const [toasts, setToasts] = useState<{ id: number; msg: string }[]>([]);
  const toast = useCallback((msg: string) => {
    const id = toastId++;
    setToasts((t) => [...t, { id, msg }]);
    setTimeout(() => setToasts((t) => t.filter((x) => x.id !== id)), 2800);
  }, []);
  return (
    <Ctx.Provider
      value={{
        tab,
        setTab,
        sidebarCollapsed,
        toggleSidebar: () => setCollapsed((v) => !v),
        paletteOpen,
        setPalette,
        deployModel,
        setDeployModel,
        mobileCompanionOpen,
        setMobileCompanionOpen,
        hitlOpen,
        setHitlOpen,
        hitlPendingCount,
        setHitlPendingCount,
        toasts,
        toast,
      }}
    >
      {children}
    </Ctx.Provider>
  );
}
export const useUI = () => {
  const v = useContext(Ctx);
  if (!v) throw new Error('no UIProvider');
  return v;
};
