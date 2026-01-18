/**
 * Staleness Management Console (Phase 5 UI parity)
 *
 * Surfaces SR-SPEC §2.3.9 staleness APIs:
 * - Mark a root as stale (fan-out to dependents)
 * - List dependents for a root
 * - Resolve staleness markers
 *
 * Client-side validation enforces required fields and valid reason/resolution kinds.
 */

import { FormEvent, useState } from 'react';
import { useAuth } from '../auth/AuthProvider';
import config from '../config';
import { Card, Button, Pill } from '../ui';
import styles from '../styles/pages.module.css';

const STALENESS_REASONS = [
  'GOVERNED_ARTIFACT_CHANGED',
  'ORACLE_SUITE_REBASED',
  'EXCEPTION_ACTIVATED',
  'DEPENDENCY_STALE',
  'MANUAL_MARK',
];

const RESOLUTION_KINDS: Array<'MECHANICAL' | 'DECISION'> = ['MECHANICAL', 'DECISION'];

interface MarkedDependent {
  stale_id: string;
  dependent_kind: string;
  dependent_id: string;
}

interface DependentsRow extends MarkedDependent {
  reason_code: string;
  reason_detail?: string | null;
  marked_at: string;
}

interface MarkResponse {
  stale_ids: string[];
  dependents_marked: MarkedDependent[];
}

interface ResolveResponse {
  stale_id: string;
  resolved: boolean;
  event_id: string;
}

interface StalenessStatus {
  root_kind: string;
  root_id: string;
  dependents: DependentsRow[];
}

interface ResolutionRef {
  kind: string;
  id: string;
  rel: string;
}

function parseTypedRefs(raw: string): ResolutionRef[] {
  if (!raw.trim()) return [];

  return raw
    .split(/\r?\n|,/)
    .map(entry => entry.trim())
    .filter(Boolean)
    .map((entry) => {
      const [kind, id, rel] = entry.split(':').map(part => part.trim());
      return {
        kind: kind || '',
        id: id || '',
        rel: rel || 'relates_to',
      };
    })
    .filter(ref => ref.kind !== '' && ref.id !== '');
}

