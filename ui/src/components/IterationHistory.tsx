/**
 * IterationHistory Component
 *
 * Per SR-PLAN-V7 Phase V7-5: Timeline view of iterations for a Work Surface.
 * Shows iteration history with status, stage, and duration information.
 * Supports starting new iterations when current iteration is completed.
 */

import { useState } from 'react';
import { Button, Pill } from '../ui';
import styles from './IterationHistory.module.css';

export interface Iteration {
  iteration_id: string;
  iteration_number: number;
  started_at: string;
  completed_at?: string;
  status: string;
  stage_id?: string;
}

export interface IterationHistoryProps {
  workSurfaceId: string;
  iterations: Iteration[];
  loopId?: string;
  onStartNewIteration?: () => Promise<void>;
  canStartNew: boolean;
  isStarting?: boolean;
}

/**
 * Format duration between two timestamps
 */
function formatDuration(startedAt: string, completedAt?: string): string {
  const start = new Date(startedAt);
  const end = completedAt ? new Date(completedAt) : new Date();
  const diffMs = end.getTime() - start.getTime();

  if (diffMs < 60000) {
    return `${Math.floor(diffMs / 1000)}s`;
  }
  if (diffMs < 3600000) {
    return `${Math.floor(diffMs / 60000)}m`;
  }
  const hours = Math.floor(diffMs / 3600000);
  const minutes = Math.floor((diffMs % 3600000) / 60000);
  return `${hours}h ${minutes}m`;
}

/**
 * Get tone for iteration status pill
 */
function getStatusTone(status: string): 'success' | 'warning' | 'danger' | 'neutral' {
  switch (status.toUpperCase()) {
    case 'COMPLETED':
      return 'success';
    case 'STARTED':
    case 'RUNNING':
      return 'warning';
    case 'FAILED':
      return 'danger';
    default:
      return 'neutral';
  }
}

/**
 * Format stage ID for display
 */
function formatStageId(stageId?: string): string {
  if (!stageId) return '—';
  return stageId.replace(/^stage:/, '').replace(/_/g, ' ');
}

export function IterationHistory({
  workSurfaceId: _workSurfaceId,
  iterations,
  loopId,
  onStartNewIteration,
  canStartNew,
  isStarting = false,
}: IterationHistoryProps): JSX.Element {
  // workSurfaceId available for future use (e.g., linking to iteration details)
  const [expanded, setExpanded] = useState<Set<string>>(new Set());

  const toggleExpanded = (iterationId: string) => {
    setExpanded((prev) => {
      const next = new Set(prev);
      if (next.has(iterationId)) {
        next.delete(iterationId);
      } else {
        next.add(iterationId);
      }
      return next;
    });
  };

  if (iterations.length === 0) {
    return (
      <div className={styles.container}>
        <div className={styles.empty}>
          <p className={styles.emptyText}>No iterations yet.</p>
          {canStartNew && onStartNewIteration && (
            <Button
              variant="primary"
              onClick={onStartNewIteration}
              disabled={isStarting}
            >
              {isStarting ? 'Starting...' : 'Start First Iteration'}
            </Button>
          )}
        </div>
      </div>
    );
  }

  // Sort iterations by number (newest first for display)
  const sortedIterations = [...iterations].sort(
    (a, b) => b.iteration_number - a.iteration_number
  );

  const currentIteration = sortedIterations[0];
  const isCurrentActive =
    currentIteration &&
    (currentIteration.status.toUpperCase() === 'STARTED' ||
      currentIteration.status.toUpperCase() === 'RUNNING');

  return (
    <div className={styles.container}>
      {/* Header with "New Iteration" button */}
      <div className={styles.header}>
        <div className={styles.headerInfo}>
          <span className={styles.headerTitle}>Iterations</span>
          <span className={styles.headerCount}>{iterations.length} total</span>
        </div>
        {canStartNew && onStartNewIteration && !isCurrentActive && (
          <Button
            variant="ghost"
            onClick={onStartNewIteration}
            disabled={isStarting}
          >
            {isStarting ? 'Starting...' : 'New Iteration'}
          </Button>
        )}
      </div>

      {/* Timeline */}
      <div className={styles.timeline}>
        {sortedIterations.map((iteration, index) => {
          const isLast = index === sortedIterations.length - 1;
          const isExpanded = expanded.has(iteration.iteration_id);
          const isCurrent = index === 0;

          return (
            <div
              key={iteration.iteration_id}
              className={`${styles.timelineItem} ${isCurrent ? styles.current : ''}`}
            >
              {/* Timeline connector */}
              <div className={styles.timelineConnector}>
                <div
                  className={`${styles.timelineDot} ${
                    iteration.status.toUpperCase() === 'COMPLETED'
                      ? styles.dotCompleted
                      : iteration.status.toUpperCase() === 'FAILED'
                      ? styles.dotFailed
                      : styles.dotActive
                  }`}
                />
                {!isLast && <div className={styles.timelineLine} />}
              </div>

              {/* Iteration card */}
              <div className={styles.iterationCard}>
                <div
                  className={styles.iterationHeader}
                  onClick={() => toggleExpanded(iteration.iteration_id)}
                  role="button"
                  tabIndex={0}
                  onKeyDown={(e) => {
                    if (e.key === 'Enter' || e.key === ' ') {
                      toggleExpanded(iteration.iteration_id);
                    }
                  }}
                >
                  <div className={styles.iterationMeta}>
                    <span className={styles.iterationNumber}>
                      #{iteration.iteration_number}
                    </span>
                    <Pill tone={getStatusTone(iteration.status)}>
                      {iteration.status.toLowerCase()}
                    </Pill>
                    {isCurrent && (
                      <span className={styles.currentBadge}>current</span>
                    )}
                  </div>
                  <div className={styles.iterationTiming}>
                    <span className={styles.duration}>
                      {formatDuration(iteration.started_at, iteration.completed_at)}
                    </span>
                    <span className={styles.expandIcon}>
                      {isExpanded ? '▼' : '▶'}
                    </span>
                  </div>
                </div>

                {/* Expanded details */}
                {isExpanded && (
                  <div className={styles.iterationDetails}>
                    <div className={styles.detailRow}>
                      <span className={styles.detailLabel}>ID</span>
                      <code className={styles.detailValue}>
                        {iteration.iteration_id}
                      </code>
                    </div>
                    <div className={styles.detailRow}>
                      <span className={styles.detailLabel}>Started</span>
                      <span className={styles.detailValue}>
                        {new Date(iteration.started_at).toLocaleString()}
                      </span>
                    </div>
                    {iteration.completed_at && (
                      <div className={styles.detailRow}>
                        <span className={styles.detailLabel}>Completed</span>
                        <span className={styles.detailValue}>
                          {new Date(iteration.completed_at).toLocaleString()}
                        </span>
                      </div>
                    )}
                    {iteration.stage_id && (
                      <div className={styles.detailRow}>
                        <span className={styles.detailLabel}>Stage</span>
                        <span className={styles.detailValue}>
                          {formatStageId(iteration.stage_id)}
                        </span>
                      </div>
                    )}
                  </div>
                )}
              </div>
            </div>
          );
        })}
      </div>

      {/* Loop ID reference */}
      {loopId && (
        <div className={styles.loopRef}>
          Loop: <code className={styles.loopId}>{loopId}</code>
        </div>
      )}
    </div>
  );
}

export default IterationHistory;
