/**
 * Templates Page
 *
 * Browse and manage configuration templates for governed artifacts.
 * Per SR-TEMPLATES: Templates define schemas for Intakes, Procedure Templates,
 * Oracle Suites, Verification Profiles, Semantic Sets, and Exceptions.
 *
 * Categories:
 * - Work Surface: Intake, Procedure Template, Work Surface Instance
 * - Oracle: Oracle Suite, Oracle Definition, Semantic Oracle
 * - Verification: Verification Profile
 * - Semantic Sets: Semantic Set, Semantic Axis
 * - Execution: Budgets, Gating Policy
 * - Exceptions: Waiver, Deviation, Deferral
 * - Reference: Gates, Portals, Evidence, Release (platform-provided)
 */

import { useState, useEffect } from 'react';
import { Link, useNavigate } from 'react-router-dom';
import { useAuth } from '../auth/AuthProvider';
import config from '../config';
import { Card, Pill, Button } from '../ui';
import styles from '../styles/pages.module.css';

// ============================================================================
// Types
// ============================================================================

type TemplateCategory =
  | 'work-surface'
  | 'oracle'
  | 'verification'
  | 'semantic-sets'
  | 'gates'
  | 'portals'
  | 'execution'
  | 'context'
  | 'evidence'
  | 'release'
  | 'exceptions';

interface CategoryInfo {
  id: TemplateCategory;
  label: string;
  type_keys: string[];
  requires_portal: boolean;
  is_user_instantiable: boolean;
  schema_count: number;
}

interface FieldSchema {
  name: string;
  field_type: string;
  description: string;
  example?: string;
}

interface TemplateSchema {
  type_key: string;
  name: string;
  description: string;
  source_ref: string;
  required_fields: FieldSchema[];
  optional_fields: FieldSchema[];
  requires_portal: boolean;
}

interface TemplateSummary {
  id: string;
  type_key: string;
  name: string;
  category: TemplateCategory;
  status: string;
  content_hash: string;
  created_at: string;
  updated_at: string;
  requires_portal: boolean;
}

interface ListSchemasResponse {
  schemas: TemplateSchema[];
  total: number;
  categories: CategoryInfo[];
}

interface ListTemplatesResponse {
  templates: TemplateSummary[];
  total: number;
  category_counts: Record<string, number>;
}

// ============================================================================
// Constants
// ============================================================================

const CATEGORY_TABS: { id: TemplateCategory; label: string }[] = [
  { id: 'work-surface', label: 'Work Surface' },
  { id: 'oracle', label: 'Oracle' },
  { id: 'verification', label: 'Verification' },
  { id: 'semantic-sets', label: 'Semantic Sets' },
  { id: 'execution', label: 'Execution' },
  { id: 'exceptions', label: 'Exceptions' },
  { id: 'gates', label: 'Reference' },
];

// ============================================================================
// Component
// ============================================================================

