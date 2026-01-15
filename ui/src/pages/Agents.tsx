/**
 * Agents Page
 *
 * Displays registered agent definitions and their status.
 * Per SR-AGENTS, agents are actor-kind AGENT: non-authoritative workers
 * that produce proposals. They cannot create binding records.
 *
 * Actor kinds in SOLVER-Ralph:
 * - HUMAN: can perform portal-bound binding actions
 * - SYSTEM: deterministic control-plane
 * - ORACLE: deterministic verifiers producing evidence
 * - AGENT: non-authoritative worker producing proposals
 */

import { useState, useEffect } from 'react';
import { Link } from 'react-router-dom';
import { useAuth } from '../auth/AuthProvider';
import config from '../config';
import { Card, Pill, getStatusTone } from '../ui';
import styles from '../styles/pages.module.css';

interface AgentDefinition {
  id: string;
  name: string;
  actor_id: string;
  status: 'active' | 'inactive' | 'suspended';
  capabilities: string[];
  current_work_unit_id: string | null;
  iterations_completed: number;
  proposals_produced: number;
  last_active_at: string | null;
  created_at: string;
}

interface AgentsResponse {
  agents: AgentDefinition[];
  actor_kinds: {
    kind: string;
    count: number;
  }[];
}

export function Agents(): JSX.Element {
  const auth = useAuth();
  const [agents, setAgents] = useState<AgentDefinition[]>([]);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);

  useEffect(() => {
    if (!auth.user?.access_token) return;

    fetch(`${config.apiUrl}/api/v1/agents`, {
      headers: {
        Authorization: `Bearer ${auth.user.access_token}`,
      },
    })
      .then(res => {
        if (!res.ok) throw new Error(`HTTP ${res.status}`);
        return res.json();
      })
      .then((data: AgentsResponse) => {
        setAgents(data.agents || []);
        setLoading(false);
      })
      .catch(err => {
        setError(err.message);
        setLoading(false);
      });
  }, [auth.user?.access_token]);

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

  return (
    <div className={styles.container}>
      <div className={styles.header}>
        <div className={styles.headerStart}>
          <h1 className={styles.title}>Agents</h1>
          <p className={styles.subtitle}>Non-authoritative workers producing proposals</p>
        </div>
      </div>

      {/* Actor Kinds Overview */}
      <Card>
        <div className={styles.statsGrid}>
          <div className={styles.stat}>
            <div className={styles.statLabel}>Actor Kind</div>
            <div className={styles.statValue}>AGENT</div>
          </div>
          <div className={styles.stat}>
            <div className={styles.statLabel}>Authority</div>
            <div className={styles.statValue}>None</div>
          </div>
          <div className={styles.stat}>
            <div className={styles.statLabel}>Output Type</div>
            <div className={styles.statValue}>Proposals</div>
          </div>
          <div className={styles.stat}>
            <div className={styles.statLabel}>Registered</div>
            <div className={styles.statValue}>{agents.length}</div>
          </div>
        </div>
      </Card>

      {/* Constraints Note */}
      <div className={styles.note}>
        Per SR-AGENTS: Agents may draft artifacts, analyses, patches, and candidate changes.
        Agents may never create binding records (approvals, freezes, governance changes).
        Agent statements are proposals until promoted via commitment objects and required approvals.
      </div>

      {/* Agents Table */}
      <Card>
        {loading ? (
          <div className={styles.placeholder}>
            <p className={styles.placeholderText}>Loading agents...</p>
          </div>
        ) : error ? (
          <div className={styles.placeholder}>
            <p className={styles.error}>Error: {error}</p>
          </div>
        ) : agents.length === 0 ? (
          <div className={styles.placeholder}>
            <p className={styles.placeholderText}>No agents registered.</p>
            <p className={styles.placeholderHint}>
              Agents are registered via config.agent_definition artifacts.
              They interact with the platform through controlled interfaces.
            </p>
          </div>
        ) : (
          <table className={styles.table}>
            <thead>
              <tr>
                <th className={styles.th}>Name</th>
                <th className={styles.th}>Actor ID</th>
                <th className={styles.th}>Status</th>
                <th className={styles.th}>Capabilities</th>
                <th className={styles.th}>Current Work</th>
                <th className={styles.th}>Proposals</th>
                <th className={styles.th}>Last Active</th>
              </tr>
            </thead>
            <tbody>
              {agents.map(agent => (
                <tr key={agent.id}>
                  <td className={styles.td}>
                    <Link to={`/agents/${agent.id}`} className={styles.link}>
                      {agent.name}
                    </Link>
                  </td>
                  <td className={styles.tdMono}>{agent.actor_id}</td>
                  <td className={styles.td}>
                    <Pill tone={getAgentStatusTone(agent.status)}>{agent.status}</Pill>
                  </td>
                  <td className={styles.td}>
                    <div className={styles.badgeGroup}>
                      {agent.capabilities.slice(0, 3).map(cap => (
                        <Pill key={cap} tone="neutral">{cap}</Pill>
                      ))}
                      {agent.capabilities.length > 3 && (
                        <Pill tone="neutral">+{agent.capabilities.length - 3}</Pill>
                      )}
                    </div>
                  </td>
                  <td className={styles.td}>
                    {agent.current_work_unit_id ? (
                      <Link to={`/loops/${agent.current_work_unit_id}`} className={styles.link}>
                        {agent.current_work_unit_id.slice(0, 12)}...
                      </Link>
                    ) : (
                      <span className={styles.placeholderText}>—</span>
                    )}
                  </td>
                  <td className={styles.td}>{agent.proposals_produced}</td>
                  <td className={styles.td}>
                    {agent.last_active_at
                      ? new Date(agent.last_active_at).toLocaleString()
                      : '—'}
                  </td>
                </tr>
              ))}
            </tbody>
          </table>
        )}
      </Card>

      {/* Trust Boundary Reference */}
      <Card>
        <h3 style={{ margin: '0 0 1rem 0', fontSize: '0.875rem', color: 'var(--ink)' }}>
          Trust Boundary Constraints (SR-CONTRACT)
        </h3>
        <table className={styles.table}>
          <thead>
            <tr>
              <th className={styles.th}>Boundary</th>
              <th className={styles.th}>Constraint</th>
              <th className={styles.th}>Enforcement</th>
            </tr>
          </thead>
          <tbody>
            <tr>
              <td className={styles.td}>Portal Binding (C-TB-1)</td>
              <td className={styles.td}>Cannot create approvals, decisions, waivers, or freezes</td>
              <td className={styles.td}>Portal actor-kind enforcement</td>
            </tr>
            <tr>
              <td className={styles.td}>Agent Output (C-TB-2)</td>
              <td className={styles.td}>Cannot establish Verified/Approved/Shippable</td>
              <td className={styles.td}>Domain core admission control</td>
            </tr>
            <tr>
              <td className={styles.td}>Verification (C-VER-1)</td>
              <td className={styles.td}>Claims cannot substitute for oracle evidence</td>
              <td className={styles.td}>Verification logic</td>
            </tr>
            <tr>
              <td className={styles.td}>Context (C-CTX-2)</td>
              <td className={styles.td}>No ghost inputs; context from IterationStarted.refs[]</td>
              <td className={styles.td}>Context compilation</td>
            </tr>
          </tbody>
        </table>
      </Card>
    </div>
  );
}

export default Agents;
