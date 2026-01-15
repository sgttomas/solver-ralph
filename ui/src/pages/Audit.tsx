/**
 * Audit Page
 *
 * Displays the append-only event log which is the sole source of truth
 * for governance-relevant state per SR-SPEC §1.5.
 *
 * Features:
 * - Chronological event stream
 * - Filtering by event type, stream kind, actor
 * - Event detail view with payload and typed refs
 * - Real-time updates via polling or SSE
 */

import { useState, useEffect, useCallback } from 'react';
import { Link } from 'react-router-dom';
import { useAuth } from '../auth/AuthProvider';
import config from '../config';
import { Card, Pill } from '../ui';
import styles from '../styles/pages.module.css';

interface TypedRef {
  kind: string;
  id: string;
  rel: string;
  meta?: Record<string, string>;
}

interface Event {
  event_id: string;
  stream_id: string;
  stream_kind: string;
  stream_seq: number;
  global_seq: number;
  event_type: string;
  occurred_at: string;
  actor_kind: 'HUMAN' | 'AGENT' | 'SYSTEM';
  actor_id: string;
  correlation_id: string | null;
  causation_id: string | null;
  refs: TypedRef[];
  payload: Record<string, unknown>;
}

interface AuditResponse {
  events: Event[];
  total_count: number;
  has_more: boolean;
}

type StreamKindFilter = 'ALL' | 'LOOP' | 'ITERATION' | 'CANDIDATE' | 'RUN' | 'APPROVAL' | 'DECISION' | 'GOVERNANCE' | 'EXCEPTION' | 'FREEZE';
type ActorKindFilter = 'ALL' | 'HUMAN' | 'AGENT' | 'SYSTEM';

const STREAM_KINDS: StreamKindFilter[] = ['ALL', 'LOOP', 'ITERATION', 'CANDIDATE', 'RUN', 'APPROVAL', 'DECISION', 'GOVERNANCE', 'EXCEPTION', 'FREEZE'];
const ACTOR_KINDS: ActorKindFilter[] = ['ALL', 'HUMAN', 'AGENT', 'SYSTEM'];

const EVENT_TYPE_CATEGORIES: Record<string, string[]> = {
  'Loop': ['LoopCreated', 'LoopActivated', 'LoopPaused', 'LoopResumed', 'LoopClosed'],
  'Iteration': ['IterationStarted', 'IterationCompleted', 'IterationSummaryRecorded'],
  'Candidate': ['CandidateMaterialized', 'CandidateVerificationComputed'],
  'Evidence': ['RunStarted', 'RunCompleted', 'EvidenceBundleRecorded', 'EvidenceMissingDetected'],
  'Approval': ['ApprovalRecorded', 'DecisionRecorded'],
  'Governance': ['GovernedArtifactVersionRecorded', 'NodeMarkedStale', 'ReEvaluationTriggered', 'StalenessResolved'],
  'Exception': ['DeviationCreated', 'DeferralCreated', 'WaiverCreated', 'ExceptionActivated', 'ExceptionResolved'],
  'Freeze': ['FreezeRecordCreated'],
  'Stop': ['StopTriggered'],
};

