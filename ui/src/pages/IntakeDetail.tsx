/**
 * Intake Detail Page
 *
 * Displays a single intake record with its full specification.
 * Per SR-WORK-SURFACE, intakes define the structured problem statement
 * for a work unit including objective, deliverables, constraints, and inputs.
 *
 * Includes lifecycle actions: Edit (draft), Activate (draft), Fork (active/archived), Archive (active).
 */

import { useState, useEffect, useCallback } from 'react';
import { useParams, Link, useNavigate } from 'react-router-dom';
import { useAuth } from '../auth/AuthProvider';
import config from '../config';
import { Card, Pill, getStatusTone, truncateHash } from '../ui';
import { IntakeLifecycleActions } from '../components/IntakeLifecycleActions';
import styles from '../styles/pages.module.css';

interface Deliverable {
  name: string;
  format: string;
  path: string;
  description?: string;
}

interface TypedRef {
  kind: string;
  id: string;
  rel: string;
  meta?: {
    content_hash?: string;
    version?: string;
    type_key?: string;
    selector?: string;
  };
  label?: string;
}

interface ActorInfo {
  kind: string;
  id: string;
}

interface IntakeDetail {
  intake_id: string;
  work_unit_id: string;
  content_hash: string | null;
  title: string;
  kind: string;
  objective: string;
  audience: string;
  deliverables: Deliverable[];
  constraints: string[];
  definitions: Record<string, string>;
  inputs: TypedRef[];
  unknowns: string[];
  completion_criteria: string[];
  status: 'draft' | 'active' | 'archived';
  version: number;
  supersedes: string | null;
  created_by: ActorInfo;
  created_at: string;
  activated_at: string | null;
  activated_by: ActorInfo | null;
  archived_at: string | null;
  archived_by: ActorInfo | null;
}

