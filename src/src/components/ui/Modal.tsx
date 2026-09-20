import React, { useEffect, useRef, useCallback } from 'react';
import { createPortal } from 'react-dom';
import { X } from 'lucide-react';

interface ModalContextValue {
  onClose: () => void;
}

const ModalContext = createContext<ModalContextValue | null>(null);

function useModalContext() {
  const context = useContext(ModalContext);
  if (!context) {
    throw new Error('Modal components must be used within a Modal.Root');
  }
  return context;
}

export interface ModalRootProps {
  open: boolean;
  onOpenChange: (open: boolean) => void;
  className?: string;
  children: React.ReactNode;
}

export const ModalRoot: React.FC<ModalRootProps> = ({ open, onOpenChange, className = '', children }) => {
  const handleClose = useCallback(() => onOpenChange(false), [onOpenChange]);

  if (!open) return null;

  return (
    <ModalContext.Provider value={{ onClose: handleClose }}>
      {createPortal(
        <div className={className}>
          <ModalOverlay onClick={handleClose} />
          <ModalContent>{children}</ModalContent>
        </div>,
        document.body
      )}
    </ModalContext.Provider>
  );
};

interface ModalOverlayProps {
  onClick: () => void;
}

const ModalOverlay: React.FC<ModalOverlayProps> = ({ onClick }) => (
  <div
    onClick={onClick}
    className="fixed inset-0 z-[var(--z-overlay-modalBackdrop)] bg-[var(--overlay-scrim)] animate-fade-in"
    aria-hidden="true"
  />
);

export interface ModalContentProps extends React.HTMLAttributes<HTMLDivElement> {
  size?: 'sm' | 'md' | 'lg' | 'xl' | 'full';
  className?: string;
}

export const ModalContent = forwardRef<HTMLDivElement, ModalContentProps>(
  ({ size = 'md', className = '', children, ...props }, ref) => {
    const sizeStyles = {
      sm: 'max-w-sm',
      md: 'max-w-md',
      lg: 'max-w-lg',
      xl: 'max-w-xl',
      full: 'max-w-[90vw]',
    };

    return (
      <div
        ref={ref}
        className={`
          fixed z-[var(--z-overlay-modal)]
          top-1/2 left-1/2 -translate-x-1/2 -translate-y-1/2
          w-full ${sizeStyles[size]} mx-4
          bg-[var(--bg-surface-elevated)]
          border border-[var(--border-default)]
          rounded-[var(--radius-modal)]
          shadow-[var(--shadow-component-modal)]
          animate-scale-in
          ${className}
        `}
        {...props}
      >
        {children}
      </div>
    );
  }
);

ModalContent.displayName = 'ModalContent';

export interface ModalHeaderProps extends React.HTMLAttributes<HTMLDivElement> {}

export const ModalHeader = forwardRef<HTMLDivElement, ModalHeaderProps>(
  ({ className = '', children, ...props }, ref) => {
    const { onClose } = useModalContext();

    return (
      <div
        ref={ref}
        className={`
          flex items-start justify-between gap-4
          p-4 pb-2
          border-b border-[var(--border-subtle)]
          ${className}
        `}
        {...props}
      >
        <div className="flex-1">{children}</div>
        <button
          type="button"
          onClick={onClose}
          className={`
            p-1.5 rounded-[var(--radius-button-sm)]
            text-[var(--text-tertiary)]
            hover:text-[var(--text-primary)]
            hover:bg-[var(--bg-surface-hover)]
            transition-colors duration-fastest
            focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-[var(--focus-ring)]
          `}
          aria-label="Close modal"
        >
          <X className="w-4 h-4" aria-hidden="true" />
        </button>
      </div>
    );
  }
);

ModalHeader.displayName = 'ModalHeader';

export interface ModalTitleProps extends React.HTMLAttributes<HTMLHeadingElement> {}

export const ModalTitle = forwardRef<HTMLHeadingElement, ModalTitleProps>(
  ({ className = '', children, ...props }, ref) => (
    <h2
      ref={ref}
      className={`text-[var(--font-size-lg)] font-semibold text-[var(--text-primary)] ${className}`}
      {...props}
    >
      {children}
    </h2>
  )
);

ModalTitle.displayName = 'ModalTitle';

export interface ModalDescriptionProps extends React.HTMLAttributes<HTMLParagraphElement> {}

export const ModalDescription = forwardRef<HTMLParagraphElement, ModalDescriptionProps>(
  ({ className = '', children, ...props }, ref) => (
    <p
      ref={ref}
      className={`text-[var(--font-size-sm)] text-[var(--text-secondary)] mt-1 ${className}`}
      {...props}
    >
      {children}
    </p>
  )
);

ModalDescription.displayName = 'ModalDescription';

export interface ModalBodyProps extends React.HTMLAttributes<HTMLDivElement> {}

export const ModalBody = forwardRef<HTMLDivElement, ModalBodyProps>(
  ({ className = '', children, ...props }, ref) => (
    <div ref={ref} className={`p-4 ${className}`} {...props}>
      {children}
    </div>
  )
);

ModalBody.displayName = 'ModalBody';

export interface ModalFooterProps extends React.HTMLAttributes<HTMLDivElement> {}

export const ModalFooter = forwardRef<HTMLDivElement, ModalFooterProps>(
  ({ className = '', children, ...props }, ref) => (
    <div
      ref={ref}
      className={`
        flex items-center justify-end gap-3
        p-4 pt-2
        border-t border-[var(--border-subtle)]
        ${className}
      `}
      {...props}
    >
      {children}
    </div>
  )
);

ModalFooter.displayName = 'ModalFooter';

export interface ModalCloseProps extends React.ButtonHTMLAttributes<HTMLButtonElement> {
  className?: string;
  children?: React.ReactNode;
}

export const ModalClose = forwardRef<HTMLButtonElement, ModalCloseProps>(
  ({ className = '', children = 'Close', ...props }, ref) => {
    const { onClose } = useModalContext();

    return (
      <button
        ref={ref}
        type="button"
        onClick={onClose}
        className={`
          px-4 py-2
          rounded-[var(--radius-button-md)]
          font-medium
          transition-colors duration-fast
          focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-[var(--focus-ring)]
          ${className}
        `}
        {...props}
      >
        {children}
      </button>
    );
  }
);

ModalClose.displayName = 'ModalClose';

export const Modal = Object.assign(ModalRoot, {
  Content: ModalContent,
  Header: ModalHeader,
  Title: ModalTitle,
  Description: ModalDescription,
  Body: ModalBody,
  Footer: ModalFooter,
  Close: ModalClose,
});