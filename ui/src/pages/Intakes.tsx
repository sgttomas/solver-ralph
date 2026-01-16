/**
 * Intakes List Page
 *
 * Lists all intakes with filtering by status and kind, search, and pagination.
 * Per SR-WORK-SURFACE, intakes define the structured problem statement
 * for a work unit including objective, deliverables, constraints, and inputs.
 */

import { useState, useEffect, useCallback } from 'react';
import { Link, useNavigate } from 'react-router-dom';
import { useAuth } from '../auth/AuthProvider';
import config from '../config';
import { Card, Button, Pill } from '../ui';
import styles from '../styles/pages.module.css';

interface IntakeListItem {
  intake_id: string;
  work_unit_id: string;
  title: string;
  kind: string;
  status: 'draft' | 'active' | 'archived';
  objective: string;
  created_at: string;
  content_hash: string | null;
}

interface IntakesFilter {
  status: string;
  kind: string;
  search: string;
}

interface IntakesPagination {
  page: number;
  pageSize: number;
  total: number;
}

const WORK_KINDS = [
  { value: 'ALL', label: 'All Kinds' },
  { value: 'research_memo', label: 'Research Memo' },
  { value: 'decision_record', label: 'Decision Record' },
  { value: 'ontology_build', label: 'Ontology Build' },
  { value: 'analysis_report', label: 'Analysis Report' },
  { value: 'design_document', label: 'Design Document' },
  { value: 'review_response', label: 'Review Response' },
  { value: 'technical_spec', label: 'Technical Spec' },
  { value: 'implementation_plan', label: 'Implementation Plan' },
  { value: 'intake_processing', label: 'Intake Processing' },
];

const STATUS_OPTIONS = [
  { value: 'ALL', label: 'All Statuses' },
  { value: 'draft', label: 'Draft' },
  { value: 'active', label: 'Active' },
  { value: 'archived', label: 'Archived' },
];

const PAGE_SIZE_OPTIONS = [10, 25, 50];

