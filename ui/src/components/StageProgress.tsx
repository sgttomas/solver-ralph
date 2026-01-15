/**
 * StageProgress Component
 *
 * Visual representation of stage flow in a procedure template.
 * Shows completed, current, and pending stages with status indicators.
 */

import { useState } from 'react';
import { Link } from 'react-router-dom';
import styles from './StageProgress.module.css';

export interface Stage {
  stage_id: string;
  stage_name: string;
  purpose: string;
  required_outputs: string[];
  required_oracle_suites: string[];
  gate_rule: string;
  transition_on_pass: string | null;
}

export interface StageStatusInfo {
  status: 'pending' | 'in_progress' | 'completed' | 'failed';
  evidence_bundle_ref: string | null;
}

interface StageProgressProps {
  stages: Stage[];
  currentStageId: string | null;
  stageStatus: Record<string, StageStatusInfo> | null;
  terminalStageId: string;
  onStageClick?: (stageId: string) => void;
  compact?: boolean;
}

export function StageProgress({
  stages,
  currentStageId,
  stageStatus,
  terminalStageId,
  onStageClick,
  compact = false,
}: StageProgressProps): JSX.Element {
  const [expandedStage, setExpandedStage] = useState<string | null>(null);

  const getStageStatus = (stageId: string): StageStatusInfo['status'] => {
    if (stageStatus?.[stageId]) {
      return stageStatus[stageId].status;
    }
    if (stageId === currentStageId) {
      return 'in_progress';
    }
    // Determine if before or after current stage
    const currentIndex = stages.findIndex(s => s.stage_id === currentStageId);
    const thisIndex = stages.findIndex(s => s.stage_id === stageId);
    if (currentIndex >= 0 && thisIndex < currentIndex) {
      return 'completed';
    }
    return 'pending';
  };

  const getStatusIcon = (status: StageStatusInfo['status']) => {
    switch (status) {
      case 'completed':
        return '\u2713'; // checkmark
      case 'in_progress':
        return '\u25CF'; // filled circle
      case 'failed':
        return '\u2717'; // x mark
      default:
        return '\u25CB'; // empty circle
    }
  };

  const handleStageClick = (stageId: string) => {
    if (compact) {
      onStageClick?.(stageId);
    } else {
      setExpandedStage(expandedStage === stageId ? null : stageId);
    }
  };

  if (compact) {
    return (
      <div className={styles.compactContainer}>
        {stages.map((stage, idx) => {
          const status = getStageStatus(stage.stage_id);
          const isTerminal = stage.stage_id === terminalStageId;

          return (
            <div key={stage.stage_id} className={styles.compactItem}>
              <button
                className={`${styles.compactStage} ${styles[status]} ${isTerminal ? styles.terminal : ''}`}
                onClick={() => handleStageClick(stage.stage_id)}
                title={`${stage.stage_name}: ${stage.purpose}`}
              >
                <span className={styles.compactIcon}>{getStatusIcon(status)}</span>
                <span className={styles.compactName}>{stage.stage_name}</span>
              </button>
              {idx < stages.length - 1 && (
                <span className={styles.compactArrow}>&rarr;</span>
              )}
            </div>
          );
        })}
      </div>
    );
  }

  return (
    <div className={styles.container}>
      {/* Flow visualization */}
      <div className={styles.flow}>
        {stages.map((stage, idx) => {
          const status = getStageStatus(stage.stage_id);
          const isTerminal = stage.stage_id === terminalStageId;
          const isCurrent = stage.stage_id === currentStageId;

          return (
            <div key={stage.stage_id} className={styles.flowItem}>
              <button
                className={`${styles.stageButton} ${styles[status]} ${isTerminal ? styles.terminal : ''} ${isCurrent ? styles.current : ''}`}
                onClick={() => handleStageClick(stage.stage_id)}
              >
                <span className={styles.icon}>{getStatusIcon(status)}</span>
                <span className={styles.name}>{stage.stage_name}</span>
              </button>
              {idx < stages.length - 1 && (
                <span className={styles.arrow}>&rarr;</span>
              )}
            </div>
          );
        })}
      </div>

      {/* Expanded stage details */}
      {expandedStage && (
        <div className={styles.details}>
          {stages
            .filter(s => s.stage_id === expandedStage)
            .map(stage => {
              const status = getStageStatus(stage.stage_id);
              const evidenceRef = stageStatus?.[stage.stage_id]?.evidence_bundle_ref;

              return (
                <div key={stage.stage_id} className={styles.detailCard}>
                  <div className={styles.detailHeader}>
                    <div>
                      <h4 className={styles.detailTitle}>{stage.stage_name}</h4>
                      <code className={styles.stageId}>{stage.stage_id}</code>
                    </div>
                    <span className={`${styles.statusBadge} ${styles[status]}`}>
                      {status}
                    </span>
                  </div>

                  <p className={styles.purpose}>{stage.purpose}</p>

                  <div className={styles.detailSection}>
                    <h5 className={styles.sectionLabel}>Required Outputs</h5>
                    {stage.required_outputs.length === 0 ? (
                      <span className={styles.none}>None</span>
                    ) : (
                      <ul className={styles.outputList}>
                        {stage.required_outputs.map((output, idx) => (
                          <li key={idx}>{output}</li>
                        ))}
                      </ul>
                    )}
                  </div>

                  <div className={styles.detailSection}>
                    <h5 className={styles.sectionLabel}>Oracle Suites</h5>
                    {stage.required_oracle_suites.length === 0 ? (
                      <span className={styles.none}>None required</span>
                    ) : (
                      <div className={styles.suiteList}>
                        {stage.required_oracle_suites.map((suite, idx) => (
                          <code key={idx} className={styles.suite}>{suite}</code>
                        ))}
                      </div>
                    )}
                  </div>

                  <div className={styles.detailRow}>
                    <div>
                      <h5 className={styles.sectionLabel}>Gate Rule</h5>
                      <code className={styles.gateRule}>{stage.gate_rule}</code>
                    </div>
                    <div>
                      <h5 className={styles.sectionLabel}>Next Stage</h5>
                      <code className={styles.gateRule}>
                        {stage.transition_on_pass || 'terminal'}
                      </code>
                    </div>
                  </div>

                  {evidenceRef && (
                    <div className={styles.detailSection}>
                      <h5 className={styles.sectionLabel}>Evidence Bundle</h5>
                      <Link to={`/artifacts/${evidenceRef}`} className={styles.evidenceLink}>
                        {evidenceRef.substring(0, 16)}...
                      </Link>
                    </div>
                  )}
                </div>
              );
            })}
        </div>
      )}
    </div>
  );
}

export default StageProgress;
