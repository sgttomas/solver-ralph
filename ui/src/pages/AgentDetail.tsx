/**
 * Agent Detail Page
 *
 * Displays a single agent definition with its capabilities and constraints.
 * Per SR-AGENTS, agents are actor-kind AGENT: non-authoritative workers
 * that produce proposals. They cannot create binding records.
 */

import { useState, useEffect } from 'react';
import { useParams, Link } from 'react-router-dom';
import { useAuth } from '../auth/AuthProvider';
import config from '../config';
import { Card, Pill, getStatusTone } from '../ui';
import styles from '../styles/pages.module.css';

interface WorkEnvelope {
  allowed_kinds: string[];
  max_concurrent_work_units: number;
  timeout_hours: number;
}

interface TrustConstraint {
  constraint_id: string;
  boundary: string;
  description: string;
  enforcement: string;
}

interface RecentProposal {
  id: string;
  work_unit_id: string;
  created_at: string;
  status: string;
}

interface AgentDetail {
  id: string;
  name: string;
  actor_id: string;
  status: 'active' | 'inactive' | 'suspended';
  description: string | null;
  capabilities: string[];
  current_work_unit_id: string | null;
  iterations_completed: number;
  proposals_produced: number;
  last_active_at: string | null;
  created_at: string;
  work_envelope: WorkEnvelope;
  trust_constraints: TrustConstraint[];
  recent_proposals: RecentProposal[];
}

