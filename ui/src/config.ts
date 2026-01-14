/**
 * SOLVER-Ralph UI Configuration (D-28)
 *
 * Environment-based configuration for the UI.
 * Uses Vite's import.meta.env for build-time environment variables.
 */

export interface AppConfig {
  /** API base URL */
  apiUrl: string;
  /** OIDC issuer URL (Zitadel) */
  oidcIssuer: string;
  /** OIDC client ID */
  oidcClientId: string;
  /** OIDC redirect URI */
  oidcRedirectUri: string;
  /** OIDC post-logout redirect URI */
  oidcPostLogoutRedirectUri: string;
  /** OIDC scopes */
  oidcScopes: string;
  /** Dev mode: bypass OIDC authentication */
  devAuthBypass: boolean;
}

function getEnv(key: string, defaultValue: string): string {
  // Vite exposes env variables via import.meta.env
  const value = (import.meta.env as Record<string, string | undefined>)[key];
  return value ?? defaultValue;
}

export const config: AppConfig = {
  apiUrl: getEnv('VITE_API_URL', 'http://localhost:3000'),
  oidcIssuer: getEnv('VITE_OIDC_ISSUER', 'http://localhost:8080'),
  oidcClientId: getEnv('VITE_OIDC_CLIENT_ID', 'solver-ralph-ui'),
  oidcRedirectUri: getEnv('VITE_OIDC_REDIRECT_URI', window.location.origin + '/callback'),
  oidcPostLogoutRedirectUri: getEnv('VITE_OIDC_POST_LOGOUT_REDIRECT_URI', window.location.origin),
  oidcScopes: getEnv('VITE_OIDC_SCOPES', 'openid profile email'),
  devAuthBypass: getEnv('VITE_DEV_AUTH_BYPASS', 'false') === 'true',
};

export default config;
