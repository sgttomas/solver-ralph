/**
 * OIDC Authentication Provider (D-28)
 *
 * Configures react-oidc-context for Zitadel authentication.
 * Per SR-SPEC §2.1, identity validation is required for portal operations.
 */

import { ReactNode } from 'react';
import { AuthProvider as OidcAuthProvider, AuthProviderProps } from 'react-oidc-context';
import { WebStorageStateStore, User } from 'oidc-client-ts';
import config from '../config';

interface AuthProviderWrapperProps {
  children: ReactNode;
}

/**
 * OIDC configuration for Zitadel
 *
 * Uses Authorization Code flow with PKCE (recommended for SPAs)
 */
const oidcConfig: AuthProviderProps = {
  authority: config.oidcIssuer,
  client_id: config.oidcClientId,
  redirect_uri: config.oidcRedirectUri,
  post_logout_redirect_uri: config.oidcPostLogoutRedirectUri,
  scope: config.oidcScopes,
  // Use sessionStorage for tokens (cleared on browser close)
  userStore: new WebStorageStateStore({ store: window.sessionStorage }),
  // Automatically sign in silently when token expires
  automaticSilentRenew: true,
  // Load user info from userinfo endpoint
  loadUserInfo: true,
  // Callback handlers
  onSigninCallback: (_user: User | void): void => {
    // Remove OIDC state from URL after login
    window.history.replaceState({}, document.title, window.location.pathname);
  },
};

/**
 * Authentication provider wrapper
 *
 * Wraps the application with OIDC context for authentication.
 */
export function AuthProvider({ children }: AuthProviderWrapperProps): JSX.Element {
  return (
    <OidcAuthProvider {...oidcConfig}>
      {children}
    </OidcAuthProvider>
  );
}

export default AuthProvider;
