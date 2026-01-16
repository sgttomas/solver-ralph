/**
 * Governed Artifact Detail Page
 *
 * Displays a single governed artifact (SR-* document) with its metadata.
 * Per SR-SPEC §3.2.1.1, governed artifacts are normative governance documents
 * that define the system's contracts, specifications, and types.
 *
 * Fetches from: GET /api/v1/references/governed-artifacts/:id
 */

import { useState, useEffect } from 'react';
import { useParams, Link } from 'react-router-dom';
import { useAuth } from '../auth/AuthProvider';
import config from '../config';
import { Card, Pill } from '../ui';
import styles from '../styles/pages.module.css';

// ============================================================================
// Types
// ============================================================================

interface ActorInfo {
  kind: string;
  id: string;
}

interface GovernedArtifactDetailData {
  artifact_id: string;
  artifact_type: string;
  version: string;
  content_hash: string;
  status: string;
  normative_status: string;
  authority_kind: string;
  governed_by: string[];
  tags: string[];
  supersedes: string[];
  is_current: boolean;
  recorded_at: string;
  recorded_by: ActorInfo;
}

// ============================================================================
// Helpers
// ============================================================================

function getStatusTone(status: string): 'neutral' | 'success' | 'warning' | 'danger' {
  switch (status.toLowerCase()) {
    case 'active':
    case 'governed':
      return 'success';
    case 'draft':
      return 'warning';
    case 'archived':
    case 'superseded':
      return 'neutral';
    default:
      return 'neutral';
  }
}

