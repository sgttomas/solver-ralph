/**
 * References Page
 *
 * Browser for all typed references in the system per SR-PLAN-V3 Phase 3.
 * Provides a category sidebar with 11 reference categories, each backed
 * by the References API endpoints from Phase 0c.
 *
 * Per SR-SPEC §3.2.1.1: Endpoints for browsing all typed references.
 */

import { useState, useEffect, useCallback, useRef } from 'react';
import { useNavigate } from 'react-router-dom';
import { useAuth } from '../auth/AuthProvider';
import config from '../config';
import { Card, Button, Pill } from '../ui';
import styles from '../styles/pages.module.css';

// ============================================================================
// Types
// ============================================================================

interface TypedRef {
  kind: string;
  id: string;
  rel: string;
  meta: {
    content_hash?: string;
    version?: string;
    type_key?: string;
    selector?: string;
  };
  label?: string;
}

interface ReferencesListResponse {
  refs: TypedRef[];
  total: number;
  page: number;
  page_size: number;
}

interface Category {
  id: string;
  label: string;
  endpoint: string;
  count?: number;
}

interface Pagination {
  page: number;
  pageSize: number;
  total: number;
}

// ============================================================================
// Constants
// ============================================================================

const CATEGORIES: Category[] = [
  { id: 'governed-artifacts', label: 'Governing Artifacts', endpoint: '/references/governed-artifacts' },
  { id: 'templates', label: 'Templates', endpoint: '/references/templates' },
  { id: 'oracle-suites', label: 'Oracle Suites', endpoint: '/references/oracle-suites' },
  { id: 'evidence-bundles', label: 'Evidence Bundles', endpoint: '/references/evidence-bundles' },
  { id: 'iteration-summaries', label: 'Iterations', endpoint: '/references/iteration-summaries' },
  { id: 'candidates', label: 'Candidates', endpoint: '/references/candidates' },
  { id: 'exceptions', label: 'Exceptions', endpoint: '/references/exceptions' },
  { id: 'agent-definitions', label: 'Agent Definitions', endpoint: '/references/agent-definitions' },
  { id: 'gating-policies', label: 'Gating Policies', endpoint: '/references/gating-policies' },
  { id: 'intakes', label: 'Intakes', endpoint: '/references/intakes' },
];

const PAGE_SIZE_OPTIONS = [10, 20, 50];

// ============================================================================
// Helpers
// ============================================================================

function truncateHash(hash: string | undefined): string {
  if (!hash) return '';
  if (hash.startsWith('sha256:')) {
    return hash.slice(0, 15) + '...' + hash.slice(-6);
  }
  return hash.length > 20 ? hash.slice(0, 10) + '...' + hash.slice(-6) : hash;
}

function getDetailPath(ref: TypedRef): string | null {
  switch (ref.kind) {
    case 'GovernedArtifact':
      return `/references/governed-artifacts/${ref.id}`;
    case 'EvidenceBundle':
      return ref.meta.content_hash
        ? `/references/bundles/${ref.meta.content_hash}`
        : null;
    case 'Intake':
      return `/intakes/${ref.id}`;
    case 'Candidate':
      return `/candidates/${ref.id}`;
    case 'Iteration':
      return `/iterations/${ref.id}`;
    case 'Template':
      return `/protocols/${ref.id}`;
    case 'OracleSuite':
      return `/oracles/suites/${ref.id}`;
    default:
      return null;
  }
}

function getKindTone(kind: string): 'neutral' | 'success' | 'warning' | 'danger' {
  switch (kind) {
    case 'GovernedArtifact':
      return 'success';
    case 'EvidenceBundle':
    case 'Candidate':
      return 'neutral';
    case 'Deviation':
    case 'Deferral':
    case 'Waiver':
      return 'warning';
    default:
      return 'neutral';
  }
}

// ============================================================================
// Component
// ============================================================================

