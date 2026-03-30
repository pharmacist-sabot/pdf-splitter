<script setup lang="ts">
/**
 * FileCard — "ready" state view.
 *
 * Displays metadata for the selected PDF file, lets the user optionally
 * override the output directory, and exposes a prominent "Split PDF" button
 * to begin processing.
 *
 * Props
 * ─────
 * • `fileName`       — basename of the selected PDF, e.g. `"report.pdf"`.
 * • `pageCount`      — number of pages (from `get_page_count`).
 * • `fileSizeFormatted` — human-readable file size, e.g. `"2.4 MB"`.
 * • `outputDirShort` — abbreviated display path for the output directory.
 * • `busy`           — disables all controls while an async op is in flight.
 *
 * Events
 * ──────
 * • `split`           — user clicked "Split PDF".
 * • `change-file`     — user clicked the dismiss / change file button.
 * • `change-output`   — user clicked "Change" next to the output directory.
 */

defineProps<{
    /** Basename of the selected file, e.g. `"annual-report.pdf"`. */
    fileName: string
    /** Number of pages reported by `get_page_count`. */
    pageCount: number
    /** Human-readable file size string, e.g. `"2.4 MB"`. */
    fileSizeFormatted: string
    /** Abbreviated output directory display path. */
    outputDirShort: string
    /** Disables all interactive controls when true. */
    busy: boolean
}>()

const emit = defineEmits<{
    /** Begin the split operation. */
    split: []
    /** Dismiss the current file and return to idle state. */
    'change-file': []
    /** Open the output directory picker. */
    'change-output': []
}>()

function onSplit(): void {
    emit('split')
}

function onChangeFile(): void {
    emit('change-file')
}

function onChangeOutput(): void {
    emit('change-output')
}
</script>