export function Audit(): JSX.Element {
  const auth = useAuth();
  const [events, setEvents] = useState<Event[]>([]);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);
  const [selectedEvent, setSelectedEvent] = useState<Event | null>(null);

  // Filters
  const [streamKindFilter, setStreamKindFilter] = useState<StreamKindFilter>('ALL');
  const [actorKindFilter, setActorKindFilter] = useState<ActorKindFilter>('ALL');
  const [eventTypeFilter, setEventTypeFilter] = useState<string>('');
  const [limit, setLimit] = useState(50);

  // Stats
  const [totalCount, setTotalCount] = useState(0);

  const fetchEvents = useCallback(() => {
    if (!auth.user?.access_token) return;

    setLoading(true);
    const params = new URLSearchParams();
    params.set('limit', limit.toString());
    if (streamKindFilter !== 'ALL') params.set('stream_kind', streamKindFilter);
    if (actorKindFilter !== 'ALL') params.set('actor_kind', actorKindFilter);
    if (eventTypeFilter) params.set('event_type', eventTypeFilter);

    fetch(`${config.apiUrl}/api/v1/events?${params}`, {
      headers: {
        Authorization: `Bearer ${auth.user.access_token}`,
      },
    })
      .then(res => {
        // Treat 404 as "no data yet" rather than an error
        if (res.status === 404) {
          return { events: [], total_count: 0, has_more: false };
        }
        if (!res.ok) throw new Error(`HTTP ${res.status}`);
        return res.json();
      })
      .then((data: AuditResponse) => {
        setEvents(data.events || []);
        setTotalCount(data.total_count || 0);
        setLoading(false);
      })
      .catch(err => {
        setError(err.message);
        setLoading(false);
      });
  }, [auth.user?.access_token, streamKindFilter, actorKindFilter, eventTypeFilter, limit]);

  useEffect(() => {
    fetchEvents();
  }, [fetchEvents]);

  const getActorTone = (actorKind: string) => {
    switch (actorKind) {
      case 'HUMAN': return 'success';
      case 'SYSTEM': return 'neutral';
      case 'AGENT': return 'warning';
      default: return 'neutral';
    }
  };

  const getEventTypeTone = (eventType: string) => {
    if (eventType.includes('Created') || eventType.includes('Started')) return 'success';
    if (eventType.includes('Completed') || eventType.includes('Recorded')) return 'success';
    if (eventType.includes('Stop') || eventType.includes('Paused') || eventType.includes('Missing')) return 'danger';
    if (eventType.includes('Stale') || eventType.includes('Exception')) return 'warning';
    return 'neutral';
  };

  const truncateId = (id: string): string => {
    if (!id) return '';
    return id.length > 24 ? id.slice(0, 12) + '...' + id.slice(-8) : id;
  };

  const formatTimestamp = (ts: string): string => {
    const date = new Date(ts);
    return date.toLocaleString();
  };

  const getRefLink = (ref: TypedRef): string | null => {
    switch (ref.kind) {
      case 'Loop': return `/loops/${ref.id}`;
      case 'Iteration': return `/iterations/${ref.id}`;
      case 'Candidate': return `/candidates/${ref.id}`;
      case 'EvidenceBundle': return `/artifacts/${ref.id}`;
      case 'Approval': return `/approvals`;
      default: return null;
    }
  };

  return (
    <div className={styles.container}>
      <div className={styles.header}>
        <div className={styles.headerStart}>
          <h1 className={styles.title}>Audit Log</h1>
          <p className={styles.subtitle}>Append-only event stream — sole source of truth</p>
        </div>
        <button
          onClick={fetchEvents}
          style={{
            padding: '0.5rem 1rem',
            border: '1px solid var(--border)',
            borderRadius: 'var(--radiusSm)',
            background: 'white',
            cursor: 'pointer',
            fontSize: '0.875rem',
          }}
        >
          Refresh
        </button>
      </div>

      {/* Stats Overview */}
      <Card>
        <div className={styles.statsGrid}>
          <div className={styles.stat}>
            <div className={styles.statLabel}>Total Events</div>
            <div className={styles.statValue}>{totalCount.toLocaleString()}</div>
          </div>
          <div className={styles.stat}>
            <div className={styles.statLabel}>Showing</div>
            <div className={styles.statValue}>{events.length}</div>
          </div>
          <div className={styles.stat}>
            <div className={styles.statLabel}>Store</div>
            <div className={styles.statValue}>es.events</div>
          </div>
          <div className={styles.stat}>
            <div className={styles.statLabel}>Ordering</div>
            <div className={styles.statValue}>global_seq</div>
          </div>
        </div>
      </Card>

      {/* Info Note */}
      <div className={styles.note}>
        Per SR-SPEC §1.5: The append-only event log is the sole source of truth for governance-relevant
        state. Events MUST NOT be updated or deleted. Corrections are represented as new events with
        `supersedes` populated. All projections are rebuildable from this log.
      </div>

      {/* Filters */}
      <Card>
        <div style={{ display: 'flex', gap: '1rem', flexWrap: 'wrap', alignItems: 'flex-end' }}>
          <div className={styles.formGroup} style={{ minWidth: '150px' }}>
            <label className={styles.label}>Stream Kind</label>
            <select
              className={styles.select}
              value={streamKindFilter}
              onChange={(e) => setStreamKindFilter(e.target.value as StreamKindFilter)}
            >
              {STREAM_KINDS.map(kind => (
                <option key={kind} value={kind}>{kind}</option>
              ))}
            </select>
          </div>

          <div className={styles.formGroup} style={{ minWidth: '120px' }}>
            <label className={styles.label}>Actor Kind</label>
            <select
              className={styles.select}
              value={actorKindFilter}
              onChange={(e) => setActorKindFilter(e.target.value as ActorKindFilter)}
            >
              {ACTOR_KINDS.map(kind => (
                <option key={kind} value={kind}>{kind}</option>
              ))}
            </select>
          </div>

          <div className={styles.formGroup} style={{ minWidth: '200px' }}>
            <label className={styles.label}>Event Type</label>
            <input
              className={styles.input}
              type="text"
              placeholder="e.g., LoopCreated"
              value={eventTypeFilter}
              onChange={(e) => setEventTypeFilter(e.target.value)}
            />
          </div>

          <div className={styles.formGroup} style={{ minWidth: '100px' }}>
            <label className={styles.label}>Limit</label>
            <select
              className={styles.select}
              value={limit}
              onChange={(e) => setLimit(Number(e.target.value))}
            >
              <option value={25}>25</option>
              <option value={50}>50</option>
              <option value={100}>100</option>
              <option value={250}>250</option>
            </select>
          </div>
        </div>
      </Card>

      {/* Events Table */}
      <Card>
        {loading ? (
          <div className={styles.placeholder}>
            <p className={styles.placeholderText}>Loading events...</p>
          </div>
        ) : error ? (
          <div className={styles.placeholder}>
            <p className={styles.error}>Error: {error}</p>
          </div>
        ) : events.length === 0 ? (
          <div className={styles.placeholder}>
            <p className={styles.placeholderText}>No events found.</p>
            <p className={styles.placeholderHint}>
              Events are recorded as governance-relevant state changes occur.
              Try adjusting your filters or check back later.
            </p>
          </div>
        ) : (
          <table className={styles.table}>
            <thead>
              <tr>
                <th className={styles.th}>Seq</th>
                <th className={styles.th}>Event Type</th>
                <th className={styles.th}>Stream</th>
                <th className={styles.th}>Actor</th>
                <th className={styles.th}>Refs</th>
                <th className={styles.th}>Occurred</th>
                <th className={styles.th}>Actions</th>
              </tr>
            </thead>
            <tbody>
              {events.map(event => (
                <tr key={event.event_id}>
                  <td className={styles.tdMono}>{event.global_seq}</td>
                  <td className={styles.td}>
                    <Pill tone={getEventTypeTone(event.event_type)}>
                      {event.event_type}
                    </Pill>
                  </td>
                  <td className={styles.td}>
                    <div style={{ display: 'flex', flexDirection: 'column', gap: '0.25rem' }}>
                      <Pill tone="neutral">{event.stream_kind}</Pill>
                      <code style={{ fontSize: '0.7rem', color: 'var(--muted)' }}>
                        {truncateId(event.stream_id)}
                      </code>
                    </div>
                  </td>
                  <td className={styles.td}>
                    <div style={{ display: 'flex', flexDirection: 'column', gap: '0.25rem' }}>
                      <Pill tone={getActorTone(event.actor_kind)}>{event.actor_kind}</Pill>
                      <code style={{ fontSize: '0.7rem', color: 'var(--muted)' }}>
                        {truncateId(event.actor_id)}
                      </code>
                    </div>
                  </td>
                  <td className={styles.td}>
                    {event.refs.length > 0 ? (
                      <span style={{ fontSize: '0.875rem' }}>{event.refs.length} refs</span>
                    ) : (
                      <span style={{ color: 'var(--muted)' }}>—</span>
                    )}
                  </td>
                  <td className={styles.td} style={{ fontSize: '0.8rem' }}>
                    {formatTimestamp(event.occurred_at)}
                  </td>
                  <td className={styles.td}>
                    <button
                      className={styles.actionLink}
                      onClick={() => setSelectedEvent(
                        selectedEvent?.event_id === event.event_id ? null : event
                      )}
                    >
                      {selectedEvent?.event_id === event.event_id ? 'Hide' : 'Details'}
                    </button>
                  </td>
                </tr>
              ))}
            </tbody>
          </table>
        )}
      </Card>

      {/* Selected Event Detail */}
      {selectedEvent && (
        <Card>
          <h3 style={{ margin: '0 0 1rem 0', fontSize: '1rem', color: 'var(--ink)' }}>
            Event Detail: {selectedEvent.event_type}
          </h3>

          <div className={styles.statsGrid} style={{ marginBottom: '1.5rem' }}>
            <div className={styles.stat}>
              <div className={styles.statLabel}>Event ID</div>
              <div style={{ fontFamily: 'var(--mono)', fontSize: '0.75rem' }}>
                {selectedEvent.event_id}
              </div>
            </div>
            <div className={styles.stat}>
              <div className={styles.statLabel}>Global Seq</div>
              <div className={styles.statValue}>{selectedEvent.global_seq}</div>
            </div>
            <div className={styles.stat}>
              <div className={styles.statLabel}>Stream Seq</div>
              <div className={styles.statValue}>{selectedEvent.stream_seq}</div>
            </div>
          </div>

          {/* Refs */}
          {selectedEvent.refs.length > 0 && (
            <div style={{ marginBottom: '1.5rem' }}>
              <h4 style={{ margin: '0 0 0.75rem 0', fontSize: '0.875rem', color: 'var(--ink)' }}>
                Typed References ({selectedEvent.refs.length})
              </h4>
              <ul className={styles.refList}>
                {selectedEvent.refs.map((ref, idx) => {
                  const link = getRefLink(ref);
                  return (
                    <li key={idx} className={styles.refItem}>
                      <Pill tone="neutral">{ref.kind}</Pill>
                      <span className={styles.refRel}>{ref.rel}</span>
                      {link ? (
                        <Link to={link} className={styles.link}>
                          {truncateId(ref.id)}
                        </Link>
                      ) : (
                        <span>{truncateId(ref.id)}</span>
                      )}
                      {ref.meta?.content_hash && (
                        <code style={{ fontSize: '0.65rem', color: 'var(--muted)' }}>
                          {truncateId(ref.meta.content_hash)}
                        </code>
                      )}
                    </li>
                  );
                })}
              </ul>
            </div>
          )}

          {/* Correlation/Causation */}
          {(selectedEvent.correlation_id || selectedEvent.causation_id) && (
            <div style={{ marginBottom: '1.5rem' }}>
              <h4 style={{ margin: '0 0 0.75rem 0', fontSize: '0.875rem', color: 'var(--ink)' }}>
                Correlation
              </h4>
              <div className={styles.infoRow}>
                <span className={styles.infoLabel}>Correlation ID</span>
                <span className={styles.infoValue} style={{ fontFamily: 'var(--mono)', fontSize: '0.75rem' }}>
                  {selectedEvent.correlation_id || '—'}
                </span>
              </div>
              <div className={styles.infoRow}>
                <span className={styles.infoLabel}>Causation ID</span>
                <span className={styles.infoValue} style={{ fontFamily: 'var(--mono)', fontSize: '0.75rem' }}>
                  {selectedEvent.causation_id || '—'}
                </span>
              </div>
            </div>
          )}

          {/* Payload */}
          <div>
            <h4 style={{ margin: '0 0 0.75rem 0', fontSize: '0.875rem', color: 'var(--ink)' }}>
              Payload
            </h4>
            <pre style={{
              background: 'rgba(0,0,0,0.03)',
              padding: '1rem',
              borderRadius: 'var(--radiusSm)',
              overflow: 'auto',
              fontSize: '0.75rem',
              fontFamily: 'var(--mono)',
              maxHeight: '300px',
              margin: 0,
            }}>
              {JSON.stringify(selectedEvent.payload, null, 2)}
            </pre>
          </div>
        </Card>
      )}

      {/* Event Types Reference */}
      <Card>
        <h3 style={{ margin: '0 0 1rem 0', fontSize: '0.875rem', color: 'var(--ink)' }}>
          Event Types Reference (SR-SPEC Appendix A)
        </h3>
        <div style={{ display: 'grid', gridTemplateColumns: 'repeat(auto-fit, minmax(200px, 1fr))', gap: '1rem' }}>
          {Object.entries(EVENT_TYPE_CATEGORIES).map(([category, types]) => (
            <div key={category}>
              <h4 style={{ margin: '0 0 0.5rem 0', fontSize: '0.75rem', color: 'var(--muted)', textTransform: 'uppercase' }}>
                {category}
              </h4>
              <ul style={{ margin: 0, padding: 0, listStyle: 'none' }}>
                {types.map(type => (
                  <li
                    key={type}
                    style={{
                      fontSize: '0.75rem',
                      fontFamily: 'var(--mono)',
                      padding: '0.25rem 0',
                      cursor: 'pointer',
                      color: eventTypeFilter === type ? 'var(--accent)' : 'var(--ink)',
                    }}
                    onClick={() => setEventTypeFilter(eventTypeFilter === type ? '' : type)}
                  >
                    {type}
                  </li>
                ))}
              </ul>
            </div>
          ))}
        </div>
      </Card>
    </div>
  );
}

export default Audit;
