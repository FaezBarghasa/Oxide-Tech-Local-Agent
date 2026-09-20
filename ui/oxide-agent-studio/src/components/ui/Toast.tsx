import React, { createContext, useContext, useState, useCallback, useEffect, useId } from 'react';
import { createPortal } from 'react-dom';
import { X, CheckCircle, AlertCircle, AlertTriangle, Info } from 'lucide-react';

interface Toast {
  id: string;
  type: 'success' | 'error' | 'warning' | 'info';
  title: string;
  description?: string;
  duration?: number;
  action?: { label: string; onClick: () => void };
}

interface ToastContextValue {
  toasts: Toast[];
  addToast: (toast: Omit<Toast, 'id'>) => string;
  removeToast: (id: string) => void;
}

const ToastContext = createContext<ToastContextValue | null>(null);

export function ToastProvider({ children }: { children: React.ReactNode }) {
  const [toasts, setToasts] = useState<Toast[]>([]);

  const addToast = useCallback((toast: Omit<Toast, 'id'>) => {
    const id = `toast-${Date.now()}-${Math.random().toString(36).substr(2, 9)}`;
    const newToast = { ...toast, id };
    setToasts((prev) => [...prev, newToast]);
    return id;
  }, []);

  const removeToast = useCallback((id: string) => {
    setToasts((prev) => prev.filter((t) => t.id !== id));
  }, []);

  return (
    <ToastContext.Provider value={{ toasts, addToast, removeToast }}>
      {children}
      <ToastViewport />
    </ToastContext.Provider>
  );
}

function useToast() {
  const context = useContext(ToastContext);
  if (!context) {
    throw new Error('useToast must be used within a ToastProvider');
  }
  return context;
}

export function useToasts() {
  const { toasts } = useToast();
  return toasts;
}

export function useAddToast() {
  const { addToast } = useToast();
  return addToast;
}

export function useRemoveToast() {
  const { removeToast } = useToast();
  return removeToast;
}

function ToastViewport() {
  const { toasts, removeToast } = useToast();

  return createPortal(
    <div
      className="fixed bottom-4 right-4 z-[var(--z-overlay-toast)] flex flex-col gap-2 pointer-events-none"
      aria-live="polite"
      aria-label="Notifications"
    >
      {toasts.map((toast) => (
        <ToastItem key={toast.id} toast={toast} onClose={() => removeToast(toast.id)} />
      ))}
    </div>,
    document.body
  );
}

interface ToastItemProps {
  toast: Toast;
  onClose: () => void;
}

const ToastItem: React.FC<ToastItemProps> = ({ toast, onClose }) => {
  const toastId = useId();
  const [visible, setVisible] = useState(true);

  useEffect(() => {
    const duration = toast.duration ?? 5000;
    if (duration > 0) {
      const timer = setTimeout(() => {
        setVisible(false);
        setTimeout(() => onClose(), 200);
      }, duration);
      return () => clearTimeout(timer);
    }
  }, [toast, onClose]);

  if (!visible) return null;

  const iconMap = {
    success: <CheckCircle className="w-5 h-5 text-[var(--accent-success)]" aria-hidden="true" />,
    error: <AlertCircle className="w-5 h-5 text-[var(--accent-error)]" aria-hidden="true" />,
    warning: <AlertTriangle className="w-5 h-5 text-[var(--accent-warning)]" aria-hidden="true" />,
    info: <Info className="w-5 h-5 text-[var(--accent-info)]" aria-hidden="true" />,
  };

  const bgMap = {
    success: 'bg-[var(--accent-success-subtle)] border-[var(--accent-success)]/30',
    error: 'bg-[var(--accent-error-subtle)] border-[var(--accent-error)]/30',
    warning: 'bg-[var(--accent-warning-subtle)] border-[var(--accent-warning)]/30',
    info: 'bg-[var(--accent-info-subtle)] border-[var(--accent-info)]/30',
  };

  return (
    <div
      id={toastId}
      role="alert"
      className={`
        pointer-events-auto
        flex items-start gap-3
        min-w-[300px] max-w-[420px]
        p-4
        rounded-[var(--radius-card-md)]
        border
        shadow-[var(--shadow-component-toast)]
        animate-slide-up
        ${bgMap[toast.type]}
      `}
    >
      <div className="flex-shrink-0 mt-0.5">{iconMap[toast.type]}</div>
      <div className="flex-1 min-w-0">
        <p className="font-medium text-[var(--text-primary)] text-[var(--font-size-sm)]">{toast.title}</p>
        {toast.description && (
          <p className="text-[var(--text-secondary)] text-[var(--font-size-xs)] mt-1">{toast.description}</p>
        )}
        {toast.action && (
          <button
            onClick={() => {
              toast.action.onClick();
              onClose();
            }}
            className="mt-2 text-[var(--font-size-xs)] font-medium underline hover:no-underline focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-[var(--focus-ring)] rounded"
          >
            {toast.action.label}
          </button>
        )}
      </div>
      <button
        onClick={onClose}
        className="flex-shrink-0 p-1 text-[var(--text-tertiary)] hover:text-[var(--text-primary)] transition-colors duration-fastest rounded focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-[var(--focus-ring)]"
        aria-label="Dismiss"
      >
        <X className="w-4 h-4" aria-hidden="true" />
      </button>
    </div>
  );
}

export function toast(toast: Omit<Toast, 'id'>) {
  if (typeof window === 'undefined') return;
  const event = new CustomEvent('oxide:toast', { detail: toast });
  window.dispatchEvent(event);
}

toast.success = (title: string, options?: Partial<Toast>) =>
  toast({ type: 'success', title, ...options });
toast.error = (title: string, options?: Partial<Toast>) =>
  toast({ type: 'error', title, ...options });
toast.warning = (title: string, options?: Partial<Toast>) =>
  toast({ type: 'warning', title, ...options });
toast.info = (title: string, options?: Partial<Toast>) =>
  toast({ type: 'info', title, ...options });

if (typeof window !== 'undefined') {
  window.addEventListener('oxide:toast', (event: CustomEvent<Omit<Toast, 'id'>>) => {
    const root = document.querySelector('[data-toast-root]');
    if (root && root._reactRootContainer) {
      const fiber = root._reactRootContainer.current;
      if (fiber) {
        const provider = fiber.memoizedState;
        if (provider?.addToast) {
          provider.addToast(event.detail);
        }
      }
    }
  });
}