export function Intakes(): JSX.Element {
  const auth = useAuth();
  const navigate = useNavigate();

  const [intakes, setIntakes] = useState<IntakeListItem[]>([]);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);
  const [filter, setFilter] = useState<IntakesFilter>({
    status: 'ALL',
    kind: 'ALL',
    search: '',
  });
  const [pagination, setPagination] = useState<IntakesPagination>({
    page: 1,
    pageSize: 20,
    total: 0,
  });

  const fetchIntakes = useCallback(async () => {
    if (!auth.user?.access_token) return;

    setLoading(true);
    setError(null);

    try {
      const params = new URLSearchParams();
      if (filter.status !== 'ALL') {
        params.set('status', filter.status);
      }
      if (filter.kind !== 'ALL') {
        params.set('kind', filter.kind);
      }
      params.set('page', pagination.page.toString());
      params.set('page_size', pagination.pageSize.toString());

      const res = await fetch(`${config.apiUrl}/api/v1/intakes?${params}`, {
        headers: { Authorization: `Bearer ${auth.user.access_token}` },
      });

      if (!res.ok) {
        throw new Error(`HTTP ${res.status}`);
      }

      const data = await res.json();
      setIntakes(data.intakes || []);
      setPagination((prev) => ({ ...prev, total: data.total || 0 }));
    } catch (err) {
      setError(err instanceof Error ? err.message : 'Failed to load intakes');
    } finally {
      setLoading(false);
    }
  }, [auth.user?.access_token, filter.status, filter.kind, pagination.page, pagination.pageSize]);

  useEffect(() => {
    fetchIntakes();
  }, [fetchIntakes]);

  const handleStatusChange = (status: string) => {
    setFilter((prev) => ({ ...prev, status }));
    setPagination((prev) => ({ ...prev, page: 1 }));
  };

  const handleKindChange = (kind: string) => {
    setFilter((prev) => ({ ...prev, kind }));
    setPagination((prev) => ({ ...prev, page: 1 }));
  };

  const handleSearchChange = (search: string) => {
    setFilter((prev) => ({ ...prev, search }));
  };

  const handlePageChange = (page: number) => {
    setPagination((prev) => ({ ...prev, page }));
  };

  const handlePageSizeChange = (pageSize: number) => {
    setPagination((prev) => ({ ...prev, pageSize, page: 1 }));
  };

  const getStatusTone = (status: string) => {
    switch (status) {
      case 'active':
        return 'success';
      case 'draft':
        return 'warning';
      case 'archived':
        return 'neutral';
      default:
        return 'neutral';
    }
  };

  // Client-side search filtering
  const filteredIntakes = filter.search
    ? intakes.filter(
        (intake) =>
          intake.title.toLowerCase().includes(filter.search.toLowerCase()) ||
          intake.objective.toLowerCase().includes(filter.search.toLowerCase()) ||
          intake.intake_id.toLowerCase().includes(filter.search.toLowerCase())
      )
    : intakes;

  const totalPages = Math.ceil(pagination.total / pagination.pageSize);

  return (
    <div className={styles.container}>
      <div className={styles.header}>
        <div className={styles.headerStart}>
          <h1 className={styles.title}>Intakes</h1>
          <p className={styles.subtitle}>Work unit specifications and problem statements</p>
        </div>
        <Link to="/intakes/new">
          <Button variant="primary">New Intake</Button>
        </Link>
      </div>

      {/* Filters */}
      <Card className={styles.cardSpacing}>
        <div
          style={{
            display: 'flex',
            gap: 'var(--space3)',
            flexWrap: 'wrap',
            alignItems: 'flex-end',
          }}
        >
          <div className={styles.formGroup} style={{ marginBottom: 0 }}>
            <label className={styles.labelSmall}>Status</label>
            <select
              value={filter.status}
              onChange={(e) => handleStatusChange(e.target.value)}
              className={styles.select}
            >
              {STATUS_OPTIONS.map((opt) => (
                <option key={opt.value} value={opt.value}>
                  {opt.label}
                </option>
              ))}
            </select>
          </div>
          <div className={styles.formGroup} style={{ marginBottom: 0 }}>
            <label className={styles.labelSmall}>Kind</label>
            <select
              value={filter.kind}
              onChange={(e) => handleKindChange(e.target.value)}
              className={styles.select}
            >
              {WORK_KINDS.map((opt) => (
                <option key={opt.value} value={opt.value}>
                  {opt.label}
                </option>
              ))}
            </select>
          </div>
          <div className={styles.formGroup} style={{ marginBottom: 0, flex: 1, minWidth: '200px' }}>
            <label className={styles.labelSmall}>Search</label>
            <input
              type="text"
              value={filter.search}
              onChange={(e) => handleSearchChange(e.target.value)}
              placeholder="Search by title, objective, or ID..."
              className={styles.input}
            />
          </div>
          <Button variant="ghost" onClick={fetchIntakes}>
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
            <p className={styles.placeholderText}>Loading intakes...</p>
          </div>
        </Card>
      ) : filteredIntakes.length === 0 ? (
        <Card>
          <div className={styles.placeholder}>
            <p className={styles.placeholderText}>No intakes found.</p>
            <p className={styles.placeholderHint}>
              {filter.status !== 'ALL' || filter.kind !== 'ALL' || filter.search
                ? 'Try adjusting your filters.'
                : 'Create your first intake to get started.'}
            </p>
          </div>
        </Card>
      ) : (
        <>
          {/* Intakes Table */}
          <Card>
            <table className={styles.table}>
              <thead>
                <tr>
                  <th className={styles.th}>Title</th>
                  <th className={styles.th}>Kind</th>
                  <th className={styles.th}>Status</th>
                  <th className={styles.th}>Work Unit</th>
                  <th className={styles.th}>Created</th>
                </tr>
              </thead>
              <tbody>
                {filteredIntakes.map((intake) => (
                  <tr
                    key={intake.intake_id}
                    onClick={() => navigate(`/intakes/${intake.intake_id}`)}
                    style={{ cursor: 'pointer' }}
                    className={styles.tableRowHover}
                  >
                    <td className={styles.td}>
                      <div>
                        <div style={{ fontWeight: 500 }}>{intake.title}</div>
                        <div
                          style={{
                            fontSize: '0.75rem',
                            color: 'var(--muted)',
                            marginTop: 'var(--space1)',
                          }}
                        >
                          {intake.objective.length > 80
                            ? intake.objective.slice(0, 80) + '...'
                            : intake.objective}
                        </div>
                      </div>
                    </td>
                    <td className={styles.td}>
                      <Pill tone="neutral">{intake.kind.replace(/_/g, ' ')}</Pill>
                    </td>
                    <td className={styles.td}>
                      <Pill tone={getStatusTone(intake.status)}>{intake.status}</Pill>
                    </td>
                    <td className={styles.tdMono} style={{ fontSize: '0.75rem' }}>
                      {intake.work_unit_id}
                    </td>
                    <td className={styles.td} style={{ fontSize: '0.75rem' }}>
                      {new Date(intake.created_at).toLocaleDateString()}
                    </td>
                  </tr>
                ))}
              </tbody>
            </table>
          </Card>

          {/* Pagination */}
          {totalPages > 1 && (
            <div
              style={{
                display: 'flex',
                justifyContent: 'space-between',
                alignItems: 'center',
                marginTop: 'var(--space4)',
              }}
            >
              <div style={{ display: 'flex', alignItems: 'center', gap: 'var(--space2)' }}>
                <span style={{ fontSize: '0.875rem', color: 'var(--muted)' }}>Rows per page:</span>
                <select
                  value={pagination.pageSize}
                  onChange={(e) => handlePageSizeChange(Number(e.target.value))}
                  className={styles.select}
                  style={{ width: 'auto' }}
                >
                  {PAGE_SIZE_OPTIONS.map((size) => (
                    <option key={size} value={size}>
                      {size}
                    </option>
                  ))}
                </select>
              </div>

              <div style={{ display: 'flex', alignItems: 'center', gap: 'var(--space2)' }}>
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
            </div>
          )}
        </>
      )}
    </div>
  );
}

export default Intakes;
