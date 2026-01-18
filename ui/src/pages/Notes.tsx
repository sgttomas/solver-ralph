/**
 * Human Judgment Notes Console
 *
 * Surfaces SR-SPEC §2.3.10 note APIs:
 * - Evaluation notes (verification evidence review)
 * - Assessment notes (validation judgment)
 * - Intervention notes (actions taken)
 *
 * Client-side validation enforces HUMAN-only use, required content, and typed refs.
 */

import { FormEvent, useState } from 'react';
import { useAuth } from '../auth/AuthProvider';
import config from '../config';
import { Card, Button, Pill } from '../ui';
import styles from '../styles/pages.module.css';

interface TypedRef {
  kind: string;
  id: string;
  rel: string;
}

interface RecordEntry {
  record_id: string;
  record_type: string;
  subject_refs: TypedRef[] | unknown;
  evidence_refs: string[];
  content: string;
  severity?: string | null;
  fitness_judgment?: string | null;
  recommendations?: string | null;
  details?: Record<string, unknown>;
  recorded_by_kind: string;
  recorded_by_id: string;
  recorded_at: string;
}

interface NotePayload {
  subject_refs: TypedRef[];
  evidence_refs: string[];
  content: string;
  severity?: string | null;
  recommendations?: string | null;
  fitness_judgment?: string | null;
  context?: string | null;
  actions_taken?: string | null;
  impact?: string | null;
}

function parseSubjectRefs(raw: string): TypedRef[] {
  if (!raw.trim()) return [];

  return raw
    .split(/\r?\n|,/)
    .map(entry => entry.trim())
    .filter(Boolean)
    .map(entry => {
      const [kind, id, rel] = entry.split(':').map(part => part.trim());
      return {
        kind: kind || '',
        id: id || '',
        rel: rel || 'relates_to',
      };
    })
    .filter(ref => ref.kind !== '' && ref.id !== '');
}

function parseEvidenceRefs(raw: string): string[] {
  if (!raw.trim()) return [];
  return raw
    .split(/,|\r?\n/)
    .map(ref => ref.trim())
    .filter(Boolean);
}

function formatSubjectRefs(refs: TypedRef[] | unknown): string {
  if (!Array.isArray(refs)) return '';
  return refs
    .map(ref => `${(ref as TypedRef).kind}:${(ref as TypedRef).id} (${(ref as TypedRef).rel})`)
    .join(', ');
}

