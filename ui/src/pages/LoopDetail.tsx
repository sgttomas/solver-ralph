/**
 * Loop Detail Page
 *
 * Displays a single loop with comprehensive details including:
 * - Overview with state transitions
 * - Work Surface (Intake, Procedure, Stage, Oracle Suite)
 * - Stage Progress visualization
 * - Budgets with progress bars
 * - Stop Triggers and Exceptions
 * - Iterations table
 * - Governed Artifacts
 *
 * Per SR-SPEC, loops are bounded work units containing iterations.
 */

import { useState, useEffect } from 'react';
import { useParams, Link } from 'react-router-dom';
import { useAuth } from '../auth/AuthProvider';
import config from '../config';
import { Card, Pill, Button, getStatusTone, truncateHash } from '../ui';
import {
  BudgetProgress,
  StageProgress,
  StateTransitionButton,
  LoopEditModal,
} from '../components';
import type { Stage, StageStatusInfo } from '../components';
import styles from '../styles/pages.module.css';

interface TypedRef {
  type_key: string;
  id: string;
  rel: string;
  meta?: {
    version?: string;
    content_hash?: string;
  };
}

interface WorkSurface {
  intake_id: string | null;
  intake_title: string | null;
  intake_objective: string | null;
  procedure_template_id: string | null;
  procedure_template_name: string | null;
  current_stage_id: string | null;
  current_stage_name: string | null;
  oracle_suite_id: string | null;
  oracle_suite_hash: string | null;
}

interface StopTrigger {
  trigger_code: string;
  fired_at: string;
  decision_id: string | null;
  resolved: boolean;
}

interface ActiveException {
  id: string;
  kind: 'DEVIATION' | 'DEFERRAL' | 'WAIVER';
  status: string;
  scope: string;
  rationale: string;
  expires_at: string | null;
}

interface Loop {
  id: string;
  goal: string;
  work_unit: string | null;
  work_surface_id: string | null; // SR-PLAN-V5 Phase 5b: bound Work Surface
  state: 'CREATED' | 'ACTIVE' | 'PAUSED' | 'CLOSED';
  created_at: string;
  activated_at: string | null;
  closed_at: string | null;
  last_event_id: string | null;
  directive_ref: TypedRef;
  budgets: {
    max_iterations: number;
    max_oracle_runs: number;
    max_wallclock_hours: number;
  };
  iteration_count: number;
  oracle_run_count: number;
  work_surface: WorkSurface | null;
  current_stage_id: string | null;
  stage_status: Record<string, StageStatusInfo> | null;
  stop_triggers_fired: StopTrigger[];
  active_exceptions: ActiveException[];
  governed_artifacts: TypedRef[];
}

interface Iteration {
  id: string;
  loop_id: string;
  sequence_number: number;
  state: string;
  started_at: string;
  completed_at: string | null;
  outcome: string | null;
  candidates_count: number;
  evidence_count: number;
}

interface ProcedureTemplate {
  stages: Stage[];
  terminal_stage_id: string;
}

