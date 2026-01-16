/**
 * useLoops Hook
 *
 * Data fetching hook for loops with filtering, sorting, and pagination.
 * Handles API calls and state management for the Loops list page.
 */

import { useState, useEffect, useCallback } from 'react';
import { useAuth } from '../auth/AuthProvider';
import config from '../config';

export type LoopState = 'CREATED' | 'ACTIVE' | 'PAUSED' | 'CLOSED';

export interface LoopBudgets {
  max_iterations: number;
  max_oracle_runs: number;
  max_wallclock_hours: number;
}

export interface TypedRef {
  type_key: string;
  id: string;
  rel: string;
  meta?: Record<string, unknown>;
}

export interface WorkSurface {
  intake_id: string | null;
  intake_title: string | null;
  procedure_template_id: string | null;
  procedure_template_name: string | null;
  current_stage_id: string | null;
  oracle_suite_id: string | null;
  oracle_suite_hash: string | null;
}

export interface StageStatus {
  status: 'pending' | 'in_progress' | 'completed' | 'failed';
  evidence_bundle_ref: string | null;
}

export interface Loop {
  id: string;
  loop_id?: string; // API may return this
  goal: string;
  work_unit: string | null;
  state: LoopState;
  budgets: LoopBudgets;
  directive_ref: TypedRef;
  iteration_count: number;
  oracle_run_count: number;
  created_at: string;
  activated_at: string | null;
  closed_at: string | null;
  last_event_id: string | null;
  work_surface: WorkSurface | null;
  current_stage_id: string | null;
  stage_status: Record<string, StageStatus> | null;
  stop_triggers_fired: string[];
  active_exceptions: string[];
}

export interface LoopListItem {
  id: string;
  goal: string;
  work_unit: string | null;
  work_surface_id: string | null; // SR-PLAN-V5 Phase 5b: bound Work Surface
  state: LoopState;
  iteration_count: number;
  max_iterations: number;
  current_stage_id: string | null;
  created_at: string;
  last_activity_at: string | null;
}

export interface LoopsFilter {
  search: string;
  state: LoopState | 'ALL';
  workUnit: string;
}

export interface LoopsPagination {
  page: number;
  pageSize: number;
  total: number;
}

export type SortField = 'created_at' | 'last_activity_at' | 'state' | 'progress';
export type SortDirection = 'asc' | 'desc';

export interface LoopsSort {
  field: SortField;
  direction: SortDirection;
}

interface UseLoopsReturn {
  loops: LoopListItem[];
  loading: boolean;
  error: string | null;
  filter: LoopsFilter;
  setFilter: (filter: LoopsFilter) => void;
  pagination: LoopsPagination;
  setPage: (page: number) => void;
  setPageSize: (size: number) => void;
  sort: LoopsSort;
  setSort: (sort: LoopsSort) => void;
  refresh: () => void;
  transitionState: (loopId: string, action: 'activate' | 'pause' | 'resume' | 'close') => Promise<void>;
  transitioning: string | null;
}

export function useLoops(): UseLoopsReturn {
  const auth = useAuth();
  const [loops, setLoops] = useState<LoopListItem[]>([]);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);
  const [transitioning, setTransitioning] = useState<string | null>(null);

  const [filter, setFilter] = useState<LoopsFilter>({
    search: '',
    state: 'ALL',
    workUnit: '',
  });

  const [pagination, setPagination] = useState<LoopsPagination>({
    page: 1,
    pageSize: 25,
    total: 0,
  });

  const [sort, setSort] = useState<LoopsSort>({
    field: 'created_at',
    direction: 'desc',
  });

  const fetchLoops = useCallback(async () => {
    if (!auth.user?.access_token) return;

    setLoading(true);
    setError(null);

    try {
      // Build query params
      const params = new URLSearchParams();
      if (filter.state !== 'ALL') {
        params.set('state', filter.state);
      }
      if (filter.workUnit) {
        params.set('work_unit', filter.workUnit);
      }
      params.set('limit', pagination.pageSize.toString());
      params.set('offset', ((pagination.page - 1) * pagination.pageSize).toString());
      params.set('sort', sort.field);
      params.set('order', sort.direction);

      const url = `${config.apiUrl}/api/v1/loops${params.toString() ? '?' + params.toString() : ''}`;

      const res = await fetch(url, {
        headers: {
          Authorization: `Bearer ${auth.user.access_token}`,
        },
      });

      if (res.status === 404) {
        // No loops endpoint yet - return empty
        setLoops([]);
        setPagination(prev => ({ ...prev, total: 0 }));
        setLoading(false);
        return;
      }

      if (!res.ok) {
        throw new Error(`HTTP ${res.status}`);
      }

      const data = await res.json();
      const rawLoops = data.loops || [];

      // Transform API response to LoopListItem format
      const items: LoopListItem[] = rawLoops.map((loop: Record<string, unknown>) => ({
        id: (loop.loop_id || loop.id) as string,
        goal: (loop.goal || loop.name || '') as string,
        work_unit: (loop.work_unit || null) as string | null,
        work_surface_id: (loop.work_surface_id || null) as string | null, // SR-PLAN-V5 Phase 5b
        state: ((loop.state || loop.status || 'CREATED') as string).toUpperCase() as LoopState,
        iteration_count: (loop.iteration_count || 0) as number,
        max_iterations: ((loop.budgets as LoopBudgets)?.max_iterations || 5) as number,
        current_stage_id: (loop.current_stage_id || null) as string | null,
        created_at: (loop.created_at || new Date().toISOString()) as string,
        last_activity_at: (loop.last_activity_at || loop.activated_at || null) as string | null,
      }));

      // Client-side search filter (if API doesn't support it)
      let filtered = items;
      if (filter.search) {
        const searchLower = filter.search.toLowerCase();
        filtered = items.filter(
          loop =>
            loop.id.toLowerCase().includes(searchLower) ||
            loop.goal.toLowerCase().includes(searchLower) ||
            (loop.work_unit?.toLowerCase().includes(searchLower) ?? false)
        );
      }

      setLoops(filtered);
      setPagination(prev => ({ ...prev, total: data.total || filtered.length }));
    } catch (err) {
      setError(err instanceof Error ? err.message : 'Failed to fetch loops');
    } finally {
      setLoading(false);
    }
  }, [auth.user?.access_token, filter, pagination.page, pagination.pageSize, sort]);

  useEffect(() => {
    fetchLoops();
  }, [fetchLoops]);

  const setPage = useCallback((page: number) => {
    setPagination(prev => ({ ...prev, page }));
  }, []);

  const setPageSize = useCallback((pageSize: number) => {
    setPagination(prev => ({ ...prev, pageSize, page: 1 }));
  }, []);

  const transitionState = useCallback(
    async (loopId: string, action: 'activate' | 'pause' | 'resume' | 'close') => {
      if (!auth.user?.access_token) {
        throw new Error('Not authenticated');
      }

      setTransitioning(loopId);
      setError(null);

      try {
        const res = await fetch(`${config.apiUrl}/api/v1/loops/${loopId}/${action}`, {
          method: 'POST',
          headers: {
            Authorization: `Bearer ${auth.user.access_token}`,
            'Content-Type': 'application/json',
          },
        });

        if (!res.ok) {
          const errorData = await res.json().catch(() => ({}));
          throw new Error(errorData.message || `Failed to ${action} loop: HTTP ${res.status}`);
        }

        // Refresh the list after successful transition
        await fetchLoops();
      } catch (err) {
        setError(err instanceof Error ? err.message : `Failed to ${action} loop`);
        throw err;
      } finally {
        setTransitioning(null);
      }
    },
    [auth.user?.access_token, fetchLoops]
  );

  return {
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
    refresh: fetchLoops,
    transitionState,
    transitioning,
  };
}

