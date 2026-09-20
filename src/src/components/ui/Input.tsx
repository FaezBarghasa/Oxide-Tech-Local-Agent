import React, { forwardRef, useId } from 'react';

export interface InputProps extends React.InputHTMLAttributes<HTMLInputElement> {
  label?: string;
  error?: string;
  hint?: string;
  leadingIcon?: React.ReactNode;
  trailingIcon?: React.ReactNode;
  fullWidth?: boolean;
}

export const Input = forwardRef<HTMLInputElement, InputProps>(
  (
    {
      label,
      error,
      hint,
      leadingIcon,
      trailingIcon,
      fullWidth = false,
      className = '',
      id: providedId,
      disabled,
      required,
      'aria-describedby': ariaDescribedBy,
      ...props
    },
    ref
  ) => {
    const generatedId = useId();
    const id = providedId || generatedId;
    const errorId = `${id}-error`;
    const hintId = `${id}-hint`;
    const describedBy = [error && errorId, hint && hintId, ariaDescribedBy].filter(Boolean).join(' ') || undefined;

    return (
      <div className={`${fullWidth ? 'w-full' : ''} ${className}`}>
        {label && (
          <label
            htmlFor={id}
            className="block text-[var(--font-size-sm)] font-medium text-[var(--text-primary)] mb-1.5"
          >
            {label}
            {required && <span className="text-[var(--accent-error)] ml-1" aria-hidden="true">*</span>}
          </label>
        )}
        <div className="relative">
          {leadingIcon && (
            <div
              className="absolute inset-y-0 start-0 flex items-center pl-3 pointer-events-none text-[var(--text-tertiary)]"
              aria-hidden="true"
            >
              {leadingIcon}
            </div>
          )}
          <input
            ref={ref}
            id={id}
            className={`
              w-full
              bg-[var(--bg-surface)]
              border border-[var(--border-subtle)]
              rounded-[var(--radius-input-md)]
              text-[var(--text-primary)]
              placeholder:text-[var(--text-tertiary)]
              transition-colors duration-fast
              focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-[var(--focus-ring)]
              focus-visible:border-transparent
              disabled:opacity-50 disabled:cursor-not-allowed
              ${leadingIcon ? 'pl-10' : 'pl-4'}
              ${trailingIcon ? 'pr-10' : 'pr-4'}
              py-2.5
              text-[var(--font-size-sm)]
            `}
            disabled={disabled}
            required={required}
            aria-invalid={error ? 'true' : 'false'}
            aria-describedby={describedBy}
            aria-required={required}
            {...props}
          />
          {trailingIcon && (
            <div
              className="absolute inset-y-0 end-0 flex items-center pr-3 pointer-events-none text-[var(--text-tertiary)]"
              aria-hidden="true"
            >
              {trailingIcon}
            </div>
          )}
        </div>
        {error && (
          <p
            id={errorId}
            className="mt-1.5 text-[var(--font-size-xs)] text-[var(--accent-error)] flex items-center gap-1"
            role="alert"
          >
            <svg className="w-3 h-3 flex-shrink-0" fill="currentColor" viewBox="0 0 20 20" aria-hidden="true">
              <path fillRule="evenodd" d="M18 10a8 8 0 11-16 0 8 8 0 0116 0zm-7 4a1 1 0 11-2 0 1 1 0 012 0zm-1-9a1 1 0 00-1 1v4a1 1 0 102 0V6a1 1 0 00-1-1z" clipRule="evenodd" />
            </svg>
            {error}
          </p>
        )}
        {hint && !error && (
          <p id={hintId} className="mt-1.5 text-[var(--font-size-xs)] text-[var(--text-tertiary)]">
            {hint}
          </p>
        )}
      </div>
    );
  }
);

Input.displayName = 'Input';

export interface TextareaProps extends React.TextareaHTMLAttributes<HTMLTextAreaElement> {
  label?: string;
  error?: string;
  hint?: string;
  fullWidth?: boolean;
  minRows?: number;
  maxRows?: number;
}

