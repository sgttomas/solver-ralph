/**
 * Intake Edit Page
 *
 * Form to edit an existing draft intake.
 * Only draft intakes can be edited - active/archived are immutable.
 *
 * Stub implementation for Phase 1 - full implementation in Phase 2.
 */

import { useParams, Link } from 'react-router-dom';
import { Card, Button } from '../ui';
import styles from '../styles/pages.module.css';

export function IntakeEdit(): JSX.Element {
  const { intakeId } = useParams<{ intakeId: string }>();

  return (
    <div className={styles.container}>
      {/* Breadcrumb */}
      <div className={styles.breadcrumb}>
        <Link to="/intakes" className={styles.breadcrumbLink}>Intakes</Link>
        <span className={styles.breadcrumbSeparator}>/</span>
        <Link to={`/intakes/${intakeId}`} className={styles.breadcrumbLink}>
          {intakeId?.slice(0, 16)}...
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

      <Card>
        <div className={styles.placeholder}>
          <p className={styles.placeholderText}>Intake edit form coming in Phase 2</p>
          <p className={styles.placeholderHint}>
            This form will allow editing draft intakes. Only intakes with status "draft"
            can be edited - active and archived intakes are immutable commitment objects.
          </p>
        </div>
      </Card>

      <div style={{ display: 'flex', gap: 'var(--space3)', marginTop: 'var(--space4)' }}>
        <Link to={`/intakes/${intakeId}`}>
          <Button variant="secondary">Cancel</Button>
        </Link>
      </div>
    </div>
  );
}

export default IntakeEdit;
