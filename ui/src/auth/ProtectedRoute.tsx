/**
 * Protected Route Component (D-28)
 *
 * Requires authentication before rendering child routes.
 * Per SR-CONTRACT, portal operations require human authority.
 */

import { ReactNode } from 'react';
import { useAuth } from 'react-oidc-context';

interface ProtectedRouteProps {
  children: ReactNode;
}

/**
 * Route guard that requires authentication
 *
 * Shows loading state during auth check, redirects to login if not authenticated.
 */
export function ProtectedRoute({ children }: ProtectedRouteProps): JSX.Element {
  const auth = useAuth();

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
