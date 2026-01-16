/**
 * IntakeLifecycleActions Component
 *
 * Lifecycle action buttons for intakes based on current status.
 * - Draft: Edit, Activate
 * - Active: Fork, Archive
 * - Archived: Fork
 */

import { useState } from 'react';
import { Link } from 'react-router-dom';
import { Button } from '../ui';
import styles from './StateTransitionButton.module.css';

type IntakeStatus = 'draft' | 'active' | 'archived';
type IntakeAction = 'activate' | 'archive' | 'fork';

interface IntakeLifecycleActionsProps {
  intakeId: string;
  status: IntakeStatus;
  onAction: (action: IntakeAction) => Promise<void>;
  disabled?: boolean;
}

interface ActionConfig {
  action: IntakeAction;
  label: string;
  variant: 'primary' | 'secondary' | 'ghost';
  confirmTitle: string;
  confirmMessage: string;
  warning?: string;
}

const ACTION_CONFIGS: Record<IntakeAction, ActionConfig> = {
  activate: {
    action: 'activate',
    label: 'Activate',
    variant: 'primary',
    confirmTitle: 'Activate Intake',
    confirmMessage:
      'This will make the intake a commitment object. It will be content-addressed and immutable.',
    warning: 'Once activated, the intake cannot be edited. You can fork it to create a new draft.',
  },
  archive: {
    action: 'archive',
    label: 'Archive',
    variant: 'ghost',
    confirmTitle: 'Archive Intake',
    confirmMessage: 'This will archive the intake. It will remain retrievable but no longer active.',
  },
  fork: {
    action: 'fork',
    label: 'Fork to New Draft',
    variant: 'secondary',
    confirmTitle: 'Fork Intake',
    confirmMessage:
      'This will create a new draft intake with the same content. The new draft can be edited and activated.',
  },
};

function getAvailableActions(status: IntakeStatus): IntakeAction[] {
  switch (status) {
    case 'draft':
      return ['activate'];
    case 'active':
      return ['fork', 'archive'];
    case 'archived':
      return ['fork'];
    default:
      return [];
  }
}

export function IntakeLifecycleActions({
  intakeId,
  status,
  onAction,
  disabled = false,
}: IntakeLifecycleActionsProps): JSX.Element {
  const [showConfirm, setShowConfirm] = useState<IntakeAction | null>(null);
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);

  const availableActions = getAvailableActions(status);

  const handleClick = (action: IntakeAction) => {
    setError(null);
    setShowConfirm(action);
  };

  const handleConfirm = async () => {
    if (!showConfirm) return;

    setLoading(true);
    setError(null);

    try {
      await onAction(showConfirm);
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

  const config = showConfirm ? ACTION_CONFIGS[showConfirm] : null;

  return (
    <div className={styles.container}>
      {/* Action buttons */}
      <div className={styles.buttons}>
        {/* Edit button for draft status */}
        {status === 'draft' && (
          <Link to={`/intakes/${intakeId}/edit`}>
            <Button variant="secondary" disabled={disabled}>
              Edit
            </Button>
          </Link>
        )}

        {/* Lifecycle action buttons */}
        {availableActions.map((action) => {
          const actionConfig = ACTION_CONFIGS[action];
          return (
            <Button
              key={action}
              variant={actionConfig.variant}
              onClick={() => handleClick(action)}
              disabled={disabled || loading}
            >
              {actionConfig.label}
            </Button>
          );
        })}
      </div>

      {/* Confirmation dialog */}
      {showConfirm && config && (
        <div className={styles.overlay} onClick={handleCancel}>
          <div className={styles.dialog} onClick={(e) => e.stopPropagation()}>
            <h3 className={styles.dialogTitle}>{config.confirmTitle}</h3>
            <p className={styles.dialogMessage}>{config.confirmMessage}</p>
            {config.warning && <p className={styles.dialogWarning}>{config.warning}</p>}
            {error && <p className={styles.dialogError}>{error}</p>}
            <div className={styles.dialogButtons}>
              <Button variant="ghost" onClick={handleCancel} disabled={loading}>
                Cancel
              </Button>
              <Button variant={config.variant} onClick={handleConfirm} disabled={loading}>
                {loading ? 'Processing...' : config.label}
              </Button>
            </div>
          </div>
        </div>
      )}
    </div>
  );
}

export default IntakeLifecycleActions;
