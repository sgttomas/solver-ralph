/**
 * Context Document Detail Page
 *
 * Displays a single context document with its metadata and references.
 * Per SR-SPEC, context documents are content-addressed and can be
 * referenced in IterationStarted.refs[].
 */

import { useState, useEffect } from 'react';
import { useParams, Link } from 'react-router-dom';
import { useAuth } from '../auth/AuthProvider';
import config from '../config';
import { Card, Pill, Button } from '../ui';
import styles from '../styles/pages.module.css';

interface DocumentReference {
  work_unit_id: string;
  iteration_id: string;
  rel: string;
}

interface ContextDocumentDetail {
  id: string;
  filename: string;
  media_type: string;
  content_hash: string;
  size_bytes: number;
  description: string | null;
  tags: string[];
  referenced_by: number;
  uploaded_by: string;
  uploaded_at: string;
  preview: string | null;
  references: DocumentReference[];
}

function formatBytes(bytes: number): string {
  if (bytes === 0) return '0 B';
  const k = 1024;
  const sizes = ['B', 'KB', 'MB', 'GB'];
  const i = Math.floor(Math.log(bytes) / Math.log(k));
  return parseFloat((bytes / Math.pow(k, i)).toFixed(2)) + ' ' + sizes[i];
}

export function ContextDocumentDetail(): JSX.Element {
  const { documentId } = useParams<{ documentId: string }>();
  const auth = useAuth();
  const [document, setDocument] = useState<ContextDocumentDetail | null>(null);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);
  const [showPreview, setShowPreview] = useState(false);

  useEffect(() => {
    if (!auth.user?.access_token || !documentId) return;

    fetch(`${config.apiUrl}/api/v1/context/documents/${documentId}`, {
      headers: { Authorization: `Bearer ${auth.user.access_token}` },
    })
      .then(res => {
        if (res.status === 404) {
          throw new Error('Document not found');
        }
        if (!res.ok) throw new Error(`HTTP ${res.status}`);
        return res.json();
      })
      .then(data => {
        setDocument(data);
        setLoading(false);
      })
      .catch(err => {
        setError(err.message);
        setLoading(false);
      });
  }, [auth.user?.access_token, documentId]);

  if (loading) {
    return (
      <div className={styles.container}>
        <div className={styles.placeholder}>
          <p className={styles.placeholderText}>Loading document details...</p>
        </div>
      </div>
    );
  }

  if (error || !document) {
    return (
      <div className={styles.container}>
        <div className={styles.placeholder}>
          <p className={styles.error}>Error: {error || 'Document not found'}</p>
          <Link to="/context" className={styles.link}>Back to Context</Link>
        </div>
      </div>
    );
  }

  const isTextType = document.media_type.startsWith('text/') ||
    document.media_type === 'application/json' ||
    document.media_type === 'application/yaml';

  return (
    <div className={styles.container}>
      {/* Breadcrumb */}
      <div className={styles.breadcrumb}>
        <Link to="/context" className={styles.breadcrumbLink}>Context</Link>
        <span className={styles.breadcrumbSeparator}>/</span>
        <span>Documents</span>
        <span className={styles.breadcrumbSeparator}>/</span>
        <span>{document.filename}</span>
      </div>

      {/* Header */}
      <div className={styles.header}>
        <div className={styles.headerStart}>
          <h1 className={styles.title}>{document.filename}</h1>
          <p className={styles.subtitle}>{document.id}</p>
        </div>
        <Pill tone="neutral">{document.media_type}</Pill>
      </div>

      {/* Overview Card */}
      <Card title="Overview" className={styles.cardSpacing}>
        <div className={styles.infoRow}>
          <span className={styles.infoLabel}>Content Hash</span>
          <code className={styles.mono}>{document.content_hash}</code>
        </div>
        <div className={styles.infoRow}>
          <span className={styles.infoLabel}>Media Type</span>
          <span className={styles.infoValue}>{document.media_type}</span>
        </div>
        <div className={styles.infoRow}>
          <span className={styles.infoLabel}>Size</span>
          <span className={styles.infoValue}>{formatBytes(document.size_bytes)}</span>
        </div>
        <div className={styles.infoRow}>
          <span className={styles.infoLabel}>Uploaded By</span>
          <span className={styles.infoValue}>{document.uploaded_by}</span>
        </div>
        <div className={styles.infoRow}>
          <span className={styles.infoLabel}>Uploaded</span>
          <span className={styles.infoValue}>
            {new Date(document.uploaded_at).toLocaleString()}
          </span>
        </div>
      </Card>

      {/* Description Card */}
      {document.description && (
        <Card title="Description" className={styles.cardSpacing}>
          <p style={{ margin: 0, fontSize: '0.875rem', lineHeight: 1.6 }}>
            {document.description}
          </p>
        </Card>
      )}

      {/* Tags Card */}
      {document.tags.length > 0 && (
        <Card title="Tags" className={styles.cardSpacing}>
          <div className={styles.badgeGroup}>
            {document.tags.map(tag => (
              <Pill key={tag} tone="neutral">{tag}</Pill>
            ))}
          </div>
        </Card>
      )}

      {/* Content Preview Card */}
      <Card
        title="Content"
        right={
          <div style={{ display: 'flex', gap: 'var(--space2)' }}>
            {isTextType && document.preview && (
              <Button
                variant={showPreview ? 'ghost' : 'secondary'}
                onClick={() => setShowPreview(!showPreview)}
              >
                {showPreview ? 'Hide Preview' : 'Show Preview'}
              </Button>
            )}
            <a
              href={`${config.apiUrl}/api/v1/context/documents/${document.id}/download`}
              target="_blank"
              rel="noopener noreferrer"
              style={{ textDecoration: 'none' }}
            >
              <Button variant="primary">Download</Button>
            </a>
          </div>
        }
        className={styles.cardSpacing}
      >
        {showPreview && document.preview ? (
          <pre style={{
            backgroundColor: 'var(--ink)',
            color: '#e0e0e0',
            padding: 'var(--space4)',
            borderRadius: 'var(--radiusSm)',
            fontFamily: 'var(--mono)',
            fontSize: '0.75rem',
            overflow: 'auto',
            maxHeight: '400px',
            whiteSpace: 'pre-wrap',
            wordBreak: 'break-all',
            margin: 0,
          }}>
            {document.preview}
          </pre>
        ) : (
          <div className={styles.placeholder}>
            <p className={styles.placeholderText}>
              {isTextType
                ? 'Click "Show Preview" to view content'
                : 'Preview not available for this file type'}
            </p>
            <p className={styles.placeholderHint}>
              Use the Download button to access the full file.
            </p>
          </div>
        )}
      </Card>

      {/* References Card */}
      <Card title={`References (${document.references.length})`}>
        {document.references.length === 0 ? (
          <div className={styles.placeholder}>
            <p className={styles.placeholderText}>
              This document is not currently referenced by any work units.
            </p>
          </div>
        ) : (
          <table className={styles.table}>
            <thead>
              <tr>
                <th className={styles.th}>Loop</th>
                <th className={styles.th}>Iteration</th>
                <th className={styles.th}>Relation</th>
              </tr>
            </thead>
            <tbody>
              {document.references.map((ref, idx) => (
                <tr key={idx}>
                  <td className={styles.td}>
                    <Link to={`/loops/${ref.work_unit_id}`} className={styles.link}>
                      {ref.work_unit_id}
                    </Link>
                  </td>
                  <td className={styles.td}>
                    <Link to={`/iterations/${ref.iteration_id}`} className={styles.link}>
                      {ref.iteration_id}
                    </Link>
                  </td>
                  <td className={styles.td}>{ref.rel}</td>
                </tr>
              ))}
            </tbody>
          </table>
        )}
      </Card>
    </div>
  );
}

export default ContextDocumentDetail;
