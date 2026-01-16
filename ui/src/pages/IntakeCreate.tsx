/**
 * Intake Create Page
 *
 * Form to create a new intake draft.
 * Per SR-WORK-SURFACE §3.1, all required fields must be provided.
 *
 * Stub implementation for Phase 1 - full implementation in Phase 2.
 */

import { Link } from 'react-router-dom';
import { Card, Button } from '../ui';
import styles from '../styles/pages.module.css';

export function IntakeCreate(): JSX.Element {
  return (
    <div className={styles.container}>
      {/* Breadcrumb */}
      <div className={styles.breadcrumb}>
        <Link to="/intakes" className={styles.breadcrumbLink}>Intakes</Link>
        <span className={styles.breadcrumbSeparator}>/</span>
        <span>New</span>
      </div>

      <div className={styles.header}>
        <div className={styles.headerStart}>
          <h1 className={styles.title}>New Intake</h1>
          <p className={styles.subtitle}>Create a new work unit specification</p>
        </div>
      </div>

      <Card>
        <div className={styles.placeholder}>
          <p className={styles.placeholderText}>Intake creation form coming in Phase 2</p>
          <p className={styles.placeholderHint}>
            This form will allow creating intakes with all SR-WORK-SURFACE §3.1 fields:
            title, kind, objective, audience, deliverables, constraints, definitions,
            inputs, unknowns, and completion criteria.
          </p>
        </div>
      </Card>

      <div style={{ display: 'flex', gap: 'var(--space3)', marginTop: 'var(--space4)' }}>
        <Link to="/intakes">
          <Button variant="secondary">Cancel</Button>
        </Link>
      </div>
    </div>
  );
}

export default IntakeCreate;
