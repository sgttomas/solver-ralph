/**
 * Reference Bundle Detail Page
 *
 * Displays a single reference bundle with its refs and metadata.
 * Per SR-SPEC, reference bundles capture the complete effective context
 * for an iteration via IterationStarted.refs[].
 */

import { useState, useEffect } from 'react';
import { useParams, Link } from 'react-router-dom';
import { useAuth } from '../auth/AuthProvider';
import config from '../config';
import { Card, Pill, Button, truncateHash } from '../ui';
import styles from '../styles/pages.module.css';

interface TypedRef {
  type_key: string;
  id: string;
  rel: string;
  content_hash: string | null;
}

interface ReferenceBundleDetailData {
  id: string;
  iteration_id: string;
  loop_id: string;
  refs_count: number;
  content_hash: string;
  created_at: string;
  refs: TypedRef[];
  intake_ref: string | null;
  effective_context_hash: string;
}

export function ReferenceBundleDetail(): JSX.Element {
  const { bundleId } = useParams<{ bundleId: string }>();
  const auth = useAuth();
  const [bundle, setBundle] = useState<ReferenceBundleDetailData | null>(null);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);
  const [showRawBundle, setShowRawBundle] = useState(false);

  useEffect(() => {
    if (!auth.user?.access_token || !bundleId) return;

    fetch(`${config.apiUrl}/api/v1/context/bundles/${bundleId}`, {
      headers: { Authorization: `Bearer ${auth.user.access_token}` },
    })
      .then(res => {
        if (res.status === 404) {
          throw new Error('Bundle not found');
        }
        if (!res.ok) throw new Error(`HTTP ${res.status}`);
        return res.json();
      })
      .then(data => {
        setBundle(data);
        setLoading(false);
      })
      .catch(err => {
        setError(err.message);
        setLoading(false);
      });
  }, [auth.user?.access_token, bundleId]);

  if (loading) {
    return (
      <div className={styles.container}>
        <div className={styles.placeholder}>
          <p className={styles.placeholderText}>Loading bundle details...</p>
        </div>
      </div>
    );
  }

  if (error || !bundle) {
    return (
      <div className={styles.container}>
        <div className={styles.placeholder}>
          <p className={styles.error}>Error: {error || 'Bundle not found'}</p>
          <Link to="/references" className={styles.link}>Back to References</Link>
        </div>
      </div>
    );
  }

  const getRefLink = (ref: TypedRef): string | null => {
    switch (ref.type_key) {
      case 'domain.work_unit':
      case 'Loop':
        return `/loops/${ref.id}`;
      case 'Iteration':
        return `/iterations/${ref.id}`;
      case 'Candidate':
        return `/candidates/${ref.id}`;
      case 'ContextDocument':
        return `/references/documents/${ref.id}`;
      case 'Intake':
        return `/intakes/${ref.id}`;
      default:
        return null;
    }
  };

  return (
    <div className={styles.container}>
      {/* Breadcrumb */}
      <div className={styles.breadcrumb}>
        <Link to="/references" className={styles.breadcrumbLink}>References</Link>
        <span className={styles.breadcrumbSeparator}>/</span>
        <span>Bundles</span>
        <span className={styles.breadcrumbSeparator}>/</span>
        <span>{truncateHash(bundle.id, 12)}</span>
      </div>

      {/* Header */}
      <div className={styles.header}>
        <div className={styles.headerStart}>
          <h1 className={styles.title}>Reference Bundle</h1>
          <p className={styles.subtitle}>{bundle.id}</p>
        </div>
        <Pill tone="neutral">{bundle.refs_count} refs</Pill>
      </div>

      {/* Overview Card */}
      <Card title="Overview" className={styles.cardSpacing}>
        <div className={styles.infoRow}>
          <span className={styles.infoLabel}>Bundle ID</span>
          <code className={styles.mono}>{bundle.id}</code>
        </div>
        <div className={styles.infoRow}>
          <span className={styles.infoLabel}>Content Hash</span>
          <code className={styles.mono}>{bundle.content_hash}</code>
        </div>
        <div className={styles.infoRow}>
          <span className={styles.infoLabel}>Effective Context Hash</span>
          <code className={styles.mono}>{bundle.effective_context_hash}</code>
        </div>
        <div className={styles.infoRow}>
          <span className={styles.infoLabel}>References</span>
          <span className={styles.infoValue}>{bundle.refs_count}</span>
        </div>
        <div className={styles.infoRow}>
          <span className={styles.infoLabel}>Created</span>
          <span className={styles.infoValue}>
            {new Date(bundle.created_at).toLocaleString()}
          </span>
        </div>
      </Card>

      {/* Linked Entities Card */}
      <Card title="Linked Entities" className={styles.cardSpacing}>
        <div className={styles.infoRow}>
          <span className={styles.infoLabel}>Iteration</span>
          <Link to={`/iterations/${bundle.iteration_id}`} className={styles.link}>
            {bundle.iteration_id}
          </Link>
        </div>
        <div className={styles.infoRow}>
          <span className={styles.infoLabel}>Loop</span>
          <Link to={`/loops/${bundle.loop_id}`} className={styles.link}>
            {bundle.loop_id}
          </Link>
        </div>
        {bundle.intake_ref && (
          <div className={styles.infoRow}>
            <span className={styles.infoLabel}>Intake</span>
            <Link to={`/intakes/${bundle.intake_ref}`} className={styles.link}>
              {bundle.intake_ref}
            </Link>
          </div>
        )}
      </Card>

      {/* Context References Card */}
      <Card title={`Context References (${bundle.refs.length})`} className={styles.cardSpacing}>
        {bundle.refs.length === 0 ? (
          <div className={styles.placeholder}>
            <p className={styles.placeholderText}>No context references in this bundle.</p>
          </div>
        ) : (
          <table className={styles.table}>
            <thead>
              <tr>
                <th className={styles.th}>Type</th>
                <th className={styles.th}>ID</th>
                <th className={styles.th}>Relation</th>
                <th className={styles.th}>Content Hash</th>
              </tr>
            </thead>
            <tbody>
              {bundle.refs.map((ref, idx) => {
                const link = getRefLink(ref);
                return (
                  <tr key={idx}>
                    <td className={styles.td}>
                      <Pill tone="neutral">{ref.type_key}</Pill>
                    </td>
                    <td className={styles.td}>
                      {link ? (
                        <Link to={link} className={styles.link}>
                          {truncateHash(ref.id, 16)}
                        </Link>
                      ) : (
                        <code className={styles.mono}>{truncateHash(ref.id, 16)}</code>
                      )}
                    </td>
                    <td className={styles.td}>{ref.rel}</td>
                    <td className={styles.tdMono}>
                      {ref.content_hash ? truncateHash(ref.content_hash, 16) : '—'}
                    </td>
                  </tr>
                );
              })}
            </tbody>
          </table>
        )}
      </Card>

      {/* Raw Bundle Card */}
      <Card
        title="Raw Bundle"
        right={
          <Button
            variant={showRawBundle ? 'ghost' : 'secondary'}
            onClick={() => setShowRawBundle(!showRawBundle)}
          >
            {showRawBundle ? 'Hide' : 'Show'}
          </Button>
        }
      >
        {showRawBundle && (
          <pre style={{
            backgroundColor: 'var(--ink)',
            color: '#e0e0e0',
            padding: 'var(--space4)',
            borderRadius: 'var(--radiusSm)',
            fontFamily: 'var(--mono)',
            fontSize: '0.75rem',
            overflow: 'auto',
            maxHeight: '400px',
            whiteSpace: 'pre-wrap',
            wordBreak: 'break-all',
            margin: 0,
          }}>
            {JSON.stringify(bundle, null, 2)}
          </pre>
        )}
      </Card>
    </div>
  );
}

export default ReferenceBundleDetail;
