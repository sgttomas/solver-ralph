/**
 * Reference Bundle Detail Page
 *
 * Displays a single evidence bundle with its refs and metadata.
 * Per SR-SPEC, evidence bundles capture the verification outputs
 * from oracle runs on candidates.
 *
 * Fetches from: GET /api/v1/references/evidence-bundles/:hash
 */

import { useState, useEffect } from 'react';
import { useParams, Link } from 'react-router-dom';
import { useAuth } from '../auth/AuthProvider';
import config from '../config';
import { Card, Pill, Button } from '../ui';
import styles from '../styles/pages.module.css';

// ============================================================================
// Types
// ============================================================================

interface ActorInfo {
  kind: string;
  id: string;
}

interface EvidenceBundleDetailData {
  content_hash: string;
  bundle_id: string;
  run_id: string;
  candidate_id: string;
  iteration_id: string | null;
  oracle_suite_id: string;
  oracle_suite_hash: string;
  verdict: string;
  artifact_count: number;
  run_completed_at: string;
  recorded_at: string;
  recorded_by: ActorInfo;
}

// ============================================================================
// Helpers
// ============================================================================

function truncateHash(hash: string, length: number = 16): string {
  if (!hash) return '';
  if (hash.length <= length) return hash;
  return hash.slice(0, length / 2) + '...' + hash.slice(-length / 2);
}

function getVerdictTone(verdict: string): 'neutral' | 'success' | 'warning' | 'danger' {
  switch (verdict.toLowerCase()) {
    case 'pass':
    case 'passed':
      return 'success';
    case 'fail':
    case 'failed':
      return 'danger';
    case 'partial':
      return 'warning';
    default:
      return 'neutral';
  }
}

function formatActorId(actor: ActorInfo): string {
  return `${actor.kind}: ${actor.id}`;
}

// ============================================================================
// Component
// ============================================================================

