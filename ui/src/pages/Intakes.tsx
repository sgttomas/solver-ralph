/**
 * Intakes List Page
 *
 * Lists all intakes with filtering and status management.
 * Per SR-WORK-SURFACE, intakes define the structured problem statement
 * for a work unit including objective, deliverables, constraints, and inputs.
 *
 * Stub implementation for Phase 1 - full implementation in Phase 2.
 */

import { Link } from 'react-router-dom';
import { Card, Button } from '../ui';
import styles from '../styles/pages.module.css';

export function Intakes(): JSX.Element {
  return (
    <div className={styles.container}>
      <div className={styles.header}>
        <div className={styles.headerStart}>
          <h1 className={styles.title}>Intakes</h1>
          <p className={styles.subtitle}>Work unit specifications and problem statements</p>
        </div>
        <Link to="/intakes/new">
          <Button variant="primary">New Intake</Button>
        </Link>
      </div>

      <Card>
        <div className={styles.placeholder}>
          <p className={styles.placeholderText}>Intakes list coming in Phase 2</p>
          <p className={styles.placeholderHint}>
            This page will display all intakes with filtering by status (draft, active, archived)
            and kind (research_memo, decision_record, etc.). Full CRUD operations will be available.
          </p>
        </div>
      </Card>

      {/* Info Note */}
      <div className={styles.note}>
        Per SR-WORK-SURFACE §3.1: Intakes define the objective, scope, constraints, and deliverables
        for a work unit. When activated, they become commitment objects (content-addressed, immutable).
      </div>
    </div>
  );
}

export default Intakes;
