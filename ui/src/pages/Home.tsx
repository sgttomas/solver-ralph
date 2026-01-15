/**
 * Home Page (D-28)
 *
 * Landing page showing system status and quick links.
 */

import { useState, useEffect } from 'react';
import { useAuth } from '../auth/AuthProvider';
import { Link } from 'react-router-dom';
import config from '../config';

interface ApiInfo {
  name: string;
  version: string;
  description: string;
}

const styles = {
  container: {
    maxWidth: '1200px',
    margin: '0 auto',
  },
  hero: {
    backgroundColor: 'white',
    borderRadius: '8px',
    padding: '2rem',
    marginBottom: '1.5rem',
    boxShadow: '0 1px 3px rgba(0, 0, 0, 0.1)',
  },
  heroTitle: {
    margin: '0 0 0.5rem 0',
    fontSize: '1.75rem',
    color: '#1a1a2e',
  },
  heroSubtitle: {
    margin: 0,
    color: '#666',
    fontSize: '1rem',
  },
  grid: {
    display: 'grid',
    gridTemplateColumns: 'repeat(auto-fit, minmax(280px, 1fr))',
    gap: '1.5rem',
  },
  card: {
    backgroundColor: 'white',
    borderRadius: '8px',
    padding: '1.5rem',
    boxShadow: '0 1px 3px rgba(0, 0, 0, 0.1)',
  },
  cardTitle: {
    margin: '0 0 0.75rem 0',
    fontSize: '1.125rem',
    color: '#1a1a2e',
  },
  cardContent: {
    margin: 0,
    color: '#666',
    fontSize: '0.875rem',
    lineHeight: 1.6,
  },
  statusBadge: {
    display: 'inline-block',
    padding: '0.25rem 0.5rem',
    borderRadius: '4px',
    fontSize: '0.75rem',
    fontWeight: 500,
  },
  statusOnline: {
    backgroundColor: '#d4edda',
    color: '#155724',
  },
  statusOffline: {
    backgroundColor: '#f8d7da',
    color: '#721c24',
  },
  link: {
    color: '#0066cc',
    textDecoration: 'none',
  },
  list: {
    margin: '0.5rem 0 0 0',
    paddingLeft: '1.25rem',
    color: '#666',
    fontSize: '0.875rem',
  },
  listItem: {
    marginBottom: '0.25rem',
  },
};

export function Home(): JSX.Element {
  const auth = useAuth();
  const [apiInfo, setApiInfo] = useState<ApiInfo | null>(null);
  const [apiError, setApiError] = useState<string | null>(null);
  const isDevMode = config.devAuthBypass;
  const isAuthenticated = auth.isAuthenticated;

  useEffect(() => {
    fetch(`${config.apiUrl}/api/v1/info`)
      .then(res => {
        if (!res.ok) throw new Error(`HTTP ${res.status}`);
        return res.json();
      })
      .then(data => setApiInfo(data))
      .catch(err => setApiError(err.message));
  }, []);

  return (
    <div style={styles.container}>
      <div style={styles.hero}>
        <h1 style={styles.heroTitle}>
          {isDevMode
            ? 'Welcome, Dev User'
            : 'Welcome to SOLVER-Ralph'}
        </h1>
        <p style={styles.heroSubtitle}>
          Governance-first platform for controlled agentic semantic work
        </p>
      </div>

      <div style={styles.grid}>
        {/* API Status Card */}
        <div style={styles.card}>
          <h2 style={styles.cardTitle}>API Status</h2>
          {apiInfo ? (
            <>
              <p style={styles.cardContent}>
                <span style={{ ...styles.statusBadge, ...styles.statusOnline }}>
                  Online
                </span>
              </p>
              <p style={styles.cardContent}>
                <strong>Name:</strong> {apiInfo.name}<br />
                <strong>Version:</strong> {apiInfo.version}
              </p>
            </>
          ) : apiError ? (
            <>
              <p style={styles.cardContent}>
                <span style={{ ...styles.statusBadge, ...styles.statusOffline }}>
                  Offline
                </span>
              </p>
              <p style={styles.cardContent}>
                Error: {apiError}
              </p>
            </>
          ) : (
            <p style={styles.cardContent}>Checking...</p>
          )}
        </div>

        {/* Authentication Card */}
        <div style={styles.card}>
          <h2 style={styles.cardTitle}>Authentication</h2>
          {isDevMode ? (
            <>
              <p style={styles.cardContent}>
                <span style={{ ...styles.statusBadge, backgroundColor: '#fff3cd', color: '#856404' }}>
                  Dev Bypass
                </span>
              </p>
              <p style={styles.cardContent}>
                <strong>Mode:</strong> Development<br />
                <strong>User:</strong> Dev User<br />
                <strong>Auth:</strong> Disabled
              </p>
            </>
          ) : auth.isLoading ? (
            <>
              <p style={styles.cardContent}>
                <span style={{ ...styles.statusBadge, backgroundColor: '#e2e3e5', color: '#383d41' }}>
                  Checking...
                </span>
              </p>
              <p style={styles.cardContent}>Verifying session with identity provider.</p>
            </>
          ) : isAuthenticated ? (
            <>
              <p style={styles.cardContent}>
                <span style={{ ...styles.statusBadge, ...styles.statusOnline }}>
                  Authenticated
                </span>
              </p>
              <p style={styles.cardContent}>
                <strong>User:</strong> {auth.user?.profile?.name || auth.user?.profile?.email || 'User'}<br />
                <strong>Subject:</strong> {auth.user?.profile?.sub || 'n/a'}
              </p>
            </>
          ) : (
            <>
              <p style={styles.cardContent}>
                <span style={{ ...styles.statusBadge, ...styles.statusOffline }}>
                  Not Authenticated
                </span>
              </p>
              <p style={styles.cardContent}>
                Log in to access portal workflows and governance features.
              </p>
            </>
          )}
        </div>

        {/* Quick Links Card */}
        <div style={styles.card}>
          <h2 style={styles.cardTitle}>Quick Links</h2>
          <ul style={styles.list}>
            <li style={styles.listItem}>
              <Link to="/loops" style={styles.link}>View Loops</Link> - Active work units
            </li>
            <li style={styles.listItem}>
              <Link to="/prompt" style={styles.link}>Prompt Loop</Link> - Run a prompt through the governed loop
            </li>
            <li style={styles.listItem}>
              <Link to="/evidence" style={styles.link}>Artifacts</Link> - Oracle outputs
            </li>
            <li style={styles.listItem}>
              <Link to="/approvals" style={styles.link}>Approvals</Link> - Portal decisions
            </li>
          </ul>
        </div>

        {/* Portal Guide Card */}
        <div style={styles.card}>
          <h2 style={styles.cardTitle}>Portal Guide</h2>
          <p style={styles.cardContent}>
            Per SR-CONTRACT, portals are trust boundaries requiring human authority:
          </p>
          <ul style={styles.list}>
            <li style={styles.listItem}>
              <strong>Approvals</strong> - Bind candidates to freeze baselines
            </li>
            <li style={styles.listItem}>
              <strong>Exceptions</strong> - Scoped deviations from governing docs
            </li>
            <li style={styles.listItem}>
              <strong>Decisions</strong> - Precedent-setting governance choices
            </li>
          </ul>
        </div>
      </div>
    </div>
  );
}

export default Home;