export function Notes(): JSX.Element {
  const auth = useAuth();
  const [records, setRecords] = useState<RecordEntry[]>([]);

  // Evaluation state
  const [evalSubjectRefs, setEvalSubjectRefs] = useState('');
  const [evalEvidenceRefs, setEvalEvidenceRefs] = useState('');
  const [evalContent, setEvalContent] = useState('');
  const [evalSeverity, setEvalSeverity] = useState('');
  const [evalRecommendations, setEvalRecommendations] = useState('');
  const [evalError, setEvalError] = useState<string | null>(null);
  const [evalSuccess, setEvalSuccess] = useState<string | null>(null);

  // Assessment state
  const [assessSubjectRefs, setAssessSubjectRefs] = useState('');
  const [assessEvidenceRefs, setAssessEvidenceRefs] = useState('');
  const [assessContent, setAssessContent] = useState('');
  const [assessFitness, setAssessFitness] = useState('');
  const [assessContext, setAssessContext] = useState('');
  const [assessError, setAssessError] = useState<string | null>(null);
  const [assessSuccess, setAssessSuccess] = useState<string | null>(null);

  // Intervention state
  const [interSubjectRefs, setInterSubjectRefs] = useState('');
  const [interEvidenceRefs, setInterEvidenceRefs] = useState('');
  const [interContent, setInterContent] = useState('');
  const [interActions, setInterActions] = useState('');
  const [interImpact, setInterImpact] = useState('');
  const [interError, setInterError] = useState<string | null>(null);
  const [interSuccess, setInterSuccess] = useState<string | null>(null);

  // Lookup state
  const [lookupId, setLookupId] = useState('');
  const [lookupError, setLookupError] = useState<string | null>(null);

  const headers = (): Record<string, string> => {
    const h: Record<string, string> = { 'Content-Type': 'application/json' };
    if (auth.user?.access_token) {
      h.Authorization = `Bearer ${auth.user.access_token}`;
    }
    return h;
  };

  const fetchRecord = async (recordId: string) => {
    const res = await fetch(`${config.apiUrl}/api/v1/records/${encodeURIComponent(recordId)}`, {
      headers: headers(),
    });
    if (!res.ok) {
      const errData = await res.json().catch(() => ({}));
      throw new Error(errData.message || `HTTP ${res.status}`);
    }
    const data: RecordEntry = await res.json();
    setRecords(prev => {
      const next = prev.filter(r => r.record_id !== data.record_id);
      next.unshift(data);
      return next.slice(0, 10);
    });
    return data.record_id;
  };

  const submitNote = async (
    endpoint: string,
    payload: NotePayload,
    onSuccess: (msg: string) => void,
    onError: (msg: string) => void,
  ) => {
    onError('');
    if (!payload.content.trim()) {
      onError('Content is required for auditability.');
      return;
    }

    try {
      const res = await fetch(`${config.apiUrl}${endpoint}`, {
        method: 'POST',
        headers: headers(),
        body: JSON.stringify(payload),
      });

      if (!res.ok) {
        const errData = await res.json().catch(() => ({}));
        throw new Error(errData.message || `HTTP ${res.status}`);
      }

      const data = await res.json();
      const recordId = data.record_id as string;
      await fetchRecord(recordId);
      onSuccess(`Recorded ${recordId}`);
    } catch (err) {
      onError(err instanceof Error ? err.message : 'Failed to record note');
    }
  };

  const handleEvalSubmit = async (e: FormEvent) => {
    e.preventDefault();
    setEvalError(null);
    setEvalSuccess(null);

    const subjectRefs = parseSubjectRefs(evalSubjectRefs);
    const evidenceRefs = parseEvidenceRefs(evalEvidenceRefs);

    await submitNote(
      '/api/v1/records/evaluation-notes',
      {
        subject_refs: subjectRefs,
        evidence_refs: evidenceRefs,
        content: evalContent,
        severity: evalSeverity || null,
        recommendations: evalRecommendations || null,
      },
      msg => setEvalSuccess(msg),
      msg => setEvalError(msg),
    );
  };

  const handleAssessmentSubmit = async (e: FormEvent) => {
    e.preventDefault();
    setAssessError(null);
    setAssessSuccess(null);

    const subjectRefs = parseSubjectRefs(assessSubjectRefs);
    const evidenceRefs = parseEvidenceRefs(assessEvidenceRefs);

    await submitNote(
      '/api/v1/records/assessment-notes',
      {
        subject_refs: subjectRefs,
        evidence_refs: evidenceRefs,
        content: assessContent,
        fitness_judgment: assessFitness || null,
        context: assessContext || null,
      },
      msg => setAssessSuccess(msg),
      msg => setAssessError(msg),
    );
  };

  const handleInterventionSubmit = async (e: FormEvent) => {
    e.preventDefault();
    setInterError(null);
    setInterSuccess(null);

    const subjectRefs = parseSubjectRefs(interSubjectRefs);
    const evidenceRefs = parseEvidenceRefs(interEvidenceRefs);

    await submitNote(
      '/api/v1/records/intervention-notes',
      {
        subject_refs: subjectRefs,
        evidence_refs: evidenceRefs,
        content: interContent,
        actions_taken: interActions || null,
        impact: interImpact || null,
      },
      msg => setInterSuccess(msg),
      msg => setInterError(msg),
    );
  };

  const handleLookup = async (e: FormEvent) => {
    e.preventDefault();
    setLookupError(null);
    if (!lookupId.trim()) {
      setLookupError('Record ID required to fetch note.');
      return;
    }
    try {
      await fetchRecord(lookupId.trim());
      setLookupId('');
    } catch (err) {
      setLookupError(err instanceof Error ? err.message : 'Failed to fetch record');
    }
  };

  return (
    <div className={styles.container}>
      <div className={styles.header}>
        <div className={styles.headerStart}>
          <h1 className={styles.title}>Human Notes</h1>
          <p className={styles.subtitle}>Evaluation, assessment, and intervention notes (non-binding).</p>
        </div>
      </div>

      <div className={styles.note}>
        Notes are HUMAN-only per SR-SPEC §2.3.10 and cannot advance verification or approval state.
        Attach subjects and evidence refs for traceability; approvals must still flow through seeded portals.
      </div>

      {/* Evaluation Notes */}
      <Card title="Evaluation Note" className={styles.cardSpacing}>
        {evalError && <div className={styles.error}>{evalError}</div>}
        {evalSuccess && <div className={styles.success}>{evalSuccess}</div>}

        <form className={styles.form} onSubmit={handleEvalSubmit}>
          <div className={styles.formGroup}>
            <label className={styles.label}>Subject Refs</label>
            <textarea
              className={styles.textarea}
              placeholder="One per line: Kind:id:relates_to"
              value={evalSubjectRefs}
              onChange={e => setEvalSubjectRefs(e.target.value)}
              rows={2}
            />
            <span style={{ fontSize: '0.75rem', color: 'var(--muted)' }}>
              Optional. rel defaults to relates_to.
            </span>
          </div>

          <div className={styles.formGroup}>
            <label className={styles.label}>Evidence Refs</label>
            <input
              className={styles.input}
              type="text"
              placeholder="sha256:..., sha256:..."
              value={evalEvidenceRefs}
              onChange={e => setEvalEvidenceRefs(e.target.value)}
            />
          </div>

          <div className={styles.formGroup}>
            <label className={styles.label}>Content *</label>
            <textarea
              className={styles.textarea}
              placeholder="Observed verification issues, severity, recommended actions..."
              value={evalContent}
              onChange={e => setEvalContent(e.target.value)}
              required
            />
          </div>

          <div className={styles.formRow}>
            <div className={styles.formGroup}>
              <label className={styles.label}>Severity</label>
              <input
                className={styles.input}
                type="text"
                placeholder="LOW | MEDIUM | HIGH"
                value={evalSeverity}
                onChange={e => setEvalSeverity(e.target.value)}
              />
            </div>
            <div className={styles.formGroup}>
              <label className={styles.label}>Recommendations</label>
              <input
                className={styles.input}
                type="text"
                placeholder="Actions to take..."
                value={evalRecommendations}
                onChange={e => setEvalRecommendations(e.target.value)}
              />
            </div>
          </div>

          <div className={styles.buttonRow}>
            <Button variant="primary" type="submit">
              Record Evaluation
            </Button>
          </div>
        </form>
      </Card>

      {/* Assessment Notes */}
      <Card title="Assessment Note" className={styles.cardSpacing}>
        {assessError && <div className={styles.error}>{assessError}</div>}
        {assessSuccess && <div className={styles.success}>{assessSuccess}</div>}

        <form className={styles.form} onSubmit={handleAssessmentSubmit}>
          <div className={styles.formGroup}>
            <label className={styles.label}>Subject Refs</label>
            <textarea
              className={styles.textarea}
              placeholder="One per line: Kind:id:relates_to"
              value={assessSubjectRefs}
              onChange={e => setAssessSubjectRefs(e.target.value)}
              rows={2}
            />
          </div>

          <div className={styles.formGroup}>
            <label className={styles.label}>Evidence Refs</label>
            <input
              className={styles.input}
              type="text"
              placeholder="sha256:..., sha256:..."
              value={assessEvidenceRefs}
              onChange={e => setAssessEvidenceRefs(e.target.value)}
            />
          </div>

          <div className={styles.formGroup}>
            <label className={styles.label}>Content *</label>
            <textarea
              className={styles.textarea}
              placeholder="Validation assessment, context, edge cases..."
              value={assessContent}
              onChange={e => setAssessContent(e.target.value)}
              required
            />
          </div>

          <div className={styles.formRow}>
            <div className={styles.formGroup}>
              <label className={styles.label}>Fitness Judgment</label>
              <input
                className={styles.input}
                type="text"
                placeholder="FIT / UNFIT / CONDITIONAL"
                value={assessFitness}
                onChange={e => setAssessFitness(e.target.value)}
              />
            </div>
            <div className={styles.formGroup}>
              <label className={styles.label}>Context</label>
              <input
                className={styles.input}
                type="text"
                placeholder="Scope, scenario, constraints..."
                value={assessContext}
                onChange={e => setAssessContext(e.target.value)}
              />
            </div>
          </div>

          <div className={styles.buttonRow}>
            <Button variant="primary" type="submit">
              Record Assessment
            </Button>
          </div>
        </form>
      </Card>

      {/* Intervention Notes */}
      <Card title="Intervention Note" className={styles.cardSpacing}>
        {interError && <div className={styles.error}>{interError}</div>}
        {interSuccess && <div className={styles.success}>{interSuccess}</div>}

        <form className={styles.form} onSubmit={handleInterventionSubmit}>
          <div className={styles.formGroup}>
            <label className={styles.label}>Subject Refs</label>
            <textarea
              className={styles.textarea}
              placeholder="One per line: Kind:id:relates_to"
              value={interSubjectRefs}
              onChange={e => setInterSubjectRefs(e.target.value)}
              rows={2}
            />
          </div>

          <div className={styles.formGroup}>
            <label className={styles.label}>Evidence Refs</label>
            <input
              className={styles.input}
              type="text"
              placeholder="sha256:..., sha256:..."
              value={interEvidenceRefs}
              onChange={e => setInterEvidenceRefs(e.target.value)}
            />
          </div>

          <div className={styles.formGroup}>
            <label className={styles.label}>Content *</label>
            <textarea
              className={styles.textarea}
              placeholder="Describe the intervention and why it was necessary..."
              value={interContent}
              onChange={e => setInterContent(e.target.value)}
              required
            />
          </div>

          <div className={styles.formRow}>
            <div className={styles.formGroup}>
              <label className={styles.label}>Actions Taken</label>
              <input
                className={styles.input}
                type="text"
                placeholder="Steps performed..."
                value={interActions}
                onChange={e => setInterActions(e.target.value)}
              />
            </div>
            <div className={styles.formGroup}>
              <label className={styles.label}>Impact</label>
              <input
                className={styles.input}
                type="text"
                placeholder="Observed impact..."
                value={interImpact}
                onChange={e => setInterImpact(e.target.value)}
              />
            </div>
          </div>

          <div className={styles.buttonRow}>
            <Button variant="primary" type="submit">
              Record Intervention
            </Button>
          </div>
        </form>
      </Card>

      {/* Lookup + recent */}
      <Card title="Lookup Note" className={styles.cardSpacing}>
        {lookupError && <div className={styles.error}>{lookupError}</div>}
        <form className={styles.form} onSubmit={handleLookup}>
          <div className={styles.formRow}>
            <div className={styles.formGroup} style={{ flex: 1 }}>
              <label className={styles.label}>Record ID</label>
              <input
                className={styles.input}
                type="text"
                placeholder="rec_01J...ULID"
                value={lookupId}
                onChange={e => setLookupId(e.target.value)}
              />
            </div>
            <div className={styles.formGroup} style={{ alignSelf: 'flex-end' }}>
              <Button variant="secondary" type="submit">
                Fetch
              </Button>
            </div>
          </div>
        </form>
      </Card>

      {records.length > 0 && (
        <Card title={`Recent Notes (${records.length})`}>
          <table className={styles.table}>
            <thead>
              <tr>
                <th className={styles.th}>Record</th>
                <th className={styles.th}>Type</th>
                <th className={styles.th}>Subject</th>
                <th className={styles.th}>Content</th>
                <th className={styles.th}>Recorded By</th>
              </tr>
            </thead>
            <tbody>
              {records.map(record => (
                <tr key={record.record_id}>
                  <td className={styles.tdMono}>{record.record_id}</td>
                  <td className={styles.td}>
                    <Pill tone="neutral">{record.record_type}</Pill>
                  </td>
                  <td className={styles.td}>
                    <span style={{ fontSize: '0.8125rem', color: 'var(--muted)' }}>
                      {formatSubjectRefs(record.subject_refs)}
                    </span>
                  </td>
                  <td className={styles.td}>{record.content}</td>
                  <td className={styles.td}>
                    <span style={{ fontSize: '0.75rem', color: 'var(--muted)' }}>
                      [{record.recorded_by_kind}]
                    </span>{' '}
                    {record.recorded_by_id}
                    <div style={{ fontSize: '0.75rem', color: 'var(--muted)' }}>
                      {new Date(record.recorded_at).toLocaleString()}
                    </div>
                  </td>
                </tr>
              ))}
            </tbody>
          </table>
        </Card>
      )}
    </div>
  );
}

export default Notes;