export function IntakeDetail(): JSX.Element {
  const { intakeId } = useParams<{ intakeId: string }>();
  const auth = useAuth();
  const navigate = useNavigate();
  const [intake, setIntake] = useState<IntakeDetail | null>(null);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);

  const fetchIntake = useCallback(async () => {
    if (!auth.user?.access_token || !intakeId) return;

    setLoading(true);
    setError(null);

    try {
      const res = await fetch(`${config.apiUrl}/api/v1/intakes/${intakeId}`, {
        headers: { Authorization: `Bearer ${auth.user.access_token}` },
      });

      if (res.status === 404) {
        throw new Error('Intake not found');
      }
      if (!res.ok) {
        throw new Error(`HTTP ${res.status}`);
      }

      const data = await res.json();
      setIntake(data);
    } catch (err) {
      setError(err instanceof Error ? err.message : 'Failed to load intake');
    } finally {
      setLoading(false);
    }
  }, [auth.user?.access_token, intakeId]);

  useEffect(() => {
    fetchIntake();
  }, [fetchIntake]);

  const handleAction = async (action: 'activate' | 'archive' | 'fork') => {
    if (!auth.user?.access_token || !intakeId) return;

    const res = await fetch(`${config.apiUrl}/api/v1/intakes/${intakeId}/${action}`, {
      method: 'POST',
      headers: {
        Authorization: `Bearer ${auth.user.access_token}`,
        'Content-Type': 'application/json',
      },
      body: action === 'archive' ? JSON.stringify({}) : undefined,
    });

    if (!res.ok) {
      const errorData = await res.json().catch(() => ({}));
      throw new Error(errorData.message || `Failed to ${action} intake`);
    }

    const data = await res.json();

    if (action === 'fork') {
      // Navigate to the new forked intake
      navigate(`/intakes/${data.intake_id}`);
    } else {
      // Refresh the current intake
      await fetchIntake();
    }
  };

  const getIntakeStatusTone = (status: string) => {
    switch (status) {
      case 'active':
        return 'success';
      case 'draft':
        return 'warning';
      case 'archived':
        return 'neutral';
      default:
        return getStatusTone(status);
    }
  };

  if (loading) {
    return (
      <div className={styles.container}>
        <div className={styles.placeholder}>
          <p className={styles.placeholderText}>Loading intake details...</p>
        </div>
      </div>
    );
  }

  if (error || !intake) {
    return (
      <div className={styles.container}>
        <div className={styles.placeholder}>
          <p className={styles.error}>Error: {error || 'Intake not found'}</p>
          <Link to="/intakes" className={styles.link}>
            Back to Intakes
          </Link>
        </div>
      </div>
    );
  }

  const definitionEntries = Object.entries(intake.definitions || {});

  return (
    <div className={styles.container}>
      {/* Breadcrumb */}
      <div className={styles.breadcrumb}>
        <Link to="/intakes" className={styles.breadcrumbLink}>
          Intakes
        </Link>
        <span className={styles.breadcrumbSeparator}>/</span>
        <span>{intake.title}</span>
      </div>

      {/* Header */}
      <div className={styles.header}>
        <div className={styles.headerStart}>
          <h1 className={styles.title}>{intake.title}</h1>
          <p className={styles.subtitle}>{intake.intake_id}</p>
        </div>
        <div className={styles.badgeGroup}>
          <Pill tone="neutral">{intake.kind.replace(/_/g, ' ')}</Pill>
          <Pill tone={getIntakeStatusTone(intake.status)}>{intake.status}</Pill>
        </div>
      </div>

      {/* Lifecycle Actions */}
      <div style={{ marginBottom: 'var(--space4)' }}>
        <IntakeLifecycleActions
          intakeId={intake.intake_id}
          status={intake.status}
          onAction={handleAction}
        />
      </div>

      {/* Overview Card */}
      <Card title="Overview" className={styles.cardSpacing}>
        <div className={styles.infoRow}>
          <span className={styles.infoLabel}>Intake ID</span>
          <code className={styles.mono}>{intake.intake_id}</code>
        </div>
        <div className={styles.infoRow}>
          <span className={styles.infoLabel}>Work Unit</span>
          <code className={styles.mono}>{intake.work_unit_id}</code>
        </div>
        <div className={styles.infoRow}>
          <span className={styles.infoLabel}>Kind</span>
          <Pill tone="neutral">{intake.kind.replace(/_/g, ' ')}</Pill>
        </div>
        <div className={styles.infoRow}>
          <span className={styles.infoLabel}>Status</span>
          <Pill tone={getIntakeStatusTone(intake.status)}>{intake.status}</Pill>
        </div>
        <div className={styles.infoRow}>
          <span className={styles.infoLabel}>Version</span>
          <span className={styles.infoValue}>{intake.version}</span>
        </div>
        {intake.content_hash && (
          <div className={styles.infoRow}>
            <span className={styles.infoLabel}>Content Hash</span>
            <code className={styles.mono}>{truncateHash(intake.content_hash, 20)}</code>
          </div>
        )}
        {intake.supersedes && (
          <div className={styles.infoRow}>
            <span className={styles.infoLabel}>Supersedes</span>
            <Link to={`/intakes/${intake.supersedes}`} className={styles.link}>
              <code className={styles.mono}>{truncateHash(intake.supersedes, 20)}</code>
            </Link>
          </div>
        )}
        <div className={styles.infoRow}>
          <span className={styles.infoLabel}>Created</span>
          <span className={styles.infoValue}>
            {new Date(intake.created_at).toLocaleString()} by {intake.created_by.id}
          </span>
        </div>
        {intake.activated_at && intake.activated_by && (
          <div className={styles.infoRow}>
            <span className={styles.infoLabel}>Activated</span>
            <span className={styles.infoValue}>
              {new Date(intake.activated_at).toLocaleString()} by {intake.activated_by.id}
            </span>
          </div>
        )}
        {intake.archived_at && intake.archived_by && (
          <div className={styles.infoRow}>
            <span className={styles.infoLabel}>Archived</span>
            <span className={styles.infoValue}>
              {new Date(intake.archived_at).toLocaleString()} by {intake.archived_by.id}
            </span>
          </div>
        )}
      </Card>

      {/* Objective Card - Prominent display */}
      <Card title="Objective" className={styles.cardSpacing}>
        <p
          style={{
            margin: 0,
            fontSize: '1rem',
            lineHeight: 1.6,
            fontWeight: 500,
            color: 'var(--ink)',
          }}
        >
          {intake.objective}
        </p>
      </Card>

      {/* Audience Card */}
      {intake.audience && (
        <Card title="Audience" className={styles.cardSpacing}>
          <p style={{ margin: 0, fontSize: '0.875rem', lineHeight: 1.6 }}>{intake.audience}</p>
        </Card>
      )}

      {/* Deliverables Card */}
      <Card title={`Deliverables (${intake.deliverables.length})`} className={styles.cardSpacing}>
        {intake.deliverables.length === 0 ? (
          <div className={styles.placeholder}>
            <p className={styles.placeholderText}>No deliverables defined.</p>
          </div>
        ) : (
          <table className={styles.table}>
            <thead>
              <tr>
                <th className={styles.th}>Name</th>
                <th className={styles.th}>Format</th>
                <th className={styles.th}>Path</th>
                <th className={styles.th}>Description</th>
              </tr>
            </thead>
            <tbody>
              {intake.deliverables.map((deliverable, idx) => (
                <tr key={idx}>
                  <td className={styles.td}>{deliverable.name}</td>
                  <td className={styles.td}>
                    <Pill tone="neutral">{deliverable.format}</Pill>
                  </td>
                  <td className={styles.tdMono}>{deliverable.path}</td>
                  <td className={styles.td}>{deliverable.description || '—'}</td>
                </tr>
              ))}
            </tbody>
          </table>
        )}
      </Card>

      {/* Constraints Card */}
      <Card title={`Constraints (${intake.constraints.length})`} className={styles.cardSpacing}>
        {intake.constraints.length === 0 ? (
          <div className={styles.placeholder}>
            <p className={styles.placeholderText}>No constraints defined.</p>
          </div>
        ) : (
          <ul style={{ margin: 0, paddingLeft: '1.25rem' }}>
            {intake.constraints.map((constraint, idx) => (
              <li key={idx} style={{ fontSize: '0.875rem', marginBottom: 'var(--space2)' }}>
                {constraint}
              </li>
            ))}
          </ul>
        )}
      </Card>

      {/* Definitions Card */}
      <Card title={`Definitions (${definitionEntries.length})`} className={styles.cardSpacing}>
        {definitionEntries.length === 0 ? (
          <div className={styles.placeholder}>
            <p className={styles.placeholderText}>No definitions provided.</p>
          </div>
        ) : (
          <div style={{ display: 'flex', flexDirection: 'column', gap: 'var(--space3)' }}>
            {definitionEntries.map(([term, definition]) => (
              <div key={term}>
                <dt
                  style={{
                    fontWeight: 600,
                    fontSize: '0.875rem',
                    marginBottom: 'var(--space1)',
                  }}
                >
                  {term}
                </dt>
                <dd style={{ margin: 0, fontSize: '0.875rem', color: 'var(--muted)' }}>
                  {definition}
                </dd>
              </div>
            ))}
          </div>
        )}
      </Card>

      {/* Inputs Card */}
      <Card title={`Inputs (${intake.inputs.length})`} className={styles.cardSpacing}>
        {intake.inputs.length === 0 ? (
          <div className={styles.placeholder}>
            <p className={styles.placeholderText}>No inputs defined.</p>
            <p className={styles.placeholderHint}>
              Per SR-WORK-SURFACE: inputs are content-addressed references.
            </p>
          </div>
        ) : (
          <table className={styles.table}>
            <thead>
              <tr>
                <th className={styles.th}>Kind</th>
                <th className={styles.th}>ID</th>
                <th className={styles.th}>Relation</th>
                <th className={styles.th}>Label</th>
              </tr>
            </thead>
            <tbody>
              {intake.inputs.map((input, idx) => (
                <tr key={idx}>
                  <td className={styles.td}>
                    <Pill tone="neutral">{input.kind}</Pill>
                  </td>
                  <td className={styles.tdMono}>{truncateHash(input.id, 20)}</td>
                  <td className={styles.td}>{input.rel}</td>
                  <td className={styles.td}>{input.label || '—'}</td>
                </tr>
              ))}
            </tbody>
          </table>
        )}
      </Card>

      {/* Unknowns Card */}
      <Card title={`Unknowns (${intake.unknowns.length})`} className={styles.cardSpacing}>
        {intake.unknowns.length === 0 ? (
          <div className={styles.placeholder}>
            <p className={styles.placeholderText}>No unknowns identified.</p>
          </div>
        ) : (
          <ul style={{ margin: 0, paddingLeft: '1.25rem' }}>
            {intake.unknowns.map((unknown, idx) => (
              <li key={idx} style={{ fontSize: '0.875rem', marginBottom: 'var(--space2)' }}>
                {unknown}
              </li>
            ))}
          </ul>
        )}
      </Card>

      {/* Completion Criteria Card */}
      <Card
        title={`Completion Criteria (${intake.completion_criteria.length})`}
        className={styles.cardSpacing}
      >
        {intake.completion_criteria.length === 0 ? (
          <div className={styles.placeholder}>
            <p className={styles.placeholderText}>No completion criteria defined.</p>
          </div>
        ) : (
          <ul style={{ margin: 0, paddingLeft: '1.25rem' }}>
            {intake.completion_criteria.map((criterion, idx) => (
              <li key={idx} style={{ fontSize: '0.875rem', marginBottom: 'var(--space2)' }}>
                {criterion}
              </li>
            ))}
          </ul>
        )}
      </Card>
    </div>
  );
}

export default IntakeDetail;
