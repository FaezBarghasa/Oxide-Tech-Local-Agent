import React, { useState, useRef, useEffect, useId, useCallback } from 'react';
import { createPortal } from 'react-dom';

interface DropdownContextValue {
  open: boolean;
  onOpenChange: (open: boolean) => void;
  triggerRef: React.RefObject<HTMLButtonElement>;
  contentRef: React.RefObject<HTMLDivElement>;
  value: string;
  onValueChange: (value: string) => void;
  disabled: boolean;
}

const DropdownContext = createContext<DropdownContextValue | null>(null);

function useDropdownContext() {
  const context = useContext(DropdownContext);
  if (!context) {
    throw new Error('Dropdown components must be used within a Dropdown.Root');
  }
  return context;
}

export interface DropdownRootProps {
  value: string;
  onValueChange: (value: string) => void;
  defaultOpen?: boolean;
  disabled?: boolean;
  className?: string;
  children: React.ReactNode;
}

export const DropdownRoot: React.FC<DropdownRootProps> = ({
  value,
  onValueChange,
  defaultOpen = false,
  disabled = false,
  className = '',
  children,
}) => {
  const [open, setOpen] = useState(defaultOpen);
  const triggerRef = useRef<HTMLButtonElement>(null);
  const contentRef = useRef<HTMLDivElement>(null);

  const handleOpenChange = useCallback((newOpen: boolean) => {
    if (!disabled) {
      setOpen(newOpen);
    }
  }, [disabled]);

  useEffect(() => {
    const handleClickOutside = (event: MouseEvent) => {
      if (open && triggerRef.current && contentRef.current) {
        if (!triggerRef.current.contains(event.target as Node) && !contentRef.current.contains(event.target as Node)) {
          setOpen(false);
        }
      }
    };

    document.addEventListener('mousedown', handleClickOutside);
    return () => document.removeEventListener('mousedown', handleClickOutside);
  }, [open]);

  useEffect(() => {
    const handleEscape = (event: KeyboardEvent) => {
      if (event.key === 'Escape' && open) {
        setOpen(false);
        triggerRef.current?.focus();
      }
    };

    document.addEventListener('keydown', handleEscape);
    return () => document.removeEventListener('keydown', handleEscape);
  }, [open]);

  return (
    <DropdownContext.Provider value={{ open, onOpenChange: handleOpenChange, triggerRef, contentRef, value, onValueChange, disabled }}>
      <div className={className} data-disabled={disabled}>
        {children}
      </div>
    </DropdownContext.Provider>
  );
};

export interface DropdownTriggerProps extends React.ButtonHTMLAttributes<HTMLButtonElement> {
  className?: string;
  children: React.ReactNode;
}

export const DropdownTrigger = forwardRef<HTMLButtonElement, DropdownTriggerProps>(
  ({ className = '', children, ...props }, ref) => {
    const { open, disabled, triggerRef, onOpenChange } = useDropdownContext();

    return (
      <button
        ref={(el) => {
          triggerRef.current = el;
          if (typeof ref === 'function') ref(el);
          else if (ref) ref.current = el;
        }}
        type="button"
        disabled={disabled}
        aria-haspopup="listbox"
        aria-expanded={open}
        aria-controls="dropdown-content"
        onClick={() => onOpenChange(!open)}
        className={`
          inline-flex items-center justify-between gap-2
          w-full
          bg-[var(--bg-surface)]
          border border-[var(--border-subtle)]
          rounded-[var(--radius-input-md)]
          text-[var(--text-primary)]
          transition-colors duration-fast
          focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-[var(--focus-ring)]
          focus-visible:border-transparent
          disabled:opacity-50 disabled:cursor-not-allowed
          px-4 py-2.5
          text-[var(--font-size-sm)]
          text-start
          ${className}
        `}
        {...props}
      >
        <span className="flex-1 truncate">{children}</span>
        <svg
          className={`w-4 h-4 flex-shrink-0 text-[var(--text-tertiary)] transition-transform duration-fast ${open ? 'rotate-180' : ''}`}
          fill="none"
          stroke="currentColor"
          viewBox="0 0 24 24"
          aria-hidden="true"
        >
          <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M19 9l-7 7-7-7" />
        </svg>
      </button>
    );
  }
);

DropdownTrigger.displayName = 'DropdownTrigger';

