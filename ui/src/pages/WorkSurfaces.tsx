/**
 * Work Surfaces List Page
 *
 * Lists all Work Surfaces with filtering by status, search, and pagination.
 * Per SR-WORK-SURFACE, Work Surfaces bind an Intake + Procedure Template
 * to create the iteration context for Semantic Ralph Loops.
 */

import { useState, useEffect, useCallback } from 'react';
import { Link, useNavigate } from 'react-router-dom';
import { useAuth } from '../auth/AuthProvider';
import config from '../config';
import { Card, Button, Pill } from '../ui';
import styles from '../styles/pages.module.css';

interface WorkSurfaceSummary {
  work_surface_id: string;
  work_unit_id: string;
  intake_id: string;
  intake_title: string | null;
  template_id: string;
  template_name: string | null;
  current_stage_id: string;
  status: 'active' | 'completed' | 'archived';
  bound_at: string;
}

interface WorkSurfacesFilter {
  status: string;
  search: string;
}

interface WorkSurfacesPagination {
  page: number;
  pageSize: number;
  total: number;
}

const STATUS_OPTIONS = [
  { value: 'ALL', label: 'All Statuses' },
  { value: 'active', label: 'Active' },
  { value: 'completed', label: 'Completed' },
  { value: 'archived', label: 'Archived' },
];

const PAGE_SIZE_OPTIONS = [10, 25, 50];

export function WorkSurfaces(): JSX.Element {
  const auth = useAuth();
  const navigate = useNavigate();

  const [workSurfaces, setWorkSurfaces] = useState<WorkSurfaceSummary[]>([]);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);
  const [filter, setFilter] = useState<WorkSurfacesFilter>({
    status: 'ALL',
    search: '',
  });
  const [pagination, setPagination] = useState<WorkSurfacesPagination>({
    page: 1,
    pageSize: 20,
    total: 0,
  });

  const fetchWorkSurfaces = useCallback(async () => {
    if (!auth.user?.access_token) return;

    setLoading(true);
    setError(null);

    try {
      const params = new URLSearchParams();
      if (filter.status !== 'ALL') {
        params.set('status', filter.status);
      }
      params.set('page', pagination.page.toString());
      params.set('page_size', pagination.pageSize.toString());

      const res = await fetch(`${config.apiUrl}/api/v1/work-surfaces?${params}`, {
        headers: { Authorization: `Bearer ${auth.user.access_token}` },
      });

      if (!res.ok) {
        throw new Error(`HTTP ${res.status}`);
      }

      const data = await res.json();
      setWorkSurfaces(data.work_surfaces || []);
      setPagination((prev) => ({ ...prev, total: data.total || 0 }));
    } catch (err) {
      setError(err instanceof Error ? err.message : 'Failed to load work surfaces');
    } finally {
      setLoading(false);
    }
  }, [auth.user?.access_token, filter.status, pagination.page, pagination.pageSize]);

  useEffect(() => {
    fetchWorkSurfaces();
  }, [fetchWorkSurfaces]);

  const handleStatusChange = (status: string) => {
    setFilter((prev) => ({ ...prev, status }));
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
      case 'completed':
        return 'neutral';
      case 'archived':
        return 'neutral';
      default:
        return 'neutral';
    }
  };

  // Client-side search filtering
  const filteredWorkSurfaces = filter.search
    ? workSurfaces.filter(
        (ws) =>
          ws.work_unit_id.toLowerCase().includes(filter.search.toLowerCase()) ||
          (ws.intake_title && ws.intake_title.toLowerCase().includes(filter.search.toLowerCase())) ||
          ws.work_surface_id.toLowerCase().includes(filter.search.toLowerCase())
      )
    : workSurfaces;

  const totalPages = Math.ceil(pagination.total / pagination.pageSize);

  // Format stage ID for display (remove "stage:" prefix)
  const formatStageId = (stageId: string) => {
    return stageId.replace(/^stage:/, '').replace(/_/g, ' ');
  };

  return (
    <div className={styles.container}>
      <div className={styles.header}>
        <div className={styles.headerStart}>
          <h1 className={styles.title}>Work Surfaces</h1>
          <p className={styles.subtitle}>Intake + Procedure Template bindings for work units</p>
        </div>
        <Link to="/work-surfaces/new">
          <Button variant="primary">New Work Surface</Button>
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
          <div className={styles.formGroup} style={{ marginBottom: 0, flex: 1, minWidth: '200px' }}>
            <label className={styles.labelSmall}>Search</label>
            <input
              type="text"
              value={filter.search}
              onChange={(e) => handleSearchChange(e.target.value)}
              placeholder="Search by work unit, intake title, or ID..."
              className={styles.input}
            />
          </div>
          <Button variant="ghost" onClick={fetchWorkSurfaces}>
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
            <p className={styles.placeholderText}>Loading work surfaces...</p>
          </div>
        </Card>
      ) : filteredWorkSurfaces.length === 0 ? (
        <Card>
          <div className={styles.placeholder}>
            <p className={styles.placeholderText}>No work surfaces found.</p>
            <p className={styles.placeholderHint}>
              {filter.status !== 'ALL' || filter.search
                ? 'Try adjusting your filters.'
                : 'Create your first work surface to get started.'}
            </p>
          </div>
        </Card>
      ) : (
        <>
          {/* Work Surfaces Table */}
          <Card>
            <table className={styles.table}>
              <thead>
                <tr>
                  <th className={styles.th}>Intake</th>
                  <th className={styles.th}>Template</th>
                  <th className={styles.th}>Current Stage</th>
                  <th className={styles.th}>Status</th>
                  <th className={styles.th}>Work Unit</th>
                  <th className={styles.th}>Bound</th>
                </tr>
              </thead>
              <tbody>
                {filteredWorkSurfaces.map((ws) => (
                  <tr
                    key={ws.work_surface_id}
                    onClick={() => navigate(`/work-surfaces/${ws.work_surface_id}`)}
                    style={{ cursor: 'pointer' }}
                    className={styles.tableRowHover}
                  >
                    <td className={styles.td}>
                      <div>
                        <div style={{ fontWeight: 500 }}>
                          {ws.intake_title || ws.intake_id}
                        </div>
                        <div
                          style={{
                            fontSize: '0.75rem',
                            color: 'var(--muted)',
                            marginTop: 'var(--space1)',
                            fontFamily: 'var(--mono)',
                          }}
                        >
                          {ws.intake_id}
                        </div>
                      </div>
                    </td>
                    <td className={styles.td}>
                      <div style={{ fontSize: '0.875rem' }}>
                        {ws.template_name || ws.template_id}
                      </div>
                    </td>
                    <td className={styles.td}>
                      <Pill tone="neutral">{formatStageId(ws.current_stage_id)}</Pill>
                    </td>
                    <td className={styles.td}>
                      <Pill tone={getStatusTone(ws.status)}>{ws.status}</Pill>
                    </td>
                    <td className={styles.tdMono} style={{ fontSize: '0.75rem' }}>
                      {ws.work_unit_id}
                    </td>
                    <td className={styles.td} style={{ fontSize: '0.75rem' }}>
                      {new Date(ws.bound_at).toLocaleDateString()}
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

export default WorkSurfaces;
