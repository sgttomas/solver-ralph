/**
 * WorkScreen - Unified Work View (SR-PLAN-MVP1 Task B4)
 *
 * Per SR-PLAN-MVP1, this screen provides a unified view for working on a Work Surface:
 * - Work context (intake, template)
 * - Current candidate output
 * - Auto-loaded evidence (no hash selection)
 * - Judgment actions (Approve/Reject/Waive)
 *
 * The user should NEVER manually select evidence bundles from a hash list.
 */

import { useState, useEffect, useCallback } from 'react';
import { useParams, Link } from 'react-router-dom';
import { useAuth } from '../auth/AuthProvider';
import config from '../config';
import { Card, Button, Pill, truncateHash } from '../ui';
import styles from '../styles/pages.module.css';

// ============================================================================
// Types
// ============================================================================

interface ActorInfo {
  kind: string;
  id: string;
}

interface WorkSurfaceDetail {
  work_surface_id: string;
  work_unit_id: string;
  intake_id: string;
  intake_content_hash: string;
  template_id: string;
  template_hash: string;
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

interface LoopResponse {
  loop_id: string;
  work_surface_id: string | null;
  state: string;
  goal: string;
  iteration_count: number;
  budget?: {
    max_iterations?: number;
  };
}

interface EvidenceSummary {
  content_hash: string;
  bundle_id: string;
  run_id: string;
  candidate_id: string;
  iteration_id: string | null;
  oracle_suite_id: string;
  verdict: string;
  run_completed_at: string;
  template_id: string | null;
  stage_id: string | null;
}

interface WorkSurfaceEvidenceResponse {
  work_surface_id: string;
  evidence: EvidenceSummary[];
  total: number;
}

// ============================================================================
// Component
// ============================================================================

export function WorkScreen(): JSX.Element {
  const { workSurfaceId } = useParams<{ workSurfaceId: string }>();
  const auth = useAuth();

  // State
  const [workSurface, setWorkSurface] = useState<WorkSurfaceDetail | null>(null);
  const [loop, setLoop] = useState<LoopResponse | null>(null);
  const [evidence, setEvidence] = useState<EvidenceSummary[]>([]);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);

  // Fetch work surface details
  const fetchWorkSurface = useCallback(async () => {
    if (!auth.user?.access_token || !workSurfaceId) return;

    try {
      const response = await fetch(
        `${config.apiUrl}/api/v1/work-surfaces/${workSurfaceId}`,
        {
          headers: {
            Authorization: `Bearer ${auth.user.access_token}`,
          },
        }
      );

      if (!response.ok) {
        throw new Error(`Failed to fetch work surface: ${response.statusText}`);
      }

      const data = await response.json();
      setWorkSurface(data);
    } catch (err) {
      setError(err instanceof Error ? err.message : 'Unknown error');
    }
  }, [auth.user?.access_token, workSurfaceId]);

  // Fetch associated loop
  const fetchLoop = useCallback(async () => {
    if (!auth.user?.access_token || !workSurfaceId) return;

    try {
      const response = await fetch(
        `${config.apiUrl}/api/v1/loops?work_surface_id=${workSurfaceId}`,
        {
          headers: {
            Authorization: `Bearer ${auth.user.access_token}`,
          },
        }
      );

      if (response.ok) {
        const data = await response.json();
        if (data.loops && data.loops.length > 0) {
          setLoop(data.loops[0]);
        }
      }
    } catch (err) {
      // Loop might not exist yet, which is fine
      console.log('No loop found for work surface');
    }
  }, [auth.user?.access_token, workSurfaceId]);

  // Fetch evidence (auto-linked)
  const fetchEvidence = useCallback(async () => {
    if (!auth.user?.access_token || !workSurfaceId) return;

    try {
      const response = await fetch(
        `${config.apiUrl}/api/v1/work-surfaces/${workSurfaceId}/evidence`,
        {
          headers: {
            Authorization: `Bearer ${auth.user.access_token}`,
          },
        }
      );

      if (response.ok) {
        const data: WorkSurfaceEvidenceResponse = await response.json();
        setEvidence(data.evidence);
      }
    } catch (err) {
      console.log('Failed to fetch evidence:', err);
    }
  }, [auth.user?.access_token, workSurfaceId]);

  // Load all data
  useEffect(() => {
    const loadData = async () => {
      setLoading(true);
      setError(null);
      await Promise.all([fetchWorkSurface(), fetchLoop(), fetchEvidence()]);
      setLoading(false);
    };
    loadData();
  }, [fetchWorkSurface, fetchLoop, fetchEvidence]);

  // Render loading state
  if (loading) {
    return (
      <div className={styles.page}>
        <div className={styles.loading}>Loading work surface...</div>
      </div>
    );
  }

  // Render error state
  if (error || !workSurface) {
    return (
      <div className={styles.page}>
        <Card className={styles.errorCard}>
          <h2>Error</h2>
          <p>{error || 'Work surface not found'}</p>
          <Link to="/work-surfaces">
            <Button>Back to Work Surfaces</Button>
          </Link>
        </Card>
      </div>
    );
  }

  // Calculate evidence verdict summary
  const passCount = evidence.filter((e) => e.verdict === 'PASS').length;
  const failCount = evidence.filter((e) => e.verdict === 'FAIL').length;
  const allPass = evidence.length > 0 && failCount === 0;