export function References(): JSX.Element {
  const auth = useAuth();
  const navigate = useNavigate();

  // Category state
  const [categories, setCategories] = useState<Category[]>(CATEGORIES);
  const [selectedCategory, setSelectedCategory] = useState<string>('governed-artifacts');

  // List state
  const [refs, setRefs] = useState<TypedRef[]>([]);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);
  const [pagination, setPagination] = useState<Pagination>({
    page: 1,
    pageSize: 20,
    total: 0,
  });
  const [search, setSearch] = useState('');

  // Upload state (for documents category - preserved from original)
  const [uploading, setUploading] = useState(false);
  const [uploadError, setUploadError] = useState<string | null>(null);
  const [uploadSuccess, setUploadSuccess] = useState<string | null>(null);
  const [dragActive, setDragActive] = useState(false);
  const fileInputRef = useRef<HTMLInputElement>(null);

  // Fetch category counts on mount
  const fetchCategoryCounts = useCallback(async () => {
    if (!auth.user?.access_token) return;

    const updatedCategories = await Promise.all(
      CATEGORIES.map(async (cat) => {
        try {
          const res = await fetch(`${config.apiUrl}/api/v1${cat.endpoint}?page=1&page_size=1`, {
            headers: { Authorization: `Bearer ${auth.user!.access_token}` },
          });
          if (res.ok) {
            const data: ReferencesListResponse = await res.json();
            return { ...cat, count: data.total };
          }
        } catch {
          // Ignore errors for count fetching
        }
        return { ...cat, count: 0 };
      })
    );
    setCategories(updatedCategories);
  }, [auth.user?.access_token]);

  useEffect(() => {
    fetchCategoryCounts();
  }, [fetchCategoryCounts]);

  // Fetch refs for selected category
  const fetchRefs = useCallback(async () => {
    if (!auth.user?.access_token) return;

    const category = categories.find((c) => c.id === selectedCategory);
    if (!category) return;

    setLoading(true);
    setError(null);

    try {
      const params = new URLSearchParams({
        page: pagination.page.toString(),
        page_size: pagination.pageSize.toString(),
      });

      const res = await fetch(`${config.apiUrl}/api/v1${category.endpoint}?${params}`, {
        headers: { Authorization: `Bearer ${auth.user.access_token}` },
      });

      if (!res.ok) {
        throw new Error(`HTTP ${res.status}`);
      }

      const data: ReferencesListResponse = await res.json();
      setRefs(data.refs || []);
      setPagination((prev) => ({ ...prev, total: data.total }));
    } catch (err) {
      setError(err instanceof Error ? err.message : 'Failed to load references');
      setRefs([]);
    } finally {
      setLoading(false);
    }
  }, [auth.user?.access_token, selectedCategory, pagination.page, pagination.pageSize, categories]);

  useEffect(() => {
    fetchRefs();
  }, [fetchRefs]);

  // Category selection
  const handleCategoryChange = (categoryId: string) => {
    setSelectedCategory(categoryId);
    setPagination((prev) => ({ ...prev, page: 1 }));
    setSearch('');
  };

  // Pagination
  const handlePageChange = (page: number) => {
    setPagination((prev) => ({ ...prev, page }));
  };

  const handlePageSizeChange = (pageSize: number) => {
    setPagination((prev) => ({ ...prev, pageSize, page: 1 }));
  };

  // Row click navigation
  const handleRowClick = (ref: TypedRef) => {
    const path = getDetailPath(ref);
    if (path) {
      navigate(path);
    }
  };

  // Upload handlers (preserved from original for documents category)
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
      const res = await fetch(`${config.apiUrl}/api/v1/references/documents`, {
        method: 'POST',
        headers: {
          Authorization: `Bearer ${auth.user.access_token}`,
        },
        body: formData,
      });

      if (res.status === 501) {
        throw new Error('Document upload is not yet implemented');
      }
      if (!res.ok) {
        const errData = await res.json().catch(() => ({}));
        throw new Error(errData.error || `HTTP ${res.status}`);
      }

      const data = await res.json();
      setUploadSuccess(`Successfully uploaded ${data.uploaded?.length || files.length} file(s)`);
      fetchRefs();
      fetchCategoryCounts();
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

  // Client-side search filtering
  const filteredRefs = search
    ? refs.filter(
        (ref) =>
          (ref.label?.toLowerCase().includes(search.toLowerCase())) ||
          ref.id.toLowerCase().includes(search.toLowerCase()) ||
          ref.kind.toLowerCase().includes(search.toLowerCase())
      )
    : refs;

  const totalPages = Math.ceil(pagination.total / pagination.pageSize);
  const currentCategory = categories.find((c) => c.id === selectedCategory);

  return (
    <div className={styles.container}>
      <div className={styles.header}>
        <div className={styles.headerStart}>
          <h1 className={styles.title}>References</h1>
          <p className={styles.subtitle}>Browse all typed references in the system</p>
        </div>
      </div>

      <div className={styles.referencesLayout}>
        {/* Category Sidebar */}
        <div className={styles.categorySidebar}>
          <div className={styles.categorySidebarTitle}>Categories</div>
          <ul className={styles.categoryList}>
            {categories.map((cat) => (
              <li key={cat.id}>
                <button
                  className={`${styles.categoryItem} ${
                    selectedCategory === cat.id ? styles.categoryItemActive : ''
                  }`}
                  onClick={() => handleCategoryChange(cat.id)}
                >
                  <span>{cat.label}</span>
                  {cat.count !== undefined && (
                    <span className={styles.categoryCount}>{cat.count}</span>
                  )}
                </button>
              </li>
            ))}
          </ul>
        </div>

        {/* Content Area */}
        <div className={styles.contentArea}>
          {/* Upload Section (only for documents category - currently not functional) */}
          {selectedCategory === 'documents' && (
            <Card className={styles.cardSpacing}>
              <h3 style={{ margin: '0 0 1rem 0', fontSize: '0.875rem', color: 'var(--ink)' }}>
                Upload Reference Documents
              </h3>
              <div className={styles.note} style={{ marginBottom: 'var(--space3)' }}>
                Document upload is not yet implemented in the backend.
              </div>
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
                  opacity: 0.6,
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
          )}

          {/* Search and Filters */}
          <Card className={styles.cardSpacing}>
            <div
              style={{
                display: 'flex',
                gap: 'var(--space3)',
                flexWrap: 'wrap',
                alignItems: 'flex-end',
              }}
            >
              <div className={styles.formGroup} style={{ marginBottom: 0, flex: 1, minWidth: '200px' }}>
                <label className={styles.labelSmall}>Search</label>
                <input
                  type="text"
                  value={search}
                  onChange={(e) => setSearch(e.target.value)}
                  placeholder="Filter by label, ID, or kind..."
                  className={styles.input}
                />
              </div>
              <div className={styles.formGroup} style={{ marginBottom: 0 }}>
                <label className={styles.labelSmall}>Per Page</label>
                <select
                  value={pagination.pageSize}
                  onChange={(e) => handlePageSizeChange(Number(e.target.value))}
                  className={styles.select}
                >
                  {PAGE_SIZE_OPTIONS.map((size) => (
                    <option key={size} value={size}>
                      {size}
                    </option>
                  ))}
                </select>
              </div>
              <Button variant="ghost" onClick={fetchRefs}>
                Refresh
              </Button>
            </div>
          </Card>

          {/* Error State */}
          {error && <div className={styles.error}>{error}</div>}

          {/* Loading State */}
          {loading ? (
            <Card>
              <div className={styles.placeholder}>
                <p className={styles.placeholderText}>Loading {currentCategory?.label || 'references'}...</p>
              </div>
            </Card>
          ) : filteredRefs.length === 0 ? (
            <Card>
              <div className={styles.placeholder}>
                <p className={styles.placeholderText}>No {currentCategory?.label?.toLowerCase() || 'references'} found.</p>
                <p className={styles.placeholderHint}>
                  {search
                    ? 'Try adjusting your search.'
                    : `This category is currently empty.`}
                </p>
              </div>
            </Card>
          ) : (
            <>
              {/* References Table */}
              <Card>
                <table className={styles.table}>
                  <thead>
                    <tr>
                      <th className={styles.th}>Label / ID</th>
                      <th className={styles.th}>Kind</th>
                      <th className={styles.th}>Relation</th>
                      <th className={styles.th}>Content Hash</th>
                    </tr>
                  </thead>
                  <tbody>
                    {filteredRefs.map((ref, idx) => {
                      const path = getDetailPath(ref);
                      return (
                        <tr
                          key={`${ref.id}-${idx}`}
                          onClick={() => handleRowClick(ref)}
                          style={{ cursor: path ? 'pointer' : 'default' }}
                          className={path ? styles.tableRowHover : undefined}
                        >
                          <td className={styles.td}>
                            <div>
                              <div style={{ fontWeight: 500 }}>
                                {ref.label || ref.id}
                              </div>
                              {ref.label && ref.label !== ref.id && (
                                <div
                                  style={{
                                    fontSize: '0.75rem',
                                    color: 'var(--muted)',
                                    fontFamily: 'var(--mono)',
                                    marginTop: 'var(--space1)',
                                  }}
                                >
                                  {ref.id}
                                </div>
                              )}
                            </div>
                          </td>
                          <td className={styles.td}>
                            <Pill tone={getKindTone(ref.kind)}>{ref.kind}</Pill>
                          </td>
                          <td className={styles.td}>
                            <span style={{ color: 'var(--muted)' }}>{ref.rel}</span>
                          </td>
                          <td className={styles.tdMono}>
                            {ref.meta.content_hash
                              ? truncateHash(ref.meta.content_hash)
                              : '—'}
                          </td>
                        </tr>
                      );
                    })}
                  </tbody>
                </table>
              </Card>

              {/* Pagination */}
              {totalPages > 1 && (
                <div
                  style={{
                    display: 'flex',
                    justifyContent: 'flex-end',
                    alignItems: 'center',
                    marginTop: 'var(--space4)',
                    gap: 'var(--space2)',
                  }}
                >
                  <span style={{ fontSize: '0.875rem', color: 'var(--muted)' }}>
                    Page {pagination.page} of {totalPages} ({pagination.total} total)
                  </span>
                  <Button
                    variant="ghost"
                    onClick={() => handlePageChange(1)}
                    disabled={pagination.page === 1}
                  >
                    First
                  </Button>
                  <Button
                    variant="ghost"
                    onClick={() => handlePageChange(pagination.page - 1)}
                    disabled={pagination.page === 1}
                  >
                    Prev
                  </Button>
                  <Button
                    variant="ghost"
                    onClick={() => handlePageChange(pagination.page + 1)}
                    disabled={pagination.page >= totalPages}
                  >
                    Next
                  </Button>
                  <Button
                    variant="ghost"
                    onClick={() => handlePageChange(totalPages)}
                    disabled={pagination.page >= totalPages}
                  >
                    Last
                  </Button>
                </div>
              )}
            </>
          )}
        </div>
      </div>
    </div>
  );
}

export default References;
