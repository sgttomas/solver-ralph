/**
 * Work Surface Detail Page
 *
 * Displays a single Work Surface with binding summary and stage progress.
 * Per SR-WORK-SURFACE, Work Surfaces bind an Intake + Procedure Template
 * and track stage progression through the procedure.
 */

import { useState, useEffect, useCallback } from 'react';
import { useParams, Link } from 'react-router-dom';
import { useAuth } from '../auth/AuthProvider';
import config from '../config';
import { Card, Button, Pill, truncateHash } from '../ui';
import styles from '../styles/pages.module.css';

interface ActorInfo {
  kind: string;
  id: string;
}

interface StageStatusRecord {
  stage_id: string;
  status: 'pending' | 'entered' | 'completed' | 'skipped';
  entered_at: string | null;
  completed_at: string | null;
  evidence_bundle_ref: string | null;
  iteration_count: number;
}

interface OracleSuiteBinding {
  suite_id: string;
  suite_hash: string;
}

interface WorkSurfaceDetail {
  work_surface_id: string;
  work_unit_id: string;
  intake_id: string;
  intake_content_hash: string;
  procedure_template_id: string;
  procedure_template_hash: string;
  current_stage_id: string;
  status: 'active' | 'completed' | 'archived';
  stage_status: Record<string, StageStatusRecord>;
  current_oracle_suites: OracleSuiteBinding[];
  params: Record<string, unknown>;
  content_hash: string;
  bound_at: string;
  bound_by: ActorInfo;
  completed_at: string | null;
  archived_at: string | null;
  archived_by: ActorInfo | null;
}