export const Textarea = forwardRef<HTMLTextAreaElement, TextareaProps>(
  (
    {
      label,
      error,
      hint,
      fullWidth = false,
      minRows = 3,
      maxRows = 10,
      className = '',
      id: providedId,
      disabled,
      required,
      'aria-describedby': ariaDescribedBy,
      ...props
    },
    ref
  ) => {
    const generatedId = useId();
    const id = providedId || generatedId;
    const errorId = `${id}-error`;
    const hintId = `${id}-hint`;
    const describedBy = [error && errorId, hint && hintId, ariaDescribedBy].filter(Boolean).join(' ') || undefined;

    return (
      <div className={`${fullWidth ? 'w-full' : ''} ${className}`}>
        {label && (
          <label
            htmlFor={id}
            className="block text-[var(--font-size-sm)] font-medium text-[var(--text-primary)] mb-1.5"
          >
            {label}
            {required && <span className="text-[var(--accent-error)] ml-1" aria-hidden="true">*</span>}
          </label>
        )}
        <textarea
          ref={ref}
          id={id}
          className={`
            w-full
            bg-[var(--bg-surface)]
            border border-[var(--border-subtle)]
            rounded-[var(--radius-input-md)]
            text-[var(--text-primary)]
            placeholder:text-[var(--text-tertiary)]
            transition-colors duration-fast
            focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-[var(--focus-ring)]
            focus-visible:border-transparent
            disabled:opacity-50 disabled:cursor-not-allowed
            p-3
            text-[var(--font-size-sm)]
            font-sans
            resize-y
            min-h-[${minRows * 1.5}rem]
            max-h-[${maxRows * 1.5}rem]
          `}
          disabled={disabled}
          required={required}
          aria-invalid={error ? 'true' : 'false'}
          aria-describedby={describedBy}
          aria-required={required}
          rows={minRows}
          {...props}
        />
        {error && (
          <p
            id={errorId}
            className="mt-1.5 text-[var(--font-size-xs)] text-[var(--accent-error)] flex items-center gap-1"
            role="alert"
          >
            <svg className="w-3 h-3 flex-shrink-0" fill="currentColor" viewBox="0 0 20 20" aria-hidden="true">
              <path fillRule="evenodd" d="M18 10a8 8 0 11-16 0 8 8 0 0116 0zm-7 4a1 1 0 11-2 0 1 1 0 012 0zm-1-9a1 1 0 00-1 1v4a1 1 0 102 0V6a1 1 0 00-1-1z" clipRule="evenodd" />
            </svg>
            {error}
          </p>
        )}
        {hint && !error && (
          <p id={hintId} className="mt-1.5 text-[var(--font-size-xs)] text-[var(--text-tertiary)]">
            {hint}
          </p>
        )}
      </div>
    );
  }
);

Textarea.displayName = 'Textarea';

export interface SelectProps extends React.SelectHTMLAttributes<HTMLSelectElement> {
  label?: string;
  error?: string;
  hint?: string;
  placeholder?: string;
  options: { value: string; label: string; disabled?: boolean }[];
  fullWidth?: boolean;
}

export const Select = forwardRef<HTMLSelectElement, SelectProps>(
  (
    {
      label,
      error,
      hint,
      placeholder,
      options,
      fullWidth = false,
      className = '',
      id: providedId,
      disabled,
      required,
      'aria-describedby': ariaDescribedBy,
      ...props
    },
    ref
  ) => {
    const generatedId = useId();
    const id = providedId || generatedId;
    const errorId = `${id}-error`;
    const hintId = `${id}-hint`;
    const describedBy = [error && errorId, hint && hintId, ariaDescribedBy].filter(Boolean).join(' ') || undefined;

    return (
      <div className={`${fullWidth ? 'w-full' : ''} ${className}`}>
        {label && (
          <label
            htmlFor={id}
            className="block text-[var(--font-size-sm)] font-medium text-[var(--text-primary)] mb-1.5"
          >
            {label}
            {required && <span className="text-[var(--accent-error)] ml-1" aria-hidden="true">*</span>}
          </label>
        )}
        <div className="relative">
          <select
            ref={ref}
            id={id}
            className={`
              w-full
              bg-[var(--bg-surface)]
              border border-[var(--border-subtle)]
              rounded-[var(--radius-input-md)]
              text-[var(--text-primary)]
              transition-colors duration-fast
              focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-[var(--focus-ring)]
              focus-visible:border-transparent
              disabled:opacity-50 disabled:cursor-not-allowed
              appearance-none
              pl-4 pr-10 py-2.5
              text-[var(--font-size-sm)]
            `}
            disabled={disabled}
            required={required}
            aria-invalid={error ? 'true' : 'false'}
            aria-describedby={describedBy}
            aria-required={required}
            {...props}
          >
            {placeholder && (
              <option value="" disabled>
                {placeholder}
              </option>
            )}
            {options.map((option) => (
              <option key={option.value} value={option.value} disabled={option.disabled}>
                {option.label}
              </option>
            ))}
          </select>
          <div className="absolute inset-y-0 end-0 flex items-center pr-3 pointer-events-none text-[var(--text-tertiary)]" aria-hidden="true">
            <svg className="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24" aria-hidden="true">
              <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M19 9l-7 7-7-7" />
            </svg>
          </div>
        </div>
        {error && (
          <p
            id={errorId}
            className="mt-1.5 text-[var(--font-size-xs)] text-[var(--accent-error)] flex items-center gap-1"
            role="alert"
          >
            <svg className="w-3 h-3 flex-shrink-0" fill="currentColor" viewBox="0 0 20 20" aria-hidden="true">
              <path fillRule="evenodd" d="M18 10a8 8 0 11-16 0 8 8 0 0116 0zm-7 4a1 1 0 11-2 0 1 1 0 012 0zm-1-9a1 1 0 00-1 1v4a1 1 0 102 0V6a1 1 0 00-1-1z" clipRule="evenodd" />
            </svg>
            {error}
          </p>
        )}
        {hint && !error && (
          <p id={hintId} className="mt-1.5 text-[var(--font-size-xs)] text-[var(--text-tertiary)]">
            {hint}
          </p>
        )}
      </div>
    );
  }
);

Select.displayName = 'Select';