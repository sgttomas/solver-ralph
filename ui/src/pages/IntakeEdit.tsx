/**
 * Intake Edit Page
 *
 * Form to edit an existing draft intake.
 * Only draft intakes can be edited - active/archived are immutable.
 */

import { useState, useEffect, useCallback } from 'react';
import { useParams, Link, useNavigate } from 'react-router-dom';
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

interface IntakeData {
  intake_id: string;
  work_unit_id: string;
  title: string;
  kind: string;
  objective: string;
  audience: string;
  deliverables: Deliverable[];
  constraints: string[];
  definitions: Record<string, string>;
  inputs: TypedRefInput[];
  unknowns: string[];
  completion_criteria: string[];
  status: 'draft' | 'active' | 'archived';
}

export function IntakeEdit(): JSX.Element {
  const { intakeId } = useParams<{ intakeId: string }>();
  const auth = useAuth();
  const navigate = useNavigate();

  // Loading state
  const [loading, setLoading] = useState(true);
  const [loadError, setLoadError] = useState<string | null>(null);

  // Form state
  const [title, setTitle] = useState('');
  const [objective, setObjective] = useState('');
  const [audience, setAudience] = useState('');
  const [deliverables, setDeliverables] = useState<Deliverable[]>([]);
  const [constraints, setConstraints] = useState<string[]>([]);
  const [definitions, setDefinitions] = useState<Record<string, string>>({});
  const [inputs, setInputs] = useState<TypedRefInput[]>([]);
  const [unknowns, setUnknowns] = useState<string[]>([]);
  const [completionCriteria, setCompletionCriteria] = useState<string[]>([]);

  // Read-only display fields
  const [workUnitId, setWorkUnitId] = useState('');
  const [kind, setKind] = useState('');

  // Submission state
  const [submitting, setSubmitting] = useState(false);
  const [error, setError] = useState<string | null>(null);

  const fetchIntake = useCallback(async () => {
    if (!auth.user?.access_token || !intakeId) return;

    setLoading(true);
    setLoadError(null);

    try {
      const res = await fetch(`${config.apiUrl}/api/v1/intakes/${intakeId}`, {
        headers: { Authorization: `Bearer ${auth.user.access_token}` },
      });

      if (res.status === 404) {
        throw new Error('Intake not found');
      }
      if (!res.ok) {
        throw new Error(`HTTP ${res.status}`);
      }

      const data: IntakeData = await res.json();

      // Check if intake is editable
      if (data.status !== 'draft') {
        navigate(`/intakes/${intakeId}`, { replace: true });
        return;
      }

      // Populate form fields
      setWorkUnitId(data.work_unit_id);
      setKind(data.kind);
      setTitle(data.title);
      setObjective(data.objective);
      setAudience(data.audience);
      setDeliverables(data.deliverables || []);
      setConstraints(data.constraints || []);
      setDefinitions(data.definitions || {});
      setInputs(data.inputs || []);
      setUnknowns(data.unknowns || []);
      setCompletionCriteria(data.completion_criteria || []);
    } catch (err) {
      setLoadError(err instanceof Error ? err.message : 'Failed to load intake');
    } finally {
      setLoading(false);
    }
  }, [auth.user?.access_token, intakeId, navigate]);

  useEffect(() => {
    fetchIntake();
  }, [fetchIntake]);

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault();

    // Validation
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

    if (!auth.user?.access_token || !intakeId) {
      setError('Not authenticated');
      return;
    }

    setSubmitting(true);
    setError(null);

    try {
      const payload = {
        title: title.trim(),
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

      const res = await fetch(`${config.apiUrl}/api/v1/intakes/${intakeId}`, {
        method: 'PUT',
        headers: {
          Authorization: `Bearer ${auth.user.access_token}`,
          'Content-Type': 'application/json',
        },
        body: JSON.stringify(payload),
      });

      if (!res.ok) {
        const errorData = await res.json().catch(() => ({}));
        throw new Error(errorData.message || `Failed to update intake: HTTP ${res.status}`);
      }

      navigate(`/intakes/${intakeId}`);
    } catch (err) {
      setError(err instanceof Error ? err.message : 'Failed to update intake');
    } finally {
      setSubmitting(false);
    }
  };

  if (loading) {
    return (
      <div className={styles.container}>
        <div className={styles.placeholder}>
          <p className={styles.placeholderText}>Loading intake...</p>
        </div>
      </div>
    );
  }

  if (loadError) {
    return (
      <div className={styles.container}>
        <div className={styles.placeholder}>
          <p className={styles.error}>Error: {loadError}</p>
          <Link to="/intakes" className={styles.link}>
            Back to Intakes
          </Link>
        </div>
      </div>
    );
  }

  return (
    <div className={styles.container}>
      {/* Breadcrumb */}
      <div className={styles.breadcrumb}>
        <Link to="/intakes" className={styles.breadcrumbLink}>
          Intakes
        </Link>
        <span className={styles.breadcrumbSeparator}>/</span>
        <Link to={`/intakes/${intakeId}`} className={styles.breadcrumbLink}>
          {title || intakeId?.slice(0, 16) + '...'}
        </Link>
        <span className={styles.breadcrumbSeparator}>/</span>
        <span>Edit</span>
      </div>

      <div className={styles.header}>
        <div className={styles.headerStart}>
          <h1 className={styles.title}>Edit Intake</h1>
          <p className={styles.subtitle}>{intakeId}</p>
        </div>
      </div>

      {error && <div className={styles.error}>{error}</div>}

      <form onSubmit={handleSubmit}>
        {/* Basic Info Card */}
        <Card title="Basic Information" className={styles.cardSpacing}>
          <div className={styles.form}>
            <div className={styles.formRow}>
              <div className={styles.formGroup}>
                <label className={styles.label}>Work Unit ID</label>
                <input
                  type="text"
                  value={workUnitId}
                  className={styles.input}
                  disabled
                  style={{ backgroundColor: 'var(--surface)', color: 'var(--muted)' }}
                />
                <p style={{ fontSize: '0.75rem', color: 'var(--muted)', margin: 'var(--space1) 0 0' }}>
                  Work Unit ID cannot be changed after creation.
                </p>
              </div>
              <div className={styles.formGroup}>
                <label className={styles.label}>Kind</label>
                <select
                  value={kind}
                  className={styles.select}
                  disabled
                  style={{ backgroundColor: 'var(--surface)', color: 'var(--muted)' }}
                >
                  {WORK_KINDS.map((k) => (
                    <option key={k.value} value={k.value}>
                      {k.label}
                    </option>
                  ))}
                </select>
                <p style={{ fontSize: '0.75rem', color: 'var(--muted)', margin: 'var(--space1) 0 0' }}>
                  Kind cannot be changed after creation.
                </p>
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
          <Link to={`/intakes/${intakeId}`}>
            <Button type="button" variant="ghost">
              Cancel
            </Button>
          </Link>
          <Button type="submit" variant="primary" disabled={submitting}>
            {submitting ? 'Saving...' : 'Save Changes'}
          </Button>
        </div>
      </form>
    </div>
  );
}

export default IntakeEdit;