  return (
    <div className={styles.page}>
      <div className={styles.pageHeader}>
        <div className={styles.breadcrumb}>
          <Link to="/work-surfaces">Work Surfaces</Link>
          <span className={styles.separator}>/</span>
          <span>{truncateHash(workSurface.work_surface_id)}</span>
        </div>
        <h1>Work Screen</h1>
      </div>

      <div className={styles.twoColumnLayout}>
        {/* Left Column: Work Context + Evidence */}
        <div className={styles.mainColumn}>
          {/* Work Context Card */}
          <Card className={styles.card}>
            <h2>Work Context</h2>
            <div className={styles.detailGrid}>
              <div className={styles.detailRow}>
                <span className={styles.detailLabel}>Intake:</span>
                <Link to={`/intakes/${workSurface.intake_id}`}>
                  {workSurface.intake_id}
                </Link>
              </div>
              <div className={styles.detailRow}>
                <span className={styles.detailLabel}>Template:</span>
                <span>{workSurface.template_id}</span>
              </div>
              <div className={styles.detailRow}>
                <span className={styles.detailLabel}>Current Stage:</span>
                <span>{workSurface.current_stage_id}</span>
              </div>
              <div className={styles.detailRow}>
                <span className={styles.detailLabel}>Status:</span>
                <Pill
                  tone={
                    workSurface.status === 'active'
                      ? 'info'
                      : workSurface.status === 'completed'
                      ? 'success'
                      : 'neutral'
                  }
                >
                  {workSurface.status}
                </Pill>
              </div>
            </div>
          </Card>

          {/* Evidence Card - Auto-loaded */}
          <Card className={styles.card}>
            <h2>
              Evidence{' '}
              {evidence.length > 0 && (
                <span className={styles.badge}>
                  {passCount} pass / {failCount} fail
                </span>
              )}
            </h2>
            {evidence.length === 0 ? (
              <div className={styles.emptyState}>
                <p>No evidence recorded yet</p>
                <p className={styles.hint}>
                  Evidence will appear here automatically when oracles run
                </p>
              </div>
            ) : (
              <div className={styles.evidenceList}>
                {evidence.map((e) => (
                  <div key={e.content_hash} className={styles.evidenceItem}>
                    <div className={styles.evidenceHeader}>
                      <Pill
                        tone={e.verdict === 'PASS' ? 'success' : 'danger'}
                      >
                        {e.verdict}
                      </Pill>
                      <span className={styles.oracleSuite}>
                        {e.oracle_suite_id}
                      </span>
                    </div>
                    <div className={styles.evidenceDetails}>
                      <span>Run: {truncateHash(e.run_id)}</span>
                      <span>
                        {new Date(e.run_completed_at).toLocaleString()}
                      </span>
                    </div>
                    <Link
                      to={`/evidence/${e.content_hash}`}
                      className={styles.evidenceLink}
                    >
                      View Details
                    </Link>
                  </div>
                ))}
              </div>
            )}
          </Card>
        </div>

        {/* Right Column: Loop State + Actions */}
        <div className={styles.sideColumn}>
          {/* Loop State Card */}
          <Card className={styles.card}>
            <h2>Loop Status</h2>
            {loop ? (
              <div className={styles.loopState}>
                <div className={styles.detailRow}>
                  <span className={styles.detailLabel}>State:</span>
                  <Pill
                    tone={
                      loop.state === 'running'
                        ? 'info'
                        : loop.state === 'stopped'
                        ? 'warning'
                        : loop.state === 'completed'
                        ? 'success'
                        : 'neutral'
                    }
                  >
                    {loop.state}
                  </Pill>
                </div>
                <div className={styles.detailRow}>
                  <span className={styles.detailLabel}>Iterations:</span>
                  <span>
                    {loop.iteration_count}
                    {loop.budget?.max_iterations &&
                      ` / ${loop.budget.max_iterations}`}
                  </span>
                </div>
                <div className={styles.detailRow}>
                  <span className={styles.detailLabel}>Goal:</span>
                  <span className={styles.goalText}>{loop.goal}</span>
                </div>
                <Link to={`/loops/${loop.loop_id}`}>
                  <Button variant="secondary" >
                    View Loop Details
                  </Button>
                </Link>
              </div>
            ) : (
              <div className={styles.emptyState}>
                <p>No loop started</p>
                <Link to={`/work-surfaces/${workSurface.work_surface_id}`}>
                  <Button variant="secondary" >
                    Start Loop
                  </Button>
                </Link>
              </div>
            )}
          </Card>

          {/* Judgment Actions Card */}
          <Card className={styles.card}>
            <h2>Judgment</h2>
            {evidence.length === 0 ? (
              <div className={styles.emptyState}>
                <p>Awaiting evidence</p>
                <p className={styles.hint}>
                  Run oracles to produce evidence before making a judgment
                </p>
              </div>
            ) : (
              <div className={styles.judgmentActions}>
                <Button
                  variant={allPass ? 'primary' : 'secondary'}
                  disabled={!allPass}
                  title={
                    allPass
                      ? 'Approve this stage'
                      : 'Cannot approve - some oracles failed'
                  }
                >
                  Approve
                </Button>
                <Button variant="secondary">Reject</Button>
                <Button variant="secondary">Waive with Reason</Button>
                {!allPass && (
                  <p className={styles.hint}>
                    Some evidence shows failures. Review before approving.
                  </p>
                )}
              </div>
            )}
          </Card>
        </div>
      </div>
    </div>
  );
}

export default WorkScreen;
