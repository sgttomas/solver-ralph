/**
 * Prompt Loop Page
 *
 * Minimal UI to call POST /api/v1/prompt-loop/stream with SSE streaming.
 * Uses OIDC token when available; in dev bypass mode a placeholder token is sent.
 */

import { FormEvent, useCallback, useMemo, useRef, useState } from 'react';
import { useAuth } from '../auth/AuthProvider';
import config from '../config';

type StreamStartEvent = {
  type: 'start';
  loop_id: string;
  iteration_id: string;
  work_surface_hash: string;
  oracle_suite_hash: string;
};

type StreamChunkEvent = {
  type: 'chunk';
  content: string;
};

type StreamDoneEvent = {
  type: 'done';
  candidate_id: string;
  run_id: string;
  evidence_content_hash: string;
  llm_output: string;
};

type StreamErrorEvent = {
  type: 'error';
  message: string;
};

type StreamEvent = StreamStartEvent | StreamChunkEvent | StreamDoneEvent | StreamErrorEvent;

type ArtifactInfo = {
  loop_id: string;
  iteration_id: string;
  work_surface_hash: string;
  oracle_suite_hash: string;
  candidate_id?: string;
  run_id?: string;
  evidence_content_hash?: string;
};

type FormState = {
  prompt: string;
  procedure_template_id: string;
  stage_id: string;
  oracle_suite_id: string;
  model: string;
  work_unit: string;
};

const defaultForm: FormState = {
  prompt: '',
  procedure_template_id: '',
  stage_id: '',
  oracle_suite_id: '',
  model: '',
  work_unit: '',
};

const styles = {
  page: {
    maxWidth: '960px',
    margin: '0 auto',
    display: 'flex',
    flexDirection: 'column' as const,
    gap: '1rem',
  },
  card: {
    background: '#fff',
    borderRadius: '12px',
    padding: '1.5rem',
    boxShadow: '0 12px 28px rgba(26, 26, 46, 0.08)',
    border: '1px solid #eceff4',
  },
  titleRow: {
    display: 'flex',
    alignItems: 'center',
    justifyContent: 'space-between',
    marginBottom: '0.75rem',
  },
  title: {
    fontSize: '1.4rem',
    fontWeight: 700,
    color: '#1a1a2e',
    margin: 0,
  },
  subtitle: {
    margin: '0 0 1rem 0',
    color: '#555',
    fontSize: '0.95rem',
  },
  formGrid: {
    display: 'grid',
    gridTemplateColumns: '1fr 1fr',
    gap: '1rem',
  },
  field: {
    display: 'flex',
    flexDirection: 'column' as const,
    gap: '0.35rem',
  },
  label: {
    fontWeight: 600,
    fontSize: '0.9rem',
    color: '#2f2f40',
  },
  textarea: {
    minHeight: '120px',
    padding: '0.75rem',
    borderRadius: '10px',
    border: '1px solid #dfe3eb',
    fontSize: '0.95rem',
    resize: 'vertical' as const,
  },
  input: {
    padding: '0.65rem 0.75rem',
    borderRadius: '10px',
    border: '1px solid #dfe3eb',
    fontSize: '0.95rem',
  },
  pillRow: {
    display: 'flex',
    gap: '0.5rem',
    flexWrap: 'wrap' as const,
    marginTop: '0.5rem',
  },
  pill: {
    padding: '0.35rem 0.75rem',
    borderRadius: '999px',
    border: '1px solid #dfe3eb',
    background: '#f7f8fb',
    fontSize: '0.85rem',
    cursor: 'pointer',
  },
  actions: {
    display: 'flex',
    justifyContent: 'flex-end',
    gap: '0.75rem',
    marginTop: '1rem',
  },
  buttonPrimary: {
    padding: '0.75rem 1.25rem',
    borderRadius: '10px',
    border: 'none',
    background: 'linear-gradient(135deg, #d68b4b, #b06b35)',
    color: '#fff',
    fontWeight: 700,
    fontSize: '0.95rem',
    cursor: 'pointer',
  },
  buttonGhost: {
    padding: '0.75rem 1rem',
    borderRadius: '10px',
    border: '1px solid #dfe3eb',
    background: '#fff',
    color: '#2f2f40',
    fontWeight: 600,
    cursor: 'pointer',
  },
  status: {
    padding: '0.5rem 0.75rem',
    borderRadius: '8px',
    background: '#fef6ec',
    border: '1px solid #f3dbc0',
    color: '#8a5a2f',
    fontSize: '0.9rem',
    marginTop: '0.5rem',
  },
  resultGrid: {
    display: 'grid',
    gridTemplateColumns: 'repeat(auto-fit, minmax(220px, 1fr))',
    gap: '0.75rem',
    marginTop: '1rem',
  },
  resultCard: {
    border: '1px solid #eceff4',
    borderRadius: '10px',
    padding: '0.75rem',
    background: '#f9fafc',
  },
  code: {
    fontFamily: 'Menlo, Consolas, monospace',
    fontSize: '0.85rem',
    wordBreak: 'break-all' as const,
    margin: 0,
  },
  outputBox: {
    marginTop: '1rem',
    border: '1px solid #eceff4',
    borderRadius: '10px',
    background: '#fdfdfd',
    padding: '0.75rem',
    whiteSpace: 'pre-wrap' as const,
    fontFamily: 'Menlo, Consolas, monospace',
    fontSize: '0.9rem',
    color: '#2c2c2c',
    minHeight: '100px',
    maxHeight: '400px',
    overflow: 'auto',
  },
  streamingIndicator: {
    display: 'inline-block',
    width: '8px',
    height: '8px',
    borderRadius: '50%',
    background: '#d68b4b',
    marginLeft: '0.5rem',
    animation: 'pulse 1s infinite',
  },
};

