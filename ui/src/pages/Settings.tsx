/**
 * Settings Page
 *
 * Platform configuration management including:
 * - Portal definitions (config.portal_definition)
 * - Oracle suite configurations (config.oracle_definition)
 * - Gating policies (config.gating_policy)
 * - Directive settings (SR-DIRECTIVE parameters)
 * - User preferences
 *
 * Per SR-SPEC §5, configuration artifacts are governed and versioned.
 */

import { useState, useEffect } from 'react';
import { useAuth } from '../auth/AuthProvider';
import config from '../config';
import { Card, Pill, Button } from '../ui';
import styles from '../styles/pages.module.css';

type SettingsTab = 'portals' | 'oracles' | 'policies' | 'directive' | 'user';

interface PortalDefinition {
  id: string;
  portal_id: string;
  name: string;
  description: string;
  required_actor_kind: 'HUMAN';
  required_roles: string[];
  status: 'active' | 'inactive';
}

interface OracleSuiteDefinition {
  id: string;
  suite_id: string;
  name: string;
  description: string;
  oracles: {
    oracle_id: string;
    name: string;
    classification: 'required' | 'advisory';
  }[];
  environment_constraints: Record<string, string>;
  content_hash: string;
  status: 'active' | 'draft' | 'deprecated';
}

interface GatingPolicy {
  id: string;
  policy_id: string;
  name: string;
  hooks: {
    hook_class: string;
    mode: 'soft' | 'hard' | 'hybrid';
    triggers: string[];
  }[];
  status: 'active' | 'draft';
}

interface DirectiveSettings {
  stop_triggers: {
    trigger_id: string;
    description: string;
    threshold: number | null;
    enabled: boolean;
  }[];
  budgets: {
    budget_type: string;
    default_value: number;
    unit: string;
  }[];
  portal_routing: {
    condition: string;
    portal_id: string;
  }[];
}

interface UserPreferences {
  theme: 'light' | 'dark' | 'system';
  notifications_enabled: boolean;
  default_page_size: number;
  timezone: string;
}

interface SettingsResponse {
  portals: PortalDefinition[];
  oracle_suites: OracleSuiteDefinition[];
  gating_policies: GatingPolicy[];
  directive: DirectiveSettings;
  user_preferences: UserPreferences;
}

