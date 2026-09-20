import React, { forwardRef } from 'react';

export interface CardProps extends React.HTMLAttributes<HTMLDivElement> {
  variant?: 'default' | 'elevated' | 'outlined' | 'interactive';
  padding?: 'none' | 'sm' | 'md' | 'lg' | 'xl';
  hoverable?: boolean;
}

const baseStyles = `
  rounded-[var(--radius-card-md)]
  transition-all duration-normal
`;

const variantStyles = {
  default: `
    bg-[var(--bg-surface-elevated)]
    border border-[var(--border-subtle)]
    shadow-[var(--shadow-component-card)]
  `,
  elevated: `
    bg-[var(--bg-surface-elevated)]
    border border-[var(--border-subtle)]
    shadow-[var(--shadow-component-card-hover)]
  `,
  outlined: `
    bg-[var(--bg-surface)]
    border border-[var(--border-default)]
  `,
  interactive: `
    bg-[var(--bg-surface-elevated)]
    border border-[var(--border-subtle)]
    shadow-[var(--shadow-component-card)]
    hover:bg-[var(--bg-surface-hover)]
    hover:border-[var(--border-focus)]
    hover:shadow-[var(--shadow-component-card-hover)]
    cursor-pointer
  `,
};

const paddingStyles = {
  none: '',
  sm: 'p-[var(--space-component-sm)]',
  md: 'p-[var(--space-component-md)]',
  lg: 'p-[var(--space-component-lg)]',
  xl: 'p-[var(--space-component-xl)]',
};

export const Card = forwardRef<HTMLDivElement, CardProps>(
  (
    {
      variant = 'default',
      padding = 'md',
      hoverable = false,
      className = '',
      children,
      ...props
    },
    ref
  ) => {
    const effectiveVariant = hoverable && variant !== 'interactive' ? 'interactive' : variant;

    return (
      <div
        ref={ref}
        className={`
          ${baseStyles}
          ${variantStyles[effectiveVariant]}
          ${paddingStyles[padding]}
          ${className}
        `}
        {...props}
      >
        {children}
      </div>
    );
  }
);

Card.displayName = 'Card';

export interface CardHeaderProps extends React.HTMLAttributes<HTMLDivElement> {}

export const CardHeader = forwardRef<HTMLDivElement, CardHeaderProps>(
  ({ className = '', children, ...props }, ref) => (
    <div
      ref={ref}
      className={`flex items-center justify-between gap-4 ${className}`}
      {...props}
    >
      {children}
    </div>
  )
);

CardHeader.displayName = 'CardHeader';

export interface CardTitleProps extends React.HTMLAttributes<HTMLHeadingElement> {
  as?: 'h1' | 'h2' | 'h3' | 'h4';
}

export const CardTitle = forwardRef<HTMLHeadingElement, CardTitleProps>(
  ({ as: Component = 'h3', className = '', children, ...props }, ref) => (
    <Component
      ref={ref}
      className={`font-semibold text-[var(--text-primary)] text-[var(--font-size-xl)] ${className}`}
      {...props}
    >
      {children}
    </Component>
  )
);

CardTitle.displayName = 'CardTitle';

export interface CardDescriptionProps extends React.HTMLAttributes<HTMLParagraphElement> {}

export const CardDescription = forwardRef<HTMLParagraphElement, CardDescriptionProps>(
  ({ className = '', children, ...props }, ref) => (
    <p
      ref={ref}
      className={`text-[var(--text-secondary)] text-[var(--font-size-sm)] ${className}`}
      {...props}
    >
      {children}
    </p>
  )
);

CardDescription.displayName = 'CardDescription';

export interface CardContentProps extends React.HTMLAttributes<HTMLDivElement> {}

export const CardContent = forwardRef<HTMLDivElement, CardContentProps>(
  ({ className = '', children, ...props }, ref) => (
    <div ref={ref} className={`pt-4 ${className}`} {...props}>
      {children}
    </div>
  )
);

CardContent.displayName = 'CardContent';

export interface CardFooterProps extends React.HTMLAttributes<HTMLDivElement> {}

export const CardFooter = forwardRef<HTMLDivElement, CardFooterProps>(
  ({ className = '', children, ...props }, ref) => (
    <div
      ref={ref}
      className={`flex items-center gap-3 pt-4 border-t border-[var(--border-subtle)] ${className}`}
      {...props}
    >
      {children}
    </div>
  )
);

CardFooter.displayName = 'CardFooter';