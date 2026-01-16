/**
 * Governed Artifact Detail Page
 *
 * Displays a single governed artifact (SR-* document) with its metadata.
 * Per SR-SPEC, governed artifacts are normative governance documents
 * that define the system's contracts, specifications, and types.
 *
 * Stub implementation for Phase 1 - full implementation in Phase 3.
 */

import { useParams, Link } from 'react-router-dom';
import { Card } from '../ui';
import styles from '../styles/pages.module.css';

export function GovernedArtifactDetail(): JSX.Element {
  const { artifactId } = useParams<{ artifactId: string }>();

  return (
    <div className={styles.container}>
      {/* Breadcrumb */}
      <div className={styles.breadcrumb}>
        <Link to="/references" className={styles.breadcrumbLink}>References</Link>
        <span className={styles.breadcrumbSeparator}>/</span>
        <span>Governed Artifacts</span>
        <span className={styles.breadcrumbSeparator}>/</span>
        <span>{artifactId}</span>
      </div>

      <div className={styles.header}>
        <div className={styles.headerStart}>
          <h1 className={styles.title}>Governed Artifact</h1>
          <p className={styles.subtitle}>{artifactId}</p>
        </div>
      </div>

      <Card>
        <div className={styles.placeholder}>
          <p className={styles.placeholderText}>Governed artifact detail coming in Phase 3</p>
          <p className={styles.placeholderHint}>
            This page will display governed artifacts (SR-* documents) with their
            metadata, version history, and relationships to other artifacts.
          </p>
        </div>
      </Card>

      {/* Info Note */}
      <div className={styles.note}>
        Per SR-SPEC §3.2.1.1: Governed artifacts are normative governance documents
        including SR-CONTRACT, SR-SPEC, SR-TYPES, SR-WORK-SURFACE, and other SR-* documents.
      </div>
    </div>
  );
}

export default GovernedArtifactDetail;