function getNormativeStatusTone(status: string): 'neutral' | 'success' | 'warning' | 'danger' {
  switch (status.toLowerCase()) {
    case 'normative':
      return 'success';
    case 'informative':
      return 'neutral';
    case 'deprecated':
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

export function GovernedArtifactDetail(): JSX.Element {
  const { artifactId } = useParams<{ artifactId: string }>();
  const auth = useAuth();
  const [artifact, setArtifact] = useState<GovernedArtifactDetailData | null>(null);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);

  useEffect(() => {
    if (!auth.user?.access_token || !artifactId) return;

    setLoading(true);
    setError(null);

    fetch(`${config.apiUrl}/api/v1/references/governed-artifacts/${encodeURIComponent(artifactId)}`, {
      headers: { Authorization: `Bearer ${auth.user.access_token}` },
    })
      .then((res) => {
        if (res.status === 404) {
          throw new Error('Artifact not found');
        }
        if (!res.ok) {
          throw new Error(`HTTP ${res.status}`);
        }
        return res.json();
      })
      .then((data: GovernedArtifactDetailData) => {
        setArtifact(data);
        setLoading(false);
      })
      .catch((err) => {
        setError(err.message);
        setLoading(false);
      });
  }, [auth.user?.access_token, artifactId]);

  if (loading) {
    return (
      <div className={styles.container}>
        <div className={styles.breadcrumb}>
          <Link to="/references" className={styles.breadcrumbLink}>
            References
          </Link>
          <span className={styles.breadcrumbSeparator}>/</span>
          <span>Governed Artifacts</span>
          <span className={styles.breadcrumbSeparator}>/</span>
          <span>{artifactId}</span>
        </div>
        <div className={styles.placeholder}>
          <p className={styles.placeholderText}>Loading artifact details...</p>
        </div>
      </div>
    );
  }

  if (error || !artifact) {
    return (
      <div className={styles.container}>
        <div className={styles.breadcrumb}>
          <Link to="/references" className={styles.breadcrumbLink}>
            References
          </Link>
          <span className={styles.breadcrumbSeparator}>/</span>
          <span>Governed Artifacts</span>
          <span className={styles.breadcrumbSeparator}>/</span>
          <span>{artifactId}</span>
        </div>
        <div className={styles.placeholder}>
          <p className={styles.error}>Error: {error || 'Artifact not found'}</p>
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
        <span>Governed Artifacts</span>
        <span className={styles.breadcrumbSeparator}>/</span>
        <span>{artifact.artifact_id}</span>
      </div>

      {/* Header */}
      <div className={styles.header}>
        <div className={styles.headerStart}>
          <h1 className={styles.title}>{artifact.artifact_id}</h1>
          <p className={styles.subtitle}>{artifact.artifact_type}</p>
        </div>
        <div className={styles.badgeGroup}>
          <Pill tone={getStatusTone(artifact.status)}>{artifact.status}</Pill>
          <Pill tone={getNormativeStatusTone(artifact.normative_status)}>
            {artifact.normative_status}
          </Pill>
          {artifact.is_current && <Pill tone="success">Current</Pill>}
        </div>
      </div>

      {/* Overview Card */}
      <Card title="Overview" className={styles.cardSpacing}>
        <div className={styles.infoRow}>
          <span className={styles.infoLabel}>Artifact ID</span>
          <code className={styles.mono}>{artifact.artifact_id}</code>
        </div>
        <div className={styles.infoRow}>
          <span className={styles.infoLabel}>Type</span>
          <span className={styles.infoValue}>{artifact.artifact_type}</span>
        </div>
        <div className={styles.infoRow}>
          <span className={styles.infoLabel}>Version</span>
          <span className={styles.infoValue}>{artifact.version}</span>
        </div>
        <div className={styles.infoRow}>
          <span className={styles.infoLabel}>Content Hash</span>
          <code className={styles.mono}>{artifact.content_hash}</code>
        </div>
        <div className={styles.infoRow}>
          <span className={styles.infoLabel}>Status</span>
          <span className={styles.infoValue}>
            <Pill tone={getStatusTone(artifact.status)}>{artifact.status}</Pill>
          </span>
        </div>
        <div className={styles.infoRow}>
          <span className={styles.infoLabel}>Normative Status</span>
          <span className={styles.infoValue}>
            <Pill tone={getNormativeStatusTone(artifact.normative_status)}>
              {artifact.normative_status}
            </Pill>
          </span>
        </div>
        <div className={styles.infoRow}>
          <span className={styles.infoLabel}>Authority Kind</span>
          <span className={styles.infoValue}>{artifact.authority_kind}</span>
        </div>
        <div className={styles.infoRow}>
          <span className={styles.infoLabel}>Is Current</span>
          <span className={styles.infoValue}>{artifact.is_current ? 'Yes' : 'No'}</span>
        </div>
      </Card>

      {/* Tags Card */}
      {artifact.tags.length > 0 && (
        <Card title="Tags" className={styles.cardSpacing}>
          <div className={styles.badgeGroup}>
            {artifact.tags.map((tag) => (
              <Pill key={tag} tone="neutral">
                {tag}
              </Pill>
            ))}
          </div>
        </Card>
      )}

      {/* Governed By Card */}
      {artifact.governed_by.length > 0 && (
        <Card title="Governed By" className={styles.cardSpacing}>
          <ul className={styles.refList}>
            {artifact.governed_by.map((gov) => (
              <li key={gov} className={styles.refItem}>
                <Link to={`/references/governed-artifacts/${encodeURIComponent(gov)}`} className={styles.link}>
                  {gov}
                </Link>
              </li>
            ))}
          </ul>
        </Card>
      )}

      {/* Supersedes Card */}
      {artifact.supersedes.length > 0 && (
        <Card title="Supersedes" className={styles.cardSpacing}>
          <ul className={styles.refList}>
            {artifact.supersedes.map((sup) => (
              <li key={sup} className={styles.refItem}>
                <Link to={`/references/governed-artifacts/${encodeURIComponent(sup)}`} className={styles.link}>
                  {sup}
                </Link>
              </li>
            ))}
          </ul>
        </Card>
      )}

      {/* Recording Info Card */}
      <Card title="Recording Info">
        <div className={styles.infoRow}>
          <span className={styles.infoLabel}>Recorded At</span>
          <span className={styles.infoValue}>
            {new Date(artifact.recorded_at).toLocaleString()}
          </span>
        </div>
        <div className={styles.infoRow}>
          <span className={styles.infoLabel}>Recorded By</span>
          <span className={styles.infoValue}>{formatActorId(artifact.recorded_by)}</span>
        </div>
      </Card>

      {/* Info Note */}
      <div className={styles.note} style={{ marginTop: 'var(--space5)' }}>
        Per SR-SPEC §3.2.1.1: Governed artifacts are normative governance documents including
        SR-CONTRACT, SR-SPEC, SR-TYPES, SR-WORK-SURFACE, and other SR-* documents.
      </div>
    </div>
  );
}

export default GovernedArtifactDetail;