export function Settings(): JSX.Element {
  const auth = useAuth();
  const [activeTab, setActiveTab] = useState<SettingsTab>('portals');
  const [portals, setPortals] = useState<PortalDefinition[]>([]);
  const [oracleSuites, setOracleSuites] = useState<OracleSuiteDefinition[]>([]);
  const [gatingPolicies, setGatingPolicies] = useState<GatingPolicy[]>([]);
  const [directive, setDirective] = useState<DirectiveSettings | null>(null);
  const [userPrefs, setUserPrefs] = useState<UserPreferences | null>(null);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);
  const [saveStatus, setSaveStatus] = useState<string | null>(null);

  useEffect(() => {
    if (!auth.user?.access_token) return;

    fetch(`${config.apiUrl}/api/v1/settings`, {
      headers: {
        Authorization: `Bearer ${auth.user.access_token}`,
      },
    })
      .then(res => {
        // Treat 404 as "no data yet" rather than an error
        if (res.status === 404) {
          return {
            portals: [],
            oracle_suites: [],
            gating_policies: [],
            directive: null,
            user_preferences: {
              theme: 'light' as const,
              notifications_enabled: true,
              default_page_size: 50,
              timezone: 'UTC',
            },
          };
        }
        if (!res.ok) throw new Error(`HTTP ${res.status}`);
        return res.json();
      })
      .then((data: SettingsResponse) => {
        setPortals(data.portals || []);
        setOracleSuites(data.oracle_suites || []);
        setGatingPolicies(data.gating_policies || []);
        setDirective(data.directive || null);
        setUserPrefs(data.user_preferences || null);
        setLoading(false);
      })
      .catch(err => {
        setError(err.message);
        setLoading(false);
      });
  }, [auth.user?.access_token]);

  const handleSaveUserPrefs = async () => {
    if (!auth.user?.access_token || !userPrefs) return;

    setSaveStatus(null);
    try {
      const res = await fetch(`${config.apiUrl}/api/v1/settings/user-preferences`, {
        method: 'PUT',
        headers: {
          Authorization: `Bearer ${auth.user.access_token}`,
          'Content-Type': 'application/json',
        },
        body: JSON.stringify(userPrefs),
      });

      if (!res.ok) throw new Error(`HTTP ${res.status}`);
      setSaveStatus('Preferences saved successfully');
    } catch (err) {
      setSaveStatus(`Error: ${err instanceof Error ? err.message : 'Save failed'}`);
    }
  };

  const truncateHash = (hash: string): string => {
    if (!hash) return '';
    if (hash.startsWith('sha256:')) {
      return hash.slice(0, 15) + '...' + hash.slice(-6);
    }
    return hash.length > 20 ? hash.slice(0, 10) + '...' + hash.slice(-6) : hash;
  };

  return (
    <div className={styles.container}>
      <div className={styles.header}>
        <div className={styles.headerStart}>
          <h1 className={styles.title}>Settings</h1>
          <p className={styles.subtitle}>Platform configuration and user preferences</p>
        </div>
      </div>

      {/* Overview */}
      <Card>
        <div className={styles.statsGrid}>
          <div className={styles.stat}>
            <div className={styles.statLabel}>Portals</div>
            <div className={styles.statValue}>{portals.length}</div>
          </div>
          <div className={styles.stat}>
            <div className={styles.statLabel}>Oracle Suites</div>
            <div className={styles.statValue}>{oracleSuites.length}</div>
          </div>
          <div className={styles.stat}>
            <div className={styles.statLabel}>Gating Policies</div>
            <div className={styles.statValue}>{gatingPolicies.length}</div>
          </div>
          <div className={styles.stat}>
            <div className={styles.statLabel}>Config Status</div>
            <div className={styles.statValue}>Governed</div>
          </div>
        </div>
      </Card>

      {/* Info Note */}
      <div className={styles.note}>
        Per SR-SPEC §5: Configuration artifacts are governed and versioned. Changes to portal
        definitions, oracle suites, and normative configurations require governance change
        approval (C-TB-4). User preferences are non-governed.
      </div>

      {/* Tabs */}
      <div className={styles.tabs}>
        <button
          className={`${styles.tab} ${activeTab === 'portals' ? styles.tabActive : ''}`}
          onClick={() => setActiveTab('portals')}
        >
          Portals
        </button>
        <button
          className={`${styles.tab} ${activeTab === 'oracles' ? styles.tabActive : ''}`}
          onClick={() => setActiveTab('oracles')}
        >
          Oracle Suites
        </button>
        <button
          className={`${styles.tab} ${activeTab === 'policies' ? styles.tabActive : ''}`}
          onClick={() => setActiveTab('policies')}
        >
          Gating Policies
        </button>
        <button
          className={`${styles.tab} ${activeTab === 'directive' ? styles.tabActive : ''}`}
          onClick={() => setActiveTab('directive')}
        >
          Directive
        </button>
        <button
          className={`${styles.tab} ${activeTab === 'user' ? styles.tabActive : ''}`}
          onClick={() => setActiveTab('user')}
        >
          User Preferences
        </button>
      </div>

      {/* Tab Content */}
      <Card>
        {loading ? (
          <div className={styles.placeholder}>
            <p className={styles.placeholderText}>Loading settings...</p>
          </div>
        ) : error ? (
          <div className={styles.placeholder}>
            <p className={styles.error}>Error: {error}</p>
          </div>
        ) : activeTab === 'portals' ? (
          portals.length === 0 ? (
            <div className={styles.placeholder}>
              <p className={styles.placeholderText}>No portal definitions configured.</p>
              <p className={styles.placeholderHint}>
                Portals are trust boundaries requiring human arbitration.
                Per SR-CONTRACT C-TB-4, minimum required portals are Governance Change
                and Release Approval.
              </p>
            </div>
          ) : (
            <table className={styles.table}>
              <thead>
                <tr>
                  <th className={styles.th}>Portal ID</th>
                  <th className={styles.th}>Name</th>
                  <th className={styles.th}>Description</th>
                  <th className={styles.th}>Required Roles</th>
                  <th className={styles.th}>Status</th>
                </tr>
              </thead>
              <tbody>
                {portals.map(portal => (
                  <tr key={portal.id}>
                    <td className={styles.tdMono}>{portal.portal_id}</td>
                    <td className={styles.td}>{portal.name}</td>
                    <td className={styles.td} style={{ maxWidth: '300px' }}>{portal.description}</td>
                    <td className={styles.td}>
                      <div className={styles.badgeGroup}>
                        {portal.required_roles.map(role => (
                          <Pill key={role} tone="neutral">{role}</Pill>
                        ))}
                      </div>
                    </td>
                    <td className={styles.td}>
                      <Pill tone={portal.status === 'active' ? 'success' : 'neutral'}>
                        {portal.status}
                      </Pill>
                    </td>
                  </tr>
                ))}
              </tbody>
            </table>
          )
        ) : activeTab === 'oracles' ? (
          oracleSuites.length === 0 ? (
            <div className={styles.placeholder}>
              <p className={styles.placeholderText}>No oracle suites configured.</p>
              <p className={styles.placeholderHint}>
                Oracle suites define verification procedures. They must be pinned by
                content hash when used for governed verification.
              </p>
            </div>
          ) : (
            <table className={styles.table}>
              <thead>
                <tr>
                  <th className={styles.th}>Suite ID</th>
                  <th className={styles.th}>Name</th>
                  <th className={styles.th}>Oracles</th>
                  <th className={styles.th}>Content Hash</th>
                  <th className={styles.th}>Status</th>
                </tr>
              </thead>
              <tbody>
                {oracleSuites.map(suite => (
                  <tr key={suite.id}>
                    <td className={styles.tdMono}>{suite.suite_id}</td>
                    <td className={styles.td}>{suite.name}</td>
                    <td className={styles.td}>
                      <div style={{ display: 'flex', flexDirection: 'column', gap: '0.25rem' }}>
                        {suite.oracles.slice(0, 3).map(oracle => (
                          <div key={oracle.oracle_id} style={{ display: 'flex', alignItems: 'center', gap: '0.5rem' }}>
                            <Pill tone={oracle.classification === 'required' ? 'warning' : 'neutral'}>
                              {oracle.classification}
                            </Pill>
                            <span style={{ fontSize: '0.8rem' }}>{oracle.name}</span>
                          </div>
                        ))}
                        {suite.oracles.length > 3 && (
                          <span style={{ fontSize: '0.75rem', color: 'var(--muted)' }}>
                            +{suite.oracles.length - 3} more
                          </span>
                        )}
                      </div>
                    </td>
                    <td className={styles.tdMono}>{truncateHash(suite.content_hash)}</td>
                    <td className={styles.td}>
                      <Pill tone={
                        suite.status === 'active' ? 'success' :
                        suite.status === 'draft' ? 'warning' : 'danger'
                      }>
                        {suite.status}
                      </Pill>
                    </td>
                  </tr>
                ))}
              </tbody>
            </table>
          )
        ) : activeTab === 'policies' ? (
          gatingPolicies.length === 0 ? (
            <div className={styles.placeholder}>
              <p className={styles.placeholderText}>No gating policies configured.</p>
              <p className={styles.placeholderHint}>
                Gating policies define human judgment hooks per work unit.
                Hook modes: soft, hard, or hybrid.
              </p>
            </div>
          ) : (
            <table className={styles.table}>
              <thead>
                <tr>
                  <th className={styles.th}>Policy ID</th>
                  <th className={styles.th}>Name</th>
                  <th className={styles.th}>Hooks</th>
                  <th className={styles.th}>Status</th>
                </tr>
              </thead>
              <tbody>
                {gatingPolicies.map(policy => (
                  <tr key={policy.id}>
                    <td className={styles.tdMono}>{policy.policy_id}</td>
                    <td className={styles.td}>{policy.name}</td>
                    <td className={styles.td}>
                      <div style={{ display: 'flex', flexDirection: 'column', gap: '0.5rem' }}>
                        {policy.hooks.map((hook, idx) => (
                          <div key={idx} style={{ display: 'flex', alignItems: 'center', gap: '0.5rem' }}>
                            <code style={{ fontSize: '0.75rem' }}>{hook.hook_class}</code>
                            <Pill tone={
                              hook.mode === 'hard' ? 'danger' :
                              hook.mode === 'soft' ? 'success' : 'warning'
                            }>
                              {hook.mode}
                            </Pill>
                          </div>
                        ))}
                      </div>
                    </td>
                    <td className={styles.td}>
                      <Pill tone={policy.status === 'active' ? 'success' : 'warning'}>
                        {policy.status}
                      </Pill>
                    </td>
                  </tr>
                ))}
              </tbody>
            </table>
          )
        ) : activeTab === 'directive' ? (
          !directive ? (
            <div className={styles.placeholder}>
              <p className={styles.placeholderText}>No directive settings loaded.</p>
            </div>
          ) : (
            <div>
              {/* Stop Triggers */}
              <h4 style={{ margin: '0 0 1rem 0', fontSize: '0.875rem', color: 'var(--ink)' }}>
                Stop Triggers (SR-SPEC §3.4)
              </h4>
              <table className={styles.table} style={{ marginBottom: '2rem' }}>
                <thead>
                  <tr>
                    <th className={styles.th}>Trigger ID</th>
                    <th className={styles.th}>Description</th>
                    <th className={styles.th}>Threshold</th>
                    <th className={styles.th}>Status</th>
                  </tr>
                </thead>
                <tbody>
                  {directive.stop_triggers.map(trigger => (
                    <tr key={trigger.trigger_id}>
                      <td className={styles.tdMono}>{trigger.trigger_id}</td>
                      <td className={styles.td}>{trigger.description}</td>
                      <td className={styles.td}>{trigger.threshold ?? '—'}</td>
                      <td className={styles.td}>
                        <Pill tone={trigger.enabled ? 'success' : 'neutral'}>
                          {trigger.enabled ? 'enabled' : 'disabled'}
                        </Pill>
                      </td>
                    </tr>
                  ))}
                </tbody>
              </table>

              {/* Budgets */}
              <h4 style={{ margin: '0 0 1rem 0', fontSize: '0.875rem', color: 'var(--ink)' }}>
                Default Budgets
              </h4>
              <table className={styles.table} style={{ marginBottom: '2rem' }}>
                <thead>
                  <tr>
                    <th className={styles.th}>Budget Type</th>
                    <th className={styles.th}>Default Value</th>
                    <th className={styles.th}>Unit</th>
                  </tr>
                </thead>
                <tbody>
                  {directive.budgets.map(budget => (
                    <tr key={budget.budget_type}>
                      <td className={styles.tdMono}>{budget.budget_type}</td>
                      <td className={styles.td}>{budget.default_value}</td>
                      <td className={styles.td}>{budget.unit}</td>
                    </tr>
                  ))}
                </tbody>
              </table>

              {/* Portal Routing */}
              <h4 style={{ margin: '0 0 1rem 0', fontSize: '0.875rem', color: 'var(--ink)' }}>
                Portal Routing Rules
              </h4>
              <table className={styles.table}>
                <thead>
                  <tr>
                    <th className={styles.th}>Condition</th>
                    <th className={styles.th}>Routes To</th>
                  </tr>
                </thead>
                <tbody>
                  {directive.portal_routing.map((route, idx) => (
                    <tr key={idx}>
                      <td className={styles.td}>{route.condition}</td>
                      <td className={styles.tdMono}>{route.portal_id}</td>
                    </tr>
                  ))}
                </tbody>
              </table>
            </div>
          )
        ) : (
          /* User Preferences */
          !userPrefs ? (
            <div className={styles.placeholder}>
              <p className={styles.placeholderText}>No user preferences loaded.</p>
            </div>
          ) : (
            <div className={styles.form}>
              <div className={styles.formRow}>
                <div className={styles.formGroup}>
                  <label className={styles.label}>Theme</label>
                  <select
                    className={styles.select}
                    value={userPrefs.theme}
                    onChange={(e) => setUserPrefs({
                      ...userPrefs,
                      theme: e.target.value as 'light' | 'dark' | 'system'
                    })}
                  >
                    <option value="light">Light</option>
                    <option value="dark">Dark</option>
                    <option value="system">System</option>
                  </select>
                </div>

                <div className={styles.formGroup}>
                  <label className={styles.label}>Default Page Size</label>
                  <select
                    className={styles.select}
                    value={userPrefs.default_page_size}
                    onChange={(e) => setUserPrefs({
                      ...userPrefs,
                      default_page_size: Number(e.target.value)
                    })}
                  >
                    <option value={25}>25</option>
                    <option value={50}>50</option>
                    <option value={100}>100</option>
                  </select>
                </div>
              </div>

              <div className={styles.formRow}>
                <div className={styles.formGroup}>
                  <label className={styles.label}>Timezone</label>
                  <select
                    className={styles.select}
                    value={userPrefs.timezone}
                    onChange={(e) => setUserPrefs({
                      ...userPrefs,
                      timezone: e.target.value
                    })}
                  >
                    <option value="UTC">UTC</option>
                    <option value="America/New_York">America/New_York</option>
                    <option value="America/Chicago">America/Chicago</option>
                    <option value="America/Denver">America/Denver</option>
                    <option value="America/Los_Angeles">America/Los_Angeles</option>
                    <option value="Europe/London">Europe/London</option>
                    <option value="Europe/Paris">Europe/Paris</option>
                    <option value="Asia/Tokyo">Asia/Tokyo</option>
                  </select>
                </div>

                <div className={styles.formGroup}>
                  <label className={styles.label}>Notifications</label>
                  <div style={{ display: 'flex', alignItems: 'center', gap: '0.5rem', marginTop: '0.5rem' }}>
                    <input
                      type="checkbox"
                      id="notifications"
                      checked={userPrefs.notifications_enabled}
                      onChange={(e) => setUserPrefs({
                        ...userPrefs,
                        notifications_enabled: e.target.checked
                      })}
                    />
                    <label htmlFor="notifications" style={{ fontSize: '0.875rem' }}>
                      Enable notifications
                    </label>
                  </div>
                </div>
              </div>

              {saveStatus && (
                <div className={saveStatus.startsWith('Error') ? styles.error : styles.success}>
                  {saveStatus}
                </div>
              )}

              <div className={styles.buttonRow}>
                <Button variant="primary" onClick={handleSaveUserPrefs}>
                  Save Preferences
                </Button>
              </div>
            </div>
          )
        )}
      </Card>

      {/* Configuration Types Reference */}
      <Card>
        <h3 style={{ margin: '0 0 1rem 0', fontSize: '0.875rem', color: 'var(--ink)' }}>
          Configuration Types (SR-TYPES §4.5)
        </h3>
        <table className={styles.table}>
          <thead>
            <tr>
              <th className={styles.th}>Type Key</th>
              <th className={styles.th}>Authority Kind</th>
              <th className={styles.th}>Purpose</th>
            </tr>
          </thead>
          <tbody>
            <tr>
              <td className={styles.tdMono}>config.portal_definition</td>
              <td className={styles.td}>config</td>
              <td className={styles.td}>Portal configurations for trust boundaries</td>
            </tr>
            <tr>
              <td className={styles.tdMono}>config.oracle_definition</td>
              <td className={styles.td}>config</td>
              <td className={styles.td}>Oracle suite configurations</td>
            </tr>
            <tr>
              <td className={styles.tdMono}>config.gating_policy</td>
              <td className={styles.td}>config</td>
              <td className={styles.td}>Human judgment hook configurations</td>
            </tr>
            <tr>
              <td className={styles.tdMono}>config.procedure_template</td>
              <td className={styles.td}>config</td>
              <td className={styles.td}>Stage-gated procedure definitions</td>
            </tr>
            <tr>
              <td className={styles.tdMono}>config.semantic_set</td>
              <td className={styles.td}>config</td>
              <td className={styles.td}>Semantic oracle meaning-matrix definitions</td>
            </tr>
          </tbody>
        </table>
      </Card>
    </div>
  );
}

export default Settings;
