/**
 * Template Detail Page
 *
 * Display detailed view of a template instance with its schema,
 * content, and governance information.
 */

import { useState, useEffect } from 'react';
import { useParams, Link, useSearchParams, useNavigate } from 'react-router-dom';
import { useAuth } from '../auth/AuthProvider';
import config from '../config';
import { Card, Pill, Button } from '../ui';
import styles from '../styles/pages.module.css';

// ============================================================================
// Types
// ============================================================================

interface FieldSchema {
  name: string;
  field_type: string;
  description: string;
  example?: string;
}

interface TemplateSchemaResponse {
  type_key: string;
  name: string;
  description: string;
  source_ref: string;
  required_fields: FieldSchema[];
  optional_fields: FieldSchema[];
  requires_portal: boolean;
}

interface TemplateRef {
  rel: string;
  to: string;
}

interface TemplateDetailResponse {
  id: string;
  type_key: string;
  name: string;
  category: string;
  category_label: string;
  status: string;
  content_hash: string;
  content: Record<string, unknown>;
  schema: TemplateSchemaResponse;
  created_at: string;
  updated_at: string;
  created_by: string;
  requires_portal: boolean;
  portal_approval_id?: string;
  refs: TemplateRef[];
}

// ============================================================================
// Component
// ============================================================================

