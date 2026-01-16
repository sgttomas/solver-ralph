/**
 * Loops Page
 *
 * Displays active work units (loops) with filtering, sorting, pagination,
 * and quick actions for state transitions.
 * Per SR-SPEC, loops are bounded work units containing iterations.
 */

import { useState } from 'react';
import { Link, useNavigate } from 'react-router-dom';
import { Card, Pill, Button, getStatusTone } from '../ui';
import { useLoops, type LoopState, type SortField } from '../hooks/useLoops';
import { LoopCreateModal, SingleActionButton } from '../components';
import styles from '../styles/pages.module.css';
import loopStyles from '../styles/loops.module.css';

const STATE_OPTIONS: { value: LoopState | 'ALL'; label: string }[] = [
  { value: 'ALL', label: 'All States' },
  { value: 'CREATED', label: 'Created' },
  { value: 'ACTIVE', label: 'Active' },
  { value: 'PAUSED', label: 'Paused' },
  { value: 'CLOSED', label: 'Closed' },
];

const PAGE_SIZES = [10, 25, 50];

export function Loops(): JSX.Element {
  const navigate = useNavigate();
  const {
    loops,
    loading,
    error,
    filter,
    setFilter,
    pagination,
    setPage,
    setPageSize,
    sort,
    setSort,
    refresh,
    transitionState,
    transitioning,
  } = useLoops();

  const [showCreateModal, setShowCreateModal] = useState(false);

  const handleSearchChange = (e: React.ChangeEvent<HTMLInputElement>) => {
    setFilter({ ...filter, search: e.target.value });
  };

  const handleStateChange = (e: React.ChangeEvent<HTMLSelectElement>) => {
    setFilter({ ...filter, state: e.target.value as LoopState | 'ALL' });
  };

  const handleSort = (field: SortField) => {
    if (sort.field === field) {
      setSort({ field, direction: sort.direction === 'asc' ? 'desc' : 'asc' });
    } else {
      setSort({ field, direction: 'desc' });
    }
  };

  const getSortIndicator = (field: SortField) => {
    if (sort.field !== field) return null;
    return sort.direction === 'asc' ? ' \u25B2' : ' \u25BC';
  };

  const handleLoopCreated = (loopId: string) => {
    setShowCreateModal(false);
    refresh();
    navigate(`/loops/${loopId}`);
  };

  const handleTransition = async (loopId: string, action: 'activate' | 'pause' | 'resume' | 'close') => {
    try {
      await transitionState(loopId, action);
    } catch {
      // Error is already displayed via the hook
    }
  };

  const totalPages = Math.ceil(pagination.total / pagination.pageSize);

  return (
    <div className={styles.container}>
      {/* Header */}
      <div className={styles.header}>
        <h1 className={styles.title}>Loops</h1>
        <Button variant="primary" onClick={() => setShowCreateModal(true)}>
          Create Loop
        </Button>
      </div>

      {/* Filters */}
      <Card className={styles.cardSpacing}>
        <div className={loopStyles.filterBar}>
          <div className={loopStyles.filterGroup}>
            <input
              type="text"
              className={loopStyles.searchInput}
              placeholder="Search by goal, work unit, or ID..."
              value={filter.search}
              onChange={handleSearchChange}
            />
          </div>
          <div className={loopStyles.filterGroup}>
            <select
              className={loopStyles.filterSelect}
              value={filter.state}
              onChange={handleStateChange}
            >
              {STATE_OPTIONS.map(opt => (
                <option key={opt.value} value={opt.value}>
                  {opt.label}
                </option>
              ))}
            </select>
          </div>
          <div className={loopStyles.filterGroup}>
            <Button variant="ghost" onClick={refresh}>
              Refresh
            </Button>
          </div>
        </div>
      </Card>

      {/* Error display */}
      {error && (
        <div className={styles.error}>{error}</div>
      )}

      {/* Main content */}
      <Card>
        {loading ? (
          <div className={styles.placeholder}>
            <p className={styles.placeholderText}>Loading loops...</p>
          </div>
        ) : loops.length === 0 ? (
          <div className={styles.placeholder}>
            <p className={styles.placeholderText}>No loops found.</p>
            <p className={styles.placeholderHint}>
              {filter.search || filter.state !== 'ALL'
                ? 'Try adjusting your filters.'
                : 'Create a loop to get started.'}
            </p>
          </div>
        ) : (
          <>
            <table className={styles.table}>
              <thead>
                <tr>
                  <th className={styles.th}>Goal</th>
                  <th className={styles.th}>Work Unit</th>
                  <th className={styles.th}>Work Surface</th>
                  <th className={styles.th}>State</th>
                  <th
                    className={`${styles.th} ${loopStyles.sortable}`}
                    onClick={() => handleSort('progress')}
                  >
                    Progress{getSortIndicator('progress')}
                  </th>
                  <th className={styles.th}>Stage</th>
                  <th
                    className={`${styles.th} ${loopStyles.sortable}`}
                    onClick={() => handleSort('created_at')}
                  >
                    Created{getSortIndicator('created_at')}
                  </th>
                  <th className={styles.th}>Actions</th>
                </tr>
              </thead>
              <tbody>
                {loops.map(loop => {
                  const progress = loop.max_iterations > 0
                    ? (loop.iteration_count / loop.max_iterations) * 100
                    : 0;
                  const isTransitioning = transitioning === loop.id;

                  return (
                    <tr key={loop.id}>
                      <td className={styles.td}>
                        <Link to={`/loops/${loop.id}`} className={styles.link}>
                          {loop.goal.length > 60
                            ? loop.goal.substring(0, 60) + '...'
                            : loop.goal}
                        </Link>
                        <div className={loopStyles.loopId}>{loop.id}</div>
                      </td>
                      <td className={styles.td}>
                        {loop.work_unit || <span className={loopStyles.muted}>-</span>}
                      </td>
                      <td className={styles.td}>
                        {loop.work_surface_id ? (
                          <Link to={`/work-surfaces/${loop.work_surface_id}`} className={styles.link}>
                            <code className={loopStyles.stageCode}>{loop.work_surface_id}</code>
                          </Link>
                        ) : (
                          <span className={loopStyles.muted}>—</span>
                        )}
                      </td>
                      <td className={styles.td}>
                        <Pill tone={getStatusTone(loop.state)}>{loop.state}</Pill>
                      </td>
                      <td className={styles.td}>
                        <div className={loopStyles.progressCell}>
                          <div className={loopStyles.progressBar}>
                            <div
                              className={loopStyles.progressFill}
                              style={{ width: `${progress}%` }}
                            />
                          </div>
                          <span className={loopStyles.progressText}>
                            {loop.iteration_count}/{loop.max_iterations}
                          </span>
                        </div>
                      </td>
                      <td className={styles.td}>
                        {loop.current_stage_id ? (
                          <code className={loopStyles.stageCode}>
                            {loop.current_stage_id}
                          </code>
                        ) : (
                          <span className={loopStyles.muted}>-</span>
                        )}
                      </td>
                      <td className={styles.td}>
                        {new Date(loop.created_at).toLocaleDateString()}
                      </td>
                      <td className={styles.td}>
                        <div className={loopStyles.actionButtons}>
                          {loop.state === 'CREATED' && (
                            <SingleActionButton
                              action="activate"
                              onTransition={(action) => handleTransition(loop.id, action)}
                              disabled={isTransitioning}
                              size="small"
                            />
                          )}
                          {loop.state === 'ACTIVE' && (
                            <SingleActionButton
                              action="pause"
                              onTransition={(action) => handleTransition(loop.id, action)}
                              disabled={isTransitioning}
                              size="small"
                            />
                          )}
                          {loop.state === 'PAUSED' && (
                            <SingleActionButton
                              action="resume"
                              onTransition={(action) => handleTransition(loop.id, action)}
                              disabled={isTransitioning}
                              size="small"
                            />
                          )}
                        </div>
                      </td>
                    </tr>
                  );
                })}
              </tbody>
            </table>

            {/* Pagination */}
            <div className={loopStyles.pagination}>
              <div className={loopStyles.pageInfo}>
                Showing {((pagination.page - 1) * pagination.pageSize) + 1} -{' '}
                {Math.min(pagination.page * pagination.pageSize, pagination.total)} of{' '}
                {pagination.total} loops
              </div>
              <div className={loopStyles.pageControls}>
                <select
                  className={loopStyles.pageSizeSelect}
                  value={pagination.pageSize}
                  onChange={e => setPageSize(parseInt(e.target.value))}
                >
                  {PAGE_SIZES.map(size => (
                    <option key={size} value={size}>
                      {size} per page
                    </option>
                  ))}
                </select>
                <div className={loopStyles.pageButtons}>
                  <button
                    className={loopStyles.pageButton}
                    onClick={() => setPage(1)}
                    disabled={pagination.page === 1}
                  >
                    &laquo;
                  </button>
                  <button
                    className={loopStyles.pageButton}
                    onClick={() => setPage(pagination.page - 1)}
                    disabled={pagination.page === 1}
                  >
                    &lsaquo;
                  </button>
                  <span className={loopStyles.pageNumber}>
                    Page {pagination.page} of {totalPages || 1}
                  </span>
                  <button
                    className={loopStyles.pageButton}
                    onClick={() => setPage(pagination.page + 1)}
                    disabled={pagination.page >= totalPages}
                  >
                    &rsaquo;
                  </button>
                  <button
                    className={loopStyles.pageButton}
                    onClick={() => setPage(totalPages)}
                    disabled={pagination.page >= totalPages}
                  >
                    &raquo;
                  </button>
                </div>
              </div>
            </div>
          </>
        )}
      </Card>

      {/* Create Modal */}
      <LoopCreateModal
        isOpen={showCreateModal}
        onClose={() => setShowCreateModal(false)}
        onCreated={handleLoopCreated}
      />
    </div>
  );
}

export default Loops;
