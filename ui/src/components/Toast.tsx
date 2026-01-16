/**
 * Toast Notification Component
 *
 * Displays transient notification messages with auto-dismiss.
 * Supports success, error, warning, and info variants.
 *
 * Per SR-PLAN-V7 Phase V7-2: Error Handling & UX Feedback
 */

import { useEffect } from 'react';
import styles from './Toast.module.css';

export interface Toast {
  id: string;
  type: 'success' | 'error' | 'warning' | 'info';
  message: string;
  duration?: number; // ms, default 5000
}

interface ToastItemProps {
  toast: Toast;
  onDismiss: (id: string) => void;
}

function ToastItem({ toast, onDismiss }: ToastItemProps): JSX.Element {
  const { id, type, message, duration = 5000 } = toast;

  useEffect(() => {
    if (duration > 0) {
      const timer = setTimeout(() => {
        onDismiss(id);
      }, duration);
      return () => clearTimeout(timer);
    }
  }, [id, duration, onDismiss]);

  // Use role="alert" for errors (assertive), role="status" for others (polite)
  const role = type === 'error' ? 'alert' : 'status';
  const ariaLive = type === 'error' ? 'assertive' : 'polite';

  return (
    <div
      className={`${styles.toast} ${styles[type]}`}
      role={role}
      aria-live={ariaLive}
      onClick={() => onDismiss(id)}
    >
      <span className={styles.icon}>{getIcon(type)}</span>
      <span className={styles.message}>{message}</span>
      <button
        className={styles.dismiss}
        onClick={(e) => {
          e.stopPropagation();
          onDismiss(id);
        }}
        aria-label="Dismiss notification"
      >
        &times;
      </button>
    </div>
  );
}

function getIcon(type: Toast['type']): string {
  switch (type) {
    case 'success':
      return '\u2713'; // checkmark
    case 'error':
      return '\u2717'; // x mark
    case 'warning':
      return '\u26A0'; // warning triangle
    case 'info':
      return '\u2139'; // info circle
  }
}

interface ToastContainerProps {
  toasts: Toast[];
  onDismiss: (id: string) => void;
}

export function ToastContainer({ toasts, onDismiss }: ToastContainerProps): JSX.Element | null {
  if (toasts.length === 0) {
    return null;
  }

  return (
    <div className={styles.container}>
      {toasts.map((toast) => (
        <ToastItem key={toast.id} toast={toast} onDismiss={onDismiss} />
      ))}
    </div>
  );
}

export default ToastContainer;