<template>
<div class="file-card">

    <!-- ── File info row ─────────────────────────────────────────────────── -->
    <div class="file-info">
        <!-- PDF document icon -->
        <div class="file-icon" aria-hidden="true">
            <svg viewBox="0 0 40 48" fill="none" xmlns="http://www.w3.org/2000/svg" width="40" height="48">
                <rect x="0" y="0" width="32" height="44" rx="4" fill="currentColor" class="doc-body" />
                <path d="M24 0 L32 8 L24 8 Z" fill="currentColor" class="doc-fold-shadow" />
                <path d="M24 0 L32 8 L24 8 Z" fill="currentColor" class="doc-fold" />
                <rect x="0" y="28" width="32" height="14" rx="3" fill="currentColor" class="doc-band" />
                <rect x="5" y="33" width="5" height="1.8" rx="0.9" fill="white" opacity="0.95" />
                <rect x="12" y="33" width="4" height="1.8" rx="0.9" fill="white" opacity="0.95" />
                <rect x="18" y="33" width="4" height="1.8" rx="0.9" fill="white" opacity="0.95" />
                <rect x="5" y="10" width="18" height="1.5" rx="0.75" fill="currentColor" class="doc-line" />
                <rect x="5" y="14" width="14" height="1.5" rx="0.75" fill="currentColor" class="doc-line" />
                <rect x="5" y="18" width="16" height="1.5" rx="0.75" fill="currentColor" class="doc-line" />
                <rect x="5" y="22" width="12" height="1.5" rx="0.75" fill="currentColor" class="doc-line" />
            </svg>
        </div>

        <!-- Filename + metadata -->
        <div class="file-meta">
            <span class="file-name truncate" :title="fileName">{{ fileName }}</span>
            <div class="file-details">
                <span class="file-detail">
                    <svg viewBox="0 0 16 16" fill="none" xmlns="http://www.w3.org/2000/svg" width="12" height="12"
                        aria-hidden="true">
                        <path
                            d="M2 3a1 1 0 0 1 1-1h10a1 1 0 0 1 1 1v10a1 1 0 0 1-1 1H3a1 1 0 0 1-1-1V3Zm1.5 1.5v7h9v-7h-9Z"
                            fill="currentColor" />
                    </svg>
                    <span>{{ pageCount }} {{ pageCount === 1 ? 'page' : 'pages' }}</span>
                </span>
                <span v-if="fileSizeFormatted" class="file-detail-dot" aria-hidden="true">·</span>
                <span v-if="fileSizeFormatted" class="file-detail">{{ fileSizeFormatted }}</span>
            </div>
        </div>

        <!-- Dismiss / change file button -->
        <button type="button" class="btn-icon dismiss-btn" :disabled="busy" :aria-label="`Remove ${fileName}`"
            title="Choose a different file" @click="onChangeFile">
            <svg viewBox="0 0 20 20" fill="none" xmlns="http://www.w3.org/2000/svg" class="icon-md" aria-hidden="true">
                <path fill-rule="evenodd" clip-rule="evenodd"
                    d="M4.293 4.293a1 1 0 0 1 1.414 0L10 8.586l4.293-4.293a1 1 0 1 1 1.414 1.414L11.414 10l4.293 4.293a1 1 0 0 1-1.414 1.414L10 11.414l-4.293 4.293a1 1 0 0 1-1.414-1.414L8.586 10 4.293 5.707a1 1 0 0 1 0-1.414Z"
                    fill="currentColor" />
            </svg>
        </button>
    </div>

    <!-- ── Separator ─────────────────────────────────────────────────────── -->
    <div class="separator" role="separator" />

    <!-- ── Output directory row ──────────────────────────────────────────── -->
    <div class="output-row">
        <div class="output-row__label-group">
            <div class="output-row__icon" aria-hidden="true">
                <svg viewBox="0 0 20 20" fill="none" xmlns="http://www.w3.org/2000/svg" width="18" height="18">
                    <path fill-rule="evenodd" clip-rule="evenodd"
                        d="M2 5a2 2 0 0 1 2-2h3.586A2 2 0 0 1 9 3.586L10.414 5H16a2 2 0 0 1 2 2v8a2 2 0 0 1-2 2H4a2 2 0 0 1-2-2V5Zm2-.5a.5.5 0 0 0-.5.5v10a.5.5 0 0 0 .5.5h12a.5.5 0 0 0 .5-.5V7a.5.5 0 0 0-.5-.5H10a.5.5 0 0 0-.354-.146L8.232 4.94A.5.5 0 0 0 7.879 4.5H4Z"
                        fill="currentColor" />
                </svg>
            </div>
            <div class="output-row__text">
                <span class="output-row__heading">Output folder</span>
                <span class="output-row__path truncate" :title="outputDirShort">
                    {{ outputDirShort || 'Same folder as PDF' }}
                </span>
            </div>
        </div>

        <button type="button" class="btn-secondary output-change-btn" :disabled="busy" @click="onChangeOutput">
            Change
        </button>
    </div>

    <!-- ── Separator ─────────────────────────────────────────────────────── -->
    <div class="separator" role="separator" />

    <!-- ── Action row ─────────────────────────────────────────────────────── -->
    <div class="action-row">
        <!-- Page count badge -->
        <span class="badge badge-info page-badge" aria-label="{{ pageCount }} pages will be created">
            <svg viewBox="0 0 14 14" fill="none" xmlns="http://www.w3.org/2000/svg" width="11" height="11"
                aria-hidden="true">
                <path
                    d="M2.5 1.5A1 1 0 0 1 3.5.5h5.086a1 1 0 0 1 .707.293l2.414 2.414a1 1 0 0 1 .293.707V12.5a1 1 0 0 1-1 1h-7.5a1 1 0 0 1-1-1v-11Z"
                    fill="currentColor" opacity="0.5" />
            </svg>
            {{ pageCount }} {{ pageCount === 1 ? 'page' : 'pages' }}
        </span>

        <!-- Primary action -->
        <button type="button" class="btn-primary btn-lg split-btn" :disabled="busy" @click="onSplit">
            <svg viewBox="0 0 20 20" fill="none" xmlns="http://www.w3.org/2000/svg" class="icon-md" aria-hidden="true">
                <!-- Scissors icon -->
                <path fill-rule="evenodd" clip-rule="evenodd"
                    d="M5.5 4a1.5 1.5 0 1 0 0 3 1.5 1.5 0 0 0 0-3ZM2 5.5a3.5 3.5 0 0 1 6.025-2.427L10 5.025l1.975-1.952A3.5 3.5 0 1 1 14.5 7a3.487 3.487 0 0 1-2-.623L11.414 7.5 14 10.086A3.5 3.5 0 1 1 12.525 11.9L10 9.414 7.475 11.9A3.5 3.5 0 1 1 5.5 7a3.487 3.487 0 0 1 2 .623L9.086 6 8.025 4.927A3.49 3.49 0 0 1 5.5 5.5ZM14.5 10a1.5 1.5 0 1 0 0 3 1.5 1.5 0 0 0 0-3ZM14.5 4a1.5 1.5 0 1 0 0 3 1.5 1.5 0 0 0 0-3Z"
                    fill="currentColor" />
            </svg>
            Split PDF
        </button>
    </div>
</div>
</template>

<style scoped>
/* ── Card wrapper ─────────────────────────────────────────────────────────────── */

