import React, { createContext, useContext, useState, useRef, useEffect, useId } from 'react';

interface TabsContextValue {
  activeTab: string;
  onTabChange: (tab: string) => void;
  variant: 'line' | 'enclosed' | 'soft';
  orientation: 'horizontal' | 'vertical';
}

const TabsContext = createContext<TabsContextValue | null>(null);

function useTabsContext() {
  const context = useContext(TabsContext);
  if (!context) {
    throw new Error('Tabs components must be used within a Tabs.Root');
  }
  return context;
}

export interface TabsRootProps {
  defaultValue: string;
  value?: string;
  onChange?: (value: string) => void;
  variant?: 'line' | 'enclosed' | 'soft';
  orientation?: 'horizontal' | 'vertical';
  className?: string;
  children: React.ReactNode;
}

export const TabsRoot: React.FC<TabsRootProps> = ({
  defaultValue,
  value,
  onChange,
  variant = 'line',
  orientation = 'horizontal',
  className = '',
  children,
}) => {
  const [activeTab, setActiveTab] = useState(value || defaultValue);
  const isControlled = value !== undefined;

  const handleTabChange = (tab: string) => {
    if (!isControlled) {
      setActiveTab(tab);
    }
    onChange?.(tab);
  };

  return (
    <TabsContext.Provider value={{ activeTab, onTabChange: handleTabChange, variant, orientation }}>
      <div className={className} data-orientation={orientation}>
        {children}
      </div>
    </TabsContext.Provider>
  );
};

export interface TabsListProps extends React.HTMLAttributes<HTMLDivElement> {
  className?: string;
}

export const TabsList = forwardRef<HTMLDivElement, TabsListProps>(
  ({ className = '', children, ...props }, ref) => {
    const { variant, orientation } = useTabsContext();

    const variantStyles = {
      line: 'border-b border-[var(--border-subtle)]',
      enclosed: '',
      soft: '',
    };

    const orientationStyles = {
      horizontal: 'flex gap-1',
      vertical: 'flex flex-col gap-1',
    };

    return (
      <div
        ref={ref}
        role="tablist"
        aria-orientation={orientation}
        className={`
          ${orientationStyles[orientation]}
          ${variantStyles[variant]}
          ${className}
        `}
        {...props}
      >
        {children}
      </div>
    );
  }
);

TabsList.displayName = 'TabsList';

export interface TabsTriggerProps extends React.ButtonHTMLAttributes<HTMLButtonElement> {
  value: string;
  disabled?: boolean;
  className?: string;
}

export const TabsTrigger = forwardRef<HTMLButtonElement, TabsTriggerProps>(
  ({ value, disabled = false, className = '', children, ...props }, ref) => {
    const { activeTab, onTabChange, variant, orientation } = useTabsContext();
    const isActive = activeTab === value;

    const variantStyles = {
      line: `
        border-b-2
        ${isActive ? 'border-[var(--accent-primary)] text-[var(--accent-primary)]' : 'border-transparent text-[var(--text-tertiary)]'}
        hover:text-[var(--text-secondary)] hover:border-[var(--border-default)]
      `,
      enclosed: `
        rounded-[var(--radius-button-md)]
        ${isActive ? 'bg-[var(--bg-surface-elevated)] text-[var(--text-primary)] shadow-[var(--shadow-component-card)]' : 'bg-transparent text-[var(--text-tertiary)]'}
        hover:bg-[var(--bg-surface-hover)] hover:text-[var(--text-secondary)]
      `,
      soft: `
        rounded-[var(--radius-button-md)]
        ${isActive ? 'bg-[var(--accent-primary-subtle)] text-[var(--accent-primary)]' : 'bg-transparent text-[var(--text-tertiary)]'}
        hover:bg-[var(--bg-surface-hover)] hover:text-[var(--text-secondary)]
      `,
    };

    const orientationStyles = {
      horizontal: 'px-4 py-2 whitespace-nowrap',
      vertical: 'px-3 py-2.5 text-start w-full',
    };

    return (
      <button
        ref={ref}
        role="tab"
        aria-selected={isActive}
        aria-controls={`${value}-panel`}
        id={`${value}-trigger`}
        tabIndex={isActive ? 0 : -1}
        disabled={disabled}
        onClick={() => !disabled && onTabChange(value)}
        onKeyDown={(e) => {
          if (disabled) return;
          const tabs = Array.from(document.querySelectorAll('[role="tab"]:not([disabled])'));
          const currentIndex = tabs.indexOf(e.currentTarget);
          let nextIndex = currentIndex;

          if (orientation === 'horizontal') {
            if (e.key === 'ArrowRight') nextIndex = (currentIndex + 1) % tabs.length;
            if (e.key === 'ArrowLeft') nextIndex = (currentIndex - 1 + tabs.length) % tabs.length;
          } else {
            if (e.key === 'ArrowDown') nextIndex = (currentIndex + 1) % tabs.length;
            if (e.key === 'ArrowUp') nextIndex = (currentIndex - 1 + tabs.length) % tabs.length;
          }

          if (nextIndex !== currentIndex) {
            e.preventDefault();
            (tabs[nextIndex] as HTMLButtonElement).focus();
            onTabChange(tabs[nextIndex].getAttribute('value') || '');
          }
          if (e.key === 'Home') {
            e.preventDefault();
            (tabs[0] as HTMLButtonElement).focus();
            onTabChange(tabs[0].getAttribute('value') || '');
          }
          if (e.key === 'End') {
            e.preventDefault();
            (tabs[tabs.length - 1] as HTMLButtonElement).focus();
            onTabChange(tabs[tabs.length - 1].getAttribute('value') || '');
          }
        }}
        className={`
          ${variantStyles[variant]}
          ${orientationStyles[orientation]}
          text-[var(--font-size-sm)]
          font-medium
          transition-all duration-fast
          focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-[var(--focus-ring)] focus-visible:ring-offset-2 focus-visible:ring-offset-[var(--bg-base)]
          disabled:opacity-50 disabled:cursor-not-allowed
          ${className}
        `}
        {...props}
      >
        {children}
      </button>
    );
  }
);

TabsTrigger.displayName = 'TabsTrigger';

export interface TabsContentProps extends React.HTMLAttributes<HTMLDivElement> {
  value: string;
  className?: string;
  forceMount?: boolean;
}

export const TabsContent = forwardRef<HTMLDivElement, TabsContentProps>(
  ({ value, className = '', forceMount = false, children, ...props }, ref) => {
    const { activeTab, orientation } = useTabsContext();
    const isActive = activeTab === value;
    const contentId = useId();

    if (!forceMount && !isActive) {
      return null;
    }

    return (
      <div
        ref={ref}
        role="tabpanel"
        id={`${value}-panel`}
        aria-labelledby={`${value}-trigger`}
        hidden={!isActive}
        tabIndex={0}
        className={`
          ${orientation === 'vertical' ? '' : 'animate-fade-in'}
          ${className}
        `}
        {...props}
      >
        {children}
      </div>
    );
  }
);

TabsContent.displayName = 'TabsContent';

export const Tabs = Object.assign(TabsRoot, {
  List: TabsList,
  Trigger: TabsTrigger,
  Content: TabsContent,
});