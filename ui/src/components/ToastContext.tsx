/**
 * Toast Context Provider
 *
 * Provides app-wide toast notification state management.
 * Use the useToast() hook to access toast functions from any component.
 *
 * Per SR-PLAN-V7 Phase V7-2: Error Handling & UX Feedback
 */

import { createContext, useContext, useState, useCallback, useMemo, type ReactNode } from 'react';
import { ToastContainer, type Toast } from './Toast';

interface ToastContextValue {
  toasts: Toast[];
  addToast: (toast: Omit<Toast, 'id'>) => string;
  removeToast: (id: string) => void;
  success: (message: string) => void;
  error: (message: string) => void;
  warning: (message: string) => void;
  info: (message: string) => void;
}

const ToastContext = createContext<ToastContextValue | null>(null);

let toastIdCounter = 0;

function generateId(): string {
  toastIdCounter += 1;
  return `toast-${toastIdCounter}-${Date.now()}`;
}

interface ToastProviderProps {
  children: ReactNode;
}

export function ToastProvider({ children }: ToastProviderProps): JSX.Element {
  const [toasts, setToasts] = useState<Toast[]>([]);

  const removeToast = useCallback((id: string) => {
    setToasts((current) => current.filter((t) => t.id !== id));
  }, []);

  const addToast = useCallback((toast: Omit<Toast, 'id'>): string => {
    const id = generateId();
    const newToast: Toast = { ...toast, id };
    setToasts((current) => [...current, newToast]);
    return id;
  }, []);

  const success = useCallback(
    (message: string) => {
      addToast({ type: 'success', message });
    },
    [addToast]
  );

  const error = useCallback(
    (message: string) => {
      addToast({ type: 'error', message });
    },
    [addToast]
  );

  const warning = useCallback(
    (message: string) => {
      addToast({ type: 'warning', message });
    },
    [addToast]
  );

  const info = useCallback(
    (message: string) => {
      addToast({ type: 'info', message });
    },
    [addToast]
  );

  const value = useMemo<ToastContextValue>(
    () => ({
      toasts,
      addToast,
      removeToast,
      success,
      error,
      warning,
      info,
    }),
    [toasts, addToast, removeToast, success, error, warning, info]
  );

  return (
    <ToastContext.Provider value={value}>
      {children}
      <ToastContainer toasts={toasts} onDismiss={removeToast} />
    </ToastContext.Provider>
  );
}

export function useToast(): ToastContextValue {
  const context = useContext(ToastContext);
  if (!context) {
    throw new Error('useToast must be used within a ToastProvider');
  }
  return context;
}

export default ToastProvider;
