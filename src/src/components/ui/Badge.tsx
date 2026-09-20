import React, { forwardRef } from 'react';

export interface BadgeProps extends React.HTMLAttributes<HTMLSpanElement> {
  variant?: 'default' | 'success' | 'warning' | 'error' | 'info' | 'neutral';
  size?: 'sm' | 'md' | 'lg';
  dot?: boolean;
  removable?: boolean;
  onRemove?: () => void;
}

const baseStyles = `
  inline-flex items-center gap-1.5
  font-medium
  rounded-[var(--radius-badge)]
  transition-colors duration-fast
  border border-transparent
`;

const variantStyles = {
  default: `
    bg-[var(--accent-primary-subtle)]
    text-[var(--accent-primary)]
    border-[var(--accent-primary)]/30
  `,
  success: `
    bg-[var(--accent-success-subtle)]
    text-[var(--accent-success)]
    border-[var(--accent-success)]/30
  `,
  warning: `
    bg-[var(--accent-warning-subtle)]
    text-[var(--accent-warning)]
    border-[var(--accent-warning)]/30
  `,
  error: `
    bg-[var(--accent-error-subtle)]
    text-[var(--accent-error)]
    border-[var(--accent-error)]/30
  `,
  info: `
    bg-[var(--accent-info-subtle)]
    text-[var(--accent-info)]
    border-[var(--accent-info)]/30
  `,
  neutral: `
    bg-[var(--bg-surface-elevated)]
    text-[var(--text-secondary)]
    border-[var(--border-default)]
  `,
};

const sizeStyles = {
  sm: 'px-2 py-0.5 text-[var(--font-size-2xs)] gap-1',
  md: 'px-2.5 py-1 text-[var(--font-size-xs)] gap-1.5',
  lg: 'px-3 py-1.5 text-[var(--font-size-sm)] gap-2',
};

const dotStyles = {
  default: 'bg-[var(--accent-primary)]',
  success: 'bg-[var(--accent-success)]',
  warning: 'bg-[var(--accent-warning)]',
  error: 'bg-[var(--accent-error)]',
  info: 'bg-[var(--accent-info)]',
  neutral: 'bg-[var(--text-tertiary)]',
};

export const Badge = forwardRef<HTMLSpanElement, BadgeProps>(
  (
    {
      variant = 'default',
      size = 'md',
      dot = false,
      removable = false,
      onRemove,
      className = '',
      children,
      ...props
    },
    ref
  ) => {
    return (
      <span
        ref={ref}
        className={`
          ${baseStyles}
          ${variantStyles[variant]}
          ${sizeStyles[size]}
          ${className}
        `}
        {...props}
      >
        {dot && <span className={`w-1.5 h-1.5 rounded-full ${dotStyles[variant]}`} aria-hidden="true" />}
        {children}
        {removable && (
          <button
            type="button"
            onClick={onRemove}
            className={`
              ml-1 p-0.5 rounded-full
              hover:bg-black/10 hover:bg-white/10
              focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-[var(--focus-ring)]
              transition-colors duration-fastest
              -mr-0.5
            `}
            aria-label="Remove"
          >
            <svg className="w-3 h-3" fill="none" stroke="currentColor" viewBox="0 0 24 24" aria-hidden="true">
              <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M6 18L18 6M6 6l12 12" />
            </svg>
          </button>
        )}
      </span>
    );
  }
);

Badge.displayName = 'Badge';

export interface StatusBadgeProps extends Omit<BadgeProps, 'variant'> {
  status: 'online' | 'offline' | 'busy' | 'away' | 'pending' | 'running' | 'success' | 'error' | 'warning';
}

const statusVariantMap: Record<StatusBadgeProps['status'], BadgeProps['variant']> = {
  online: 'success',
  offline: 'neutral',
  busy: 'error',
  away: 'warning',
  pending: 'info',
  running: 'info',
  success: 'success',
  error: 'error',
  warning: 'warning',
};

const statusDotMap: Record<StatusBadgeProps['status'], boolean> = {
  online: true,
  offline: true,
  busy: true,
  away: true,
  pending: false,
  running: false,
  success: false,
  error: false,
  warning: false,
};

export const StatusBadge = forwardRef<HTMLSpanElement, StatusBadgeProps>(
  ({ status, className = '', children, ...props }, ref) => {
    const variant = statusVariantMap[status];
    const dot = statusDotMap[status];

    return (
      <Badge
        ref={ref}
        variant={variant}
        dot={dot}
        className={className}
        {...props}
      >
        {children || status.charAt(0).toUpperCase() + status.slice(1)}
      </Badge>
    );
  }
);

StatusBadge.displayName = 'StatusBadge';