export function AgentDetail(): JSX.Element {
  const { agentId } = useParams<{ agentId: string }>();
  const auth = useAuth();
  const [agent, setAgent] = useState<AgentDetail | null>(null);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);

  useEffect(() => {
    if (!auth.user?.access_token || !agentId) return;

    fetch(`${config.apiUrl}/api/v1/agents/${agentId}`, {
      headers: { Authorization: `Bearer ${auth.user.access_token}` },
    })
      .then(res => {
        if (res.status === 404) {
          throw new Error('Agent not found');
        }
        if (!res.ok) throw new Error(`HTTP ${res.status}`);
        return res.json();
      })
      .then(data => {
        setAgent(data);
        setLoading(false);
      })
      .catch(err => {
        setError(err.message);
        setLoading(false);
      });
  }, [auth.user?.access_token, agentId]);

  const getAgentStatusTone = (status: string) => {
    switch (status) {
      case 'active':
        return 'success';
      case 'inactive':
        return 'neutral';
      case 'suspended':
        return 'danger';
      default:
        return getStatusTone(status);
    }
  };

  if (loading) {
    return (
      <div className={styles.container}>
        <div className={styles.placeholder}>
          <p className={styles.placeholderText}>Loading agent details...</p>
        </div>
      </div>
    );
  }

  if (error || !agent) {
    return (
      <div className={styles.container}>
        <div className={styles.placeholder}>
          <p className={styles.error}>Error: {error || 'Agent not found'}</p>
          <Link to="/agents" className={styles.link}>Back to Agents</Link>
        </div>
      </div>
    );
  }

  return (
    <div className={styles.container}>
      {/* Breadcrumb */}
      <div className={styles.breadcrumb}>
        <Link to="/agents" className={styles.breadcrumbLink}>Agents</Link>
        <span className={styles.breadcrumbSeparator}>/</span>
        <span>{agent.name}</span>
      </div>

      {/* Header */}
      <div className={styles.header}>
        <div className={styles.headerStart}>
          <h1 className={styles.title}>{agent.name}</h1>
          <p className={styles.subtitle}>{agent.id}</p>
        </div>
        <Pill tone={getAgentStatusTone(agent.status)}>{agent.status}</Pill>
      </div>

      {/* Overview Card */}
      <Card title="Overview" className={styles.cardSpacing}>
        <div className={styles.infoRow}>
          <span className={styles.infoLabel}>Actor ID</span>
          <code className={styles.mono}>{agent.actor_id}</code>
        </div>
        <div className={styles.infoRow}>
          <span className={styles.infoLabel}>Actor Kind</span>
          <Pill tone="warning">AGENT</Pill>
        </div>
        <div className={styles.infoRow}>
          <span className={styles.infoLabel}>Status</span>
          <Pill tone={getAgentStatusTone(agent.status)}>{agent.status}</Pill>
        </div>
        <div className={styles.infoRow}>
          <span className={styles.infoLabel}>Created</span>
          <span className={styles.infoValue}>
            {new Date(agent.created_at).toLocaleString()}
          </span>
        </div>
        {agent.last_active_at && (
          <div className={styles.infoRow}>
            <span className={styles.infoLabel}>Last Active</span>
            <span className={styles.infoValue}>
              {new Date(agent.last_active_at).toLocaleString()}
            </span>
          </div>
        )}
      </Card>

      {/* Description Card */}
      {agent.description && (
        <Card title="Description" className={styles.cardSpacing}>
          <p style={{ margin: 0, fontSize: '0.875rem', lineHeight: 1.6 }}>
            {agent.description}
          </p>
        </Card>
      )}

      {/* Statistics Card */}
      <Card title="Statistics" className={styles.cardSpacing}>
        <div className={styles.statsGrid}>
          <div className={styles.stat}>
            <div className={styles.statLabel}>Iterations Completed</div>
            <div className={styles.statValue}>{agent.iterations_completed}</div>
          </div>
          <div className={styles.stat}>
            <div className={styles.statLabel}>Proposals Produced</div>
            <div className={styles.statValue}>{agent.proposals_produced}</div>
          </div>
        </div>
      </Card>

      {/* Current Work Card */}
      <Card title="Current Work" className={styles.cardSpacing}>
        {agent.current_work_unit_id ? (
          <div className={styles.infoRow}>
            <span className={styles.infoLabel}>Assigned Work Unit</span>
            <Link to={`/loops/${agent.current_work_unit_id}`} className={styles.link}>
              {agent.current_work_unit_id}
            </Link>
          </div>
        ) : (
          <div className={styles.placeholder}>
            <p className={styles.placeholderText}>No work unit currently assigned.</p>
          </div>
        )}
      </Card>

      {/* Capabilities Card */}
      <Card title="Capabilities" className={styles.cardSpacing}>
        {agent.capabilities.length === 0 ? (
          <div className={styles.placeholder}>
            <p className={styles.placeholderText}>No capabilities defined.</p>
          </div>
        ) : (
          <div className={styles.badgeGroup}>
            {agent.capabilities.map(cap => (
              <Pill key={cap} tone="neutral">{cap}</Pill>
            ))}
          </div>
        )}
      </Card>

      {/* Work Envelope Card (per SR-AGENTS) */}
      <Card title="Work Envelope" className={styles.cardSpacing}>
        <div className={styles.infoRow}>
          <span className={styles.infoLabel}>Max Concurrent Work Units</span>
          <span className={styles.infoValue}>{agent.work_envelope.max_concurrent_work_units}</span>
        </div>
        <div className={styles.infoRow}>
          <span className={styles.infoLabel}>Timeout</span>
          <span className={styles.infoValue}>{agent.work_envelope.timeout_hours} hours</span>
        </div>
        <div style={{ marginTop: 'var(--space3)' }}>
          <span className={styles.infoLabel}>Allowed Work Kinds</span>
          <div className={styles.badgeGroup} style={{ marginTop: 'var(--space2)' }}>
            {agent.work_envelope.allowed_kinds.length === 0 ? (
              <span style={{ color: 'var(--muted)', fontSize: '0.875rem' }}>All kinds allowed</span>
            ) : (
              agent.work_envelope.allowed_kinds.map(kind => (
                <Pill key={kind} tone="neutral">{kind}</Pill>
              ))
            )}
          </div>
        </div>
      </Card>

      {/* Trust Constraints Card (per SR-CONTRACT) */}
      <Card title="Trust Constraints" className={styles.cardSpacing}>
        {agent.trust_constraints.length === 0 ? (
          <div className={styles.placeholder}>
            <p className={styles.placeholderText}>No trust constraints defined.</p>
            <p className={styles.placeholderHint}>
              Per SR-AGENTS: Agents may draft artifacts but cannot create binding records.
            </p>
          </div>
        ) : (
          <table className={styles.table}>
            <thead>
              <tr>
                <th className={styles.th}>Constraint</th>
                <th className={styles.th}>Boundary</th>
                <th className={styles.th}>Description</th>
                <th className={styles.th}>Enforcement</th>
              </tr>
            </thead>
            <tbody>
              {agent.trust_constraints.map(constraint => (
                <tr key={constraint.constraint_id}>
                  <td className={styles.tdMono}>{constraint.constraint_id}</td>
                  <td className={styles.td}>{constraint.boundary}</td>
                  <td className={styles.td}>{constraint.description}</td>
                  <td className={styles.td}>{constraint.enforcement}</td>
                </tr>
              ))}
            </tbody>
          </table>
        )}
      </Card>

      {/* Recent Proposals Card */}
      <Card title={`Recent Proposals (${agent.recent_proposals.length})`}>
        {agent.recent_proposals.length === 0 ? (
          <div className={styles.placeholder}>
            <p className={styles.placeholderText}>No recent proposals.</p>
          </div>
        ) : (
          <table className={styles.table}>
            <thead>
              <tr>
                <th className={styles.th}>Proposal ID</th>
                <th className={styles.th}>Work Unit</th>
                <th className={styles.th}>Status</th>
                <th className={styles.th}>Created</th>
              </tr>
            </thead>
            <tbody>
              {agent.recent_proposals.map(proposal => (
                <tr key={proposal.id}>
                  <td className={styles.tdMono}>{proposal.id}</td>
                  <td className={styles.td}>
                    <Link to={`/loops/${proposal.work_unit_id}`} className={styles.link}>
                      {proposal.work_unit_id}
                    </Link>
                  </td>
                  <td className={styles.td}>
                    <Pill tone={getStatusTone(proposal.status)}>{proposal.status}</Pill>
                  </td>
                  <td className={styles.td}>
                    {new Date(proposal.created_at).toLocaleString()}
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

export default AgentDetail;
