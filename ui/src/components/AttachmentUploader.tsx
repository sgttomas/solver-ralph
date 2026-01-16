/**
 * AttachmentUploader Component
 *
 * Drag-and-drop file upload component for human-provided supporting files.
 * Per SR-PLAN-V7 Phase V7-4: Attachments are NOT Evidence Bundles.
 *
 * Ontological distinction:
 * - Evidence Bundles: Oracle-produced, satisfies C-VER-1
 * - Attachments: Human uploads, supporting context only
 */

import { useState, useRef, useCallback } from 'react';
import { useAuth } from '../auth/AuthProvider';
import config from '../config';
import { useToast } from './ToastContext';
import { AttachmentPreview } from './AttachmentPreview';
import styles from './AttachmentUploader.module.css';

export interface UploadedAttachment {
  attachment_id: string;
  content_hash: string;
  size_bytes: number;
  media_type: string;
  filename: string;
  uploaded_by: string;
  uploaded_at: string;
}

interface AttachmentUploaderProps {
  onUploadComplete: (attachment: UploadedAttachment) => void;
  onError?: (error: string) => void;
  accept?: string; // e.g., ".pdf,.md,.json"
  maxSizeMB?: number; // default 10
  disabled?: boolean;
}

export function AttachmentUploader({
  onUploadComplete,
  onError,
  accept = '.pdf,.md,.json,.txt,.csv,.png,.jpg,.jpeg,.gif,.doc,.docx,.xls,.xlsx',
  maxSizeMB = 10,
  disabled = false,
}: AttachmentUploaderProps): JSX.Element {
  const auth = useAuth();
  const toast = useToast();
  const inputRef = useRef<HTMLInputElement>(null);

  const [isDragActive, setIsDragActive] = useState(false);
  const [isUploading, setIsUploading] = useState(false);
  const [uploadProgress, setUploadProgress] = useState(0);
  const [uploadedAttachment, setUploadedAttachment] = useState<UploadedAttachment | null>(null);
  const [error, setError] = useState<string | null>(null);

  const maxSizeBytes = maxSizeMB * 1024 * 1024;

  /**
   * Validate file before upload
   */
  const validateFile = useCallback(
    (file: File): string | null => {
      // Check file size
      if (file.size > maxSizeBytes) {
        return `File too large. Maximum size is ${maxSizeMB} MB.`;
      }

      // Check file type if accept is specified
      if (accept) {
        const acceptedTypes = accept.split(',').map((t) => t.trim().toLowerCase());
        const fileExtension = '.' + file.name.split('.').pop()?.toLowerCase();
        const fileMimeType = file.type.toLowerCase();

        const isAccepted = acceptedTypes.some(
          (type) =>
            type === fileExtension ||
            type === fileMimeType ||
            (type.endsWith('/*') && fileMimeType.startsWith(type.replace('/*', '/')))
        );

        if (!isAccepted) {
          return `File type not accepted. Supported types: ${accept}`;
        }
      }

      return null;
    },
    [accept, maxSizeBytes, maxSizeMB]
  );

  /**
   * Upload file to the attachment endpoint
   */
  const uploadFile = useCallback(
    async (file: File) => {
      const accessToken = auth.user?.access_token;
      if (!accessToken) {
        const msg = 'Not authenticated';
        setError(msg);
        onError?.(msg);
        return;
      }

      // Validate file
      const validationError = validateFile(file);
      if (validationError) {
        setError(validationError);
        onError?.(validationError);
        toast.error(validationError);
        return;
      }

      setIsUploading(true);
      setUploadProgress(0);
      setError(null);

      try {
        // Use XMLHttpRequest for progress tracking
        const result = await new Promise<UploadedAttachment>((resolve, reject) => {
          const xhr = new XMLHttpRequest();

          xhr.upload.onprogress = (e) => {
            if (e.lengthComputable) {
              const percent = Math.round((e.loaded / e.total) * 100);
              setUploadProgress(percent);
            }
          };

          xhr.onload = () => {
            if (xhr.status >= 200 && xhr.status < 300) {
              try {
                const response = JSON.parse(xhr.responseText);
                resolve(response);
              } catch {
                reject(new Error('Invalid response from server'));
              }
            } else {
              let errorMessage = `Upload failed: HTTP ${xhr.status}`;
              try {
                const errorBody = JSON.parse(xhr.responseText);
                errorMessage = errorBody.message || errorMessage;
              } catch {
                // Ignore JSON parse error
              }
              reject(new Error(errorMessage));
            }
          };

          xhr.onerror = () => {
            reject(new Error('Network error during upload'));
          };

          xhr.ontimeout = () => {
            reject(new Error('Upload timed out'));
          };

          const formData = new FormData();
          formData.append('file', file);

          xhr.open('POST', `${config.apiUrl}/api/v1/attachments`);
          xhr.setRequestHeader('Authorization', `Bearer ${accessToken}`);
          xhr.timeout = 60000; // 60 second timeout
          xhr.send(formData);
        });

        setUploadedAttachment(result);
        setUploadProgress(100);
        toast.success(`Uploaded ${file.name}`);
        onUploadComplete(result);
      } catch (err) {
        const msg = err instanceof Error ? err.message : 'Upload failed';
        setError(msg);
        onError?.(msg);
        toast.error(msg);
      } finally {
        setIsUploading(false);
      }
    },
    [auth.user?.access_token, validateFile, onUploadComplete, onError, toast]
  );

  /**
   * Handle file selection from input
   */
  const handleFileChange = useCallback(
    (e: React.ChangeEvent<HTMLInputElement>) => {
      const file = e.target.files?.[0];
      if (file) {
        uploadFile(file);
      }
      // Reset input so same file can be selected again
      e.target.value = '';
    },
    [uploadFile]
  );

  /**
   * Handle drag events
   */
  const handleDragOver = useCallback((e: React.DragEvent) => {
    e.preventDefault();
    e.stopPropagation();
    setIsDragActive(true);
  }, []);

  const handleDragLeave = useCallback((e: React.DragEvent) => {
    e.preventDefault();
    e.stopPropagation();
    setIsDragActive(false);
  }, []);

  const handleDrop = useCallback(
    (e: React.DragEvent) => {
      e.preventDefault();
      e.stopPropagation();
      setIsDragActive(false);

      if (disabled || isUploading) return;

      const file = e.dataTransfer.files?.[0];
      if (file) {
        uploadFile(file);
      }
    },
    [disabled, isUploading, uploadFile]
  );

  /**
   * Handle click to browse
   */
  const handleClick = useCallback(() => {
    if (!disabled && !isUploading) {
      inputRef.current?.click();
    }
  }, [disabled, isUploading]);

  /**
   * Handle remove attachment
   */
  const handleRemove = useCallback(() => {
    setUploadedAttachment(null);
    setUploadProgress(0);
    setError(null);
  }, []);

  // If attachment is already uploaded, show preview
  if (uploadedAttachment) {
    return (
      <div className={styles.container}>
        <AttachmentPreview attachment={uploadedAttachment} onRemove={handleRemove} />
      </div>
    );
  }

  return (
    <div className={styles.container}>
      <div
        className={`${styles.dropzone} ${isDragActive ? styles.dropzoneActive : ''} ${
          disabled ? styles.dropzoneDisabled : ''
        }`}
        onDragOver={handleDragOver}
        onDragLeave={handleDragLeave}
        onDrop={handleDrop}
        onClick={handleClick}
        role="button"
        tabIndex={disabled ? -1 : 0}
        aria-label="Upload attachment"
        onKeyDown={(e) => {
          if (e.key === 'Enter' || e.key === ' ') {
            e.preventDefault();
            handleClick();
          }
        }}
      >
        <input
          ref={inputRef}
          type="file"
          accept={accept}
          onChange={handleFileChange}
          className={styles.hiddenInput}
          disabled={disabled || isUploading}
          aria-hidden="true"
        />

        {isUploading ? (
          <div className={styles.uploadingState}>
            <div className={styles.progressContainer}>
              <div className={styles.progressBar} style={{ width: `${uploadProgress}%` }} />
            </div>
            <span className={styles.progressText}>Uploading... {uploadProgress}%</span>
          </div>
        ) : (
          <div className={styles.idleState}>
            <div className={styles.uploadIcon}>
              <svg
                width="32"
                height="32"
                viewBox="0 0 24 24"
                fill="none"
                stroke="currentColor"
                strokeWidth="2"
                strokeLinecap="round"
                strokeLinejoin="round"
              >
                <path d="M21 15v4a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2v-4" />
                <polyline points="17,8 12,3 7,8" />
                <line x1="12" y1="3" x2="12" y2="15" />
              </svg>
            </div>
            <span className={styles.dropzoneText}>
              {isDragActive ? 'Drop file here' : 'Drag & drop a file, or click to browse'}
            </span>
            <span className={styles.dropzoneHint}>
              Max {maxSizeMB} MB. Supported: {accept.replace(/\./g, '').replace(/,/g, ', ')}
            </span>
          </div>
        )}
      </div>

      {error && <div className={styles.error}>{error}</div>}

      <div className={styles.note}>
        Supporting attachments provide context for audit and reference. They do not satisfy
        verification gates (C-VER-1) &mdash; only oracle-produced Evidence Bundles can satisfy
        verification requirements.
      </div>
    </div>
  );
}

export default AttachmentUploader;
