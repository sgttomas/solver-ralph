/**
 * Intake Create Page
 *
 * Form to create a new intake draft.
 * Per SR-WORK-SURFACE §3.1, all required fields must be provided.
 */

import { useState } from 'react';
import { Link, useNavigate } from 'react-router-dom';
import { useAuth } from '../auth/AuthProvider';
import config from '../config';
import { Card, Button } from '../ui';
import { ArrayStringEditor } from '../components/ArrayStringEditor';
import { DeliverablesEditor, Deliverable } from '../components/DeliverablesEditor';
import { DefinitionsEditor } from '../components/DefinitionsEditor';
import { InputsEditor, TypedRefInput } from '../components/InputsEditor';
import styles from '../styles/pages.module.css';

const WORK_KINDS = [
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

export function IntakeCreate(): JSX.Element {
  const auth = useAuth();
  const navigate = useNavigate();

  // Form state
  const [workUnitId, setWorkUnitId] = useState('');
  const [title, setTitle] = useState('');
  const [kind, setKind] = useState('research_memo');
  const [objective, setObjective] = useState('');
  const [audience, setAudience] = useState('');
  const [deliverables, setDeliverables] = useState<Deliverable[]>([]);
  const [constraints, setConstraints] = useState<string[]>([]);
  const [definitions, setDefinitions] = useState<Record<string, string>>({});
  const [inputs, setInputs] = useState<TypedRefInput[]>([]);
  const [unknowns, setUnknowns] = useState<string[]>([]);
  const [completionCriteria, setCompletionCriteria] = useState<string[]>([]);

  // Submission state
  const [submitting, setSubmitting] = useState(false);
  const [error, setError] = useState<string | null>(null);

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault();

    // Validation
    if (!workUnitId.trim()) {
      setError('Work Unit ID is required');
      return;
    }
    if (!title.trim()) {
      setError('Title is required');
      return;
    }
    if (!objective.trim()) {
      setError('Objective is required');
      return;
    }
    if (!audience.trim()) {
      setError('Audience is required');
      return;
    }
    if (deliverables.length === 0) {
      setError('At least one deliverable is required');
      return;
    }

    if (!auth.user?.access_token) {
      setError('Not authenticated');
      return;
    }

    setSubmitting(true);
    setError(null);

    try {
      const payload = {
        work_unit_id: workUnitId.trim(),
        title: title.trim(),
        kind,
        objective: objective.trim(),
        audience: audience.trim(),
        deliverables: deliverables.map((d) => ({
          name: d.name,
          format: d.format,
          path: d.path,
          description: d.description || undefined,
        })),
        constraints,
        definitions,
        inputs: inputs.map((i) => ({
          kind: i.kind,
          id: i.id,
          rel: i.rel,
          meta: i.meta || {},
          label: i.label || undefined,
        })),
        unknowns,
        completion_criteria: completionCriteria,
      };

      const res = await fetch(`${config.apiUrl}/api/v1/intakes`, {
        method: 'POST',
        headers: {
          Authorization: `Bearer ${auth.user.access_token}`,
          'Content-Type': 'application/json',
        },
        body: JSON.stringify(payload),
      });

      if (!res.ok) {
        const errorData = await res.json().catch(() => ({}));
        throw new Error(errorData.message || `Failed to create intake: HTTP ${res.status}`);
      }

      const data = await res.json();
      navigate(`/intakes/${data.intake_id}`);
    } catch (err) {
      setError(err instanceof Error ? err.message : 'Failed to create intake');
    } finally {
      setSubmitting(false);
    }
  };

  return (
    <div className={styles.container}>
      {/* Breadcrumb */}
      <div className={styles.breadcrumb}>
        <Link to="/intakes" className={styles.breadcrumbLink}>
          Intakes
        </Link>
        <span className={styles.breadcrumbSeparator}>/</span>
        <span>New</span>
      </div>

      <div className={styles.header}>
        <div className={styles.headerStart}>
          <h1 className={styles.title}>New Intake</h1>
          <p className={styles.subtitle}>Create a new work unit specification</p>
        </div>
      </div>

      {error && <div className={styles.error}>{error}</div>}

      <form onSubmit={handleSubmit}>
        {/* Basic Info Card */}
        <Card title="Basic Information" className={styles.cardSpacing}>
          <div className={styles.form}>
            <div className={styles.formRow}>
              <div className={styles.formGroup}>
                <label className={styles.label}>Work Unit ID *</label>
                <input
                  type="text"
                  value={workUnitId}
                  onChange={(e) => setWorkUnitId(e.target.value)}
                  placeholder="e.g., wu:research-rate-limiting"
                  className={styles.input}
                  required
                />
              </div>
              <div className={styles.formGroup}>
                <label className={styles.label}>Kind *</label>
                <select
                  value={kind}
                  onChange={(e) => setKind(e.target.value)}
                  className={styles.select}
                  required
                >
                  {WORK_KINDS.map((k) => (
                    <option key={k.value} value={k.value}>
                      {k.label}
                    </option>
                  ))}
                </select>
              </div>
            </div>

            <div className={styles.formGroup}>
              <label className={styles.label}>Title *</label>
              <input
                type="text"
                value={title}
                onChange={(e) => setTitle(e.target.value)}
                placeholder="e.g., API Rate Limiting Analysis"
                className={styles.input}
                required
              />
            </div>

            <div className={styles.formGroup}>
              <label className={styles.label}>Objective *</label>
              <textarea
                value={objective}
                onChange={(e) => setObjective(e.target.value)}
                placeholder="ONE sentence describing the objective..."
                className={styles.textarea}
                rows={2}
                required
              />
              <p style={{ fontSize: '0.75rem', color: 'var(--muted)', margin: 'var(--space1) 0 0' }}>
                Per SR-WORK-SURFACE: Should be one sentence.
              </p>
            </div>

            <div className={styles.formGroup}>
              <label className={styles.label}>Audience *</label>
              <input
                type="text"
                value={audience}
                onChange={(e) => setAudience(e.target.value)}
                placeholder="e.g., Engineering team"
                className={styles.input}
                required
              />
            </div>
          </div>
        </Card>

        {/* Deliverables Card */}
        <Card title="Deliverables *" className={styles.cardSpacing}>
          <DeliverablesEditor value={deliverables} onChange={setDeliverables} />
        </Card>

        {/* Constraints Card */}
        <Card title="Constraints" className={styles.cardSpacing}>
          <ArrayStringEditor
            value={constraints}
            onChange={setConstraints}
            placeholder="e.g., Maximum 2000 words"
            addButtonLabel="Add Constraint"
          />
          <p style={{ fontSize: '0.75rem', color: 'var(--muted)', marginTop: 'var(--space2)' }}>
            Length limits, tone requirements, required sections, prohibited content, etc.
          </p>
        </Card>

        {/* Definitions Card */}
        <Card title="Definitions" className={styles.cardSpacing}>
          <DefinitionsEditor value={definitions} onChange={setDefinitions} />
        </Card>

        {/* Inputs Card */}
        <Card title="Input References" className={styles.cardSpacing}>
          <InputsEditor value={inputs} onChange={setInputs} />
        </Card>

        {/* Unknowns Card */}
        <Card title="Unknowns" className={styles.cardSpacing}>
          <ArrayStringEditor
            value={unknowns}
            onChange={setUnknowns}
            placeholder="e.g., What time horizon matters?"
            addButtonLabel="Add Unknown"
          />
          <p style={{ fontSize: '0.75rem', color: 'var(--muted)', marginTop: 'var(--space2)' }}>
            Questions to resolve during the work.
          </p>
        </Card>

        {/* Completion Criteria Card */}
        <Card title="Completion Criteria" className={styles.cardSpacing}>
          <ArrayStringEditor
            value={completionCriteria}
            onChange={setCompletionCriteria}
            placeholder="e.g., Reviewer can act on recommendation"
            addButtonLabel="Add Criterion"
          />
          <p style={{ fontSize: '0.75rem', color: 'var(--muted)', marginTop: 'var(--space2)' }}>
            Human-facing acceptance criteria (not the authoritative gate).
          </p>
        </Card>

        {/* Submit Buttons */}
        <div className={styles.buttonRow}>
          <Link to="/intakes">
            <Button type="button" variant="ghost">
              Cancel
            </Button>
          </Link>
          <Button type="submit" variant="primary" disabled={submitting}>
            {submitting ? 'Creating...' : 'Create Intake'}
          </Button>
        </div>
      </form>
    </div>
  );
}

export default IntakeCreate;