export function WorkSurfaceDetail(): JSX.Element {
  const { workSurfaceId } = useParams<{ workSurfaceId: string }>();
  const auth = useAuth();
  const [workSurface, setWorkSurface] = useState<WorkSurfaceDetail | null>(null);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);
  const [archiving, setArchiving] = useState(false);

  const fetchWorkSurface = useCallback(async () => {
    if (!auth.user?.access_token || !workSurfaceId) return;

    setLoading(true);
    setError(null);

    try {
      const res = await fetch(`${config.apiUrl}/api/v1/work-surfaces/${workSurfaceId}`, {
        headers: { Authorization: `Bearer ${auth.user.access_token}` },
      });

      if (res.status === 404) {
        throw new Error('Work Surface not found');
      }
      if (!res.ok) {
        throw new Error(`HTTP ${res.status}`);
      }

      const data = await res.json();
      setWorkSurface(data);
    } catch (err) {
      setError(err instanceof Error ? err.message : 'Failed to load work surface');
    } finally {
      setLoading(false);
    }
  }, [auth.user?.access_token, workSurfaceId]);

  useEffect(() => {
    fetchWorkSurface();
  }, [fetchWorkSurface]);

  const handleArchive = async () => {
    if (!auth.user?.access_token || !workSurfaceId) return;

    setArchiving(true);
    try {
      const res = await fetch(`${config.apiUrl}/api/v1/work-surfaces/${workSurfaceId}/archive`, {
        method: 'POST',
        headers: {
          Authorization: `Bearer ${auth.user.access_token}`,
          'Content-Type': 'application/json',
        },
        body: JSON.stringify({}),
      });

      if (!res.ok) {
        const errorData = await res.json().catch(() => ({}));
        throw new Error(errorData.message || `Failed to archive: HTTP ${res.status}`);
      }

      await fetchWorkSurface();
    } catch (err) {
      setError(err instanceof Error ? err.message : 'Failed to archive work surface');
    } finally {
      setArchiving(false);
    }
  };

  const getStatusTone = (status: string) => {
    switch (status) {
      case 'active':
        return 'success';
      case 'completed':
        return 'neutral';
      case 'archived':
        return 'neutral';
      default:
        return 'neutral';
    }
  };

  // Format stage ID for display
  const formatStageId = (stageId: string) => {
    return stageId.replace(/^stage:/, '').replace(/_/g, ' ');
  };

  // Get ordered stages from stage_status
  const getOrderedStages = (): StageStatusRecord[] => {
    if (!workSurface?.stage_status) return [];

    // Convert to array and sort by entered_at (or keep original order if not entered)
    const stages = Object.values(workSurface.stage_status);

    // Sort: completed first (by completed_at), then entered, then pending
    return stages.sort((a, b) => {
      const statusOrder = { completed: 0, entered: 1, pending: 2, skipped: 3 };
      const aOrder = statusOrder[a.status] ?? 4;
      const bOrder = statusOrder[b.status] ?? 4;

      if (aOrder !== bOrder) return aOrder - bOrder;

      // If same status, sort by entered_at
      if (a.entered_at && b.entered_at) {
        return new Date(a.entered_at).getTime() - new Date(b.entered_at).getTime();
      }
      return 0;
    });
  };

  if (loading) {
    return (
      <div className={styles.container}>
        <div className={styles.placeholder}>
          <p className={styles.placeholderText}>Loading work surface details...</p>
        </div>
      </div>
    );
  }

  if (error || !workSurface) {
    return (
      <div className={styles.container}>
        <div className={styles.placeholder}>
          <p className={styles.error}>Error: {error || 'Work Surface not found'}</p>
          <Link to="/work-surfaces" className={styles.link}>
            Back to Work Surfaces
          </Link>
        </div>
      </div>
    );
  }

  const stages = getOrderedStages();

  return (
    <div className={styles.container}>
      {/* Breadcrumb */}
      <div className={styles.breadcrumb}>
        <Link to="/work-surfaces" className={styles.breadcrumbLink}>
          Work Surfaces
        </Link>
        <span className={styles.breadcrumbSeparator}>/</span>
        <span>{truncateHash(workSurface.work_surface_id, 20)}</span>
      </div>

      {/* Header */}
      <div className={styles.header}>
        <div className={styles.headerStart}>
          <h1 className={styles.title}>Work Surface</h1>
          <p className={styles.subtitle}>{workSurface.work_surface_id}</p>
        </div>
        <div className={styles.badgeGroup}>
          <Pill tone={getStatusTone(workSurface.status)}>{workSurface.status}</Pill>
        </div>
      </div>

      {/* Actions */}
      <div style={{ marginBottom: 'var(--space4)', display: 'flex', gap: 'var(--space2)' }}>
        <Link to={`/intakes/${workSurface.intake_id}`}>
          <Button variant="ghost">View Intake</Button>
        </Link>
        {workSurface.status === 'active' && (
          <Button variant="ghost" onClick={handleArchive} disabled={archiving}>
            {archiving ? 'Archiving...' : 'Archive'}
          </Button>
        )}
      </div>

      {/* Stage Progress Card */}
      <Card title="Stage Progress" className={styles.cardSpacing}>
        {stages.length === 0 ? (
          <div className={styles.placeholder}>
            <p className={styles.placeholderText}>No stage information available.</p>
          </div>
        ) : (
          <div>
            {/* Visual progress bar */}
            <div
              style={{
                display: 'flex',
                alignItems: 'center',
                gap: 'var(--space2)',
                marginBottom: 'var(--space4)',
                overflowX: 'auto',
                padding: 'var(--space2) 0',
              }}
            >
              {stages.map((stage, idx) => {
                const isCompleted = stage.status === 'completed';
                const isCurrent = stage.status === 'entered';
                const isSkipped = stage.status === 'skipped';

                return (
                  <div key={stage.stage_id} style={{ display: 'flex', alignItems: 'center' }}>
                    <div
                      style={{
                        display: 'flex',
                        flexDirection: 'column',
                        alignItems: 'center',
                        gap: 'var(--space1)',
                      }}
                    >
                      {/* Stage indicator */}
                      <div
                        style={{
                          width: '32px',
                          height: '32px',
                          borderRadius: '50%',
                          display: 'flex',
                          alignItems: 'center',
                          justifyContent: 'center',
                          backgroundColor: isCompleted
                            ? 'var(--success)'
                            : isCurrent
                            ? 'var(--accent)'
                            : isSkipped
                            ? 'var(--muted)'
                            : 'var(--bg)',
                          border: `2px solid ${
                            isCompleted
                              ? 'var(--success)'
                              : isCurrent
                              ? 'var(--accent)'
                              : 'var(--border)'
                          }`,
                          color: isCompleted || isCurrent ? 'white' : 'var(--muted)',
                          fontSize: '0.75rem',
                          fontWeight: 600,
                        }}
                      >
                        {isCompleted ? '✓' : isCurrent ? '●' : isSkipped ? '—' : '○'}
                      </div>
                      {/* Stage name */}
                      <span
                        style={{
                          fontSize: '0.75rem',
                          fontWeight: isCurrent ? 600 : 400,
                          color: isCurrent ? 'var(--ink)' : 'var(--muted)',
                          whiteSpace: 'nowrap',
                        }}
                      >
                        {formatStageId(stage.stage_id)}
                      </span>
                    </div>
                    {/* Connector line */}
                    {idx < stages.length - 1 && (
                      <div
                        style={{
                          width: '40px',
                          height: '2px',
                          backgroundColor: isCompleted ? 'var(--success)' : 'var(--border)',
                          marginLeft: 'var(--space2)',
                          marginRight: 'var(--space2)',
                        }}
                      />
                    )}
                  </div>
                );
              })}
            </div>

            {/* Current stage info */}
            <div
              style={{
                padding: 'var(--space3)',
                backgroundColor: 'var(--bg)',
                borderRadius: 'var(--radius)',
                border: '1px solid var(--border)',
              }}
            >
              <div style={{ fontWeight: 600, marginBottom: 'var(--space2)' }}>
                Current Stage: {formatStageId(workSurface.current_stage_id)}
              </div>
              {workSurface.current_oracle_suites && workSurface.current_oracle_suites.length > 0 && (
                <div style={{ fontSize: '0.875rem', color: 'var(--muted)' }}>
                  Oracle Suites:{' '}
                  {workSurface.current_oracle_suites.map((s) => s.suite_id).join(', ')}
                </div>
              )}
            </div>
          </div>
        )}
      </Card>

      {/* Overview Card */}
      <Card title="Overview" className={styles.cardSpacing}>
        <div className={styles.infoRow}>
          <span className={styles.infoLabel}>Work Surface ID</span>
          <code className={styles.mono}>{workSurface.work_surface_id}</code>
        </div>
        <div className={styles.infoRow}>
          <span className={styles.infoLabel}>Work Unit</span>
          <code className={styles.mono}>{workSurface.work_unit_id}</code>
        </div>
        <div className={styles.infoRow}>
          <span className={styles.infoLabel}>Status</span>
          <Pill tone={getStatusTone(workSurface.status)}>{workSurface.status}</Pill>
        </div>
        <div className={styles.infoRow}>
          <span className={styles.infoLabel}>Content Hash</span>
          <code className={styles.mono}>{truncateHash(workSurface.content_hash, 20)}</code>
        </div>
        <div className={styles.infoRow}>
          <span className={styles.infoLabel}>Bound</span>
          <span className={styles.infoValue}>
            {new Date(workSurface.bound_at).toLocaleString()} by {workSurface.bound_by.id}
          </span>
        </div>
        {workSurface.completed_at && (
          <div className={styles.infoRow}>
            <span className={styles.infoLabel}>Completed</span>
            <span className={styles.infoValue}>
              {new Date(workSurface.completed_at).toLocaleString()}
            </span>
          </div>
        )}
        {workSurface.archived_at && workSurface.archived_by && (
          <div className={styles.infoRow}>
            <span className={styles.infoLabel}>Archived</span>
            <span className={styles.infoValue}>
              {new Date(workSurface.archived_at).toLocaleString()} by {workSurface.archived_by.id}
            </span>
          </div>
        )}
      </Card>

      {/* Binding Summary Card */}
      <Card title="Binding Summary" className={styles.cardSpacing}>
        <div className={styles.infoRow}>
          <span className={styles.infoLabel}>Intake</span>
          <Link to={`/intakes/${workSurface.intake_id}`} className={styles.link}>
            <code className={styles.mono}>{workSurface.intake_id}</code>
          </Link>
        </div>
        <div className={styles.infoRow}>
          <span className={styles.infoLabel}>Intake Hash</span>
          <code className={styles.mono}>{truncateHash(workSurface.intake_content_hash, 20)}</code>
        </div>
        <div className={styles.infoRow}>
          <span className={styles.infoLabel}>Procedure Template</span>
          <code className={styles.mono}>{workSurface.procedure_template_id}</code>
        </div>
        <div className={styles.infoRow}>
          <span className={styles.infoLabel}>Template Hash</span>
          <code className={styles.mono}>{truncateHash(workSurface.procedure_template_hash, 20)}</code>
        </div>
      </Card>

      {/* Stage History Card */}
      {stages.length > 0 && (
        <Card title="Stage History" className={styles.cardSpacing}>
          <table className={styles.table}>
            <thead>
              <tr>
                <th className={styles.th}>Stage</th>
                <th className={styles.th}>Status</th>
                <th className={styles.th}>Entered</th>
                <th className={styles.th}>Completed</th>
                <th className={styles.th}>Iterations</th>
              </tr>
            </thead>
            <tbody>
              {stages.map((stage) => (
                <tr key={stage.stage_id}>
                  <td className={styles.td}>
                    <code style={{ fontSize: '0.75rem' }}>{formatStageId(stage.stage_id)}</code>
                  </td>
                  <td className={styles.td}>
                    <Pill
                      tone={
                        stage.status === 'completed'
                          ? 'success'
                          : stage.status === 'entered'
                          ? 'warning'
                          : 'neutral'
                      }
                    >
                      {stage.status}
                    </Pill>
                  </td>
                  <td className={styles.td} style={{ fontSize: '0.75rem' }}>
                    {stage.entered_at ? new Date(stage.entered_at).toLocaleString() : '—'}
                  </td>
                  <td className={styles.td} style={{ fontSize: '0.75rem' }}>
                    {stage.completed_at ? new Date(stage.completed_at).toLocaleString() : '—'}
                  </td>
                  <td className={styles.td}>{stage.iteration_count}</td>
                </tr>
              ))}
            </tbody>
          </table>
        </Card>
      )}

      {/* Params Card (if any) */}
      {Object.keys(workSurface.params || {}).length > 0 && (
        <Card title="Parameters" className={styles.cardSpacing}>
          <pre
            style={{
              margin: 0,
              fontSize: '0.75rem',
              fontFamily: 'var(--mono)',
              whiteSpace: 'pre-wrap',
              wordBreak: 'break-word',
            }}
          >
            {JSON.stringify(workSurface.params, null, 2)}
          </pre>
        </Card>
      )}
    </div>
  );
}

export default WorkSurfaceDetail;