export function ReferenceBundleDetail(): JSX.Element {
  const { bundleId } = useParams<{ bundleId: string }>();
  const auth = useAuth();
  const [bundle, setBundle] = useState<EvidenceBundleDetailData | null>(null);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);
  const [showRawBundle, setShowRawBundle] = useState(false);

  useEffect(() => {
    if (!auth.user?.access_token || !bundleId) return;

    setLoading(true);
    setError(null);

    // bundleId is the content_hash when navigating from References page
    fetch(`${config.apiUrl}/api/v1/references/evidence-bundles/${encodeURIComponent(bundleId)}`, {
      headers: { Authorization: `Bearer ${auth.user.access_token}` },
    })
      .then((res) => {
        if (res.status === 404) {
          throw new Error('Bundle not found');
        }
        if (!res.ok) {
          throw new Error(`HTTP ${res.status}`);
        }
        return res.json();
      })
      .then((data: EvidenceBundleDetailData) => {
        setBundle(data);
        setLoading(false);
      })
      .catch((err) => {
        setError(err.message);
        setLoading(false);
      });
  }, [auth.user?.access_token, bundleId]);

  if (loading) {
    return (
      <div className={styles.container}>
        <div className={styles.breadcrumb}>
          <Link to="/references" className={styles.breadcrumbLink}>
            References
          </Link>
          <span className={styles.breadcrumbSeparator}>/</span>
          <span>Evidence Bundles</span>
          <span className={styles.breadcrumbSeparator}>/</span>
          <span>{truncateHash(bundleId || '', 12)}</span>
        </div>
        <div className={styles.placeholder}>
          <p className={styles.placeholderText}>Loading bundle details...</p>
        </div>
      </div>
    );
  }

  if (error || !bundle) {
    return (
      <div className={styles.container}>
        <div className={styles.breadcrumb}>
          <Link to="/references" className={styles.breadcrumbLink}>
            References
          </Link>
          <span className={styles.breadcrumbSeparator}>/</span>
          <span>Evidence Bundles</span>
          <span className={styles.breadcrumbSeparator}>/</span>
          <span>{truncateHash(bundleId || '', 12)}</span>
        </div>
        <div className={styles.placeholder}>
          <p className={styles.error}>Error: {error || 'Bundle not found'}</p>
          <Link to="/references" className={styles.link}>
            Back to References
          </Link>
        </div>
      </div>
    );
  }

  return (
    <div className={styles.container}>
      {/* Breadcrumb */}
      <div className={styles.breadcrumb}>
        <Link to="/references" className={styles.breadcrumbLink}>
          References
        </Link>
        <span className={styles.breadcrumbSeparator}>/</span>
        <span>Evidence Bundles</span>
        <span className={styles.breadcrumbSeparator}>/</span>
        <span>{truncateHash(bundle.content_hash, 12)}</span>
      </div>

      {/* Header */}
      <div className={styles.header}>
        <div className={styles.headerStart}>
          <h1 className={styles.title}>Evidence Bundle</h1>
          <p className={styles.subtitle}>{bundle.bundle_id}</p>
        </div>
        <div className={styles.badgeGroup}>
          <Pill tone={getVerdictTone(bundle.verdict)}>{bundle.verdict}</Pill>
          <Pill tone="neutral">{bundle.artifact_count} artifacts</Pill>
        </div>
      </div>

      {/* Overview Card */}
      <Card title="Overview" className={styles.cardSpacing}>
        <div className={styles.infoRow}>
          <span className={styles.infoLabel}>Bundle ID</span>
          <code className={styles.mono}>{bundle.bundle_id}</code>
        </div>
        <div className={styles.infoRow}>
          <span className={styles.infoLabel}>Content Hash</span>
          <code className={styles.mono}>{bundle.content_hash}</code>
        </div>
        <div className={styles.infoRow}>
          <span className={styles.infoLabel}>Verdict</span>
          <span className={styles.infoValue}>
            <Pill tone={getVerdictTone(bundle.verdict)}>{bundle.verdict}</Pill>
          </span>
        </div>
        <div className={styles.infoRow}>
          <span className={styles.infoLabel}>Artifact Count</span>
          <span className={styles.infoValue}>{bundle.artifact_count}</span>
        </div>
        <div className={styles.infoRow}>
          <span className={styles.infoLabel}>Run Completed</span>
          <span className={styles.infoValue}>
            {new Date(bundle.run_completed_at).toLocaleString()}
          </span>
        </div>
      </Card>

      {/* Linked Entities Card */}
      <Card title="Linked Entities" className={styles.cardSpacing}>
        <div className={styles.infoRow}>
          <span className={styles.infoLabel}>Run ID</span>
          <code className={styles.mono}>{bundle.run_id}</code>
        </div>
        <div className={styles.infoRow}>
          <span className={styles.infoLabel}>Candidate</span>
          <Link to={`/candidates/${bundle.candidate_id}`} className={styles.link}>
            {bundle.candidate_id}
          </Link>
        </div>
        {bundle.iteration_id && (
          <div className={styles.infoRow}>
            <span className={styles.infoLabel}>Iteration</span>
            <Link to={`/iterations/${bundle.iteration_id}`} className={styles.link}>
              {bundle.iteration_id}
            </Link>
          </div>
        )}
        <div className={styles.infoRow}>
          <span className={styles.infoLabel}>Oracle Suite</span>
          <Link to={`/oracles/suites/${bundle.oracle_suite_id}`} className={styles.link}>
            {bundle.oracle_suite_id}
          </Link>
        </div>
        <div className={styles.infoRow}>
          <span className={styles.infoLabel}>Suite Hash</span>
          <code className={styles.mono}>{truncateHash(bundle.oracle_suite_hash, 20)}</code>
        </div>
      </Card>

      {/* Recording Info Card */}
      <Card title="Recording Info" className={styles.cardSpacing}>
        <div className={styles.infoRow}>
          <span className={styles.infoLabel}>Recorded At</span>
          <span className={styles.infoValue}>
            {new Date(bundle.recorded_at).toLocaleString()}
          </span>
        </div>
        <div className={styles.infoRow}>
          <span className={styles.infoLabel}>Recorded By</span>
          <span className={styles.infoValue}>{formatActorId(bundle.recorded_by)}</span>
        </div>
      </Card>

      {/* Raw Bundle Card */}
      <Card
        title="Raw Bundle"
        right={
          <Button
            variant={showRawBundle ? 'ghost' : 'secondary'}
            onClick={() => setShowRawBundle(!showRawBundle)}
          >
            {showRawBundle ? 'Hide' : 'Show'}
          </Button>
        }
      >
        {showRawBundle && (
          <pre
            style={{
              backgroundColor: 'var(--ink)',
              color: '#e0e0e0',
              padding: 'var(--space4)',
              borderRadius: 'var(--radiusSm)',
              fontFamily: 'var(--mono)',
              fontSize: '0.75rem',
              overflow: 'auto',
              maxHeight: '400px',
              whiteSpace: 'pre-wrap',
              wordBreak: 'break-all',
              margin: 0,
            }}
          >
            {JSON.stringify(bundle, null, 2)}
          </pre>
        )}
      </Card>
    </div>
  );
}

export default ReferenceBundleDetail;
