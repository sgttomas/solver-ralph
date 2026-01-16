/**
 * API Error Handler
 *
 * Maps HTTP errors to user-friendly messages and provides retry logic
 * for transient failures.
 *
 * Per SR-PLAN-V7 Phase V7-2: Error Handling & UX Feedback
 */

interface ApiErrorMapping {
  status: number;
  code?: string;
  message: string;
}

interface ApiErrorBody {
  code?: string;
  message?: string;
}

// Error mappings from SR-PLAN-V7 §V7-2
const ERROR_MAPPINGS: ApiErrorMapping[] = [
  { status: 401, message: 'Session expired. Please sign in again.' },
  { status: 403, message: "You don't have permission to perform this action." },
  {
    status: 412,
    code: 'WORK_SURFACE_NOT_ACTIVE',
    message: 'Work Surface is not active. It may have been completed or archived.',
  },
  {
    status: 412,
    code: 'WORK_SURFACE_MISSING',
    message: 'Work Surface not found. It may have been deleted.',
  },
  { status: 412, message: 'The operation could not be completed. A precondition was not met.' },
  { status: 500, message: 'Something went wrong. Please try again.' },
  { status: 502, message: 'The server is temporarily unavailable. Please try again.' },
  { status: 503, message: 'The service is temporarily unavailable. Please try again.' },
  { status: 504, message: 'The request timed out. Please try again.' },
];

/**
 * Maps an API error to a user-friendly message.
 *
 * @param status HTTP status code
 * @param body Optional response body with error code
 * @returns User-friendly error message
 */
export function mapApiError(status: number, body?: ApiErrorBody): string {
  // First, try to find a mapping with both status and code match
  if (body?.code) {
    const codeMatch = ERROR_MAPPINGS.find((m) => m.status === status && m.code === body.code);
    if (codeMatch) {
      return codeMatch.message;
    }
  }

  // Fall back to status-only match
  const statusMatch = ERROR_MAPPINGS.find((m) => m.status === status && !m.code);
  if (statusMatch) {
    return statusMatch.message;
  }

  // Default messages by status range
  if (status >= 500) {
    return 'Something went wrong. Please try again.';
  }
  if (status >= 400) {
    return body?.message || 'The request could not be completed.';
  }

  return 'An unexpected error occurred.';
}

/**
 * Checks if an HTTP status code indicates a transient error that can be retried.
 *
 * @param status HTTP status code
 * @returns true if the error is retryable (5xx errors)
 */
export function isRetryable(status: number): boolean {
  return status >= 500 && status < 600;
}

/**
 * Waits for a specified duration.
 */
function delay(ms: number): Promise<void> {
  return new Promise((resolve) => setTimeout(resolve, ms));
}

/**
 * Custom error class for API errors with status and body.
 */
export class ApiError extends Error {
  constructor(
    message: string,
    public readonly status: number,
    public readonly body?: ApiErrorBody
  ) {
    super(message);
    this.name = 'ApiError';
  }

  /**
   * Returns a user-friendly message for this error.
   */
  toUserMessage(): string {
    return mapApiError(this.status, this.body);
  }
}

interface FetchWithRetryOptions extends RequestInit {
  maxRetries?: number;
  baseDelayMs?: number;
}

/**
 * Performs a fetch request with automatic retry for transient failures.
 *
 * Retry logic:
 * - Only retries on 5xx errors (transient/server errors)
 * - Does NOT retry on 4xx errors (permanent/client errors)
 * - Uses exponential backoff: 1s, 2s, 4s...
 * - Max 2 retries by default (3 total attempts)
 *
 * @param url Request URL
 * @param options Fetch options plus retry configuration
 * @returns Parsed JSON response
 * @throws ApiError if all retries exhausted or non-retryable error
 */
export async function fetchWithRetry<T>(
  url: string,
  options: FetchWithRetryOptions = {}
): Promise<T> {
  const { maxRetries = 2, baseDelayMs = 1000, ...fetchOptions } = options;

  let lastError: ApiError | null = null;

  for (let attempt = 0; attempt <= maxRetries; attempt++) {
    try {
      const res = await fetch(url, fetchOptions);

      if (res.ok) {
        return await res.json();
      }

      // Parse error body
      let body: ApiErrorBody = {};
      try {
        body = await res.json();
      } catch {
        // Ignore JSON parse errors
      }

      const error = new ApiError(
        body.message || `HTTP ${res.status}`,
        res.status,
        body
      );

      // Don't retry 4xx errors (permanent failures)
      if (!isRetryable(res.status)) {
        throw error;
      }

      lastError = error;

      // If this was the last attempt, throw
      if (attempt === maxRetries) {
        throw error;
      }

      // Exponential backoff: 1s, 2s, 4s...
      const delayMs = baseDelayMs * Math.pow(2, attempt);
      await delay(delayMs);
    } catch (err) {
      // Network errors or already-thrown ApiErrors
      if (err instanceof ApiError) {
        throw err;
      }

      // Network error - treat as retryable
      lastError = new ApiError(
        err instanceof Error ? err.message : 'Network error',
        0,
        undefined
      );

      if (attempt === maxRetries) {
        throw lastError;
      }

      const delayMs = baseDelayMs * Math.pow(2, attempt);
      await delay(delayMs);
    }
  }

  // Should not reach here, but TypeScript needs this
  throw lastError || new ApiError('Unknown error', 0);
}

export default { mapApiError, isRetryable, fetchWithRetry, ApiError };
