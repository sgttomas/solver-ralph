/**
 * BudgetProgress Component
 *
 * Displays a budget metric with a progress bar and visual warnings
 * when approaching or exceeding limits.
 */

import styles from './BudgetProgress.module.css';

interface BudgetProgressProps {
  label: string;
  current: number;
  max: number;
  warningThreshold?: number; // 0-1, default 0.8
  unit?: string;
}

export function BudgetProgress({
  label,
  current,
  max,
  warningThreshold = 0.8,
  unit,
}: BudgetProgressProps): JSX.Element {
  const percentage = max > 0 ? Math.min((current / max) * 100, 100) : 0;
  const ratio = max > 0 ? current / max : 0;

  const isWarning = ratio >= warningThreshold && ratio < 1;
  const isDanger = ratio >= 1;

  const getTone = () => {
    if (isDanger) return 'danger';
    if (isWarning) return 'warning';
    return 'normal';
  };

  const tone = getTone();

  return (
    <div className={styles.container}>
      <div className={styles.header}>
        <span className={styles.label}>{label}</span>
        <span className={`${styles.value} ${styles[tone]}`}>
          {current} / {max}
          {unit && <span className={styles.unit}> {unit}</span>}
        </span>
      </div>
      <div className={styles.track}>
        <div
          className={`${styles.fill} ${styles[tone]}`}
          style={{ width: `${percentage}%` }}
        />
      </div>
      {isDanger && (
        <span className={styles.warning}>Budget exhausted</span>
      )}
      {isWarning && !isDanger && (
        <span className={styles.caution}>Approaching limit</span>
      )}
    </div>
  );
}

export default BudgetProgress;
