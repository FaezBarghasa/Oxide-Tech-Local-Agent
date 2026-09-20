import React, { forwardRef } from 'react';

export interface ButtonProps extends React.ButtonHTMLAttributes<HTMLButtonElement> {
  variant?: 'primary' | 'secondary' | 'ghost' | 'danger' | 'link';
  size?: 'sm' | 'md' | 'lg';
  loading?: boolean;
  icon?: React.ReactNode;
  iconPosition?: 'start' | 'end';
  fullWidth?: boolean;
}

const baseStyles = `
  inline-flex items-center justify-center font-medium
  transition-colors duration-fast
  focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-offset-2
  disabled:opacity-50 disabled:cursor-not-allowed
  whitespace-nowrap
`;

const variantStyles = {
  primary: `
    bg-[var(--accent-primary-subtle)] text-[var(--accent-primary)]
    border border-[var(--accent-primary)]/30
    hover:bg-[var(--accent-primary)]/25
    hover:border-[var(--accent-primary)]/50
    active:bg-[var(--accent-primary)]/35
    focus-visible:ring-[var(--focus-ring)]
  `,
  secondary: `
    bg-[var(--bg-surface-elevated)] text-[var(--text-primary)]
    border border-[var(--border-default)]
    hover:bg-[var(--bg-surface-hover)]
    hover:border-[var(--border-focus)]
    active:bg-[var(--bg-surface-active)]
    focus-visible:ring-[var(--focus-ring)]
  `,
  ghost: `
    bg-transparent text-[var(--text-secondary)]
    hover:bg-[var(--bg-surface-hover)]
    hover:text-[var(--text-primary)]
    active:bg-[var(--bg-surface-active)]
    focus-visible:ring-[var(--focus-ring)]
  `,
  danger: `
    bg-[var(--accent-error-subtle)] text-[var(--accent-error)]
    border border-[var(--accent-error)]/30
    hover:bg-[var(--accent-error)]/25
    hover:border-[var(--accent-error)]/50
    active:bg-[var(--accent-error)]/35
    focus-visible:ring-[var(--accent-error)]
  `,
  link: `
    bg-transparent text-[var(--text-link)]
    hover:text-[var(--text-link-hover)]
    hover:underline
    active:text-[var(--text-link-hover)]
    focus-visible:ring-[var(--focus-ring)]
    p-0
  `,
};

const sizeStyles = {
  sm: 'px-3 py-1.5 text-xs gap-1.5',
  md: 'px-4 py-2 text-sm gap-2',
  lg: 'px-6 py-3 text-base gap-2.5',
};

const radiusStyles = {
  sm: 'rounded-[var(--radius-button-sm)]',
  md: 'rounded-[var(--radius-button-md)]',
  lg: 'rounded-[var(--radius-button-lg)]',
};

export const Button = forwardRef<HTMLButtonElement, ButtonProps>(
  (
    {
      variant = 'primary',
      size = 'md',
      loading = false,
      icon,
      iconPosition = 'start',
      fullWidth = false,
      className = '',
      disabled,
      children,
      ...props
    },
    ref
  ) => {
    const isDisabled = disabled || loading;

    return (
      <button
        ref={ref}
        className={`
          ${baseStyles}
          ${variantStyles[variant]}
          ${sizeStyles[size]}
          ${radiusStyles.md}
          ${fullWidth ? 'w-full' : ''}
          ${className}
        `}
        disabled={isDisabled}
        aria-busy={loading}
        aria-disabled={isDisabled}
        {...props}
      >
        {loading && (
          <svg
            className="animate-spin h-4 w-4"
            xmlns="http://www.w3.org/2000/svg"
            fill="none"
            viewBox="0 0 24 24"
            aria-hidden="true"
          >
            <circle
              className="opacity-25"
              cx="12"
              cy="12"
              r="10"
              stroke="currentColor"
              strokeWidth="3"
            />
            <path
              className="opacity-75"
              fill="currentColor"
              d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z"
            />
          </svg>
        )}
        {!loading && icon && iconPosition === 'start' && <span aria-hidden="true">{icon}</span>}
        <span>{children}</span>
        {!loading && icon && iconPosition === 'end' && <span aria-hidden="true">{icon}</span>}
      </button>
    );
  }
);

Button.displayName = 'Button';

export interface IconButtonProps extends React.ButtonHTMLAttributes<HTMLButtonElement> {
  variant?: 'primary' | 'secondary' | 'ghost' | 'danger';
  size?: 'sm' | 'md' | 'lg';
  loading?: boolean;
  'aria-label': string;
  children: React.ReactNode;
}

export const IconButton = forwardRef<HTMLButtonElement, IconButtonProps>(
  (
    {
      variant = 'ghost',
      size = 'md',
      loading = false,
      'aria-label': ariaLabel,
      className = '',
      disabled,
      children,
      ...props
    },
    ref
  ) => {
    const isDisabled = disabled || loading;

    const sizeMap = {
      sm: 'p-1.5',
      md: 'p-2',
      lg: 'p-3',
    };

    return (
      <button
        ref={ref}
        className={`
          ${baseStyles}
          ${variantStyles[variant]}
          ${sizeMap[size]}
          ${radiusStyles.md}
          ${className}
        `}
        disabled={isDisabled}
        aria-busy={loading}
        aria-disabled={isDisabled}
        aria-label={ariaLabel}
        {...props}
      >
        {loading ? (
          <svg
            className="animate-spin h-4 w-4"
            xmlns="http://www.w3.org/2000/svg"
            fill="none"
            viewBox="0 0 24 24"
            aria-hidden="true"
          >
            <circle
              className="opacity-25"
              cx="12"
              cy="12"
              r="10"
              stroke="currentColor"
              strokeWidth="3"
            />
            <path
              className="opacity-75"
              fill="currentColor"
              d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z"
            />
          </svg>
        ) : (
          children
        )}
      </button>
    );
  }
);

IconButton.displayName = 'IconButton';