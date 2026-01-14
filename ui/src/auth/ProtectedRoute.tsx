/**
 * Protected Route Component (D-28)
 *
 * Requires authentication before rendering child routes.
 * Per SR-CONTRACT, portal operations require human authority.
 *
 * Supports dev bypass mode via VITE_DEV_AUTH_BYPASS=true
 */

import { ReactNode } from 'react';
import { useAuth } from './AuthProvider';
import config from '../config';

interface ProtectedRouteProps {
  children: ReactNode;
}

/**
 * Route guard that requires authentication
 *
 * Shows loading state during auth check, redirects to login if not authenticated.
 * In dev bypass mode, always allows access (useAuth returns mock authenticated state).
 */
export function ProtectedRoute({ children }: ProtectedRouteProps): JSX.Element {
  const auth = useAuth();

  // In dev bypass mode, auth.isAuthenticated is always true
  if (config.devAuthBypass) {
    return <>{children}</>;
  }

  if (auth.isLoading) {
    return (
      <div style={{ padding: '2rem', textAlign: 'center' }}>
        <p>Loading authentication...</p>
      </div>
    );
  }

  if (auth.error) {
    return (
      <div style={{ padding: '2rem', textAlign: 'center', color: 'red' }}>
        <h2>Authentication Error</h2>
        <p>{auth.error.message}</p>
        <button onClick={() => auth.signinRedirect()}>
          Try Again
        </button>
      </div>
    );
  }

  if (!auth.isAuthenticated) {
    return (
      <div style={{ padding: '2rem', textAlign: 'center' }}>
        <h2>Authentication Required</h2>
        <p>You must be logged in to access this page.</p>
        <button
          onClick={() => auth.signinRedirect()}
          style={{
            padding: '0.5rem 1rem',
            fontSize: '1rem',
            cursor: 'pointer',
            backgroundColor: '#0066cc',
            color: 'white',
            border: 'none',
            borderRadius: '4px',
          }}
        >
          Log In
        </button>
      </div>
    );
  }

  return <>{children}</>;
}

export default ProtectedRoute;
