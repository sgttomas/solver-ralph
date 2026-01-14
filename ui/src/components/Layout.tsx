/**
 * Application Layout Component (D-28)
 *
 * Main layout with header, navigation, and authentication controls.
 * Per SR-SPEC §2, portal UI provides human review surface.
 */

import { ReactNode } from 'react';
import { useAuth } from 'react-oidc-context';
import { Link } from 'react-router-dom';

interface LayoutProps {
  children: ReactNode;
}

const styles = {
  container: {
    fontFamily: 'system-ui, -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, sans-serif',
    minHeight: '100vh',
    display: 'flex',
    flexDirection: 'column' as const,
  },
  header: {
    backgroundColor: '#1a1a2e',
    color: 'white',
    padding: '0 1rem',
    display: 'flex',
    alignItems: 'center',
    justifyContent: 'space-between',
    height: '60px',
  },
  logo: {
    display: 'flex',
    alignItems: 'center',
    gap: '0.5rem',
    textDecoration: 'none',
    color: 'white',
  },
  logoText: {
    fontSize: '1.25rem',
    fontWeight: 600,
    margin: 0,
  },
  nav: {
    display: 'flex',
    gap: '1.5rem',
    alignItems: 'center',
  },
  navLink: {
    color: 'rgba(255, 255, 255, 0.8)',
    textDecoration: 'none',
    fontSize: '0.875rem',
    padding: '0.5rem 0',
    borderBottom: '2px solid transparent',
    transition: 'color 0.2s, border-color 0.2s',
  },
  userSection: {
    display: 'flex',
    alignItems: 'center',
    gap: '1rem',
  },
  userInfo: {
    fontSize: '0.875rem',
    color: 'rgba(255, 255, 255, 0.9)',
  },
  button: {
    padding: '0.5rem 1rem',
    fontSize: '0.875rem',
    cursor: 'pointer',
    border: 'none',
    borderRadius: '4px',
    transition: 'background-color 0.2s',
  },
  loginButton: {
    backgroundColor: '#0066cc',
    color: 'white',
  },
  logoutButton: {
    backgroundColor: 'transparent',
    color: 'rgba(255, 255, 255, 0.8)',
    border: '1px solid rgba(255, 255, 255, 0.3)',
  },
  main: {
    flex: 1,
    padding: '1.5rem',
    backgroundColor: '#f5f5f5',
  },
  footer: {
    backgroundColor: '#1a1a2e',
    color: 'rgba(255, 255, 255, 0.6)',
    padding: '1rem',
    textAlign: 'center' as const,
    fontSize: '0.75rem',
  },
};

export function Layout({ children }: LayoutProps): JSX.Element {
  const auth = useAuth();

  const handleLogin = () => {
    auth.signinRedirect();
  };

  const handleLogout = () => {
    auth.signoutRedirect();
  };

  const userName = auth.user?.profile?.name || auth.user?.profile?.email || 'User';

  return (
    <div style={styles.container}>
      <header style={styles.header}>
        <Link to="/" style={styles.logo}>
          <span style={{ fontSize: '1.5rem' }}>&#9670;</span>
          <h1 style={styles.logoText}>SOLVER-Ralph</h1>
        </Link>

        <nav style={styles.nav}>
          <Link to="/" style={styles.navLink}>
            Dashboard
          </Link>
          {auth.isAuthenticated && (
            <>
              <Link to="/loops" style={styles.navLink}>
                Loops
              </Link>
              <Link to="/evidence" style={styles.navLink}>
                Evidence
              </Link>
              <Link to="/approvals" style={styles.navLink}>
                Approvals
              </Link>
            </>
          )}
        </nav>

        <div style={styles.userSection}>
          {auth.isLoading ? (
            <span style={styles.userInfo}>Loading...</span>
          ) : auth.isAuthenticated ? (
            <>
              <span style={styles.userInfo}>
                {userName}
              </span>
              <button
                onClick={handleLogout}
                style={{ ...styles.button, ...styles.logoutButton }}
              >
                Log Out
              </button>
            </>
          ) : (
            <button
              onClick={handleLogin}
              style={{ ...styles.button, ...styles.loginButton }}
            >
              Log In
            </button>
          )}
        </div>
      </header>

      <main style={styles.main}>
        {children}
      </main>

      <footer style={styles.footer}>
        <p style={{ margin: 0 }}>
          SOLVER-Ralph &middot; Governance-first platform for controlled agentic work
        </p>
      </footer>
    </div>
  );
}

export default Layout;