export function Templates(): JSX.Element {
  const auth = useAuth();
  const navigate = useNavigate();
  const [activeTab, setActiveTab] = useState<TemplateCategory>('work-surface');
  const [schemas, setSchemas] = useState<TemplateSchema[]>([]);
  const [templates, setTemplates] = useState<TemplateSummary[]>([]);
  const [categories, setCategories] = useState<CategoryInfo[]>([]);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);
  const [expandedSchema, setExpandedSchema] = useState<string | null>(null);

  useEffect(() => {
    if (!auth.user?.access_token) return;

    const fetchSchemas = fetch(`${config.apiUrl}/api/v1/templates/schemas`, {
      headers: {
        Authorization: `Bearer ${auth.user.access_token}`,
      },
    })
      .then(res => {
        if (res.status === 404) return { schemas: [], total: 0, categories: [] };
        if (!res.ok) throw new Error(`HTTP ${res.status}`);
        return res.json();
      })
      .then((data: ListSchemasResponse) => {
        setSchemas(data.schemas || []);
        setCategories(data.categories || []);
      });

    const fetchTemplates = fetch(`${config.apiUrl}/api/v1/templates`, {
      headers: {
        Authorization: `Bearer ${auth.user.access_token}`,
      },
    })
      .then(res => {
        if (res.status === 404) return { templates: [], total: 0, category_counts: {} };
        if (!res.ok) throw new Error(`HTTP ${res.status}`);
        return res.json();
      })
      .then((data: ListTemplatesResponse) => {
        setTemplates(data.templates || []);
      });

    Promise.all([fetchSchemas, fetchTemplates])
      .then(() => setLoading(false))
      .catch(err => {
        setError(err.message);
        setLoading(false);
      });
  }, [auth.user?.access_token]);

  const truncateHash = (hash: string) => {
    if (hash.startsWith('sha256:')) {
      return `sha256:${hash.slice(7, 15)}...`;
    }
    return hash.length > 16 ? `${hash.slice(0, 16)}...` : hash;
  };

  const getStatusTone = (status: string): 'success' | 'warning' | 'danger' | 'neutral' | 'info' => {
    switch (status.toLowerCase()) {
      case 'governed':
        return 'success';
      case 'draft':
        return 'neutral';
      case 'pending_approval':
        return 'warning';
      case 'superseded':
        return 'danger';
      case 'reference':
        return 'info';
      default:
        return 'neutral';
    }
  };

  const formatDate = (dateStr: string) => {
    const date = new Date(dateStr);
    return date.toLocaleDateString('en-US', {
      month: 'short',
      day: 'numeric',
      year: 'numeric',
    });
  };

  // Get schemas for the active category (map 'gates' to reference categories)
  const getCategorySchemas = () => {
    if (activeTab === 'gates') {
      // Reference tab shows gates, portals, evidence, release
      return schemas.filter(s =>
        ['gate_definition', 'portal_playbook', 'domain.evidence_bundle', 'sr.semantic_eval.v1', 'record.freeze'].includes(s.type_key)
      );
    }
    const categoryMapping: Record<TemplateCategory, string[]> = {
      'work-surface': ['record.intake', 'config.procedure_template', 'domain.work_surface'],
      'oracle': ['oracle_suite', 'oracle_definition', 'semantic_oracle'],
      'verification': ['verification_profile', 'profile_selection_matrix'],
      'semantic-sets': ['config.semantic_set', 'semantic_axis', 'decision_rule'],
      'gates': ['gate_definition'],
      'portals': ['portal_playbook'],
      'execution': ['budget_config', 'stop_trigger', 'config.gating_policy'],
      'context': ['required_refs', 'iteration_summary'],
      'evidence': ['domain.evidence_bundle', 'sr.semantic_eval.v1'],
      'release': ['record.freeze'],
      'exceptions': ['record.waiver', 'record.deviation', 'record.deferral'],
    };
    const typeKeys = categoryMapping[activeTab] || [];
    return schemas.filter(s => typeKeys.includes(s.type_key));
  };

  // Get templates for the active category
  const getCategoryTemplates = () => {
    const categoryKey = activeTab === 'gates' ? null : activeTab;
    if (!categoryKey) return { reference: [], user: [] };

    // Debug: log templates and active tab
    console.log('Templates:', templates);
    console.log('Active tab:', activeTab, 'categoryKey:', categoryKey);

    const filtered = templates.filter(t => {
      // The backend returns category in kebab-case (e.g., "work-surface")
      const templateCategory = String(t.category).toLowerCase().replace(/_/g, '-');
      console.log(`Template ${t.id}: category="${t.category}" -> "${templateCategory}", matches=${templateCategory === categoryKey}`);
      return templateCategory === categoryKey;
    });

    console.log('Filtered templates:', filtered);

    // Split into reference and user-created templates
    const reference = filtered.filter(t => t.status === 'reference').sort((a, b) => a.name.localeCompare(b.name));
    const user = filtered.filter(t => t.status !== 'reference').sort((a, b) => a.name.localeCompare(b.name));

    console.log('Reference templates:', reference);
    console.log('User templates:', user);

    return { reference, user };
  };

  // Clone a template and navigate to the new copy
  const handleClone = async (template: TemplateSummary) => {
    if (!auth.user?.access_token) return;

    try {
      // Fetch full template details first
      const detailRes = await fetch(
        `${config.apiUrl}/api/v1/templates/${encodeURIComponent(template.id)}`,
        {
          headers: {
            Authorization: `Bearer ${auth.user.access_token}`,
          },
        }
      );
      if (!detailRes.ok) throw new Error(`HTTP ${detailRes.status}`);
      const detail = await detailRes.json();

      // Create new template with cloned content
      const createRes = await fetch(`${config.apiUrl}/api/v1/templates`, {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json',
          Authorization: `Bearer ${auth.user.access_token}`,
        },
        body: JSON.stringify({
          type_key: template.type_key,
          name: `${template.name} (Copy)`,
          content: detail.content,
          refs: detail.refs || [],
        }),
      });

      if (!createRes.ok) {
        const errorData = await createRes.json().catch(() => ({}));
        throw new Error(errorData.message || `HTTP ${createRes.status}`);
      }

      const newTemplate = await createRes.json();

      // Navigate to the new template's detail page for editing
      navigate(`/templates/${activeTab}/${encodeURIComponent(newTemplate.id)}?edit=true`);
    } catch (err) {
      console.error('Clone failed:', err);
      setError(err instanceof Error ? err.message : 'Clone failed');
    }
  };

  const categorySchemas = getCategorySchemas();
  const { reference: referenceTemplates, user: userTemplates } = getCategoryTemplates();
  const isUserInstantiable = categories.find(c =>
    String(c.id).toLowerCase().replace('_', '-') === activeTab
  )?.is_user_instantiable ?? false;

  return (
    <div className={styles.container}>
      <div className={styles.header}>
        <div className={styles.headerStart}>
          <h1 className={styles.title}>Templates</h1>
          <p className={styles.subtitle}>Browse and instantiate configuration templates for governed artifacts</p>
        </div>
        {isUserInstantiable && (
          <Button variant="primary" disabled>
            + Create
          </Button>
        )}
      </div>

      {/* Overview Stats */}
      <Card>
        <div className={styles.statsGrid}>
          <div className={styles.stat}>
            <div className={styles.statLabel}>Template Schemas</div>
            <div className={styles.statValue}>{schemas.length}</div>
          </div>
          <div className={styles.stat}>
            <div className={styles.statLabel}>User Instances</div>
            <div className={styles.statValue}>{templates.length}</div>
          </div>
          <div className={styles.stat}>
            <div className={styles.statLabel}>Categories</div>
            <div className={styles.statValue}>{categories.length}</div>
          </div>
          <div className={styles.stat}>
            <div className={styles.statLabel}>Portal Required</div>
            <div className={styles.statValue}>
              {schemas.filter(s => s.requires_portal).length}
            </div>
          </div>
        </div>
      </Card>

      {/* Info Note */}
      <div className={styles.note}>
        Per SR-TEMPLATES: Templates define schemas for all user-configurable artifacts.
        Some templates require portal approval (Oracle, Verification, Exceptions) while
        others are self-service (Work Surface, Execution Policy).
      </div>

      {/* Category Tabs */}
      <div className={styles.tabs}>
        {CATEGORY_TABS.map(tab => (
          <button
            key={tab.id}
            className={`${styles.tab} ${activeTab === tab.id ? styles.tabActive : ''}`}
            onClick={() => {
              setActiveTab(tab.id);
              setExpandedSchema(null);
            }}
          >
            {tab.label} ({tab.id === 'gates' ? getCategorySchemas().length :
              schemas.filter(s => {
                const categoryMapping: Record<string, string[]> = {
                  'work-surface': ['record.intake', 'config.procedure_template', 'domain.work_surface'],
                  'oracle': ['oracle_suite', 'oracle_definition', 'semantic_oracle'],
                  'verification': ['verification_profile', 'profile_selection_matrix'],
                  'semantic-sets': ['config.semantic_set', 'semantic_axis', 'decision_rule'],
                  'execution': ['budget_config', 'stop_trigger', 'config.gating_policy'],
                  'exceptions': ['record.waiver', 'record.deviation', 'record.deferral'],
                };
                return (categoryMapping[tab.id] || []).includes(s.type_key);
              }).length
            })
          </button>
        ))}
      </div>

      {/* Schema List */}
      <Card title={`${activeTab === 'gates' ? 'Reference' : CATEGORY_TABS.find(t => t.id === activeTab)?.label} Schemas`}>
        {loading ? (
          <div className={styles.placeholder}>
            <p className={styles.placeholderText}>Loading templates...</p>
          </div>
        ) : error ? (
          <div className={styles.placeholder}>
            <p className={styles.error}>Error: {error}</p>
          </div>
        ) : categorySchemas.length === 0 ? (
          <div className={styles.placeholder}>
            <p className={styles.placeholderText}>No schemas found for this category.</p>
          </div>
        ) : (
          <table className={styles.table}>
            <thead>
              <tr>
                <th className={styles.th}>Type Key</th>
                <th className={styles.th}>Name</th>
                <th className={styles.th}>Description</th>
                <th className={styles.th}>Fields</th>
                <th className={styles.th}>Portal</th>
              </tr>
            </thead>
            <tbody>
              {categorySchemas.map(schema => (
                <>
                  <tr
                    key={schema.type_key}
                    style={{ cursor: 'pointer' }}
                    onClick={() => setExpandedSchema(expandedSchema === schema.type_key ? null : schema.type_key)}
                  >
                    <td className={styles.tdMono}>{schema.type_key}</td>
                    <td className={styles.td}>{schema.name}</td>
                    <td className={styles.td} style={{ maxWidth: '300px' }}>{schema.description}</td>
                    <td className={styles.td}>
                      {schema.required_fields.length} required, {schema.optional_fields.length} optional
                    </td>
                    <td className={styles.td}>
                      {schema.requires_portal ? (
                        <Pill tone="warning">Required</Pill>
                      ) : (
                        <Pill tone="success">Self-service</Pill>
                      )}
                    </td>
                  </tr>
                  {expandedSchema === schema.type_key && (
                    <tr key={`${schema.type_key}-detail`}>
                      <td colSpan={5} style={{ background: 'var(--paper2)', padding: '1rem' }}>
                        <div style={{ marginBottom: '0.5rem' }}>
                          <strong>Source:</strong>{' '}
                          <span style={{ fontFamily: 'var(--mono)', fontSize: '0.75rem' }}>{schema.source_ref}</span>
                        </div>
                        <div style={{ marginBottom: '1rem' }}>
                          <strong>Required Fields:</strong>
                        </div>
                        <table className={styles.table} style={{ marginBottom: '1rem' }}>
                          <thead>
                            <tr>
                              <th className={styles.th}>Field</th>
                              <th className={styles.th}>Type</th>
                              <th className={styles.th}>Description</th>
                              <th className={styles.th}>Example</th>
                            </tr>
                          </thead>
                          <tbody>
                            {schema.required_fields.map(field => (
                              <tr key={field.name}>
                                <td className={styles.tdMono}>{field.name}</td>
                                <td className={styles.td}><Pill tone="neutral">{field.field_type}</Pill></td>
                                <td className={styles.td}>{field.description}</td>
                                <td className={styles.tdMono}>{field.example || '-'}</td>
                              </tr>
                            ))}
                          </tbody>
                        </table>
                        {schema.optional_fields.length > 0 && (
                          <>
                            <div style={{ marginBottom: '1rem' }}>
                              <strong>Optional Fields:</strong>
                            </div>
                            <table className={styles.table}>
                              <thead>
                                <tr>
                                  <th className={styles.th}>Field</th>
                                  <th className={styles.th}>Type</th>
                                  <th className={styles.th}>Description</th>
                                </tr>
                              </thead>
                              <tbody>
                                {schema.optional_fields.map(field => (
                                  <tr key={field.name}>
                                    <td className={styles.tdMono}>{field.name}</td>
                                    <td className={styles.td}><Pill tone="neutral">{field.field_type}</Pill></td>
                                    <td className={styles.td}>{field.description}</td>
                                  </tr>
                                ))}
                              </tbody>
                            </table>
                          </>
                        )}
                      </td>
                    </tr>
                  )}
                </>
              ))}
            </tbody>
          </table>
        )}
      </Card>

      {/* Debug: Show template counts */}
      <Card title="Debug Info">
        <p>Total templates loaded: {templates.length}</p>
        <p>Active tab: {activeTab}</p>
        <p>Reference templates for tab: {referenceTemplates.length}</p>
        <p>User templates for tab: {userTemplates.length}</p>
        {templates.length > 0 && (
          <details>
            <summary>All templates</summary>
            <pre style={{ fontSize: '0.75rem', maxHeight: '200px', overflow: 'auto' }}>
              {JSON.stringify(templates.map(t => ({ id: t.id, category: t.category, status: t.status })), null, 2)}
            </pre>
          </details>
        )}
      </Card>

      {/* Starter Templates - Always visible when available */}
      {referenceTemplates.length > 0 && (
        <Card title="Starter Templates">
          <div style={{ marginBottom: '1rem', color: 'var(--muted)', fontSize: '0.875rem' }}>
            Pre-configured reference templates you can clone and customize for your use case.
          </div>
          <div style={{ display: 'grid', gap: '1rem', gridTemplateColumns: 'repeat(auto-fill, minmax(300px, 1fr))' }}>
            {referenceTemplates.map(template => (
              <div
                key={template.id}
                style={{
                  border: '1px solid var(--border)',
                  borderRadius: 'var(--radiusSm)',
                  padding: '1rem',
                  background: 'var(--paper)',
                }}
              >
                <div style={{ display: 'flex', justifyContent: 'space-between', alignItems: 'flex-start', marginBottom: '0.5rem' }}>
                  <div>
                    <h4 style={{ margin: 0, fontSize: '1rem' }}>{template.name}</h4>
                    <span style={{ fontFamily: 'var(--mono)', fontSize: '0.75rem', color: 'var(--muted)' }}>
                      {template.type_key}
                    </span>
                  </div>
                  <Pill tone="info">Reference</Pill>
                </div>
                <div style={{ display: 'flex', gap: '0.5rem', marginTop: '1rem' }}>
                  <Link
                    to={`/templates/${activeTab}/${encodeURIComponent(template.id)}`}
                    style={{ flex: 1 }}
                  >
                    <Button variant="ghost" style={{ width: '100%' }}>
                      View
                    </Button>
                  </Link>
                  <Button
                    variant="primary"
                    onClick={() => handleClone(template)}
                    style={{ flex: 1 }}
                  >
                    Use Template
                  </Button>
                </div>
              </div>
            ))}
          </div>
        </Card>
      )}

      {/* User-Created Instances */}
      {userTemplates.length > 0 && (
        <Card title="Your Templates">
          <table className={styles.table}>
            <thead>
              <tr>
                <th className={styles.th}>ID</th>
                <th className={styles.th}>Name</th>
                <th className={styles.th}>Type</th>
                <th className={styles.th}>Status</th>
                <th className={styles.th}>Hash</th>
                <th className={styles.th}>Created</th>
              </tr>
            </thead>
            <tbody>
              {userTemplates.map(template => (
                <tr key={template.id}>
                  <td className={styles.td}>
                    <Link
                      to={`/templates/${activeTab}/${encodeURIComponent(template.id)}`}
                      className={styles.link}
                    >
                      {template.id}
                    </Link>
                  </td>
                  <td className={styles.td}>{template.name}</td>
                  <td className={styles.tdMono}>{template.type_key}</td>
                  <td className={styles.td}>
                    <Pill tone={getStatusTone(template.status)}>{template.status}</Pill>
                  </td>
                  <td className={styles.tdMono} title={template.content_hash}>
                    {truncateHash(template.content_hash)}
                  </td>
                  <td className={styles.td}>{formatDate(template.created_at)}</td>
                </tr>
              ))}
            </tbody>
          </table>
        </Card>
      )}

      {/* Governance Info */}
      <Card title="Governance Model">
        <table className={styles.table}>
          <thead>
            <tr>
              <th className={styles.th}>Category</th>
              <th className={styles.th}>Portal Required</th>
              <th className={styles.th}>User Instantiable</th>
              <th className={styles.th}>Status Flow</th>
            </tr>
          </thead>
          <tbody>
            <tr>
              <td className={styles.td}>Work Surface</td>
              <td className={styles.td}><Pill tone="success">No</Pill></td>
              <td className={styles.td}><Pill tone="success">Yes</Pill></td>
              <td className={styles.tdMono}>draft → governed</td>
            </tr>
            <tr>
              <td className={styles.td}>Oracle Configuration</td>
              <td className={styles.td}><Pill tone="warning">GovernanceChangePortal</Pill></td>
              <td className={styles.td}><Pill tone="success">Yes</Pill></td>
              <td className={styles.tdMono}>draft → pending → governed</td>
            </tr>
            <tr>
              <td className={styles.td}>Verification Profiles</td>
              <td className={styles.td}><Pill tone="warning">GovernanceChangePortal</Pill></td>
              <td className={styles.td}><Pill tone="success">Yes</Pill></td>
              <td className={styles.tdMono}>draft → pending → governed</td>
            </tr>
            <tr>
              <td className={styles.td}>Semantic Sets</td>
              <td className={styles.td}><Pill tone="warning">GovernanceChangePortal</Pill></td>
              <td className={styles.td}><Pill tone="success">Yes</Pill></td>
              <td className={styles.tdMono}>draft → pending → governed</td>
            </tr>
            <tr>
              <td className={styles.td}>Exceptions</td>
              <td className={styles.td}><Pill tone="warning">HumanAuthorityException</Pill></td>
              <td className={styles.td}><Pill tone="success">Yes</Pill></td>
              <td className={styles.tdMono}>pending → active → resolved</td>
            </tr>
            <tr>
              <td className={styles.td}>Reference (Gates, Portals)</td>
              <td className={styles.td}><Pill tone="neutral">N/A</Pill></td>
              <td className={styles.td}><Pill tone="danger">No</Pill></td>
              <td className={styles.tdMono}>platform-provided</td>
            </tr>
          </tbody>
        </table>
      </Card>
    </div>
  );
}

export default Templates;
