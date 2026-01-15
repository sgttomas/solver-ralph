/**
 * Intake Detail Page
 *
 * Displays a single intake record with its full specification.
 * Per SR-WORK-SURFACE, intakes define the structured problem statement
 * for a work unit including objective, deliverables, constraints, and inputs.
 */

import { useState, useEffect } from 'react';
import { useParams, Link } from 'react-router-dom';
import { useAuth } from '../auth/AuthProvider';
import config from '../config';
import { Card, Pill, getStatusTone, truncateHash } from '../ui';
import styles from '../styles/pages.module.css';

interface Deliverable {
  name: string;
  format: string;
  path: string;
  required: boolean;
}

interface Constraint {
  type: string;
  value: string;
}

interface InputRef {
  ref: string;
  rel: string;
  description: string | null;
}

interface IntakeDetail {
  id: string;
  work_unit_id: string;
  title: string;
  kind: string;
  objective: string;
  content_hash: string;
  status: 'draft' | 'active' | 'archived';
  created_at: string;
  updated_at: string;
  deliverables: Deliverable[];
  constraints: Constraint[];
  inputs: InputRef[];
  non_goals: string[];
  audience: string | null;
}

export function IntakeDetail(): JSX.Element {
  const { intakeId } = useParams<{ intakeId: string }>();
  const auth = useAuth();
  const [intake, setIntake] = useState<IntakeDetail | null>(null);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);

  useEffect(() => {
    if (!auth.user?.access_token || !intakeId) return;

    fetch(`${config.apiUrl}/api/v1/context/intakes/${intakeId}`, {
      headers: { Authorization: `Bearer ${auth.user.access_token}` },
    })
      .then(res => {
        if (res.status === 404) {
          throw new Error('Intake not found');
        }
        if (!res.ok) throw new Error(`HTTP ${res.status}`);
        return res.json();
      })
      .then(data => {
        setIntake(data);
        setLoading(false);
      })
      .catch(err => {
        setError(err.message);
        setLoading(false);
      });
  }, [auth.user?.access_token, intakeId]);

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

  // Group constraints by type
  const groupedConstraints = (intake?.constraints || []).reduce((acc, constraint) => {
    if (!acc[constraint.type]) {
      acc[constraint.type] = [];
    }
    acc[constraint.type].push(constraint.value);
    return acc;
  }, {} as Record<string, string[]>);

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
          <Link to="/context" className={styles.link}>Back to Context</Link>
        </div>
      </div>
    );
  }

  return (
    <div className={styles.container}>
      {/* Breadcrumb */}
      <div className={styles.breadcrumb}>
        <Link to="/context" className={styles.breadcrumbLink}>Context</Link>
        <span className={styles.breadcrumbSeparator}>/</span>
        <span>Intakes</span>
        <span className={styles.breadcrumbSeparator}>/</span>
        <span>{intake.title}</span>
      </div>

      {/* Header */}
      <div className={styles.header}>
        <div className={styles.headerStart}>
          <h1 className={styles.title}>{intake.title}</h1>
          <p className={styles.subtitle}>{intake.id}</p>
        </div>
        <div className={styles.badgeGroup}>
          <Pill tone="neutral">{intake.kind}</Pill>
          <Pill tone={getIntakeStatusTone(intake.status)}>{intake.status}</Pill>
        </div>
      </div>

      {/* Overview Card */}
      <Card title="Overview" className={styles.cardSpacing}>
        <div className={styles.infoRow}>
          <span className={styles.infoLabel}>Intake ID</span>
          <code className={styles.mono}>{intake.id}</code>
        </div>
        <div className={styles.infoRow}>
          <span className={styles.infoLabel}>Work Unit</span>
          <Link to={`/loops/${intake.work_unit_id}`} className={styles.link}>
            {intake.work_unit_id}
          </Link>
        </div>
        <div className={styles.infoRow}>
          <span className={styles.infoLabel}>Kind</span>
          <Pill tone="neutral">{intake.kind}</Pill>
        </div>
        <div className={styles.infoRow}>
          <span className={styles.infoLabel}>Status</span>
          <Pill tone={getIntakeStatusTone(intake.status)}>{intake.status}</Pill>
        </div>
        <div className={styles.infoRow}>
          <span className={styles.infoLabel}>Content Hash</span>
          <code className={styles.mono}>{intake.content_hash}</code>
        </div>
        <div className={styles.infoRow}>
          <span className={styles.infoLabel}>Created</span>
          <span className={styles.infoValue}>
            {new Date(intake.created_at).toLocaleString()}
          </span>
        </div>
        <div className={styles.infoRow}>
          <span className={styles.infoLabel}>Updated</span>
          <span className={styles.infoValue}>
            {new Date(intake.updated_at).toLocaleString()}
          </span>
        </div>
      </Card>

      {/* Objective Card - Prominent display */}
      <Card title="Objective" className={styles.cardSpacing}>
        <p style={{
          margin: 0,
          fontSize: '1rem',
          lineHeight: 1.6,
          fontWeight: 500,
          color: 'var(--ink)',
        }}>
          {intake.objective}
        </p>
      </Card>

      {/* Audience Card */}
      {intake.audience && (
        <Card title="Audience" className={styles.cardSpacing}>
          <p style={{ margin: 0, fontSize: '0.875rem', lineHeight: 1.6 }}>
            {intake.audience}
          </p>
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
                <th className={styles.th}>Required</th>
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
                  <td className={styles.td}>
                    <Pill tone={deliverable.required ? 'warning' : 'neutral'}>
                      {deliverable.required ? 'required' : 'optional'}
                    </Pill>
                  </td>
                </tr>
              ))}
            </tbody>
          </table>
        )}
      </Card>

      {/* Constraints Card - Grouped by type */}
      <Card title={`Constraints (${intake.constraints.length})`} className={styles.cardSpacing}>
        {intake.constraints.length === 0 ? (
          <div className={styles.placeholder}>
            <p className={styles.placeholderText}>No constraints defined.</p>
          </div>
        ) : (
          <div style={{ display: 'flex', flexDirection: 'column', gap: 'var(--space4)' }}>
            {Object.entries(groupedConstraints).map(([type, values]) => (
              <div key={type}>
                <h4 style={{
                  margin: '0 0 var(--space2) 0',
                  fontSize: '0.75rem',
                  fontWeight: 600,
                  textTransform: 'uppercase',
                  color: 'var(--muted)',
                }}>
                  {type}
                </h4>
                <ul style={{ margin: 0, paddingLeft: '1.25rem' }}>
                  {values.map((value, idx) => (
                    <li key={idx} style={{ fontSize: '0.875rem', marginBottom: 'var(--space1)' }}>
                      {value}
                    </li>
                  ))}
                </ul>
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
                <th className={styles.th}>Reference</th>
                <th className={styles.th}>Relation</th>
                <th className={styles.th}>Description</th>
              </tr>
            </thead>
            <tbody>
              {intake.inputs.map((input, idx) => (
                <tr key={idx}>
                  <td className={styles.tdMono}>{truncateHash(input.ref, 20)}</td>
                  <td className={styles.td}>{input.rel}</td>
                  <td className={styles.td}>{input.description || '—'}</td>
                </tr>
              ))}
            </tbody>
          </table>
        )}
      </Card>

      {/* Non-Goals Card */}
      {intake.non_goals.length > 0 && (
        <Card title={`Non-Goals (${intake.non_goals.length})`}>
          <ul style={{ margin: 0, paddingLeft: '1.25rem' }}>
            {intake.non_goals.map((nonGoal, idx) => (
              <li key={idx} style={{ fontSize: '0.875rem', marginBottom: 'var(--space2)' }}>
                {nonGoal}
              </li>
            ))}
          </ul>
        </Card>
      )}
    </div>
  );
}

export default IntakeDetail;