export function PromptLoop(): JSX.Element {
  const auth = useAuth();
  const [form, setForm] = useState<FormState>(defaultForm);
  const [loading, setLoading] = useState(false);
  const [streaming, setStreaming] = useState(false);
  const [error, setError] = useState<string | null>(null);
  const [artifacts, setArtifacts] = useState<ArtifactInfo | null>(null);
  const [streamedOutput, setStreamedOutput] = useState('');
  const abortControllerRef = useRef<AbortController | null>(null);
  const outputRef = useRef<HTMLDivElement>(null);

  const token = useMemo(() => {
    if (config.devAuthBypass) return 'dev-bypass';
    return auth.user?.access_token ?? '';
  }, [auth.user]);

  const handleChange = (field: keyof FormState) => (e: React.ChangeEvent<HTMLInputElement | HTMLTextAreaElement>) => {
    setForm(prev => ({ ...prev, [field]: e.target.value }));
  };

  const handleReset = useCallback(() => {
    if (abortControllerRef.current) {
      abortControllerRef.current.abort();
    }
    setForm(defaultForm);
    setError(null);
    setArtifacts(null);
    setStreamedOutput('');
    setLoading(false);
    setStreaming(false);
  }, []);

  const submit = useCallback(async (e: FormEvent) => {
    e.preventDefault();
    setError(null);
    setArtifacts(null);
    setStreamedOutput('');

    if (!form.prompt.trim()) {
      setError('Prompt is required.');
      return;
    }
    if (!token) {
      setError('No auth token available. Log in first.');
      return;
    }

    const payload: Record<string, string> = { prompt: form.prompt };
    if (form.procedure_template_id) payload.procedure_template_id = form.procedure_template_id;
    if (form.stage_id) payload.stage_id = form.stage_id;
    if (form.oracle_suite_id) payload.oracle_suite_id = form.oracle_suite_id;
    if (form.model) payload.model = form.model;
    if (form.work_unit) payload.work_unit = form.work_unit;

    abortControllerRef.current = new AbortController();
    setLoading(true);

    try {
      const res = await fetch(`${config.apiUrl}/api/v1/prompt-loop/stream`, {
        method: 'POST',
        headers: {
          Authorization: `Bearer ${token}`,
          'Content-Type': 'application/json',
          Accept: 'text/event-stream',
        },
        body: JSON.stringify(payload),
        signal: abortControllerRef.current.signal,
      });

      if (!res.ok) {
        const text = await res.text();
        throw new Error(`HTTP ${res.status}: ${text}`);
      }

      if (!res.body) {
        throw new Error('No response body');
      }

      setStreaming(true);
      const reader = res.body.getReader();
      const decoder = new TextDecoder();
      let buffer = '';

      // eslint-disable-next-line no-constant-condition
      while (true) {
        const { done, value } = await reader.read();
        if (done) break;

        buffer += decoder.decode(value, { stream: true });

        // Process complete SSE messages
        const lines = buffer.split('\n');
        buffer = lines.pop() || ''; // Keep incomplete line in buffer

        for (const line of lines) {
          if (line.startsWith('data: ')) {
            const jsonStr = line.slice(6);
            try {
              const event = JSON.parse(jsonStr) as StreamEvent;

              switch (event.type) {
                case 'start':
                  setArtifacts({
                    loop_id: event.loop_id,
                    iteration_id: event.iteration_id,
                    work_surface_hash: event.work_surface_hash,
                    oracle_suite_hash: event.oracle_suite_hash,
                  });
                  break;

                case 'chunk':
                  setStreamedOutput(prev => prev + event.content);
                  // Auto-scroll to bottom
                  if (outputRef.current) {
                    outputRef.current.scrollTop = outputRef.current.scrollHeight;
                  }
                  break;

                case 'done':
                  setArtifacts(prev => prev ? {
                    ...prev,
                    candidate_id: event.candidate_id,
                    run_id: event.run_id,
                    evidence_content_hash: event.evidence_content_hash,
                  } : null);
                  setStreaming(false);
                  break;

                case 'error':
                  setError(event.message);
                  setStreaming(false);
                  break;
              }
            } catch {
              // Ignore JSON parse errors for incomplete messages
            }
          }
        }
      }
    } catch (err) {
      if (err instanceof Error && err.name === 'AbortError') {
        // Request was cancelled
        return;
      }
      setError(err instanceof Error ? err.message : 'Request failed');
    } finally {
      setLoading(false);
      setStreaming(false);
    }
  }, [form, token]);

  return (
    <div style={styles.page}>
      {/* Add keyframes for pulse animation */}
      <style>{`
        @keyframes pulse {
          0%, 100% { opacity: 1; }
          50% { opacity: 0.4; }
        }
      `}</style>

      <div style={styles.card}>
        <div style={styles.titleRow}>
          <h1 style={styles.title}>
            Prompt Loop
            {streaming && <span style={styles.streamingIndicator} />}
          </h1>
        </div>
        <p style={styles.subtitle}>
          Materialize a governed work surface from a free-form prompt, run a loop/iteration, and capture candidate + evidence with streaming output.
        </p>

        <form onSubmit={submit}>
          <div style={{ ...styles.field, marginBottom: '1rem' }}>
            <label style={styles.label}>Prompt *</label>
            <textarea
              style={styles.textarea}
              placeholder="What do you want the loop to answer?"
              value={form.prompt}
              onChange={handleChange('prompt')}
              disabled={loading}
            />
          </div>

          <div style={styles.formGrid}>
            <div style={styles.field}>
              <label style={styles.label}>Procedure Template ID (optional)</label>
              <input
                style={styles.input}
                placeholder="e.g., PROBLEM-STATEMENT-INGESTION"
                value={form.procedure_template_id}
                onChange={handleChange('procedure_template_id')}
                disabled={loading}
              />
              <div style={styles.pillRow}>
                <button
                  type="button"
                  style={styles.pill}
                  onClick={() => setForm(f => ({ ...f, procedure_template_id: 'PROBLEM-STATEMENT-INGESTION' }))}
                  disabled={loading}
                >
                  PROBLEM-STATEMENT-INGESTION
                </button>
                <button
                  type="button"
                  style={styles.pill}
                  onClick={() => setForm(f => ({ ...f, procedure_template_id: 'GENERIC-KNOWLEDGE-WORK' }))}
                  disabled={loading}
                >
                  GENERIC-KNOWLEDGE-WORK
                </button>
              </div>
            </div>

            <div style={styles.field}>
              <label style={styles.label}>Stage ID (optional)</label>
              <input
                style={styles.input}
                placeholder="e.g., stage:FRAME"
                value={form.stage_id}
                onChange={handleChange('stage_id')}
                disabled={loading}
              />
            </div>

            <div style={styles.field}>
              <label style={styles.label}>Oracle Suite ID (optional)</label>
              <input
                style={styles.input}
                placeholder="e.g., suite:SR-SUITE-GOV"
                value={form.oracle_suite_id}
                onChange={handleChange('oracle_suite_id')}
                disabled={loading}
              />
            </div>

            <div style={styles.field}>
              <label style={styles.label}>Model (optional)</label>
              <input
                style={styles.input}
                placeholder="e.g., gpt-4o-mini"
                value={form.model}
                onChange={handleChange('model')}
                disabled={loading}
              />
            </div>

            <div style={styles.field}>
              <label style={styles.label}>Work Unit Name (optional)</label>
              <input
                style={styles.input}
                placeholder="Custom work unit identifier"
                value={form.work_unit}
                onChange={handleChange('work_unit')}
                disabled={loading}
              />
            </div>
          </div>

          {error && <div style={styles.status}>Error: {error}</div>}
          {loading && !streaming && <div style={styles.status}>Initializing prompt loop...</div>}

          <div style={styles.actions}>
            <button type="button" style={styles.buttonGhost} onClick={handleReset} disabled={loading && !streaming}>
              {streaming ? 'Cancel' : 'Reset'}
            </button>
            <button type="submit" style={styles.buttonPrimary} disabled={loading}>
              {loading ? 'Running…' : 'Run Prompt Loop'}
            </button>
          </div>
        </form>
      </div>

      {(artifacts || streamedOutput) && (
        <div style={styles.card}>
          <h2 style={{ ...styles.title, fontSize: '1.1rem', marginBottom: '0.5rem' }}>
            {streaming ? 'Streaming Output' : 'Artifacts'}
          </h2>

          {artifacts && (
            <div style={styles.resultGrid}>
              <div style={styles.resultCard}>
                <strong>Loop</strong>
                <p style={styles.code}>{artifacts.loop_id}</p>
              </div>
              <div style={styles.resultCard}>
                <strong>Iteration</strong>
                <p style={styles.code}>{artifacts.iteration_id}</p>
              </div>
              {artifacts.candidate_id && (
                <div style={styles.resultCard}>
                  <strong>Candidate</strong>
                  <p style={styles.code}>{artifacts.candidate_id}</p>
                </div>
              )}
              {artifacts.run_id && (
                <div style={styles.resultCard}>
                  <strong>Run</strong>
                  <p style={styles.code}>{artifacts.run_id}</p>
                </div>
              )}
              {artifacts.evidence_content_hash && (
                <div style={styles.resultCard}>
                  <strong>Evidence Hash</strong>
                  <p style={styles.code}>{artifacts.evidence_content_hash}</p>
                </div>
              )}
              <div style={styles.resultCard}>
                <strong>Work Surface Hash</strong>
                <p style={styles.code}>{artifacts.work_surface_hash}</p>
              </div>
              <div style={styles.resultCard}>
                <strong>Oracle Suite Hash</strong>
                <p style={styles.code}>{artifacts.oracle_suite_hash}</p>
              </div>
            </div>
          )}

          <div ref={outputRef} style={styles.outputBox}>
            <strong>LLM Output {streaming && <span style={{ color: '#d68b4b' }}>(streaming...)</span>}</strong>
            <div style={{ height: '8px' }} />
            {streamedOutput || (streaming ? 'Waiting for response...' : '')}
            {streaming && <span style={{ animation: 'pulse 1s infinite' }}>▌</span>}
          </div>
        </div>
      )}
    </div>
  );
}

export default PromptLoop;
