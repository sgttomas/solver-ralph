/**
 * Work Surface Composition Wizard
 *
 * 3-step wizard to create a Work Surface by binding an Intake to a Procedure Template.
 * Step 1: Select Intake (only active intakes)
 * Step 2: Select compatible Procedure Template
 * Step 3: Review binding summary and confirm
 */

import { useState, useEffect, useCallback } from 'react';
import { Link, useNavigate } from 'react-router-dom';
import { useAuth } from '../auth/AuthProvider';
import { useToast } from '../components/ToastContext';
import { fetchWithRetry, ApiError } from '../components/ApiErrorHandler';
import config from '../config';
import { Card, Button, Pill } from '../ui';
import styles from '../styles/pages.module.css';

type WizardStep = 'intake' | 'template' | 'review';

interface IntakeSummary {
  intake_id: string;
  work_unit_id: string;
  title: string;
  kind: string;
  status: string;
  objective: string;
  content_hash: string | null;
}

interface TemplateSummary {
  procedure_template_id: string;
  name: string | null;
  description: string | null;
  stages_count: number;
  supported_kinds: string[];
}

interface CompatibleTemplatesResponse {
  intake_id: string;
  intake_kind: string;
  templates: TemplateSummary[];
}

const WORK_KINDS = [
  { value: 'ALL', label: 'All Kinds' },
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

export function WorkSurfaceCompose(): JSX.Element {
  const auth = useAuth();
  const navigate = useNavigate();
  const toast = useToast();

  // Wizard state
  const [step, setStep] = useState<WizardStep>('intake');
  const [selectedIntake, setSelectedIntake] = useState<IntakeSummary | null>(null);
  const [selectedTemplate, setSelectedTemplate] = useState<TemplateSummary | null>(null);

  // Data state
  const [intakes, setIntakes] = useState<IntakeSummary[]>([]);
  const [templates, setTemplates] = useState<TemplateSummary[]>([]);
  const [intakeKind, setIntakeKind] = useState<string>('');

  // UI state
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);
  const [submitting, setSubmitting] = useState(false);
  const [submitProgress, setSubmitProgress] = useState<string | null>(null);
  const [kindFilter, setKindFilter] = useState('ALL');
  const [searchFilter, setSearchFilter] = useState('');

  // Fetch active intakes for step 1
  const fetchIntakes = useCallback(async () => {
    if (!auth.user?.access_token) return;

    setLoading(true);
    setError(null);

    try {
      const params = new URLSearchParams({
        status: 'active',
        page: '1',
        page_size: '100',
      });

      const res = await fetch(`${config.apiUrl}/api/v1/intakes?${params}`, {
        headers: { Authorization: `Bearer ${auth.user.access_token}` },
      });

      if (!res.ok) {
        throw new Error(`HTTP ${res.status}`);
      }

      const data = await res.json();
      setIntakes(data.intakes || []);
    } catch (err) {
      setError(err instanceof Error ? err.message : 'Failed to load intakes');
    } finally {
      setLoading(false);
    }
  }, [auth.user?.access_token]);

  // Fetch compatible templates for step 2
  const fetchCompatibleTemplates = useCallback(async () => {
    if (!auth.user?.access_token || !selectedIntake) return;

    setLoading(true);
    setError(null);

    try {
      const params = new URLSearchParams({
        intake_id: selectedIntake.intake_id,
      });

      const res = await fetch(`${config.apiUrl}/api/v1/work-surfaces/compatible-templates?${params}`, {
        headers: { Authorization: `Bearer ${auth.user.access_token}` },
      });

      if (!res.ok) {
        throw new Error(`HTTP ${res.status}`);
      }

      const data: CompatibleTemplatesResponse = await res.json();
      setTemplates(data.templates || []);
      setIntakeKind(data.intake_kind);
    } catch (err) {
      setError(err instanceof Error ? err.message : 'Failed to load templates');
    } finally {
      setLoading(false);
    }
  }, [auth.user?.access_token, selectedIntake]);

  // Load intakes on mount
  useEffect(() => {
    fetchIntakes();
  }, [fetchIntakes]);

  // Load templates when intake is selected
  useEffect(() => {
    if (step === 'template' && selectedIntake) {
      fetchCompatibleTemplates();
    }
  }, [step, selectedIntake, fetchCompatibleTemplates]);

  const handleIntakeSelect = (intake: IntakeSummary) => {
    setSelectedIntake(intake);
    setSelectedTemplate(null);
    setStep('template');
  };

  const handleTemplateSelect = (template: TemplateSummary) => {
    setSelectedTemplate(template);
    setStep('review');
  };

  const handleBack = () => {
    if (step === 'template') {
      setSelectedTemplate(null);
      setStep('intake');
    } else if (step === 'review') {
      setStep('template');
    }
  };

  const handleSubmit = async () => {
    if (!auth.user?.access_token || !selectedIntake || !selectedTemplate) return;

    setSubmitting(true);
    setError(null);
    setSubmitProgress('Creating Work Surface...');

    try {
      // Step 1: Create Work Surface
      const payload = {
        work_unit_id: selectedIntake.work_unit_id,
        intake_id: selectedIntake.intake_id,
        procedure_template_id: selectedTemplate.procedure_template_id,
        params: {},
      };

      interface CreateResponse {
        work_surface_id: string;
      }

      const data = await fetchWithRetry<CreateResponse>(
        `${config.apiUrl}/api/v1/work-surfaces`,
        {
          method: 'POST',
          headers: {
            Authorization: `Bearer ${auth.user.access_token}`,
            'Content-Type': 'application/json',
          },
          body: JSON.stringify(payload),
        }
      );

      // Step 2: Start work (SR-PLAN-V6 Phase V6-2)
      // Creates Loop, activates it, and starts first Iteration as SYSTEM actor
      setSubmitProgress('Starting work...');

      interface StartResponse {
        loop_id: string;
        iteration_id: string;
        already_started?: boolean;
      }

      try {
        const startData = await fetchWithRetry<StartResponse>(
          `${config.apiUrl}/api/v1/work-surfaces/${data.work_surface_id}/start`,
          {
            method: 'POST',
            headers: {
              Authorization: `Bearer ${auth.user.access_token}`,
            },
          }
        );

        if (startData.already_started) {
          toast.info('Work was already started on this Work Surface');
        } else {
          toast.success('Work Surface created and work started');
        }
      } catch (startErr) {
        // Work Surface was created, but start failed - show warning and continue
        const message = startErr instanceof ApiError
          ? startErr.toUserMessage()
          : 'Work was created but could not be started automatically';
        toast.warning(message);
      }

      navigate(`/work-surfaces/${data.work_surface_id}`);
    } catch (err) {
      const message = err instanceof ApiError
        ? err.toUserMessage()
        : 'Failed to create work surface';
      setError(message);
      toast.error(message);
    } finally {
      setSubmitting(false);
      setSubmitProgress(null);
    }
  };

  // Filter intakes
  const filteredIntakes = intakes.filter((intake) => {
    if (kindFilter !== 'ALL' && intake.kind !== kindFilter) return false;
    if (searchFilter) {
      const search = searchFilter.toLowerCase();
      return (
        intake.title.toLowerCase().includes(search) ||
        intake.objective.toLowerCase().includes(search) ||
        intake.intake_id.toLowerCase().includes(search)
      );
    }
    return true;
  });

  // Step indicator component
  const StepIndicator = () => (
    <div
      style={{
        display: 'flex',
        justifyContent: 'center',
        marginBottom: 'var(--space5)',
        gap: 'var(--space4)',
      }}
    >
      {(['intake', 'template', 'review'] as WizardStep[]).map((s, idx) => {
        const isActive = step === s;
        const isCompleted =
          (s === 'intake' && (step === 'template' || step === 'review')) ||
          (s === 'template' && step === 'review');
        const labels = { intake: '1. Select Intake', template: '2. Select Template', review: '3. Review' };

        return (
          <div key={s} style={{ display: 'flex', alignItems: 'center' }}>
            <div
              style={{
                display: 'flex',
                alignItems: 'center',
                gap: 'var(--space2)',
              }}
            >
              <div
                style={{
                  width: '24px',
                  height: '24px',
                  borderRadius: '50%',
                  display: 'flex',
                  alignItems: 'center',
                  justifyContent: 'center',
                  backgroundColor: isActive ? 'var(--accent)' : isCompleted ? 'var(--success)' : 'var(--bg)',
                  border: `2px solid ${isActive ? 'var(--accent)' : isCompleted ? 'var(--success)' : 'var(--border)'}`,
                  color: isActive || isCompleted ? 'white' : 'var(--muted)',
                  fontSize: '0.75rem',
                  fontWeight: 600,
                }}
              >
                {isCompleted ? '✓' : idx + 1}
              </div>
              <span
                style={{
                  fontSize: '0.875rem',
                  fontWeight: isActive ? 600 : 400,
                  color: isActive ? 'var(--ink)' : 'var(--muted)',
                }}
              >
                {labels[s]}
              </span>
            </div>
          </div>
        );
      })}
    </div>
  );

  return (
    <div className={styles.container}>
      {/* Breadcrumb */}
      <div className={styles.breadcrumb}>
        <Link to="/work-surfaces" className={styles.breadcrumbLink}>
          Work Surfaces
        </Link>
        <span className={styles.breadcrumbSeparator}>/</span>
        <span>New</span>
      </div>

      <div className={styles.header}>
        <div className={styles.headerStart}>
          <h1 className={styles.title}>Create Work Surface</h1>
          <p className={styles.subtitle}>Bind an Intake to a Procedure Template</p>
        </div>
      </div>

      <StepIndicator />

      {error && <div className={styles.error}>{error}</div>}

      {/* Step 1: Select Intake */}
      {step === 'intake' && (
        <Card title="Select Intake" className={styles.cardSpacing}>
          <p style={{ marginBottom: 'var(--space3)', color: 'var(--muted)', fontSize: '0.875rem' }}>
            Choose an active intake to bind. Only active intakes can be bound to a Work Surface.
          </p>

          {/* Filters */}
          <div
            style={{
              display: 'flex',
              gap: 'var(--space3)',
              marginBottom: 'var(--space4)',
              flexWrap: 'wrap',
            }}
          >
            <div className={styles.formGroup} style={{ marginBottom: 0 }}>
              <label className={styles.labelSmall}>Kind</label>
              <select
                value={kindFilter}
                onChange={(e) => setKindFilter(e.target.value)}
                className={styles.select}
              >
                {WORK_KINDS.map((k) => (
                  <option key={k.value} value={k.value}>
                    {k.label}
                  </option>
                ))}
              </select>
            </div>
            <div className={styles.formGroup} style={{ marginBottom: 0, flex: 1, minWidth: '200px' }}>
              <label className={styles.labelSmall}>Search</label>
              <input
                type="text"
                value={searchFilter}
                onChange={(e) => setSearchFilter(e.target.value)}
                placeholder="Search by title, objective, or ID..."
                className={styles.input}
              />
            </div>
          </div>

          {loading ? (
            <div className={styles.placeholder}>
              <p className={styles.placeholderText}>Loading intakes...</p>
            </div>
          ) : filteredIntakes.length === 0 ? (
            <div className={styles.placeholder}>
              <p className={styles.placeholderText}>No active intakes found.</p>
              <p className={styles.placeholderHint}>
                Create and activate an intake first, then return here.
              </p>
              <Link to="/intakes/new">
                <Button variant="primary">Create Intake</Button>
              </Link>
            </div>
          ) : (
            <div style={{ display: 'flex', flexDirection: 'column', gap: 'var(--space2)' }}>
              {filteredIntakes.map((intake) => (
                <div
                  key={intake.intake_id}
                  onClick={() => handleIntakeSelect(intake)}
                  style={{
                    padding: 'var(--space3)',
                    border: '1px solid var(--border)',
                    borderRadius: 'var(--radius)',
                    cursor: 'pointer',
                    backgroundColor:
                      selectedIntake?.intake_id === intake.intake_id ? 'var(--bg)' : 'transparent',
                  }}
                  className={styles.tableRowHover}
                >
                  <div style={{ display: 'flex', justifyContent: 'space-between', alignItems: 'flex-start' }}>
                    <div>
                      <div style={{ fontWeight: 500, marginBottom: 'var(--space1)' }}>{intake.title}</div>
                      <div style={{ fontSize: '0.75rem', color: 'var(--muted)', marginBottom: 'var(--space1)' }}>
                        {intake.objective.length > 100
                          ? intake.objective.slice(0, 100) + '...'
                          : intake.objective}
                      </div>
                      <div style={{ fontSize: '0.75rem', fontFamily: 'var(--mono)', color: 'var(--muted)' }}>
                        {intake.intake_id}
                      </div>
                    </div>
                    <div style={{ display: 'flex', gap: 'var(--space2)' }}>
                      <Pill tone="neutral">{intake.kind.replace(/_/g, ' ')}</Pill>
                    </div>
                  </div>
                </div>
              ))}
            </div>
          )}

          <div className={styles.buttonRow} style={{ marginTop: 'var(--space4)' }}>
            <Link to="/work-surfaces">
              <Button variant="ghost">Cancel</Button>
            </Link>
          </div>
        </Card>
      )}

      {/* Step 2: Select Template */}
      {step === 'template' && selectedIntake && (
        <Card title="Select Procedure Template" className={styles.cardSpacing}>
          <p style={{ marginBottom: 'var(--space3)', color: 'var(--muted)', fontSize: '0.875rem' }}>
            Choose a compatible procedure template for intake kind:{' '}
            <Pill tone="neutral">{intakeKind.replace(/_/g, ' ')}</Pill>
          </p>

          <div
            style={{
              marginBottom: 'var(--space4)',
              padding: 'var(--space3)',
              backgroundColor: 'var(--bg)',
              borderRadius: 'var(--radius)',
            }}
          >
            <div style={{ fontSize: '0.75rem', color: 'var(--muted)', marginBottom: 'var(--space1)' }}>
              Selected Intake:
            </div>
            <div style={{ fontWeight: 500 }}>{selectedIntake.title}</div>
          </div>

          {loading ? (
            <div className={styles.placeholder}>
              <p className={styles.placeholderText}>Loading templates...</p>
            </div>
          ) : templates.length === 0 ? (
            <div className={styles.placeholder}>
              <p className={styles.placeholderText}>No compatible templates found.</p>
              <p className={styles.placeholderHint}>
                No procedure templates support the kind "{intakeKind}".
              </p>
            </div>
          ) : (
            <div style={{ display: 'flex', flexDirection: 'column', gap: 'var(--space2)' }}>
              {templates.map((template) => (
                <div
                  key={template.procedure_template_id}
                  onClick={() => handleTemplateSelect(template)}
                  style={{
                    padding: 'var(--space3)',
                    border: '1px solid var(--border)',
                    borderRadius: 'var(--radius)',
                    cursor: 'pointer',
                    backgroundColor:
                      selectedTemplate?.procedure_template_id === template.procedure_template_id
                        ? 'var(--bg)'
                        : 'transparent',
                  }}
                  className={styles.tableRowHover}
                >
                  <div style={{ display: 'flex', justifyContent: 'space-between', alignItems: 'flex-start' }}>
                    <div>
                      <div style={{ fontWeight: 500, marginBottom: 'var(--space1)' }}>
                        {template.name || template.procedure_template_id}
                      </div>
                      {template.description && (
                        <div style={{ fontSize: '0.875rem', color: 'var(--muted)', marginBottom: 'var(--space1)' }}>
                          {template.description}
                        </div>
                      )}
                      <div style={{ fontSize: '0.75rem', fontFamily: 'var(--mono)', color: 'var(--muted)' }}>
                        {template.procedure_template_id}
                      </div>
                    </div>
                    <div style={{ textAlign: 'right' }}>
                      <div style={{ fontSize: '0.75rem', color: 'var(--muted)' }}>
                        {template.stages_count} stages
                      </div>
                    </div>
                  </div>
                </div>
              ))}
            </div>
          )}

          <div className={styles.buttonRow} style={{ marginTop: 'var(--space4)' }}>
            <Button variant="ghost" onClick={handleBack}>
              Back
            </Button>
          </div>
        </Card>
      )}

      {/* Step 3: Review & Confirm */}
      {step === 'review' && selectedIntake && selectedTemplate && (
        <Card title="Review & Confirm" className={styles.cardSpacing}>
          <p style={{ marginBottom: 'var(--space4)', color: 'var(--muted)', fontSize: '0.875rem' }}>
            Review the Work Surface binding before creating.
          </p>

          <div
            style={{
              display: 'grid',
              gap: 'var(--space4)',
              gridTemplateColumns: '1fr 1fr',
            }}
          >
            {/* Intake Summary */}
            <div
              style={{
                padding: 'var(--space3)',
                backgroundColor: 'var(--bg)',
                borderRadius: 'var(--radius)',
                border: '1px solid var(--border)',
              }}
            >
              <div style={{ fontSize: '0.75rem', color: 'var(--muted)', marginBottom: 'var(--space2)' }}>
                Intake
              </div>
              <div style={{ fontWeight: 500, marginBottom: 'var(--space1)' }}>{selectedIntake.title}</div>
              <div style={{ fontSize: '0.75rem', fontFamily: 'var(--mono)', color: 'var(--muted)' }}>
                {selectedIntake.intake_id}
              </div>
              <div style={{ marginTop: 'var(--space2)' }}>
                <Pill tone="neutral">{selectedIntake.kind.replace(/_/g, ' ')}</Pill>
              </div>
            </div>

            {/* Template Summary */}
            <div
              style={{
                padding: 'var(--space3)',
                backgroundColor: 'var(--bg)',
                borderRadius: 'var(--radius)',
                border: '1px solid var(--border)',
              }}
            >
              <div style={{ fontSize: '0.75rem', color: 'var(--muted)', marginBottom: 'var(--space2)' }}>
                Procedure Template
              </div>
              <div style={{ fontWeight: 500, marginBottom: 'var(--space1)' }}>
                {selectedTemplate.name || selectedTemplate.procedure_template_id}
              </div>
              <div style={{ fontSize: '0.75rem', fontFamily: 'var(--mono)', color: 'var(--muted)' }}>
                {selectedTemplate.procedure_template_id}
              </div>
              <div style={{ marginTop: 'var(--space2)', fontSize: '0.75rem', color: 'var(--muted)' }}>
                {selectedTemplate.stages_count} stages
              </div>
            </div>
          </div>

          {/* Work Unit */}
          <div style={{ marginTop: 'var(--space4)' }}>
            <div className={styles.infoRow}>
              <span className={styles.infoLabel}>Work Unit</span>
              <code className={styles.mono}>{selectedIntake.work_unit_id}</code>
            </div>
          </div>

          <div className={styles.buttonRow} style={{ marginTop: 'var(--space4)' }}>
            <Button variant="ghost" onClick={handleBack} disabled={submitting}>
              Back
            </Button>
            <Button variant="primary" onClick={handleSubmit} disabled={submitting}>
              {submitProgress || 'Create Work Surface'}
            </Button>
          </div>
        </Card>
      )}
    </div>
  );
}

export default WorkSurfaceCompose;
