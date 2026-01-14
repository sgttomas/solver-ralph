/**
 * OIDC Callback Page (D-28)
 *
 * Handles the redirect from Zitadel after authentication.
 * This page is shown briefly while processing the auth response.
 */

import { useEffect } from 'react';
import { useAuth } from '../auth/AuthProvider';
import { useNavigate } from 'react-router-dom';

const styles = {
  container: {
    display: 'flex',
    flexDirection: 'column' as const,
    alignItems: 'center',
    justifyContent: 'center',
    minHeight: '60vh',
    textAlign: 'center' as const,
  },
  spinner: {
    width: '40px',
    height: '40px',
    border: '3px solid #f3f3f3',
    borderTop: '3px solid #0066cc',
    borderRadius: '50%',
    animation: 'spin 1s linear infinite',
    marginBottom: '1rem',
  },
  message: {
    color: '#666',
    fontSize: '1rem',
  },
  error: {
    color: '#dc3545',
    backgroundColor: '#f8d7da',
    padding: '1rem',
    borderRadius: '4px',
    marginTop: '1rem',
  },
};

export function Callback(): JSX.Element {
  const auth = useAuth();
  const navigate = useNavigate();

  useEffect(() => {
    // After successful auth, redirect to home
    if (auth.isAuthenticated && !auth.isLoading) {
      navigate('/', { replace: true });
    }
  }, [auth.isAuthenticated, auth.isLoading, navigate]);

  if (auth.error) {
    return (
      <div style={styles.container}>
        <h2>Authentication Error</h2>
        <div style={styles.error}>
          <p>{auth.error.message}</p>
        </div>
        <button
          onClick={() => navigate('/', { replace: true })}
          style={{
            marginTop: '1rem',
            padding: '0.5rem 1rem',
            cursor: 'pointer',
          }}
        >
          Return Home
        </button>
      </div>
    );
  }

  return (
    <div style={styles.container}>
      <style>
        {`
          @keyframes spin {
            0% { transform: rotate(0deg); }
            100% { transform: rotate(360deg); }
          }
        `}
      </style>
      <div style={styles.spinner} />
      <p style={styles.message}>
        Completing authentication...
      </p>
    </div>
  );
}

export default Callback;
