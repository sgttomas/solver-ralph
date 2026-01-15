/**
 * Context Page
 *
 * Manages context documents that can be referenced in semantic work.
 * Per SR-SPEC and SR-WORK-SURFACE, all iteration context must be:
 * - Content-addressed (sha256 hash)
 * - Explicitly referenced in IterationStarted.refs[]
 * - No ghost inputs allowed
 *
 * This page allows:
 * - Uploading context documents
 * - Viewing existing context artifacts
 * - Managing intake records
 * - Viewing context bundles for iterations
 */

import { useState, useEffect, useRef, useCallback } from 'react';
import { Link } from 'react-router-dom';
import { useAuth } from '../auth/AuthProvider';
import config from '../config';
import { Card, Pill } from '../ui';
import styles from '../styles/pages.module.css';

type ContextTab = 'documents' | 'intakes' | 'bundles';

interface ContextDocument {
  id: string;
  filename: string;
  media_type: string;
  content_hash: string;
  size_bytes: number;
  description: string | null;
  tags: string[];
  referenced_by: number; // count of work units referencing this
  uploaded_by: string;
  uploaded_at: string;
}

interface Intake {
  id: string;
  work_unit_id: string;
  title: string;
  kind: string;
  objective: string;
  content_hash: string;
  status: 'draft' | 'active' | 'archived';
  created_at: string;
}

interface ContextBundle {
  id: string;
  iteration_id: string;
  loop_id: string;
  refs_count: number;
  content_hash: string;
  created_at: string;
}

interface ContextResponse {
  documents: ContextDocument[];
  intakes: Intake[];
  bundles: ContextBundle[];
}