/**
 * Hook for fetching a single loop with full details
 */
export function useLoop(loopId: string | undefined) {
  const auth = useAuth();
  const [loop, setLoop] = useState<Loop | null>(null);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);

  const fetchLoop = useCallback(async () => {
    if (!auth.user?.access_token || !loopId) return;

    setLoading(true);
    setError(null);

    try {
      const res = await fetch(`${config.apiUrl}/api/v1/loops/${loopId}`, {
        headers: {
          Authorization: `Bearer ${auth.user.access_token}`,
        },
      });

      if (res.status === 404) {
        throw new Error('Loop not found');
      }

      if (!res.ok) {
        throw new Error(`HTTP ${res.status}`);
      }

      const data = await res.json();

      // Normalize the response
      const normalized: Loop = {
        id: data.loop_id || data.id,
        loop_id: data.loop_id,
        goal: data.goal || '',
        work_unit: data.work_unit || null,
        state: (data.state || 'CREATED').toUpperCase() as LoopState,
        budgets: data.budgets || {
          max_iterations: 5,
          max_oracle_runs: 25,
          max_wallclock_hours: 16,
        },
        directive_ref: data.directive_ref || { type_key: '', id: '', rel: '' },
        iteration_count: data.iteration_count || 0,
        oracle_run_count: data.oracle_run_count || 0,
        created_at: data.created_at || new Date().toISOString(),
        activated_at: data.activated_at || null,
        closed_at: data.closed_at || null,
        last_event_id: data.last_event_id || null,
        work_surface: data.work_surface || null,
        current_stage_id: data.current_stage_id || null,
        stage_status: data.stage_status || null,
        stop_triggers_fired: data.stop_triggers_fired || [],
        active_exceptions: data.active_exceptions || [],
      };

      setLoop(normalized);
    } catch (err) {
      setError(err instanceof Error ? err.message : 'Failed to fetch loop');
    } finally {
      setLoading(false);
    }
  }, [auth.user?.access_token, loopId]);

  useEffect(() => {
    fetchLoop();
  }, [fetchLoop]);

  const transitionState = useCallback(
    async (action: 'activate' | 'pause' | 'resume' | 'close') => {
      if (!auth.user?.access_token || !loopId) {
        throw new Error('Not authenticated or no loop ID');
      }

      try {
        const res = await fetch(`${config.apiUrl}/api/v1/loops/${loopId}/${action}`, {
          method: 'POST',
          headers: {
            Authorization: `Bearer ${auth.user.access_token}`,
            'Content-Type': 'application/json',
          },
        });

        if (!res.ok) {
          const errorData = await res.json().catch(() => ({}));
          throw new Error(errorData.message || `Failed to ${action} loop: HTTP ${res.status}`);
        }

        // Refresh loop data
        await fetchLoop();
      } catch (err) {
        setError(err instanceof Error ? err.message : `Failed to ${action} loop`);
        throw err;
      }
    },
    [auth.user?.access_token, loopId, fetchLoop]
  );

  return {
    loop,
    loading,
    error,
    refresh: fetchLoop,
    transitionState,
  };
}
