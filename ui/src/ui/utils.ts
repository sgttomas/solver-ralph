import type { PillTone } from './Pill';

/**
 * Maps common status strings to Pill tones.
 *
 * Success: ACTIVE, COMPLETED, SUCCESS, VERIFIED, APPROVED, PASS, RESOLVED
 * Warning: PENDING, CREATED, PAUSED, DEFERRED, DEFERRAL, RUNNING
 * Danger: FAILURE, REJECTED, CLOSED, ERROR, FAIL, WAIVER, EXPIRED
 * Neutral: everything else
 */
export function getStatusTone(status: string): PillTone {
  const upper = status.toUpperCase();

  // Success states
  if (['ACTIVE', 'COMPLETED', 'SUCCESS', 'VERIFIED', 'APPROVED', 'PASS', 'RESOLVED'].includes(upper)) {
    return 'success';
  }

  // Warning states
  if (['PENDING', 'CREATED', 'PAUSED', 'DEFERRED', 'DEFERRAL', 'RUNNING', 'DEVIATION'].includes(upper)) {
    return 'warning';
  }

  // Danger states
  if (['FAILURE', 'REJECTED', 'CLOSED', 'ERROR', 'FAIL', 'WAIVER', 'EXPIRED'].includes(upper)) {
    return 'danger';
  }

  return 'neutral';
}

/**
 * Truncates a string to a maximum length with ellipsis.
 */
export function truncate(str: string, maxLength: number): string {
  if (str.length <= maxLength) return str;
  return str.substring(0, maxLength) + '...';
}

/**
 * Truncates a hash string for display (e.g., "sha256:abc123..." → "abc123...")
 */
export function truncateHash(hash: string, prefixLength = 12): string {
  // Remove common prefixes like "sha256:" for display
  const withoutPrefix = hash.replace(/^(sha256:|git:)/, '');
  if (withoutPrefix.length <= prefixLength) return withoutPrefix;
  return withoutPrefix.substring(0, prefixLength) + '...';
}