export function LoopDetail(): JSX.Element {
  const { loopId } = useParams<{ loopId: string }>();
  const auth = useAuth();
  const [loop, setLoop] = useState<Loop | null>(null);
  const [iterations, setIterations] = useState<Iteration[]>([]);
  const [template, setTemplate] = useState<ProcedureTemplate | null>(null);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);
  const [showEditModal, setShowEditModal] = useState(false);

  const fetchData = async () => {
    if (!auth.user?.access_token || !loopId) return;

    setLoading(true);
    setError(null);

    const headers = { Authorization: `Bearer ${auth.user.access_token}` };

    try {
      const [loopRes, iterationsRes] = await Promise.all([
        fetch(`${config.apiUrl}/api/v1/loops/${loopId}`, { headers }),
        fetch(`${config.apiUrl}/api/v1/loops/${loopId}/iterations`, { headers }),
      ]);

      if (!loopRes.ok) {
        throw new Error(loopRes.status === 404 ? 'Loop not found' : `HTTP ${loopRes.status}`);
      }

      const loopData = await loopRes.json();
      const iterationsData = iterationsRes.ok ? await iterationsRes.json() : { iterations: [] };

      // Normalize loop data
      const normalizedLoop: Loop = {
        id: loopData.loop_id || loopData.id,
        goal: loopData.goal || '',
        work_unit: loopData.work_unit || null,
        work_surface_id: loopData.work_surface_id || null, // SR-PLAN-V5 Phase 5b
        state: (loopData.state || 'CREATED').toUpperCase(),
        created_at: loopData.created_at,
        activated_at: loopData.activated_at || null,
        closed_at: loopData.closed_at || null,
        last_event_id: loopData.last_event_id || null,
        directive_ref: loopData.directive_ref || { type_key: '', id: '', rel: '' },
        budgets: loopData.budgets || { max_iterations: 5, max_oracle_runs: 25, max_wallclock_hours: 16 },
        iteration_count: loopData.iteration_count || 0,
        oracle_run_count: loopData.oracle_run_count || 0,
        work_surface: loopData.work_surface || null,
        current_stage_id: loopData.current_stage_id || null,
        stage_status: loopData.stage_status || null,
        stop_triggers_fired: loopData.stop_triggers_fired || [],
        active_exceptions: loopData.active_exceptions || [],
        governed_artifacts: loopData.governed_artifacts || [],
      };

      setLoop(normalizedLoop);
      setIterations(iterationsData.iterations || []);

      // Fetch procedure template if work surface has one
      if (normalizedLoop.work_surface?.procedure_template_id) {
        try {
          const templateRes = await fetch(
            `${config.apiUrl}/api/v1/procedure-templates/${normalizedLoop.work_surface.procedure_template_id}`,
            { headers }
          );
          if (templateRes.ok) {
            const templateData = await templateRes.json();
            setTemplate({
              stages: templateData.stages || [],
              terminal_stage_id: templateData.terminal_stage_id || '',
            });
          }
        } catch {
          // Template fetch is optional
        }
      }
    } catch (err) {
      setError(err instanceof Error ? err.message : 'Failed to load workflow');
    } finally {
      setLoading(false);
    }
  };

  useEffect(() => {
    fetchData();
  }, [auth.user?.access_token, loopId]);

  const handleStateTransition = async (action: 'activate' | 'pause' | 'resume' | 'close') => {
    if (!auth.user?.access_token || !loopId) return;

    const res = await fetch(`${config.apiUrl}/api/v1/loops/${loopId}/${action}`, {
      method: 'POST',
      headers: {
        Authorization: `Bearer ${auth.user.access_token}`,
        'Content-Type': 'application/json',
      },
    });

    if (!res.ok) {
      const errorData = await res.json().catch(() => ({}));
      throw new Error(errorData.message || `Failed to ${action} loop`);
    }

    await fetchData();
  };

  if (loading) {
    return (
      <div className={styles.container}>
        <div className={styles.placeholder}>
          <p className={styles.placeholderText}>Loading workflow details...</p>
        </div>
      </div>
    );
  }

  if (error || !loop) {
    return (
      <div className={styles.container}>
        <div className={styles.placeholder}>
          <p className={styles.error}>Error: {error || 'Loop not found'}</p>
          <Link to="/loops" className={styles.link}>Back to Loops</Link>
        </div>
      </div>
    );
  }

  return (
    <div className={styles.container}>
      {/* Breadcrumb */}
      <div className={styles.breadcrumb}>
        <Link to="/loops" className={styles.breadcrumbLink}>Loops</Link>
        <span className={styles.breadcrumbSeparator}>/</span>
        <span>{loop.id}</span>
      </div>

      {/* Header */}
      <div className={styles.header}>
        <div className={styles.headerStart}>
          <h1 className={styles.title}>{loop.work_unit || loop.goal.substring(0, 50)}</h1>
          <p className={styles.subtitle}>{loop.id}</p>
        </div>
        <div className={styles.badgeGroup}>
          <Pill tone={getStatusTone(loop.state)}>{loop.state}</Pill>
          <StateTransitionButton
            currentState={loop.state}
            onTransition={handleStateTransition}
            compact
          />
          <Button variant="ghost" onClick={() => setShowEditModal(true)}>
            Edit
          </Button>
        </div>
      </div>

      {/* Overview Card */}
      <Card title="Overview" className={styles.cardSpacing}>
        <div className={styles.infoRow}>
          <span className={styles.infoLabel}>Loop ID</span>
          <code className={styles.mono}>{loop.id}</code>
        </div>
        <div className={styles.infoRow}>
          <span className={styles.infoLabel}>Goal</span>
          <span className={styles.infoValue}>{loop.goal}</span>
        </div>
        {loop.work_unit && (
          <div className={styles.infoRow}>
            <span className={styles.infoLabel}>Work Unit</span>
            <span className={styles.infoValue}>{loop.work_unit}</span>
          </div>
        )}
        <div className={styles.infoRow}>
          <span className={styles.infoLabel}>State</span>
          <Pill tone={getStatusTone(loop.state)}>{loop.state}</Pill>
        </div>
        <div className={styles.infoRow}>
          <span className={styles.infoLabel}>Created</span>
          <span className={styles.infoValue}>{new Date(loop.created_at).toLocaleString()}</span>
        </div>
        {loop.activated_at && (
          <div className={styles.infoRow}>
            <span className={styles.infoLabel}>Activated</span>
            <span className={styles.infoValue}>{new Date(loop.activated_at).toLocaleString()}</span>
          </div>
        )}
        {loop.closed_at && (
          <div className={styles.infoRow}>
            <span className={styles.infoLabel}>Closed</span>
            <span className={styles.infoValue}>{new Date(loop.closed_at).toLocaleString()}</span>
          </div>
        )}
        <div className={styles.infoRow}>
          <span className={styles.infoLabel}>Directive Ref</span>
          <code className={styles.mono}>
            {loop.directive_ref.type_key}:{loop.directive_ref.id}
          </code>
        </div>
        {loop.last_event_id && (
          <div className={styles.infoRow}>
            <span className={styles.infoLabel}>Last Event</span>
            <code className={styles.mono}>{loop.last_event_id}</code>
          </div>
        )}
      </Card>

      {/* Work Surface Card - SR-PLAN-V5 Phase 5b: Show when bound */}
      {(loop.work_surface_id || loop.work_surface) && (
        <Card title="Work Surface" className={styles.cardSpacing}>
          {loop.work_surface_id && (
            <div className={styles.infoRow}>
              <span className={styles.infoLabel}>Work Surface ID</span>
              <Link to={`/work-surfaces/${loop.work_surface_id}`} className={styles.link}>
                <code className={styles.mono}>{loop.work_surface_id}</code>
              </Link>
            </div>
          )}
          {loop.work_surface?.intake_id && (
            <div className={styles.infoRow}>
              <span className={styles.infoLabel}>Intake</span>
              <div>
                <Link to={`/context/intakes/${loop.work_surface?.intake_id}`} className={styles.link}>
                  {loop.work_surface?.intake_title || loop.work_surface?.intake_id}
                </Link>
                {loop.work_surface?.intake_objective && (
                  <p style={{ margin: 'var(--space1) 0 0 0', fontSize: '0.8125rem', color: 'var(--muted)' }}>
                    {loop.work_surface?.intake_objective}
                  </p>
                )}
              </div>
            </div>
          )}
          {loop.work_surface?.procedure_template_id && (
            <div className={styles.infoRow}>
              <span className={styles.infoLabel}>Procedure</span>
              <Link to={`/protocols/${loop.work_surface?.procedure_template_id}`} className={styles.link}>
                {loop.work_surface?.procedure_template_name || loop.work_surface?.procedure_template_id}
              </Link>
            </div>
          )}
          {loop.work_surface?.current_stage_id && (
            <div className={styles.infoRow}>
              <span className={styles.infoLabel}>Current Stage</span>
              <div>
                <code className={styles.mono}>{loop.work_surface?.current_stage_id}</code>
                {loop.work_surface?.current_stage_name && (
                  <span style={{ marginLeft: 'var(--space2)', color: 'var(--muted)' }}>
                    ({loop.work_surface?.current_stage_name})
                  </span>
                )}
              </div>
            </div>
          )}
          {loop.work_surface?.oracle_suite_id && (
            <div className={styles.infoRow}>
              <span className={styles.infoLabel}>Oracle Suite</span>
              <div>
                <code className={styles.mono}>{loop.work_surface?.oracle_suite_id}</code>
                {loop.work_surface?.oracle_suite_hash && (
                  <span style={{ marginLeft: 'var(--space2)', fontSize: '0.75rem', color: 'var(--muted)' }}>
                    {truncateHash(loop.work_surface?.oracle_suite_hash, 12)}
                  </span>
                )}
              </div>
            </div>
          )}
        </Card>
      )}

      {/* Stage Progress Card */}
      {template && template.stages.length > 0 && (
        <Card title="Stage Progress" className={styles.cardSpacing}>
          <StageProgress
            stages={template.stages}
            currentStageId={loop.current_stage_id}
            stageStatus={loop.stage_status}
            terminalStageId={template.terminal_stage_id}
          />
        </Card>
      )}

      {/* Budgets Card */}
      <Card title="Budgets & Usage" className={styles.cardSpacing}>
        <div className={styles.statsGrid}>
          <BudgetProgress
            label="Iterations"
            current={loop.iteration_count}
            max={loop.budgets.max_iterations}
          />
          <BudgetProgress
            label="Oracle Runs"
            current={loop.oracle_run_count}
            max={loop.budgets.max_oracle_runs}
          />
          <BudgetProgress
            label="Wallclock Hours"
            current={0} // Would need actual runtime tracking
            max={loop.budgets.max_wallclock_hours}
            unit="h"
          />
        </div>
      </Card>

      {/* Stop Triggers Card */}
      {loop.stop_triggers_fired.length > 0 && (
        <Card title={`Stop Triggers (${loop.stop_triggers_fired.length})`} className={styles.cardSpacing}>
          <table className={styles.table}>
            <thead>
              <tr>
                <th className={styles.th}>Trigger</th>
                <th className={styles.th}>Fired At</th>
                <th className={styles.th}>Status</th>
                <th className={styles.th}>Decision</th>
              </tr>
            </thead>
            <tbody>
              {loop.stop_triggers_fired.map((trigger, idx) => (
                <tr key={idx}>
                  <td className={styles.tdMono}>{trigger.trigger_code}</td>
                  <td className={styles.td}>{new Date(trigger.fired_at).toLocaleString()}</td>
                  <td className={styles.td}>
                    <Pill tone={trigger.resolved ? 'success' : 'warning'}>
                      {trigger.resolved ? 'resolved' : 'pending'}
                    </Pill>
                  </td>
                  <td className={styles.td}>
                    {trigger.decision_id ? (
                      <Link to={`/decisions/${trigger.decision_id}`} className={styles.link}>
                        {trigger.decision_id}
                      </Link>
                    ) : (
                      <span style={{ color: 'var(--muted)' }}>-</span>
                    )}
                  </td>
                </tr>
              ))}
            </tbody>
          </table>
        </Card>
      )}

      {/* Active Exceptions Card */}
      {loop.active_exceptions.length > 0 && (
        <Card title={`Active Exceptions (${loop.active_exceptions.length})`} className={styles.cardSpacing}>
          <table className={styles.table}>
            <thead>
              <tr>
                <th className={styles.th}>ID</th>
                <th className={styles.th}>Kind</th>
                <th className={styles.th}>Status</th>
                <th className={styles.th}>Scope</th>
                <th className={styles.th}>Expires</th>
              </tr>
            </thead>
            <tbody>
              {loop.active_exceptions.map(exception => (
                <tr key={exception.id}>
                  <td className={styles.tdMono}>{exception.id}</td>
                  <td className={styles.td}>
                    <Pill tone={getStatusTone(exception.kind)}>{exception.kind}</Pill>
                  </td>
                  <td className={styles.td}>
                    <Pill tone={getStatusTone(exception.status)}>{exception.status}</Pill>
                  </td>
                  <td className={styles.td}>{exception.scope}</td>
                  <td className={styles.td}>
                    {exception.expires_at
                      ? new Date(exception.expires_at).toLocaleDateString()
                      : <span style={{ color: 'var(--muted)' }}>never</span>}
                  </td>
                </tr>
              ))}
            </tbody>
          </table>
        </Card>
      )}

      {/* Iterations Card */}
      <Card title={`Iterations (${iterations.length})`} className={styles.cardSpacing}>
        {iterations.length === 0 ? (
          <div className={styles.placeholder}>
            <p className={styles.placeholderText}>No iterations yet.</p>
            <p className={styles.placeholderHint}>
              Iterations are started by the SYSTEM actor per SR-SPEC.
            </p>
          </div>
        ) : (
          <table className={styles.table}>
            <thead>
              <tr>
                <th className={styles.th}>#</th>
                <th className={styles.th}>ID</th>
                <th className={styles.th}>State</th>
                <th className={styles.th}>Outcome</th>
                <th className={styles.th}>Candidates</th>
                <th className={styles.th}>Started</th>
                <th className={styles.th}>Duration</th>
              </tr>
            </thead>
            <tbody>
              {iterations.map(iteration => {
                const duration = iteration.completed_at
                  ? Math.round((new Date(iteration.completed_at).getTime() - new Date(iteration.started_at).getTime()) / 1000 / 60)
                  : null;

                return (
                  <tr key={iteration.id}>
                    <td className={styles.td}>{iteration.sequence_number}</td>
                    <td className={styles.td}>
                      <Link to={`/iterations/${iteration.id}`} className={styles.link}>
                        {iteration.id}
                      </Link>
                    </td>
                    <td className={styles.td}>
                      <Pill tone={getStatusTone(iteration.state)}>{iteration.state}</Pill>
                    </td>
                    <td className={styles.td}>
                      {iteration.outcome ? (
                        <Pill tone={getStatusTone(iteration.outcome)}>{iteration.outcome}</Pill>
                      ) : (
                        <span style={{ color: 'var(--muted)' }}>-</span>
                      )}
                    </td>
                    <td className={styles.td}>
                      {iteration.candidates_count || 0}
                    </td>
                    <td className={styles.td}>{new Date(iteration.started_at).toLocaleString()}</td>
                    <td className={styles.td}>
                      {duration !== null ? `${duration}m` : <span style={{ color: 'var(--muted)' }}>-</span>}
                    </td>
                  </tr>
                );
              })}
            </tbody>
          </table>
        )}
      </Card>

      {/* Governed Artifacts Card */}
      {loop.governed_artifacts.length > 0 && (
        <Card title={`Governed Artifacts (${loop.governed_artifacts.length})`}>
          <ul className={styles.refList}>
            {loop.governed_artifacts.map((artifact, idx) => (
              <li key={idx} className={styles.refItem}>
                <code>{artifact.type_key}:{artifact.id}</code>
                <span className={styles.refRel}>({artifact.rel})</span>
                {artifact.meta?.version && (
                  <Pill tone="neutral">{artifact.meta.version}</Pill>
                )}
                {artifact.meta?.content_hash && (
                  <span style={{ fontSize: '0.6875rem', color: 'var(--muted)' }}>
                    {truncateHash(artifact.meta.content_hash, 12)}
                  </span>
                )}
              </li>
            ))}
          </ul>
        </Card>
      )}

      {/* Edit Modal */}
      <LoopEditModal
        isOpen={showEditModal}
        loop={loop}
        onClose={() => setShowEditModal(false)}
        onUpdated={fetchData}
      />
    </div>
  );
}

export default LoopDetail;