.file-card {
    display: flex;
    flex-direction: column;
    gap: 0;
    width: 100%;
    background: var(--color-surface);
    backdrop-filter: var(--blur-lg);
    -webkit-backdrop-filter: var(--blur-lg);
    border: 1px solid var(--color-border);
    border-radius: var(--radius-xl);
    box-shadow: var(--shadow-md);
    overflow: hidden;
}

/* ── File info row ────────────────────────────────────────────────────────────── */

.file-info {
    display: flex;
    align-items: center;
    gap: var(--space-4);
    padding: var(--space-5) var(--space-6);
}

/* ── PDF document icon ────────────────────────────────────────────────────────── */

.file-icon {
    flex-shrink: 0;
    color: var(--color-accent);
    filter: drop-shadow(0 2px 8px var(--color-accent-glow));
}

.doc-body {
    opacity: 0.15;
}

.doc-fold-shadow {
    opacity: 0;
}

.doc-fold {
    opacity: 0.08;
}

.doc-band {
    opacity: 0.80;
}

.doc-line {
    opacity: 0.18;
}

/* ── File metadata ────────────────────────────────────────────────────────────── */

.file-meta {
    flex: 1;
    min-width: 0;
    display: flex;
    flex-direction: column;
    gap: var(--space-1);
}

.file-name {
    font-size: var(--text-md);
    font-weight: var(--weight-semibold);
    letter-spacing: var(--tracking-tight);
    color: var(--color-text-primary);
    display: block;
    max-width: 360px;
}

.file-details {
    display: flex;
    align-items: center;
    gap: var(--space-2);
}

.file-detail {
    display: inline-flex;
    align-items: center;
    gap: 4px;
    font-size: var(--text-sm);
    color: var(--color-text-tertiary);
    font-weight: var(--weight-regular);
}

.file-detail-dot {
    font-size: var(--text-sm);
    color: var(--color-text-quaternary);
    line-height: 1;
}

/* ── Dismiss button ───────────────────────────────────────────────────────────── */

.dismiss-btn {
    flex-shrink: 0;
    width: 30px;
    height: 30px;
    border-radius: var(--radius-sm);
    color: var(--color-text-quaternary);
    transition:
        background-color var(--duration-fast) var(--ease-out),
        color var(--duration-fast) var(--ease-out);
}

.dismiss-btn:hover:not(:disabled) {
    background: var(--color-error-subtle);
    color: var(--color-error-text);
}

/* ── Output directory row ─────────────────────────────────────────────────────── */

.output-row {
    display: flex;
    align-items: center;
    justify-content: space-between;
    gap: var(--space-4);
    padding: var(--space-4) var(--space-6);
    background: var(--color-surface-inset);
}

.output-row__label-group {
    display: flex;
    align-items: center;
    gap: var(--space-3);
    min-width: 0;
    flex: 1;
}

.output-row__icon {
    flex-shrink: 0;
    color: var(--color-text-tertiary);
}

.output-row__text {
    display: flex;
    flex-direction: column;
    gap: 2px;
    min-width: 0;
    flex: 1;
}

.output-row__heading {
    font-size: var(--text-xs);
    font-weight: var(--weight-semibold);
    letter-spacing: var(--tracking-wider);
    text-transform: uppercase;
    color: var(--color-text-tertiary);
    line-height: 1;
}

.output-row__path {
    font-size: var(--text-sm);
    color: var(--color-text-secondary);
    font-family: var(--font-mono);
    letter-spacing: 0;
    display: block;
    max-width: 340px;
}

.output-change-btn {
    flex-shrink: 0;
    font-size: var(--text-sm);
    padding: var(--space-2) var(--space-4);
    border-radius: var(--radius-sm);
    height: 30px;
}

/* ── Action row ───────────────────────────────────────────────────────────────── */

.action-row {
    display: flex;
    align-items: center;
    justify-content: space-between;
    padding: var(--space-4) var(--space-6);
    gap: var(--space-4);
}

.page-badge {
    font-size: var(--text-xs);
}

.split-btn {
    min-width: 140px;
    height: 44px;
    font-size: var(--text-base);
    border-radius: var(--radius-md);
    padding: var(--space-3) var(--space-6);
    box-shadow:
        var(--shadow-sm),
        0 4px 20px var(--color-accent-glow);
    transition:
        background var(--duration-fast) var(--ease-out),
        box-shadow var(--duration-fast) var(--ease-out),
        transform var(--duration-fast) var(--ease-spring),
        opacity var(--duration-fast) var(--ease-out);
}

.split-btn:hover:not(:disabled) {
    box-shadow:
        var(--shadow-md),
        0 8px 28px var(--color-accent-glow);
    transform: translateY(-1px);
}

.split-btn:active:not(:disabled) {
    transform: scale(0.97) translateY(0);
    box-shadow: var(--shadow-sm);
}
</style>
