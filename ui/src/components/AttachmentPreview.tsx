/**
 * AttachmentPreview Component
 *
 * Displays uploaded attachment metadata.
 * Per SR-PLAN-V7 Phase V7-4: Shows filename, size, type, and truncated hash.
 */

import styles from './AttachmentUploader.module.css';

export interface AttachmentPreviewProps {
  attachment: {
    attachment_id: string;
    content_hash: string;
    size_bytes: number;
    media_type: string;
    filename: string;
  };
  onRemove?: () => void;
}

/**
 * Format file size for display
 */
function formatFileSize(bytes: number): string {
  if (bytes < 1024) return `${bytes} B`;
  if (bytes < 1024 * 1024) return `${(bytes / 1024).toFixed(1)} KB`;
  return `${(bytes / (1024 * 1024)).toFixed(1)} MB`;
}

/**
 * Get file icon based on media type
 */
function getFileIcon(mediaType: string): JSX.Element {
  const iconProps = {
    width: 20,
    height: 20,
    viewBox: '0 0 24 24',
    fill: 'none',
    stroke: 'currentColor',
    strokeWidth: 2,
    strokeLinecap: 'round' as const,
    strokeLinejoin: 'round' as const,
  };

  // PDF
  if (mediaType === 'application/pdf') {
    return (
      <svg {...iconProps}>
        <path d="M14 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V8z" />
        <polyline points="14,2 14,8 20,8" />
        <line x1="16" y1="13" x2="8" y2="13" />
        <line x1="16" y1="17" x2="8" y2="17" />
        <polyline points="10,9 9,9 8,9" />
      </svg>
    );
  }

  // Images
  if (mediaType.startsWith('image/')) {
    return (
      <svg {...iconProps}>
        <rect x="3" y="3" width="18" height="18" rx="2" ry="2" />
        <circle cx="8.5" cy="8.5" r="1.5" />
        <polyline points="21,15 16,10 5,21" />
      </svg>
    );
  }

  // Spreadsheets
  if (
    mediaType.includes('spreadsheet') ||
    mediaType.includes('excel') ||
    mediaType === 'text/csv'
  ) {
    return (
      <svg {...iconProps}>
        <path d="M14 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V8z" />
        <polyline points="14,2 14,8 20,8" />
        <line x1="8" y1="13" x2="16" y2="13" />
        <line x1="8" y1="17" x2="16" y2="17" />
        <line x1="12" y1="9" x2="12" y2="21" />
      </svg>
    );
  }

  // Code/JSON/Markdown
  if (
    mediaType === 'application/json' ||
    mediaType === 'text/markdown' ||
    mediaType === 'text/plain'
  ) {
    return (
      <svg {...iconProps}>
        <polyline points="16,18 22,12 16,6" />
        <polyline points="8,6 2,12 8,18" />
      </svg>
    );
  }

  // Default document icon
  return (
    <svg {...iconProps}>
      <path d="M14 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V8z" />
      <polyline points="14,2 14,8 20,8" />
    </svg>
  );
}

/**
 * Truncate hash for display
 */
function truncateHash(hash: string, length: number = 16): string {
  if (hash.length <= length) return hash;
  return hash.slice(0, length) + '...';
}

/**
 * Get human-readable media type label
 */
function getMediaTypeLabel(mediaType: string): string {
  const typeMap: Record<string, string> = {
    'application/pdf': 'PDF',
    'application/json': 'JSON',
    'text/markdown': 'Markdown',
    'text/plain': 'Text',
    'text/csv': 'CSV',
    'text/html': 'HTML',
    'image/png': 'PNG',
    'image/jpeg': 'JPEG',
    'image/gif': 'GIF',
    'image/svg+xml': 'SVG',
    'image/webp': 'WebP',
    'application/msword': 'Word',
    'application/vnd.openxmlformats-officedocument.wordprocessingml.document': 'Word',
    'application/vnd.ms-excel': 'Excel',
    'application/vnd.openxmlformats-officedocument.spreadsheetml.sheet': 'Excel',
    'application/zip': 'ZIP',
    'application/gzip': 'GZIP',
  };

  return typeMap[mediaType] || mediaType.split('/').pop()?.toUpperCase() || 'File';
}

export function AttachmentPreview({ attachment, onRemove }: AttachmentPreviewProps): JSX.Element {
  return (
    <div className={styles.previewContainer}>
      <div className={styles.fileIcon}>{getFileIcon(attachment.media_type)}</div>

      <div className={styles.fileInfo}>
        <div className={styles.fileName} title={attachment.filename}>
          {attachment.filename}
        </div>

        <div className={styles.fileMeta}>
          <span className={styles.fileMetaItem}>{getMediaTypeLabel(attachment.media_type)}</span>
          <span className={styles.fileMetaItem}>{formatFileSize(attachment.size_bytes)}</span>
          <span className={styles.fileMetaItem}>
            <span className={styles.fileHash} title={attachment.content_hash}>
              {truncateHash(attachment.content_hash, 20)}
            </span>
          </span>
        </div>
      </div>

      {onRemove && (
        <button
          type="button"
          className={styles.removeButton}
          onClick={onRemove}
          aria-label="Remove attachment"
        >
          Remove
        </button>
      )}
    </div>
  );
}

export default AttachmentPreview;
