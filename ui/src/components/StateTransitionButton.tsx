/**
 * StateTransitionButton Component
 *
 * Button for triggering loop state transitions with confirmation dialogs.
 * Handles Activate, Pause, Resume, and Close actions.
 */

import { useState } from 'react';
import { Button } from '../ui';
import type { LoopState } from '../hooks/useLoops';
import styles from './StateTransitionButton.module.css';

type TransitionAction = 'activate' | 'pause' | 'resume' | 'close';

interface StateTransitionButtonProps {
  currentState: LoopState;
  onTransition: (action: TransitionAction) => Promise<void>;
  disabled?: boolean;
  compact?: boolean;
}

interface ActionConfig {
  action: TransitionAction;
  label: string;
  variant: 'primary' | 'secondary' | 'ghost';
  confirmTitle: string;
  confirmMessage: string;
  warning?: string;
}

const ACTION_CONFIGS: Record<TransitionAction, ActionConfig> = {
  activate: {
    action: 'activate',
    label: 'Activate',
    variant: 'primary',
    confirmTitle: 'Activate Loop',
    confirmMessage: 'This will start the loop and allow iterations to run.',
  },
  pause: {
    action: 'pause',
    label: 'Pause',
    variant: 'secondary',
    confirmTitle: 'Pause Loop',
    confirmMessage: 'This will pause the loop. It can be resumed later.',
  },
  resume: {
    action: 'resume',
    label: 'Resume',
    variant: 'primary',
    confirmTitle: 'Resume Loop',
    confirmMessage: 'This will resume the loop from where it was paused.',
  },
  close: {
    action: 'close',
    label: 'Close',
    variant: 'ghost',
    confirmTitle: 'Close Loop',
    confirmMessage: 'Are you sure you want to close this loop?',
    warning: 'This action cannot be undone. The loop will be permanently closed.',
  },
};

function getAvailableActions(state: LoopState): TransitionAction[] {
  switch (state) {
    case 'CREATED':
      return ['activate', 'close'];
    case 'ACTIVE':
      return ['pause', 'close'];
    case 'PAUSED':
      return ['resume', 'close'];
    case 'CLOSED':
      return [];
    default:
      return [];
  }
}

export function StateTransitionButton({
  currentState,
  onTransition,
  disabled = false,
  compact = false,
}: StateTransitionButtonProps): JSX.Element {
  const [showConfirm, setShowConfirm] = useState<TransitionAction | null>(null);
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);

  const availableActions = getAvailableActions(currentState);

  const handleClick = (action: TransitionAction) => {
    setError(null);
    setShowConfirm(action);
  };

  const handleConfirm = async () => {
    if (!showConfirm) return;

    setLoading(true);
    setError(null);

    try {
      await onTransition(showConfirm);
      setShowConfirm(null);
    } catch (err) {
      setError(err instanceof Error ? err.message : 'Action failed');
    } finally {
      setLoading(false);
    }
  };

  const handleCancel = () => {
    setShowConfirm(null);
    setError(null);
  };

  if (availableActions.length === 0) {
    return <span className={styles.noActions}>No actions available</span>;
  }

  const config = showConfirm ? ACTION_CONFIGS[showConfirm] : null;

  return (
    <div className={styles.container}>
      {/* Action buttons */}
      <div className={compact ? styles.compactButtons : styles.buttons}>
        {availableActions.map(action => {
          const actionConfig = ACTION_CONFIGS[action];
          return (
            <Button
              key={action}
              variant={actionConfig.variant}
              onClick={() => handleClick(action)}
              disabled={disabled || loading}
              className={compact ? styles.compactButton : undefined}
            >
              {actionConfig.label}
            </Button>
          );
        })}
      </div>

      {/* Confirmation dialog */}
      {showConfirm && config && (
        <div className={styles.overlay} onClick={handleCancel}>
          <div className={styles.dialog} onClick={e => e.stopPropagation()}>
            <h3 className={styles.dialogTitle}>{config.confirmTitle}</h3>
            <p className={styles.dialogMessage}>{config.confirmMessage}</p>
            {config.warning && (
              <p className={styles.dialogWarning}>{config.warning}</p>
            )}
            {error && (
              <p className={styles.dialogError}>{error}</p>
            )}
            <div className={styles.dialogButtons}>
              <Button variant="ghost" onClick={handleCancel} disabled={loading}>
                Cancel
              </Button>
              <Button
                variant={config.action === 'close' ? 'secondary' : 'primary'}
                onClick={handleConfirm}
                disabled={loading}
              >
                {loading ? 'Processing...' : config.label}
              </Button>
            </div>
          </div>
        </div>
      )}
    </div>
  );
}

/**
 * Single action button variant for inline use
 */
interface SingleActionButtonProps {
  action: TransitionAction;
  onTransition: (action: TransitionAction) => Promise<void>;
  disabled?: boolean;
  size?: 'small' | 'normal';
}

export function SingleActionButton({
  action,
  onTransition,
  disabled = false,
  size = 'normal',
}: SingleActionButtonProps): JSX.Element {
  const [loading, setLoading] = useState(false);
  const config = ACTION_CONFIGS[action];

  const handleClick = async () => {
    setLoading(true);
    try {
      await onTransition(action);
    } finally {
      setLoading(false);
    }
  };

  return (
    <Button
      variant={config.variant}
      onClick={handleClick}
      disabled={disabled || loading}
      className={size === 'small' ? styles.smallButton : undefined}
    >
      {loading ? '...' : config.label}
    </Button>
  );
}

export default StateTransitionButton;
