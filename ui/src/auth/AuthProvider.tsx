/**
 * OIDC Authentication Provider (D-28)
 *
 * Configures react-oidc-context for Zitadel authentication.
 * Per SR-SPEC §2.1, identity validation is required for portal operations.
 *
 * Supports dev bypass mode via VITE_DEV_AUTH_BYPASS=true
 */

import { createContext, ReactNode, useContext } from 'react';
import { AuthProvider as OidcAuthProvider, AuthProviderProps, useAuth as useOidcAuth, AuthContextProps } from 'react-oidc-context';
import { WebStorageStateStore, User } from 'oidc-client-ts';
import config from '../config';

interface AuthProviderWrapperProps {
  children: ReactNode;
}

/**
 * Mock auth context for dev bypass mode
 */
const DevAuthContext = createContext<AuthContextProps | null>(null);

const mockUser: User = {
  access_token: 'dev-bypass',
  token_type: 'Bearer',
  profile: {
    sub: 'dev-user',
    name: 'Dev User',
    email: 'dev@example.com',
    iss: 'dev-issuer',
    aud: 'dev-audience',
    exp: Math.floor(Date.now() / 1000) + 3600,
    iat: Math.floor(Date.now() / 1000),
  },
  expires_in: 3600,
  expired: false,
  scopes: ['openid', 'profile', 'email'],
  toStorageString: () => '{}',
} as User;

const mockAuthContext: AuthContextProps = {
  user: mockUser,
  isLoading: false,
  isAuthenticated: true,
  activeNavigator: undefined,
  error: undefined,
  signinRedirect: async () => {},
  signinSilent: async () => mockUser,
  signinPopup: async () => mockUser,
  signinResourceOwnerCredentials: async () => mockUser,
  signoutRedirect: async () => {},
  signoutPopup: async () => {},
  signoutSilent: async () => {},
  removeUser: async () => {},
  querySessionStatus: async () => null,
  revokeTokens: async () => {},
  startSilentRenew: () => {},
  stopSilentRenew: () => {},
  clearStaleState: async () => {},
  settings: {} as AuthContextProps['settings'],
  events: {} as AuthContextProps['events'],
};

/**
 * Custom useAuth hook that works in both dev bypass and real OIDC modes
 */
export function useAuth(): AuthContextProps {
  const devContext = useContext(DevAuthContext);

  // In dev bypass mode, return the mock context
  if (config.devAuthBypass) {
    if (devContext) return devContext;
    // Fallback if context not available
    return mockAuthContext;
  }

  // In real mode, use the OIDC hook
  // Conditionally calling hook is intentional - dev bypass returns early
  // eslint-disable-next-line
  return useOidcAuth();
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
 * In dev bypass mode, provides a mock auth context so useAuth() still works.
 */
export function AuthProvider({ children }: AuthProviderWrapperProps): JSX.Element {
  if (config.devAuthBypass) {
    console.warn('[Auth] DEV BYPASS MODE - Authentication disabled for development');
    return (
      <DevAuthContext.Provider value={mockAuthContext}>
        {children}
      </DevAuthContext.Provider>
    );
  }

  return (
    <OidcAuthProvider {...oidcConfig}>
      {children}
    </OidcAuthProvider>
  );
}

export default AuthProvider;
