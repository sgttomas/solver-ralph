/// <reference types="vite/client" />

/**
 * Vite Environment Variables Type Declarations (D-28)
 *
 * Provides TypeScript types for Vite's import.meta.env
 */

interface ImportMetaEnv {
  /** API base URL */
  readonly VITE_API_URL: string;
  /** OIDC issuer URL (Zitadel) */
  readonly VITE_OIDC_ISSUER: string;
  /** OIDC client ID */
  readonly VITE_OIDC_CLIENT_ID: string;
  /** OIDC redirect URI */
  readonly VITE_OIDC_REDIRECT_URI?: string;
  /** OIDC post-logout redirect URI */
  readonly VITE_OIDC_POST_LOGOUT_REDIRECT_URI?: string;
  /** OIDC scopes */
  readonly VITE_OIDC_SCOPES?: string;
}

interface ImportMeta {
  readonly env: ImportMetaEnv;
}