export function TemplateDetail(): JSX.Element {
  const { category: _category, templateId } = useParams<{ category: string; templateId: string }>();
  const [searchParams] = useSearchParams();
  const navigate = useNavigate();
  const auth = useAuth();
  const [template, setTemplate] = useState<TemplateDetailResponse | null>(null);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);
  const [showRawJson, setShowRawJson] = useState(false);

  // Edit mode state
  const [isEditing, setIsEditing] = useState(searchParams.get('edit') === 'true');
  const [editName, setEditName] = useState('');
  const [editContent, setEditContent] = useState('');
  const [saving, setSaving] = useState(false);

  useEffect(() => {
    if (!auth.user?.access_token || !templateId) return;

    fetch(`${config.apiUrl}/api/v1/templates/${encodeURIComponent(templateId)}`, {
      headers: {
        Authorization: `Bearer ${auth.user.access_token}`,
      },
    })
      .then(res => {
        if (res.status === 404) throw new Error('Template not found');
        if (!res.ok) throw new Error(`HTTP ${res.status}`);
        return res.json();
      })
      .then((data: TemplateDetailResponse) => {
        setTemplate(data);
        setEditName(data.name);
        setEditContent(JSON.stringify(data.content, null, 2));
        setLoading(false);
        // Auto-open edit mode for new cloned templates
        if (searchParams.get('edit') === 'true' && data.status !== 'reference') {
          setIsEditing(true);
        }
      })
      .catch(err => {
        setError(err.message);
        setLoading(false);
      });
  }, [auth.user?.access_token, templateId, searchParams]);

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

  const handleClone = async () => {
    if (!auth.user?.access_token || !template) return;

    try {
      const createRes = await fetch(`${config.apiUrl}/api/v1/templates`, {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json',
          Authorization: `Bearer ${auth.user.access_token}`,
        },
        body: JSON.stringify({
          type_key: template.type_key,
          name: `${template.name} (Copy)`,
          content: template.content,
          refs: template.refs || [],
        }),
      });

      if (!createRes.ok) {
        const errorData = await createRes.json().catch(() => ({}));
        throw new Error(errorData.message || `HTTP ${createRes.status}`);
      }

      const newTemplate = await createRes.json();
      // Navigate to the new template in edit mode
      navigate(`/templates/${template.category}/${encodeURIComponent(newTemplate.id)}?edit=true`);
    } catch (err) {
      console.error('Clone failed:', err);
      setError(err instanceof Error ? err.message : 'Clone failed');
    }
  };

  const handleSave = async () => {
    if (!auth.user?.access_token || !template) return;

    // Validate JSON
    let parsedContent;
    try {
      parsedContent = JSON.parse(editContent);
    } catch {
      setError('Invalid JSON content');
      return;
    }

    setSaving(true);
    setError(null);

    try {
      const res = await fetch(
        `${config.apiUrl}/api/v1/templates/${encodeURIComponent(template.id)}`,
        {
          method: 'PUT',
          headers: {
            'Content-Type': 'application/json',
            Authorization: `Bearer ${auth.user.access_token}`,
          },
          body: JSON.stringify({
            name: editName,
            content: parsedContent,
          }),
        }
      );

      if (!res.ok) {
        const errorData = await res.json().catch(() => ({}));
        throw new Error(errorData.message || `HTTP ${res.status}`);
      }

      // Refresh template data
      const refreshRes = await fetch(
        `${config.apiUrl}/api/v1/templates/${encodeURIComponent(template.id)}`,
        {
          headers: {
            Authorization: `Bearer ${auth.user.access_token}`,
          },
        }
      );

      if (refreshRes.ok) {
        const data: TemplateDetailResponse = await refreshRes.json();
        setTemplate(data);
        setEditContent(JSON.stringify(data.content, null, 2));
      }

      setIsEditing(false);
      // Remove edit param from URL
      navigate(`/templates/${_category}/${encodeURIComponent(template.id)}`, { replace: true });
    } catch (err) {
      console.error('Save failed:', err);
      setError(err instanceof Error ? err.message : 'Save failed');
    } finally {
      setSaving(false);
    }
  };

  const handleCancelEdit = () => {
    if (template) {
      setEditName(template.name);
      setEditContent(JSON.stringify(template.content, null, 2));
    }
    setIsEditing(false);
    // Remove edit param from URL
    navigate(`/templates/${_category}/${templateId}`, { replace: true });
  };

  const formatDate = (dateStr: string) => {
    const date = new Date(dateStr);
    return date.toLocaleDateString('en-US', {
      month: 'short',
      day: 'numeric',
      year: 'numeric',
      hour: '2-digit',
      minute: '2-digit',
    });
  };

  if (loading) {
    return (
      <div className={styles.container}>
        <div className={styles.placeholder}>
          <p className={styles.placeholderText}>Loading template...</p>
        </div>
      </div>
    );
  }

  if (error || !template) {
    return (
      <div className={styles.container}>
        <div className={styles.breadcrumbs}>
          <Link to="/templates" className={styles.link}>Templates</Link>
          <span className={styles.breadcrumbSep}>/</span>
          <span>{templateId}</span>
        </div>
        <div className={styles.placeholder}>
          <p className={styles.error}>Error: {error || 'Template not found'}</p>
          <Link to="/templates">
            <Button variant="secondary">Back to Templates</Button>
          </Link>
        </div>
      </div>
    );
  }

  return (
    <div className={styles.container}>
      {/* Breadcrumbs */}
      <div className={styles.breadcrumbs}>
        <Link to="/templates" className={styles.link}>Templates</Link>
        <span className={styles.breadcrumbSep}>/</span>
        <span>{template.category_label}</span>
        <span className={styles.breadcrumbSep}>/</span>
        <span>{template.id}</span>
      </div>

      {/* Header */}
      <div className={styles.header}>
        <div className={styles.headerStart}>
          <h1 className={styles.title}>{isEditing ? 'Edit Template' : template.name}</h1>
          <p className={styles.subtitle}>
            <span style={{ fontFamily: 'var(--mono)', fontSize: '0.75rem' }}>{template.type_key}</span>
            {' '}&middot;{' '}
            {template.content_hash.slice(0, 20)}...
          </p>
        </div>
        <div className={styles.headerEnd}>
          <Pill tone={getStatusTone(template.status)}>{template.status}</Pill>
          {template.requires_portal && (
            <Pill tone="warning">Portal Required</Pill>
          )}
          {template.status === 'reference' ? (
            <Button variant="primary" onClick={handleClone}>
              Use Template
            </Button>
          ) : !isEditing ? (
            <Button variant="secondary" onClick={() => setIsEditing(true)}>
              Edit
            </Button>
          ) : null}
        </div>
      </div>

      {/* Reference Template Banner */}
      {template.status === 'reference' && (
        <div
          style={{
            background: 'color-mix(in srgb, var(--info) 10%, white)',
            border: '1px solid color-mix(in srgb, var(--info) 25%, transparent)',
            borderRadius: 'var(--radiusSm)',
            padding: '1rem',
            marginBottom: '1rem',
            color: 'var(--info)',
            display: 'flex',
            justifyContent: 'space-between',
            alignItems: 'center',
            gap: '1rem',
          }}
        >
          <div>
            <strong>Reference Template</strong>
            <p style={{ margin: '0.5rem 0 0', fontSize: '0.875rem' }}>
              This is a system-provided reference template. Clone it to create your own editable copy.
            </p>
          </div>
          <Button variant="primary" onClick={handleClone}>
            Clone Template
          </Button>
        </div>
      )}

      {/* Overview */}
      <Card title="Overview">
        <div className={styles.infoRows}>
          <div className={styles.infoRow}>
            <span className={styles.infoLabel}>ID</span>
            <span className={styles.infoValue} style={{ fontFamily: 'var(--mono)' }}>{template.id}</span>
          </div>
          <div className={styles.infoRow}>
            <span className={styles.infoLabel}>Type Key</span>
            <span className={styles.infoValue} style={{ fontFamily: 'var(--mono)' }}>{template.type_key}</span>
          </div>
          <div className={styles.infoRow}>
            <span className={styles.infoLabel}>Category</span>
            <span className={styles.infoValue}>{template.category_label}</span>
          </div>
          <div className={styles.infoRow}>
            <span className={styles.infoLabel}>Status</span>
            <span className={styles.infoValue}>
              <Pill tone={getStatusTone(template.status)}>{template.status}</Pill>
            </span>
          </div>
          <div className={styles.infoRow}>
            <span className={styles.infoLabel}>Content Hash</span>
            <span className={styles.infoValue} style={{ fontFamily: 'var(--mono)', fontSize: '0.75rem' }}>
              {template.content_hash}
            </span>
          </div>
          <div className={styles.infoRow}>
            <span className={styles.infoLabel}>Created</span>
            <span className={styles.infoValue}>{formatDate(template.created_at)}</span>
          </div>
          <div className={styles.infoRow}>
            <span className={styles.infoLabel}>Updated</span>
            <span className={styles.infoValue}>{formatDate(template.updated_at)}</span>
          </div>
          <div className={styles.infoRow}>
            <span className={styles.infoLabel}>Created By</span>
            <span className={styles.infoValue}>{template.created_by}</span>
          </div>
          {template.portal_approval_id && (
            <div className={styles.infoRow}>
              <span className={styles.infoLabel}>Portal Approval</span>
              <span className={styles.infoValue}>
                <Link to={`/approvals/${template.portal_approval_id}`} className={styles.link}>
                  {template.portal_approval_id}
                </Link>
              </span>
            </div>
          )}
        </div>
      </Card>

      {/* Schema Info */}
      <Card title={`Schema: ${template.schema.name}`}>
        <div style={{ marginBottom: '1rem' }}>
          <p style={{ margin: 0, color: 'var(--muted)' }}>{template.schema.description}</p>
          <p style={{ margin: '0.5rem 0 0', fontFamily: 'var(--mono)', fontSize: '0.75rem' }}>
            Source: {template.schema.source_ref}
          </p>
        </div>

        <div style={{ marginBottom: '1rem' }}>
          <strong>Required Fields ({template.schema.required_fields.length})</strong>
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
            {template.schema.required_fields.map(field => (
              <tr key={field.name}>
                <td className={styles.tdMono}>{field.name}</td>
                <td className={styles.td}><Pill tone="neutral">{field.field_type}</Pill></td>
                <td className={styles.td}>{field.description}</td>
              </tr>
            ))}
          </tbody>
        </table>

        {template.schema.optional_fields.length > 0 && (
          <>
            <div style={{ margin: '1rem 0' }}>
              <strong>Optional Fields ({template.schema.optional_fields.length})</strong>
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
                {template.schema.optional_fields.map(field => (
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
      </Card>

      {/* References */}
      {template.refs.length > 0 && (
        <Card title="References">
          <table className={styles.table}>
            <thead>
              <tr>
                <th className={styles.th}>Relationship</th>
                <th className={styles.th}>Target</th>
              </tr>
            </thead>
            <tbody>
              {template.refs.map((ref, idx) => (
                <tr key={idx}>
                  <td className={styles.td}>{ref.rel}</td>
                  <td className={styles.tdMono}>{ref.to}</td>
                </tr>
              ))}
            </tbody>
          </table>
        </Card>
      )}

      {/* Edit Form or Content Display */}
      {isEditing ? (
        <Card title="Edit Template">
          {error && (
            <div style={{ color: 'var(--danger)', marginBottom: '1rem', padding: '0.5rem', background: 'color-mix(in srgb, var(--danger) 10%, white)', borderRadius: 'var(--radiusSm)' }}>
              {error}
            </div>
          )}
          <div style={{ marginBottom: '1rem' }}>
            <label style={{ display: 'block', marginBottom: '0.5rem', fontWeight: 500 }}>
              Template Name
            </label>
            <input
              type="text"
              value={editName}
              onChange={(e) => setEditName(e.target.value)}
              style={{
                width: '100%',
                padding: '0.75rem',
                border: '1px solid var(--border)',
                borderRadius: 'var(--radiusSm)',
                fontSize: '1rem',
                fontFamily: 'var(--font)',
              }}
            />
          </div>
          <div style={{ marginBottom: '1rem' }}>
            <label style={{ display: 'block', marginBottom: '0.5rem', fontWeight: 500 }}>
              Content (JSON)
            </label>
            <p style={{ margin: '0 0 0.5rem', fontSize: '0.875rem', color: 'var(--muted)' }}>
              Customize the template content for your specific use case.
            </p>
            <textarea
              value={editContent}
              onChange={(e) => setEditContent(e.target.value)}
              rows={20}
              style={{
                width: '100%',
                padding: '0.75rem',
                border: '1px solid var(--border)',
                borderRadius: 'var(--radiusSm)',
                fontSize: '0.75rem',
                fontFamily: 'var(--mono)',
                resize: 'vertical',
                background: 'var(--paper2)',
              }}
            />
          </div>
          <div style={{ display: 'flex', gap: '0.5rem', justifyContent: 'flex-end' }}>
            <Button variant="ghost" onClick={handleCancelEdit} disabled={saving}>
              Cancel
            </Button>
            <Button variant="primary" onClick={handleSave} disabled={saving}>
              {saving ? 'Saving...' : 'Save Changes'}
            </Button>
          </div>
        </Card>
      ) : (
        <Card
          title="Content"
          right={
            <Button variant="ghost" onClick={() => setShowRawJson(!showRawJson)}>
              {showRawJson ? 'Hide JSON' : 'Show JSON'}
            </Button>
          }
        >
          {showRawJson ? (
            <pre
              style={{
                background: 'var(--ink)',
                color: 'var(--paper)',
                padding: '1rem',
                borderRadius: 'var(--radiusSm)',
                overflow: 'auto',
                maxHeight: '400px',
                fontSize: '0.75rem',
                fontFamily: 'var(--mono)',
              }}
            >
              {JSON.stringify(template.content, null, 2)}
            </pre>
          ) : (
            <div className={styles.placeholder}>
              <p className={styles.placeholderText}>Click "Show JSON" to view raw content</p>
            </div>
          )}
        </Card>
      )}
    </div>
  );
}

export default TemplateDetail;
