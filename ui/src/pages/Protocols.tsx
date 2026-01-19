/**
 * Protocols Page
 *
 * Displays registered Procedure Templates for semantic knowledge work.
 * Per SR-PROCEDURE-KIT, procedure templates define stage-gated workflows
 * with required outputs and oracle suites at each stage.
 *
 * A Procedure Template includes:
 * - template_id (proc:<NAME>)
 * - supported work kinds
 * - stages[] with required outputs, oracle suites, gate rules
 * - terminal_stage_id
 */

import { useState, useEffect } from 'react';
import { Link } from 'react-router-dom';
import { useAuth } from '../auth/AuthProvider';
import config from '../config';
import { Card, Pill } from '../ui';
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

interface Template {
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
}

interface ProtocolsResponse {
  templates: Template[];
}

export function Protocols(): JSX.Element {
  const auth = useAuth();
  const [templates, setTemplates] = useState<Template[]>([]);
  const [selectedTemplate, setSelectedTemplate] = useState<Template | null>(null);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);

  useEffect(() => {
    if (!auth.user?.access_token) return;

    fetch(`${config.apiUrl}/api/v1/templates`, {
      headers: {
        Authorization: `Bearer ${auth.user.access_token}`,
      },
    })
      .then(res => {
        // Treat 404 as "no data yet" rather than an error
        if (res.status === 404) {
          return { templates: [] };
        }
        if (!res.ok) throw new Error(`HTTP ${res.status}`);
        return res.json();
      })
      .then((data: ProtocolsResponse) => {
        setTemplates(data.templates || []);
        setLoading(false);
      })
      .catch(err => {
        setError(err.message);
        setLoading(false);
      });
  }, [auth.user?.access_token]);

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

  return (
    <div className={styles.container}>
      <div className={styles.header}>
        <div className={styles.headerStart}>
          <h1 className={styles.title}>Protocols</h1>
          <p className={styles.subtitle}>Stage-gated procedure templates for semantic work</p>
        </div>
      </div>

      {/* Overview */}
      <Card>
        <div className={styles.statsGrid}>
          <div className={styles.stat}>
            <div className={styles.statLabel}>Type</div>
            <div className={styles.statValue}>config.template</div>
          </div>
          <div className={styles.stat}>
            <div className={styles.statLabel}>Registered</div>
            <div className={styles.statValue}>{templates.length}</div>
          </div>
          <div className={styles.stat}>
            <div className={styles.statLabel}>Active</div>
            <div className={styles.statValue}>
              {templates.filter(t => t.status === 'active').length}
            </div>
          </div>
          <div className={styles.stat}>
            <div className={styles.statLabel}>Default Gate Rule</div>
            <div className={styles.statValue}>all_oracles_pass</div>
          </div>
        </div>
      </Card>

      {/* Info Note */}
      <div className={styles.note}>
        Per SR-PROCEDURE-KIT: Procedure templates proceduralize candidate generation so that
        semantic oracle suites can be attached with specificity. Each stage declares required
        outputs and oracle suites. Stage completion is computed from recorded evidence bundles.
      </div>

      {/* Templates List */}
      <Card>
        {loading ? (
          <div className={styles.placeholder}>
            <p className={styles.placeholderText}>Loading procedure templates...</p>
          </div>
        ) : error ? (
          <div className={styles.placeholder}>
            <p className={styles.error}>Error: {error}</p>
          </div>
        ) : templates.length === 0 ? (
          <div className={styles.placeholder}>
            <p className={styles.placeholderText}>No procedure templates registered.</p>
            <p className={styles.placeholderHint}>
              Procedure templates define stage-gated workflows for semantic knowledge work.
              Register templates via the governance change process.
            </p>
          </div>
        ) : (
          <table className={styles.table}>
            <thead>
              <tr>
                <th className={styles.th}>Template ID</th>
                <th className={styles.th}>Name</th>
                <th className={styles.th}>Work Kinds</th>
                <th className={styles.th}>Stages</th>
                <th className={styles.th}>Status</th>
                <th className={styles.th}>In Use</th>
                <th className={styles.th}>Actions</th>
              </tr>
            </thead>
            <tbody>
              {templates.map(template => (
                <tr key={template.id}>
                  <td className={styles.tdMono}>{template.template_id}</td>
                  <td className={styles.td}>
                    <Link to={`/protocols/${template.id}`} className={styles.link}>
                      {template.name}
                    </Link>
                  </td>
                  <td className={styles.td}>
                    <div className={styles.badgeGroup}>
                      {template.kind.slice(0, 2).map(k => (
                        <Pill key={k} tone="neutral">{k}</Pill>
                      ))}
                      {template.kind.length > 2 && (
                        <Pill tone="neutral">+{template.kind.length - 2}</Pill>
                      )}
                    </div>
                  </td>
                  <td className={styles.td}>{template.stages.length} stages</td>
                  <td className={styles.td}>
                    <Pill tone={getStatusTone(template.status)}>{template.status}</Pill>
                  </td>
                  <td className={styles.td}>{template.work_units_using}</td>
                  <td className={styles.td}>
                    <button
                      className={styles.actionLink}
                      onClick={() => setSelectedTemplate(
                        selectedTemplate?.id === template.id ? null : template
                      )}
                    >
                      {selectedTemplate?.id === template.id ? 'Hide' : 'View'} Stages
                    </button>
                  </td>
                </tr>
              ))}
            </tbody>
          </table>
        )}
      </Card>

      {/* Selected Template Stage Detail */}
      {selectedTemplate && (
        <Card>
          <h3 style={{ margin: '0 0 1rem 0', fontSize: '1rem', color: 'var(--ink)' }}>
            {selectedTemplate.name} — Stage Flow
          </h3>
          <p style={{ margin: '0 0 1.5rem 0', fontSize: '0.875rem', color: 'var(--muted)' }}>
            {selectedTemplate.description}
          </p>

          {/* Stage Flow Visualization */}
          <div style={{
            display: 'flex',
            alignItems: 'center',
            gap: '0.5rem',
            marginBottom: '1.5rem',
            flexWrap: 'wrap'
          }}>
            {selectedTemplate.stages.map((stage, idx) => (
              <div key={stage.stage_id} style={{ display: 'flex', alignItems: 'center', gap: '0.5rem' }}>
                <Pill tone={stage.stage_id === selectedTemplate.terminal_stage_id ? 'success' : 'neutral'}>
                  {stage.stage_name}
                </Pill>
                {idx < selectedTemplate.stages.length - 1 && (
                  <span style={{ color: 'var(--muted)' }}>→</span>
                )}
              </div>
            ))}
          </div>

          {/* Stages Detail Table */}
          <table className={styles.table}>
            <thead>
              <tr>
                <th className={styles.th}>Stage</th>
                <th className={styles.th}>Purpose</th>
                <th className={styles.th}>Required Outputs</th>
                <th className={styles.th}>Oracle Suites</th>
                <th className={styles.th}>Gate Rule</th>
              </tr>
            </thead>
            <tbody>
              {selectedTemplate.stages.map(stage => (
                <tr key={stage.stage_id}>
                  <td className={styles.tdMono}>
                    <span style={{ display: 'flex', alignItems: 'center', gap: '0.5rem' }}>
                      {stage.stage_id}
                      {stage.stage_id === selectedTemplate.terminal_stage_id && (
                        <Pill tone="success">terminal</Pill>
                      )}
                    </span>
                  </td>
                  <td className={styles.td} style={{ maxWidth: '250px' }}>
                    {stage.purpose}
                  </td>
                  <td className={styles.td}>
                    <ul className={styles.bulletList} style={{ margin: 0, paddingLeft: '1rem' }}>
                      {stage.required_outputs.slice(0, 3).map(output => (
                        <li key={output} style={{ fontSize: '0.75rem', fontFamily: 'var(--mono)' }}>
                          {output}
                        </li>
                      ))}
                      {stage.required_outputs.length > 3 && (
                        <li style={{ fontSize: '0.75rem', color: 'var(--muted)' }}>
                          +{stage.required_outputs.length - 3} more
                        </li>
                      )}
                    </ul>
                  </td>
                  <td className={styles.td}>
                    <div style={{ display: 'flex', flexDirection: 'column', gap: '0.25rem' }}>
                      {stage.required_oracle_suites.map(suite => (
                        <code key={suite} className={styles.mono}>{suite}</code>
                      ))}
                    </div>
                  </td>
                  <td className={styles.tdMono}>{stage.gate_rule}</td>
                </tr>
              ))}
            </tbody>
          </table>
        </Card>
      )}

      {/* Baseline Template Reference */}
      <Card>
        <h3 style={{ margin: '0 0 1rem 0', fontSize: '0.875rem', color: 'var(--ink)' }}>
          Baseline Template: GENERIC-KNOWLEDGE-WORK (SR-PROCEDURE-KIT)
        </h3>
        <p style={{ margin: '0 0 1rem 0', fontSize: '0.875rem', color: 'var(--muted)' }}>
          The baseline template provides a consistent stage structure across many kinds of
          knowledge work while allowing different semantic manifolds per stage.
        </p>
        <div style={{
          display: 'flex',
          alignItems: 'center',
          gap: '0.5rem',
          flexWrap: 'wrap'
        }}>
          <Pill tone="neutral">FRAME</Pill>
          <span style={{ color: 'var(--muted)' }}>→</span>
          <Pill tone="neutral">OPTIONS</Pill>
          <span style={{ color: 'var(--muted)' }}>→</span>
          <Pill tone="neutral">DRAFT</Pill>
          <span style={{ color: 'var(--muted)' }}>→</span>
          <Pill tone="neutral">SEMANTIC_EVAL</Pill>
          <span style={{ color: 'var(--muted)' }}>→</span>
          <Pill tone="success">FINAL</Pill>
        </div>
        <table className={styles.table} style={{ marginTop: '1rem' }}>
          <thead>
            <tr>
              <th className={styles.th}>Stage</th>
              <th className={styles.th}>Purpose</th>
              <th className={styles.th}>Example Oracle Suites</th>
            </tr>
          </thead>
          <tbody>
            <tr>
              <td className={styles.tdMono}>stage:FRAME</td>
              <td className={styles.td}>Restate objective, audience, non-goals; extract constraints</td>
              <td className={styles.tdMono}>suite:SR-SUITE-STRUCTURE</td>
            </tr>
            <tr>
              <td className={styles.tdMono}>stage:OPTIONS</td>
              <td className={styles.td}>Generate multiple candidate approaches before drafting</td>
              <td className={styles.tdMono}>suite:SR-SUITE-NONTRIVIALITY</td>
            </tr>
            <tr>
              <td className={styles.tdMono}>stage:DRAFT</td>
              <td className={styles.td}>Produce candidate deliverables and traceability artifacts</td>
              <td className={styles.tdMono}>suite:SR-SUITE-TRACEABILITY</td>
            </tr>
            <tr>
              <td className={styles.tdMono}>stage:SEMANTIC_EVAL</td>
              <td className={styles.td}>Evaluate against stage manifold; capture semantic measurements</td>
              <td className={styles.tdMono}>suite:SR-SUITE-SEMANTIC:*</td>
            </tr>
            <tr>
              <td className={styles.tdMono}>stage:FINAL</td>
              <td className={styles.td}>Package final candidate; ensure evidence bundle completeness</td>
              <td className={styles.tdMono}>suite:SR-SUITE-REFS</td>
            </tr>
          </tbody>
        </table>
      </Card>
    </div>
  );
}

export default Protocols;