export interface DropdownContentProps extends React.HTMLAttributes<HTMLDivElement> {
  className?: string;
  position?: 'bottom' | 'top';
  align?: 'start' | 'end';
  className?: string;
}

export const DropdownContent = forwardRef<HTMLDivElement, DropdownContentProps>(
  ({ className = '', position = 'bottom', align = 'start', children, ...props }, ref) => {
    const { open, contentRef, disabled } = useDropdownContext();

    if (!open || disabled) return null;

    const positionStyles = {
      bottom: 'top-full mt-1.5',
      top: 'bottom-full mb-1.5',
    };

    const alignStyles = {
      start: 'start-0',
      end: 'end-0',
    };

    const content = (
      <div
        ref={(el) => {
          contentRef.current = el;
          if (typeof ref === 'function') ref(el);
          else if (ref) ref.current = el;
        }}
        id="dropdown-content"
        role="listbox"
        aria-orientation="vertical"
        className={`
          fixed z-[var(--z-overlay-popover)]
          min-w-[200px] max-h-[300px] overflow-y-auto
          bg-[var(--bg-surface-elevated)]
          border border-[var(--border-default)]
          rounded-[var(--radius-popover)]
          shadow-[var(--shadow-component-dropdown)]
          animate-fade-in
          ${positionStyles[position]}
          ${alignStyles[align]}
          ${className}
        `}
        {...props}
      >
        {children}
      </div>
    );

    return createPortal(content, document.body);
  }
);

DropdownContent.displayName = 'DropdownContent';

export interface DropdownItemProps extends React.HTMLAttributes<HTMLDivElement> {
  value: string;
  disabled?: boolean;
  className?: string;
  children: React.ReactNode;
}

export const DropdownItem = forwardRef<HTMLDivElement, DropdownItemProps>(
  ({ value, disabled = false, className = '', children, ...props }, ref) => {
    const { value: currentValue, onValueChange, onOpenChange } = useDropdownContext();
    const isSelected = currentValue === value;
    const itemId = useId();

    const handleClick = () => {
      if (!disabled) {
        onValueChange(value);
        onOpenChange(false);
      }
    };

    const handleKeyDown = (event: React.KeyboardEvent) => {
      if (event.key === 'Enter' || event.key === ' ') {
        event.preventDefault();
        handleClick();
      }
    };

    return (
      <div
        ref={ref}
        role="option"
        id={itemId}
        aria-selected={isSelected}
        aria-disabled={disabled}
        tabIndex={isSelected ? 0 : -1}
        onClick={handleClick}
        onKeyDown={handleKeyDown}
        className={`
          flex items-center gap-3
          px-3 py-2
          text-[var(--font-size-sm)]
          transition-colors duration-fastest
          cursor-pointer
          ${isSelected ? 'bg-[var(--accent-primary-subtle)] text-[var(--accent-primary)]' : 'text-[var(--text-primary)] hover:bg-[var(--bg-surface-hover)]'}
          ${disabled ? 'opacity-50 cursor-not-allowed' : ''}
          ${className}
        `}
        {...props}
      >
        {children}
        {isSelected && (
          <svg className="w-4 h-4 flex-shrink-0" fill="currentColor" viewBox="0 0 20 20" aria-hidden="true">
            <path fillRule="evenodd" d="M16.707 5.293a1 1 0 010 1.414l-8 8a1 1 0 01-1.414 0l-4-4a1 1 0 011.414-1.414L8 12.586l7.293-7.293a1 1 0 011.414 0z" clipRule="evenodd" />
          </svg>
        )}
      </div>
    );
  }
);

DropdownItem.displayName = 'DropdownItem';

export interface DropdownSeparatorProps extends React.HTMLAttributes<HTMLHRElement> {}

export const DropdownSeparator = forwardRef<HTMLHRElement, DropdownSeparatorProps>(
  ({ className = '', ...props }, ref) => (
    <hr
      ref={ref}
      role="separator"
      className={`border-[var(--border-subtle)] my-1 ${className}`}
      {...props}
    />
  )
);

DropdownSeparator.displayName = 'DropdownSeparator';

export const Dropdown = Object.assign(DropdownRoot, {
  Trigger: DropdownTrigger,
  Content: DropdownContent,
  Item: DropdownItem,
  Separator: DropdownSeparator,
});