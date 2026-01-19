/**
 * Protocol Detail Page
 *
 * Displays a single procedure template with its stages and oracle suites.
 * Per SR-PROCEDURE-KIT, procedure templates define stage-gated workflows
 * with required outputs and oracle suites at each stage.
 */

import { useState, useEffect } from 'react';
import { useParams, Link } from 'react-router-dom';
import { useAuth } from '../auth/AuthProvider';
import config from '../config';
import { Card, Pill, truncateHash } from '../ui';
import styles from '../styles/pages.module.css';

interface Stage {
  stage_id: string;
  stage_name: string;
  purpose: string;
  required_outputs: string[];
  required_oracle_suites: string[];
  gate_rule: string;
  transition_on_pass: string | null;
}

interface ActiveWorkUnit {
  work_unit_id: string;
  title: string;
  current_stage: string;
  status: string;
}

interface TemplateDetail {
  id: string;
  template_id: string;
  name: string;
  description: string;
  kind: string[];
  stages: Stage[];
  terminal_stage_id: string;
  version: string;
  content_hash: string;
  status: 'active' | 'draft' | 'deprecated';
  work_units_using: number;
  created_at: string;
  updated_at: string;
  active_work_units: ActiveWorkUnit[];
}

export function ProtocolDetail(): JSX.Element {
  const { templateId } = useParams<{ templateId: string }>();
  const auth = useAuth();
  const [template, setTemplate] = useState<TemplateDetail | null>(null);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);
  const [expandedStage, setExpandedStage] = useState<string | null>(null);

  useEffect(() => {
    if (!auth.user?.access_token || !templateId) return;

    fetch(`${config.apiUrl}/api/v1/templates/${templateId}`, {
      headers: { Authorization: `Bearer ${auth.user.access_token}` },
    })
      .then(res => {
        if (res.status === 404) {
          throw new Error('Protocol not found');
        }
        if (!res.ok) throw new Error(`HTTP ${res.status}`);
        return res.json();
      })
      .then(data => {
        setTemplate(data);
        setLoading(false);
      })
      .catch(err => {
        setError(err.message);
        setLoading(false);
      });
  }, [auth.user?.access_token, templateId]);

  const getStatusTone = (status: string) => {
    switch (status) {
      case 'active':
        return 'success';
      case 'draft':
        return 'warning';
      case 'deprecated':
        return 'danger';
      default:
        return 'neutral';
    }
  };

  if (loading) {
    return (
      <div className={styles.container}>
        <div className={styles.placeholder}>
          <p className={styles.placeholderText}>Loading protocol details...</p>
        </div>
      </div>
    );
  }

  if (error || !template) {
    return (
      <div className={styles.container}>
        <div className={styles.placeholder}>
          <p className={styles.error}>Error: {error || 'Protocol not found'}</p>
          <Link to="/protocols" className={styles.link}>Back to Protocols</Link>
        </div>
      </div>
    );
  }

  return (
    <div className={styles.container}>
      {/* Breadcrumb */}
      <div className={styles.breadcrumb}>
        <Link to="/protocols" className={styles.breadcrumbLink}>Protocols</Link>
        <span className={styles.breadcrumbSeparator}>/</span>
        <span>{template.name}</span>
      </div>

      {/* Header */}
      <div className={styles.header}>
        <div className={styles.headerStart}>
          <h1 className={styles.title}>{template.name}</h1>
          <p className={styles.subtitle}>{template.template_id}</p>
        </div>
        <Pill tone={getStatusTone(template.status)}>{template.status}</Pill>
      </div>

      {/* Overview Card */}
      <Card title="Overview" className={styles.cardSpacing}>
        <div className={styles.infoRow}>
          <span className={styles.infoLabel}>Template ID</span>
          <code className={styles.mono}>{template.template_id}</code>
        </div>
        <div className={styles.infoRow}>
          <span className={styles.infoLabel}>Version</span>
          <span className={styles.infoValue}>{template.version}</span>
        </div>
        <div className={styles.infoRow}>
          <span className={styles.infoLabel}>Content Hash</span>
          <code className={styles.mono}>{truncateHash(template.content_hash, 20)}</code>
        </div>
        <div className={styles.infoRow}>
          <span className={styles.infoLabel}>Status</span>
          <Pill tone={getStatusTone(template.status)}>{template.status}</Pill>
        </div>
        <div className={styles.infoRow}>
          <span className={styles.infoLabel}>Work Units Using</span>
          <span className={styles.infoValue}>{template.work_units_using}</span>
        </div>
        <div className={styles.infoRow}>
          <span className={styles.infoLabel}>Created</span>
          <span className={styles.infoValue}>
            {new Date(template.created_at).toLocaleString()}
          </span>
        </div>
        <div className={styles.infoRow}>
          <span className={styles.infoLabel}>Updated</span>
          <span className={styles.infoValue}>
            {new Date(template.updated_at).toLocaleString()}
          </span>
        </div>
      </Card>

      {/* Description Card */}
      <Card title="Description" className={styles.cardSpacing}>
        <p style={{ margin: 0, fontSize: '0.875rem', lineHeight: 1.6 }}>
          {template.description}
        </p>
      </Card>

      {/* Work Kinds Card */}
      <Card title="Supported Work Kinds" className={styles.cardSpacing}>
        {template.kind.length === 0 ? (
          <div className={styles.placeholder}>
            <p className={styles.placeholderText}>All work kinds supported.</p>
          </div>
        ) : (
          <div className={styles.badgeGroup}>
            {template.kind.map(k => (
              <Pill key={k} tone="neutral">{k}</Pill>
            ))}
          </div>
        )}
      </Card>

      {/* Stage Flow Visualization */}
      <Card title="Stage Flow" className={styles.cardSpacing}>
        <div style={{
          display: 'flex',
          alignItems: 'center',
          gap: 'var(--space2)',
          flexWrap: 'wrap',
          marginBottom: 'var(--space4)',
        }}>
          {template.stages.map((stage, idx) => (
            <div key={stage.stage_id} style={{ display: 'flex', alignItems: 'center', gap: 'var(--space2)' }}>
              <Pill tone={stage.stage_id === template.terminal_stage_id ? 'success' : 'neutral'}>
                {stage.stage_name}
              </Pill>
              {idx < template.stages.length - 1 && (
                <span style={{ color: 'var(--muted)', fontSize: '1.2rem' }}>→</span>
              )}
            </div>
          ))}
        </div>
        <p style={{ margin: 0, fontSize: '0.75rem', color: 'var(--muted)' }}>
          Terminal stage: <code style={{ fontFamily: 'var(--mono)' }}>{template.terminal_stage_id}</code>
        </p>
      </Card>

      {/* Stages Detail Card */}
      <Card title={`Stages (${template.stages.length})`} className={styles.cardSpacing}>
        {template.stages.map(stage => (
          <div
            key={stage.stage_id}
            style={{
              borderBottom: '1px solid var(--border)',
              paddingBottom: 'var(--space4)',
              marginBottom: 'var(--space4)',
            }}
          >
            {/* Stage Header */}
            <div
              style={{
                display: 'flex',
                justifyContent: 'space-between',
                alignItems: 'center',
                cursor: 'pointer',
              }}
              onClick={() => setExpandedStage(expandedStage === stage.stage_id ? null : stage.stage_id)}
            >
              <div style={{ display: 'flex', alignItems: 'center', gap: 'var(--space2)' }}>
                <code style={{ fontFamily: 'var(--mono)', fontSize: '0.875rem', fontWeight: 600 }}>
                  {stage.stage_id}
                </code>
                {stage.stage_id === template.terminal_stage_id && (
                  <Pill tone="success">terminal</Pill>
                )}
              </div>
              <span style={{ color: 'var(--muted)', fontSize: '0.875rem' }}>
                {expandedStage === stage.stage_id ? '▼' : '▶'}
              </span>
            </div>

            {/* Stage Name & Purpose */}
            <h4 style={{ margin: 'var(--space2) 0 var(--space1) 0', fontSize: '0.875rem' }}>
              {stage.stage_name}
            </h4>
            <p style={{ margin: '0 0 var(--space2) 0', fontSize: '0.8rem', color: 'var(--muted)' }}>
              {stage.purpose}
            </p>

            {/* Expanded Content */}
            {expandedStage === stage.stage_id && (
              <div style={{ marginTop: 'var(--space3)', paddingLeft: 'var(--space3)' }}>
                {/* Required Outputs */}
                <div style={{ marginBottom: 'var(--space3)' }}>
                  <h5 style={{ margin: '0 0 var(--space1) 0', fontSize: '0.75rem', color: 'var(--muted)', textTransform: 'uppercase' }}>
                    Required Outputs ({stage.required_outputs.length})
                  </h5>
                  {stage.required_outputs.length === 0 ? (
                    <span style={{ fontSize: '0.8rem', color: 'var(--muted)' }}>None</span>
                  ) : (
                    <ul style={{ margin: 0, paddingLeft: '1rem' }}>
                      {stage.required_outputs.map((output, idx) => (
                        <li key={idx} style={{ fontFamily: 'var(--mono)', fontSize: '0.75rem' }}>
                          {output}
                        </li>
                      ))}
                    </ul>
                  )}
                </div>

                {/* Oracle Suites */}
                <div style={{ marginBottom: 'var(--space3)' }}>
                  <h5 style={{ margin: '0 0 var(--space1) 0', fontSize: '0.75rem', color: 'var(--muted)', textTransform: 'uppercase' }}>
                    Oracle Suites ({stage.required_oracle_suites.length})
                  </h5>
                  {stage.required_oracle_suites.length === 0 ? (
                    <span style={{ fontSize: '0.8rem', color: 'var(--muted)' }}>None required</span>
                  ) : (
                    <div style={{ display: 'flex', flexDirection: 'column', gap: 'var(--space1)' }}>
                      {stage.required_oracle_suites.map((suite, idx) => (
                        <code key={idx} style={{ fontFamily: 'var(--mono)', fontSize: '0.75rem' }}>
                          {suite}
                        </code>
                      ))}
                    </div>
                  )}
                </div>

                {/* Gate Rule & Transition */}
                <div style={{ display: 'flex', gap: 'var(--space4)' }}>
                  <div>
                    <h5 style={{ margin: '0 0 var(--space1) 0', fontSize: '0.75rem', color: 'var(--muted)', textTransform: 'uppercase' }}>
                      Gate Rule
                    </h5>
                    <code style={{ fontFamily: 'var(--mono)', fontSize: '0.75rem' }}>
                      {stage.gate_rule}
                    </code>
                  </div>
                  <div>
                    <h5 style={{ margin: '0 0 var(--space1) 0', fontSize: '0.75rem', color: 'var(--muted)', textTransform: 'uppercase' }}>
                      Transition on Pass
                    </h5>
                    <code style={{ fontFamily: 'var(--mono)', fontSize: '0.75rem' }}>
                      {stage.transition_on_pass || 'terminal'}
                    </code>
                  </div>
                </div>
              </div>
            )}
          </div>
        ))}
      </Card>

      {/* Active Work Units Card */}
      <Card title={`Active Work Units (${template.active_work_units.length})`}>
        {template.active_work_units.length === 0 ? (
          <div className={styles.placeholder}>
            <p className={styles.placeholderText}>No work units currently using this protocol.</p>
          </div>
        ) : (
          <table className={styles.table}>
            <thead>
              <tr>
                <th className={styles.th}>Work Unit</th>
                <th className={styles.th}>Title</th>
                <th className={styles.th}>Current Stage</th>
                <th className={styles.th}>Status</th>
              </tr>
            </thead>
            <tbody>
              {template.active_work_units.map(wu => (
                <tr key={wu.work_unit_id}>
                  <td className={styles.td}>
                    <Link to={`/loops/${wu.work_unit_id}`} className={styles.link}>
                      {wu.work_unit_id}
                    </Link>
                  </td>
                  <td className={styles.td}>{wu.title}</td>
                  <td className={styles.tdMono}>{wu.current_stage}</td>
                  <td className={styles.td}>
                    <Pill tone={getStatusTone(wu.status)}>{wu.status}</Pill>
                  </td>
                </tr>
              ))}
            </tbody>
          </table>
        )}
      </Card>
    </div>
  );
}

export default ProtocolDetail;