export function Staleness(): JSX.Element {
  const auth = useAuth();
  const [markRootKind, setMarkRootKind] = useState('GovernedArtifact');
  const [markRootId, setMarkRootId] = useState('');
  const [markReason, setMarkReason] = useState<string>(STALENESS_REASONS[4]);
  const [markReasonDetail, setMarkReasonDetail] = useState('');
  const [markDepth, setMarkDepth] = useState(5);
  const [markResult, setMarkResult] = useState<MarkResponse | null>(null);
  const [markError, setMarkError] = useState<string | null>(null);
  const [markSuccess, setMarkSuccess] = useState<string | null>(null);
  const [markLoading, setMarkLoading] = useState(false);

  const [dependentsRootKind, setDependentsRootKind] = useState('GovernedArtifact');
  const [dependentsRootId, setDependentsRootId] = useState('');
  const [dependentsStatus, setDependentsStatus] = useState<StalenessStatus | null>(null);
  const [dependentsError, setDependentsError] = useState<string | null>(null);
  const [dependentsLoading, setDependentsLoading] = useState(false);

  const [resolveStaleId, setResolveStaleId] = useState('');
  const [resolveKind, setResolveKind] = useState<'MECHANICAL' | 'DECISION'>('MECHANICAL');
  const [resolveNote, setResolveNote] = useState('');
  const [resolveRefsRaw, setResolveRefsRaw] = useState('');
  const [resolveError, setResolveError] = useState<string | null>(null);
  const [resolveSuccess, setResolveSuccess] = useState<string | null>(null);
  const [resolveResult, setResolveResult] = useState<ResolveResponse | null>(null);
  const [resolveLoading, setResolveLoading] = useState(false);

  const headers = (): Record<string, string> => {
    const h: Record<string, string> = { 'Content-Type': 'application/json' };
    if (auth.user?.access_token) {
      h.Authorization = `Bearer ${auth.user.access_token}`;
    }
    return h;
  };

  const handleMark = async (e: FormEvent) => {
    e.preventDefault();
    setMarkError(null);
    setMarkSuccess(null);
    setMarkResult(null);

    if (!markRootKind.trim() || !markRootId.trim()) {
      setMarkError('Root kind and root id are required to mark staleness.');
      return;
    }

    setMarkLoading(true);
    try {
      const res = await fetch(`${config.apiUrl}/api/v1/staleness/mark`, {
        method: 'POST',
        headers: headers(),
        body: JSON.stringify({
          root_ref: { kind: markRootKind.trim(), id: markRootId.trim() },
          reason_code: markReason,
          reason_detail: markReasonDetail || null,
          max_depth: markDepth,
        }),
      });

      if (!res.ok) {
        const errData = await res.json().catch(() => ({}));
        throw new Error(errData.message || `HTTP ${res.status}`);
      }

      const data: MarkResponse = await res.json();
      setMarkResult(data);
      setMarkSuccess(`Marked ${data.dependents_marked.length || 1} dependents as stale.`);
      // Pre-populate dependents table for the marked root
      setDependentsStatus({
        root_kind: markRootKind.trim(),
        root_id: markRootId.trim(),
        dependents: data.dependents_marked.map(d => ({
          ...d,
          reason_code: markReason,
          reason_detail: markReasonDetail,
          marked_at: new Date().toISOString(),
        })),
      });
    } catch (err) {
      setMarkError(err instanceof Error ? err.message : 'Failed to mark staleness');
    } finally {
      setMarkLoading(false);
    }
  };

  const handleListDependents = async (e: FormEvent) => {
    e.preventDefault();
    setDependentsError(null);
    setDependentsStatus(null);

    if (!dependentsRootKind.trim() || !dependentsRootId.trim()) {
      setDependentsError('Root kind and root id are required to list dependents.');
      return;
    }

    setDependentsLoading(true);
    try {
      const params = new URLSearchParams({
        root_kind: dependentsRootKind.trim(),
        root_id: dependentsRootId.trim(),
      });
      const res = await fetch(`${config.apiUrl}/api/v1/staleness/dependents?${params.toString()}`, {
        headers: headers(),
      });

      if (!res.ok) {
        const errData = await res.json().catch(() => ({}));
        throw new Error(errData.message || `HTTP ${res.status}`);
      }

      const data: StalenessStatus = await res.json();
      setDependentsStatus(data);
    } catch (err) {
      setDependentsError(err instanceof Error ? err.message : 'Failed to fetch dependents');
    } finally {
      setDependentsLoading(false);
    }
  };

  const handleResolve = async (e: FormEvent) => {
    e.preventDefault();
    setResolveError(null);
    setResolveSuccess(null);
    setResolveResult(null);

    if (!resolveStaleId.trim()) {
      setResolveError('Stale marker id is required.');
      return;
    }

    const refs = parseTypedRefs(resolveRefsRaw);

    setResolveLoading(true);
    try {
      const res = await fetch(`${config.apiUrl}/api/v1/staleness/${encodeURIComponent(resolveStaleId.trim())}/resolve`, {
        method: 'POST',
        headers: headers(),
        body: JSON.stringify({
          resolution_kind: resolveKind,
          resolution_note: resolveNote || null,
          resolution_refs: refs.map(ref => ({
            kind: ref.kind,
            id: ref.id,
            rel: ref.rel || 'relates_to',
            meta: {},
          })),
        }),
      });

      if (!res.ok) {
        const errData = await res.json().catch(() => ({}));
        throw new Error(errData.message || `HTTP ${res.status}`);
      }

      const data: ResolveResponse = await res.json();
      setResolveResult(data);
      setResolveSuccess(`Staleness ${data.stale_id} resolved (event ${data.event_id}).`);
    } catch (err) {
      setResolveError(err instanceof Error ? err.message : 'Failed to resolve staleness');
    } finally {
      setResolveLoading(false);
    }
  };

  const renderDependentsTable = (status: StalenessStatus) => {
    if (status.dependents.length === 0) {
      return (
        <div className={styles.placeholder}>
          <p className={styles.placeholderText}>No unresolved dependents for this root.</p>
        </div>
      );
    }

    return (
      <table className={styles.table}>
        <thead>
          <tr>
            <th className={styles.th}>Stale ID</th>
            <th className={styles.th}>Dependent</th>
            <th className={styles.th}>Reason</th>
            <th className={styles.th}>Marked At</th>
          </tr>
        </thead>
        <tbody>
          {status.dependents.map(dep => (
            <tr key={dep.stale_id}>
              <td className={styles.tdMono}>{dep.stale_id}</td>
              <td className={styles.td}>
                <div style={{ display: 'flex', flexDirection: 'column' }}>
                  <span style={{ fontWeight: 500 }}>{dep.dependent_kind}</span>
                  <code className={styles.mono}>{dep.dependent_id}</code>
                </div>
              </td>
              <td className={styles.td}>
                <div style={{ display: 'flex', flexDirection: 'column', gap: '4px' }}>
                  <Pill tone="warning">{dep.reason_code}</Pill>
                  {dep.reason_detail && (
                    <span style={{ fontSize: '0.75rem', color: 'var(--muted)' }}>
                      {dep.reason_detail}
                    </span>
                  )}
                </div>
              </td>
              <td className={styles.td}>
                {dep.marked_at ? new Date(dep.marked_at).toLocaleString() : '—'}
              </td>
            </tr>
          ))}
        </tbody>
      </table>
    );
  };

  return (
    <div className={styles.container}>
      <div className={styles.header}>
        <div className={styles.headerStart}>
          <h1 className={styles.title}>Staleness Management</h1>
          <p className={styles.subtitle}>SR-SPEC §2.3.9 — mark, inspect, and resolve stale dependents.</p>
        </div>
      </div>

      <div className={styles.note}>
        Staleness markers block verification, freeze, and shippable status until resolved. Use these
        controls to mark roots stale when governed artifacts change, inspect impacted dependents, and
        resolve markers with the appropriate evidence or decision context.
      </div>

      {/* Mark staleness */}
      <Card title="Mark Staleness" className={styles.cardSpacing}>
        {markError && <div className={styles.error}>{markError}</div>}
        {markSuccess && <div className={styles.success}>{markSuccess}</div>}

        <form className={styles.form} onSubmit={handleMark}>
          <div className={styles.formRow}>
            <div className={styles.formGroup}>
              <label className={styles.label}>Root Kind *</label>
              <input
                className={styles.input}
                type="text"
                placeholder="e.g., GovernedArtifact, Candidate, OracleSuite"
                value={markRootKind}
                onChange={e => setMarkRootKind(e.target.value)}
              />
            </div>
            <div className={styles.formGroup}>
              <label className={styles.label}>Root ID *</label>
              <input
                className={styles.input}
                type="text"
                placeholder="SR-SPEC, cand_..., suite:SR-SUITE-CORE"
                value={markRootId}
                onChange={e => setMarkRootId(e.target.value)}
              />
            </div>
          </div>

          <div className={styles.formRow}>
            <div className={styles.formGroup}>
              <label className={styles.label}>Reason Code *</label>
              <select
                className={styles.select}
                value={markReason}
                onChange={e => setMarkReason(e.target.value)}
              >
                {STALENESS_REASONS.map(reason => (
                  <option key={reason} value={reason}>
                    {reason}
                  </option>
                ))}
              </select>
            </div>
            <div className={styles.formGroup}>
              <label className={styles.label}>Propagation Depth</label>
              <input
                className={styles.input}
                type="number"
                min={1}
                max={10}
                value={markDepth}
                onChange={e => setMarkDepth(parseInt(e.target.value, 10) || 1)}
              />
            </div>
          </div>

          <div className={styles.formGroup}>
            <label className={styles.label}>Reason Detail (optional)</label>
            <input
              className={styles.input}
              type="text"
              placeholder="Describe why this root is stale..."
              value={markReasonDetail}
              onChange={e => setMarkReasonDetail(e.target.value)}
            />
          </div>

          <div className={styles.buttonRow}>
            <Button variant="primary" type="submit" disabled={markLoading}>
              {markLoading ? 'Marking...' : 'Mark Stale'}
            </Button>
          </div>

          <div className={styles.note} style={{ marginTop: 'var(--space3)' }}>
            Root references use <code className={styles.mono}>rel=depends_on</code> semantics for
            staleness traversal. Audit-only references (<code className={styles.mono}>supported_by</code>)
            do not propagate staleness by default.
          </div>
        </form>

        {markResult && (
          <div style={{ marginTop: 'var(--space4)' }}>
            <h4 style={{ margin: '0 0 var(--space2) 0' }}>Marked Dependents</h4>
            {markResult.dependents_marked.length === 0 ? (
              <div className={styles.placeholder}>
                <p className={styles.placeholderText}>Root marked stale (no dependents found).</p>
              </div>
            ) : (
              <ul className={styles.refList}>
                {markResult.dependents_marked.map(dep => (
                  <li key={dep.stale_id} className={styles.refItem}>
                    <code className={styles.mono}>{dep.stale_id}</code>
                    <span className={styles.refRel}>
                      {dep.dependent_kind} → {dep.dependent_id}
                    </span>
                  </li>
                ))}
              </ul>
            )}
          </div>
        )}
      </Card>

      {/* List dependents */}
      <Card title="List Stale Dependents" className={styles.cardSpacing}>
        {dependentsError && <div className={styles.error}>{dependentsError}</div>}

        <form className={styles.form} onSubmit={handleListDependents}>
          <div className={styles.formRow}>
            <div className={styles.formGroup}>
              <label className={styles.label}>Root Kind *</label>
              <input
                className={styles.input}
                type="text"
                placeholder="GovernedArtifact, Candidate, OracleSuite"
                value={dependentsRootKind}
                onChange={e => setDependentsRootKind(e.target.value)}
              />
            </div>
            <div className={styles.formGroup}>
              <label className={styles.label}>Root ID *</label>
              <input
                className={styles.input}
                type="text"
                placeholder="SR-SPEC, cand_..., suite:SR-SUITE-CORE"
                value={dependentsRootId}
                onChange={e => setDependentsRootId(e.target.value)}
              />
            </div>
          </div>

          <div className={styles.buttonRow}>
            <Button variant="secondary" type="submit" disabled={dependentsLoading}>
              {dependentsLoading ? 'Loading...' : 'Fetch Dependents'}
            </Button>
          </div>
        </form>

        {dependentsStatus && (
          <div style={{ marginTop: 'var(--space4)' }}>
            <div style={{ marginBottom: 'var(--space2)', fontWeight: 600 }}>
              {dependentsStatus.root_kind}: <code className={styles.mono}>{dependentsStatus.root_id}</code>
            </div>
            {renderDependentsTable(dependentsStatus)}
          </div>
        )}
      </Card>

      {/* Resolve staleness */}
      <Card title="Resolve Staleness" className={styles.cardSpacing}>
        {resolveError && <div className={styles.error}>{resolveError}</div>}
        {resolveSuccess && <div className={styles.success}>{resolveSuccess}</div>}

        <form className={styles.form} onSubmit={handleResolve}>
          <div className={styles.formRow}>
            <div className={styles.formGroup}>
              <label className={styles.label}>Stale ID *</label>
              <input
                className={styles.input}
                type="text"
                placeholder="stale_01J...ULID"
                value={resolveStaleId}
                onChange={e => setResolveStaleId(e.target.value)}
              />
            </div>
            <div className={styles.formGroup}>
              <label className={styles.label}>Resolution Kind *</label>
              <select
                className={styles.select}
                value={resolveKind}
                onChange={e => setResolveKind(e.target.value as 'MECHANICAL' | 'DECISION')}
              >
                {RESOLUTION_KINDS.map(kind => (
                  <option key={kind} value={kind}>
                    {kind}
                  </option>
                ))}
              </select>
            </div>
          </div>

          <div className={styles.formGroup}>
            <label className={styles.label}>Resolution Note (optional)</label>
            <textarea
              className={styles.textarea}
              placeholder="Describe the evidence or decision that clears this stale marker..."
              value={resolveNote}
              onChange={e => setResolveNote(e.target.value)}
              rows={3}
            />
          </div>

          <div className={styles.formGroup}>
            <label className={styles.label}>Resolution References (optional)</label>
            <textarea
              className={styles.textarea}
              placeholder="One per line: Kind:identifier:relates_to (rel optional)"
              value={resolveRefsRaw}
              onChange={e => setResolveRefsRaw(e.target.value)}
              rows={3}
            />
            <span style={{ fontSize: '0.75rem', color: 'var(--muted)' }}>
              Lines or comma-separated. Missing rel defaults to <code className={styles.mono}>relates_to</code>.
              Required for DECISION resolutions to attach the authorizing Decision/Approval ref.
            </span>
          </div>

          <div className={styles.buttonRow}>
            <Button variant="primary" type="submit" disabled={resolveLoading}>
              {resolveLoading ? 'Resolving...' : 'Resolve Staleness'}
            </Button>
          </div>

          <div className={styles.note} style={{ marginTop: 'var(--space3)' }}>
            DECISION resolutions require a HUMAN actor and a binding Decision/Approval reference per
            SR-SPEC §1.13.3. Mechanical resolutions may be emitted by SYSTEM actors.
          </div>
        </form>

        {resolveResult && (
          <div style={{ marginTop: 'var(--space3)', fontSize: '0.875rem' }}>
            <div style={{ display: 'flex', gap: 'var(--space2)', alignItems: 'center' }}>
              <Pill tone="success">Resolved</Pill>
              <code className={styles.mono}>{resolveResult.stale_id}</code>
              <span style={{ color: 'var(--muted)' }}>event {resolveResult.event_id}</span>
            </div>
          </div>
        )}
      </Card>
    </div>
  );
}

export default Staleness;