export function Context(): JSX.Element {
  const auth = useAuth();
  const [activeTab, setActiveTab] = useState<ContextTab>('documents');
  const [documents, setDocuments] = useState<ContextDocument[]>([]);
  const [intakes, setIntakes] = useState<Intake[]>([]);
  const [bundles, setBundles] = useState<ContextBundle[]>([]);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);

  // Upload state
  const [uploading, setUploading] = useState(false);
  const [uploadError, setUploadError] = useState<string | null>(null);
  const [uploadSuccess, setUploadSuccess] = useState<string | null>(null);
  const [dragActive, setDragActive] = useState(false);
  const fileInputRef = useRef<HTMLInputElement>(null);

  const fetchData = useCallback(() => {
    if (!auth.user?.access_token) return;

    setLoading(true);
    fetch(`${config.apiUrl}/api/v1/context`, {
      headers: {
        Authorization: `Bearer ${auth.user.access_token}`,
      },
    })
      .then(res => {
        // Treat 404 as "no data yet" rather than an error
        if (res.status === 404) {
          return { documents: [], intakes: [], bundles: [] };
        }
        if (!res.ok) throw new Error(`HTTP ${res.status}`);
        return res.json();
      })
      .then((data: ContextResponse) => {
        setDocuments(data.documents || []);
        setIntakes(data.intakes || []);
        setBundles(data.bundles || []);
        setLoading(false);
      })
      .catch(err => {
        setError(err.message);
        setLoading(false);
      });
  }, [auth.user?.access_token]);

  useEffect(() => {
    fetchData();
  }, [fetchData]);

  const handleFileUpload = async (files: FileList | null) => {
    if (!files || files.length === 0 || !auth.user?.access_token) return;

    setUploading(true);
    setUploadError(null);
    setUploadSuccess(null);

    const formData = new FormData();
    for (let i = 0; i < files.length; i++) {
      formData.append('files', files[i]);
    }

    try {
      const res = await fetch(`${config.apiUrl}/api/v1/context/upload`, {
        method: 'POST',
        headers: {
          Authorization: `Bearer ${auth.user.access_token}`,
        },
        body: formData,
      });

      if (!res.ok) {
        const errData = await res.json().catch(() => ({}));
        throw new Error(errData.error || `HTTP ${res.status}`);
      }

      const data = await res.json();
      setUploadSuccess(`Successfully uploaded ${data.uploaded?.length || files.length} file(s)`);
      fetchData(); // Refresh the list
    } catch (err) {
      setUploadError(err instanceof Error ? err.message : 'Upload failed');
    } finally {
      setUploading(false);
      if (fileInputRef.current) {
        fileInputRef.current.value = '';
      }
    }
  };

  const handleDrag = (e: React.DragEvent) => {
    e.preventDefault();
    e.stopPropagation();
    if (e.type === 'dragenter' || e.type === 'dragover') {
      setDragActive(true);
    } else if (e.type === 'dragleave') {
      setDragActive(false);
    }
  };

  const handleDrop = (e: React.DragEvent) => {
    e.preventDefault();
    e.stopPropagation();
    setDragActive(false);
    handleFileUpload(e.dataTransfer.files);
  };

  const formatBytes = (bytes: number): string => {
    if (bytes === 0) return '0 B';
    const k = 1024;
    const sizes = ['B', 'KB', 'MB', 'GB'];
    const i = Math.floor(Math.log(bytes) / Math.log(k));
    return parseFloat((bytes / Math.pow(k, i)).toFixed(1)) + ' ' + sizes[i];
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
          <h1 className={styles.title}>Context</h1>
          <p className={styles.subtitle}>Documents and artifacts for semantic work context</p>
        </div>
      </div>

      {/* Overview Stats */}
      <Card>
        <div className={styles.statsGrid}>
          <div className={styles.stat}>
            <div className={styles.statLabel}>Documents</div>
            <div className={styles.statValue}>{documents.length}</div>
          </div>
          <div className={styles.stat}>
            <div className={styles.statLabel}>Intakes</div>
            <div className={styles.statValue}>{intakes.length}</div>
          </div>
          <div className={styles.stat}>
            <div className={styles.statLabel}>Context Bundles</div>
            <div className={styles.statValue}>{bundles.length}</div>
          </div>
          <div className={styles.stat}>
            <div className={styles.statLabel}>Addressing</div>
            <div className={styles.statValue}>sha256</div>
          </div>
        </div>
      </Card>

      {/* Info Note */}
      <div className={styles.note}>
        Per SR-SPEC C-CTX-2: No ghost inputs. All agent-relevant authoritative inputs must be
        derivable from IterationStarted.refs[]. Documents uploaded here are content-addressed
        and can be referenced in work surfaces and iteration context.
      </div>

      {/* Upload Section */}
      <Card>
        <h3 style={{ margin: '0 0 1rem 0', fontSize: '0.875rem', color: 'var(--ink)' }}>
          Upload Context Documents
        </h3>

        <div
          onDragEnter={handleDrag}
          onDragLeave={handleDrag}
          onDragOver={handleDrag}
          onDrop={handleDrop}
          style={{
            border: `2px dashed ${dragActive ? 'var(--accent)' : 'var(--border)'}`,
            borderRadius: 'var(--radiusSm)',
            padding: 'var(--space5)',
            textAlign: 'center',
            backgroundColor: dragActive ? 'rgba(0, 102, 204, 0.05)' : 'transparent',
            transition: 'all 150ms ease',
            cursor: 'pointer',
          }}
          onClick={() => fileInputRef.current?.click()}
        >
          <input
            ref={fileInputRef}
            type="file"
            multiple
            style={{ display: 'none' }}
            onChange={(e) => handleFileUpload(e.target.files)}
            accept=".md,.txt,.json,.yaml,.yml,.pdf,.png,.jpg,.jpeg,.csv"
          />
          <p style={{ margin: 0, color: 'var(--muted)', fontSize: '0.875rem' }}>
            {uploading ? 'Uploading...' : 'Drag & drop files here, or click to select'}
          </p>
          <p style={{ margin: '0.5rem 0 0 0', color: 'var(--muted)', fontSize: '0.75rem' }}>
            Supported: .md, .txt, .json, .yaml, .pdf, .png, .jpg, .csv
          </p>
        </div>

        {uploadError && (
          <div className={styles.error} style={{ marginTop: '1rem', marginBottom: 0 }}>
            {uploadError}
          </div>
        )}
        {uploadSuccess && (
          <div className={styles.success} style={{ marginTop: '1rem', marginBottom: 0 }}>
            {uploadSuccess}
          </div>
        )}
      </Card>

      {/* Tabs */}
      <div className={styles.tabs}>
        <button
          className={`${styles.tab} ${activeTab === 'documents' ? styles.tabActive : ''}`}
          onClick={() => setActiveTab('documents')}
        >
          Documents ({documents.length})
        </button>
        <button
          className={`${styles.tab} ${activeTab === 'intakes' ? styles.tabActive : ''}`}
          onClick={() => setActiveTab('intakes')}
        >
          Intakes ({intakes.length})
        </button>
        <button
          className={`${styles.tab} ${activeTab === 'bundles' ? styles.tabActive : ''}`}
          onClick={() => setActiveTab('bundles')}
        >
          Context Bundles ({bundles.length})
        </button>
      </div>

      {/* Tab Content */}
      <Card>
        {loading ? (
          <div className={styles.placeholder}>
            <p className={styles.placeholderText}>Loading context data...</p>
          </div>
        ) : error ? (
          <div className={styles.placeholder}>
            <p className={styles.error}>Error: {error}</p>
          </div>
        ) : activeTab === 'documents' ? (
          documents.length === 0 ? (
            <div className={styles.placeholder}>
              <p className={styles.placeholderText}>No context documents uploaded.</p>
              <p className={styles.placeholderHint}>
                Upload documents above to make them available as context for semantic work.
                Each document is content-addressed and can be referenced by hash.
              </p>
            </div>
          ) : (
            <table className={styles.table}>
              <thead>
                <tr>
                  <th className={styles.th}>Filename</th>
                  <th className={styles.th}>Type</th>
                  <th className={styles.th}>Size</th>
                  <th className={styles.th}>Content Hash</th>
                  <th className={styles.th}>Tags</th>
                  <th className={styles.th}>Referenced</th>
                  <th className={styles.th}>Uploaded</th>
                </tr>
              </thead>
              <tbody>
                {documents.map(doc => (
                  <tr key={doc.id}>
                    <td className={styles.td}>
                      <Link to={`/context/${doc.id}`} className={styles.link}>
                        {doc.filename}
                      </Link>
                    </td>
                    <td className={styles.tdMono}>{doc.media_type}</td>
                    <td className={styles.td}>{formatBytes(doc.size_bytes)}</td>
                    <td className={styles.tdMono}>{truncateHash(doc.content_hash)}</td>
                    <td className={styles.td}>
                      <div className={styles.badgeGroup}>
                        {doc.tags.slice(0, 2).map(tag => (
                          <Pill key={tag} tone="neutral">{tag}</Pill>
                        ))}
                        {doc.tags.length > 2 && (
                          <Pill tone="neutral">+{doc.tags.length - 2}</Pill>
                        )}
                      </div>
                    </td>
                    <td className={styles.td}>
                      {doc.referenced_by > 0 ? (
                        <Pill tone="success">{doc.referenced_by} refs</Pill>
                      ) : (
                        <span style={{ color: 'var(--muted)' }}>—</span>
                      )}
                    </td>
                    <td className={styles.td}>
                      {new Date(doc.uploaded_at).toLocaleDateString()}
                    </td>
                  </tr>
                ))}
              </tbody>
            </table>
          )
        ) : activeTab === 'intakes' ? (
          intakes.length === 0 ? (
            <div className={styles.placeholder}>
              <p className={styles.placeholderText}>No intake records.</p>
              <p className={styles.placeholderHint}>
                Intakes define the objective, scope, constraints, and deliverables
                for a work unit. They are created when starting semantic work.
              </p>
            </div>
          ) : (
            <table className={styles.table}>
              <thead>
                <tr>
                  <th className={styles.th}>Title</th>
                  <th className={styles.th}>Kind</th>
                  <th className={styles.th}>Work Unit</th>
                  <th className={styles.th}>Objective</th>
                  <th className={styles.th}>Status</th>
                  <th className={styles.th}>Created</th>
                </tr>
              </thead>
              <tbody>
                {intakes.map(intake => (
                  <tr key={intake.id}>
                    <td className={styles.td}>
                      <Link to={`/context/intakes/${intake.id}`} className={styles.link}>
                        {intake.title}
                      </Link>
                    </td>
                    <td className={styles.td}>
                      <Pill tone="neutral">{intake.kind}</Pill>
                    </td>
                    <td className={styles.tdMono}>
                      <Link to={`/loops/${intake.work_unit_id}`} className={styles.link}>
                        {intake.work_unit_id.slice(0, 12)}...
                      </Link>
                    </td>
                    <td className={styles.td} style={{ maxWidth: '300px' }}>
                      {intake.objective.length > 80
                        ? intake.objective.slice(0, 80) + '...'
                        : intake.objective}
                    </td>
                    <td className={styles.td}>
                      <Pill tone={
                        intake.status === 'active' ? 'success' :
                        intake.status === 'draft' ? 'warning' : 'neutral'
                      }>
                        {intake.status}
                      </Pill>
                    </td>
                    <td className={styles.td}>
                      {new Date(intake.created_at).toLocaleDateString()}
                    </td>
                  </tr>
                ))}
              </tbody>
            </table>
          )
        ) : (
          bundles.length === 0 ? (
            <div className={styles.placeholder}>
              <p className={styles.placeholderText}>No context bundles recorded.</p>
              <p className={styles.placeholderHint}>
                Context bundles capture the complete effective context for an iteration.
                They are created automatically when iterations start.
              </p>
            </div>
          ) : (
            <table className={styles.table}>
              <thead>
                <tr>
                  <th className={styles.th}>Bundle ID</th>
                  <th className={styles.th}>Iteration</th>
                  <th className={styles.th}>Loop</th>
                  <th className={styles.th}>Refs Count</th>
                  <th className={styles.th}>Content Hash</th>
                  <th className={styles.th}>Created</th>
                </tr>
              </thead>
              <tbody>
                {bundles.map(bundle => (
                  <tr key={bundle.id}>
                    <td className={styles.tdMono}>
                      <Link to={`/context/bundles/${bundle.id}`} className={styles.link}>
                        {bundle.id.slice(0, 16)}...
                      </Link>
                    </td>
                    <td className={styles.tdMono}>
                      <Link to={`/iterations/${bundle.iteration_id}`} className={styles.link}>
                        {bundle.iteration_id.slice(0, 12)}...
                      </Link>
                    </td>
                    <td className={styles.tdMono}>
                      <Link to={`/loops/${bundle.loop_id}`} className={styles.link}>
                        {bundle.loop_id.slice(0, 12)}...
                      </Link>
                    </td>
                    <td className={styles.td}>{bundle.refs_count} refs</td>
                    <td className={styles.tdMono}>{truncateHash(bundle.content_hash)}</td>
                    <td className={styles.td}>
                      {new Date(bundle.created_at).toLocaleString()}
                    </td>
                  </tr>
                ))}
              </tbody>
            </table>
          )
        )}
      </Card>

      {/* Schema Reference */}
      <Card>
        <h3 style={{ margin: '0 0 1rem 0', fontSize: '0.875rem', color: 'var(--ink)' }}>
          Intake Schema (SR-WORK-SURFACE)
        </h3>
        <p style={{ margin: '0 0 1rem 0', fontSize: '0.875rem', color: 'var(--muted)' }}>
          Intakes define the structured problem statement for a work unit.
          Required fields per SR-WORK-SURFACE §3.1:
        </p>
        <table className={styles.table}>
          <thead>
            <tr>
              <th className={styles.th}>Field</th>
              <th className={styles.th}>Type</th>
              <th className={styles.th}>Description</th>
            </tr>
          </thead>
          <tbody>
            <tr>
              <td className={styles.tdMono}>work_unit_id</td>
              <td className={styles.td}>string</td>
              <td className={styles.td}>Stable identifier for the work unit</td>
            </tr>
            <tr>
              <td className={styles.tdMono}>title</td>
              <td className={styles.td}>string</td>
              <td className={styles.td}>Human-readable title</td>
            </tr>
            <tr>
              <td className={styles.tdMono}>kind</td>
              <td className={styles.td}>string</td>
              <td className={styles.td}>Work kind (e.g., research_memo, decision_record)</td>
            </tr>
            <tr>
              <td className={styles.tdMono}>objective</td>
              <td className={styles.td}>string</td>
              <td className={styles.td}>One sentence objective</td>
            </tr>
            <tr>
              <td className={styles.tdMono}>deliverables[]</td>
              <td className={styles.td}>array</td>
              <td className={styles.td}>Required outputs with format and paths</td>
            </tr>
            <tr>
              <td className={styles.tdMono}>constraints[]</td>
              <td className={styles.td}>array</td>
              <td className={styles.td}>Length, tone, required sections, prohibited content</td>
            </tr>
            <tr>
              <td className={styles.tdMono}>inputs[]</td>
              <td className={styles.td}>array</td>
              <td className={styles.td}>Provided context refs (content-addressed)</td>
            </tr>
          </tbody>
        </table>
      </Card>
    </div>
  );
}

export default Context;
