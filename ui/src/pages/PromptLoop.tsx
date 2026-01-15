/**
 * Task Page
 *
 * Minimal UI to call POST /api/v1/prompt-loop/stream with SSE streaming.
 * Uses OIDC token when available; in dev bypass mode a placeholder token is sent.
 */

import { FormEvent, useCallback, useMemo, useRef, useState } from 'react';
import { useAuth } from '../auth/AuthProvider';
import config from '../config';
import { Card, Button } from '../ui';
import styles from '../styles/pages.module.css';

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
    <div className={styles.container}>
      {/* Add keyframes for pulse animation */}
      <style>{`
        @keyframes pulse {
          0%, 100% { opacity: 1; }
          50% { opacity: 0.4; }
        }
      `}</style>

      <div className={styles.header}>
        <h1 className={styles.title}>
          Task
          {streaming && (
            <span style={{
              display: 'inline-block',
              width: '8px',
              height: '8px',
              borderRadius: '50%',
              background: 'var(--accent)',
              marginLeft: 'var(--space2)',
              animation: 'pulse 1s infinite',
            }} />
          )}
        </h1>
      </div>
      <p style={{ color: 'var(--muted)', marginBottom: 'var(--space5)' }}>
        Materialize a governed work surface from a free-form prompt, run a task iteration, and capture candidate + artifacts with streaming output.
      </p>

      <Card className={styles.cardSpacing}>
        <form onSubmit={submit} className={styles.form}>
          <div className={styles.formGroup}>
            <label className={styles.label}>Prompt *</label>
            <textarea
              className={styles.textarea}
              placeholder="What do you want the task to answer?"
              value={form.prompt}
              onChange={handleChange('prompt')}
              disabled={loading}
              style={{ minHeight: '120px' }}
            />
          </div>

          <div className={styles.formRow}>
            <div className={styles.formGroup}>
              <label className={styles.label}>Procedure Template ID (optional)</label>
              <input
                className={styles.input}
                placeholder="e.g., PROBLEM-STATEMENT-INGESTION"
                value={form.procedure_template_id}
                onChange={handleChange('procedure_template_id')}
                disabled={loading}
              />
              <div style={{ display: 'flex', gap: 'var(--space2)', marginTop: 'var(--space2)', flexWrap: 'wrap' }}>
                <button
                  type="button"
                  style={{
                    padding: '0.35rem 0.75rem',
                    borderRadius: '999px',
                    border: '1px solid var(--border)',
                    background: 'var(--paper)',
                    fontSize: '0.75rem',
                    cursor: 'pointer',
                    color: 'var(--muted)',
                  }}
                  onClick={() => setForm(f => ({ ...f, procedure_template_id: 'PROBLEM-STATEMENT-INGESTION' }))}
                  disabled={loading}
                >
                  PROBLEM-STATEMENT-INGESTION
                </button>
                <button
                  type="button"
                  style={{
                    padding: '0.35rem 0.75rem',
                    borderRadius: '999px',
                    border: '1px solid var(--border)',
                    background: 'var(--paper)',
                    fontSize: '0.75rem',
                    cursor: 'pointer',
                    color: 'var(--muted)',
                  }}
                  onClick={() => setForm(f => ({ ...f, procedure_template_id: 'GENERIC-KNOWLEDGE-WORK' }))}
                  disabled={loading}
                >
                  GENERIC-KNOWLEDGE-WORK
                </button>
              </div>
            </div>

            <div className={styles.formGroup}>
              <label className={styles.label}>Stage ID (optional)</label>
              <input
                className={styles.input}
                placeholder="e.g., stage:FRAME"
                value={form.stage_id}
                onChange={handleChange('stage_id')}
                disabled={loading}
              />
            </div>
          </div>

          <div className={styles.formRow}>
            <div className={styles.formGroup}>
              <label className={styles.label}>Oracle Suite ID (optional)</label>
              <input
                className={styles.input}
                placeholder="e.g., suite:SR-SUITE-GOV"
                value={form.oracle_suite_id}
                onChange={handleChange('oracle_suite_id')}
                disabled={loading}
              />
            </div>

            <div className={styles.formGroup}>
              <label className={styles.label}>Model (optional)</label>
              <input
                className={styles.input}
                placeholder="e.g., gpt-4o-mini"
                value={form.model}
                onChange={handleChange('model')}
                disabled={loading}
              />
            </div>
          </div>

          <div className={styles.formGroup}>
            <label className={styles.label}>Work Unit Name (optional)</label>
            <input
              className={styles.input}
              placeholder="Custom work unit identifier"
              value={form.work_unit}
              onChange={handleChange('work_unit')}
              disabled={loading}
            />
          </div>

          {error && <div className={styles.error}>Error: {error}</div>}
          {loading && !streaming && (
            <div className={styles.note}>Initializing task...</div>
          )}

          <div className={styles.buttonRow}>
            <Button variant="ghost" type="button" onClick={handleReset} disabled={loading && !streaming}>
              {streaming ? 'Cancel' : 'Reset'}
            </Button>
            <Button variant="primary" type="submit" disabled={loading}>
              {loading ? 'Running...' : 'Run Task'}
            </Button>
          </div>
        </form>
      </Card>

      {(artifacts || streamedOutput) && (
        <Card title={streaming ? 'Streaming Output' : 'Artifacts'}>
          {artifacts && (
            <div className={styles.statsGrid} style={{ marginBottom: 'var(--space4)' }}>
              <div className={styles.stat}>
                <div className={styles.statLabel}>Task</div>
                <code className={styles.mono} style={{ fontSize: '0.7rem' }}>{artifacts.loop_id}</code>
              </div>
              <div className={styles.stat}>
                <div className={styles.statLabel}>Iteration</div>
                <code className={styles.mono} style={{ fontSize: '0.7rem' }}>{artifacts.iteration_id}</code>
              </div>
              {artifacts.candidate_id && (
                <div className={styles.stat}>
                  <div className={styles.statLabel}>Candidate</div>
                  <code className={styles.mono} style={{ fontSize: '0.7rem' }}>{artifacts.candidate_id}</code>
                </div>
              )}
              {artifacts.run_id && (
                <div className={styles.stat}>
                  <div className={styles.statLabel}>Run</div>
                  <code className={styles.mono} style={{ fontSize: '0.7rem' }}>{artifacts.run_id}</code>
                </div>
              )}
              {artifacts.evidence_content_hash && (
                <div className={styles.stat}>
                  <div className={styles.statLabel}>Artifact Hash</div>
                  <code className={styles.mono} style={{ fontSize: '0.7rem' }}>{artifacts.evidence_content_hash}</code>
                </div>
              )}
              <div className={styles.stat}>
                <div className={styles.statLabel}>Work Surface Hash</div>
                <code className={styles.mono} style={{ fontSize: '0.7rem' }}>{artifacts.work_surface_hash}</code>
              </div>
              <div className={styles.stat}>
                <div className={styles.statLabel}>Oracle Suite Hash</div>
                <code className={styles.mono} style={{ fontSize: '0.7rem' }}>{artifacts.oracle_suite_hash}</code>
              </div>
            </div>
          )}

          <div
            ref={outputRef}
            style={{
              border: '1px solid var(--border)',
              borderRadius: 'var(--radiusSm)',
              background: 'var(--paper)',
              padding: 'var(--space3)',
              whiteSpace: 'pre-wrap',
              fontFamily: 'var(--mono)',
              fontSize: '0.875rem',
              color: 'var(--ink)',
              minHeight: '100px',
              maxHeight: '400px',
              overflow: 'auto',
            }}
          >
            <strong>LLM Output {streaming && <span style={{ color: 'var(--accent)' }}>(streaming...)</span>}</strong>
            <div style={{ height: '8px' }} />
            {streamedOutput || (streaming ? 'Waiting for response...' : '')}
            {streaming && <span style={{ animation: 'pulse 1s infinite' }}>|</span>}
          </div>
        </Card>
      )}
    </div>
  );
}

export default PromptLoop;
