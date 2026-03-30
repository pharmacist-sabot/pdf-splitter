<script setup lang="ts">
/**
 * FileCard — "ready" state view.
 *
 * Terminal aesthetic: displays file metadata in a monospace layout that
 * mimics a `ls -la` or `file` command output.  The output-directory row
 * uses a `>` prompt prefix; the split button reads like a CLI command.
 *
 * Props / Events unchanged — only the visual presentation is redesigned.
 */

defineProps<{
    fileName: string
    pageCount: number
    fileSizeFormatted: string
    outputDirShort: string
    busy: boolean
}>()

const emit = defineEmits<{
    split: []
    'change-file': []
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
            <svg viewBox="0 0 36 44" fill="none" xmlns="http://www.w3.org/2000/svg" width="36" height="44">
                <rect x="0" y="0" width="28" height="40" rx="3" fill="currentColor" class="doc-body" />
                <path d="M20 0 L28 8 L20 8 Z" fill="currentColor" class="doc-fold" />
                <rect x="0" y="26" width="28" height="13" rx="2" fill="currentColor" class="doc-band" />
                <rect x="3" y="30.5" width="5" height="1.6" rx="0.8" fill="currentColor" class="doc-label" />
                <rect x="10" y="30.5" width="4" height="1.6" rx="0.8" fill="currentColor" class="doc-label" />
                <rect x="16" y="30.5" width="4" height="1.6" rx="0.8" fill="currentColor" class="doc-label" />
                <rect x="4" y="9" width="16" height="1.4" rx="0.7" fill="currentColor" class="doc-line" />
                <rect x="4" y="13" width="13" height="1.4" rx="0.7" fill="currentColor" class="doc-line" />
                <rect x="4" y="17" width="15" height="1.4" rx="0.7" fill="currentColor" class="doc-line" />
                <rect x="4" y="21" width="10" height="1.4" rx="0.7" fill="currentColor" class="doc-line" />
            </svg>
        </div>

        <!-- Filename + metadata -->
        <div class="file-meta">
            <span class="file-prompt" aria-hidden="true">$</span>
            <div class="file-meta__text">
                <span class="file-name truncate" :title="fileName">{{ fileName }}</span>
                <div class="file-details">
                    <span class="file-detail">
                        <span class="file-detail__key">pages</span>
                        <span class="file-detail__sep">:</span>
                        <span class="file-detail__val">{{ pageCount }}</span>
                    </span>
                    <span v-if="fileSizeFormatted" class="file-detail__dot" aria-hidden="true">·</span>
                    <span v-if="fileSizeFormatted" class="file-detail">
                        <span class="file-detail__key">size</span>
                        <span class="file-detail__sep">:</span>
                        <span class="file-detail__val">{{ fileSizeFormatted }}</span>
                    </span>
                </div>
            </div>
        </div>

        <!-- Dismiss button -->
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
            <!-- Terminal > prompt -->
            <span class="output-row__prompt" aria-hidden="true">&gt;</span>
            <div class="output-row__text">
                <span class="output-row__heading">output</span>
                <span class="output-row__path truncate" :title="outputDirShort">
                    {{ outputDirShort || '(same folder as pdf)' }}
                </span>
            </div>
        </div>

        <button type="button" class="btn-secondary output-change-btn" :disabled="busy" @click="onChangeOutput">
            [change]
        </button>
    </div>

    <!-- ── Separator ─────────────────────────────────────────────────────── -->
    <div class="separator" role="separator" />

    <!-- ── Action row ─────────────────────────────────────────────────────── -->
    <div class="action-row">
        <!-- Page count badge -->
        <span class="badge badge-info page-badge">
            <svg viewBox="0 0 14 14" fill="none" xmlns="http://www.w3.org/2000/svg" width="10" height="10"
                aria-hidden="true">
                <path
                    d="M2.5 1.5A1 1 0 0 1 3.5.5h5.086a1 1 0 0 1 .707.293l2.414 2.414a1 1 0 0 1 .293.707V12.5a1 1 0 0 1-1 1h-7.5a1 1 0 0 1-1-1v-11Z"
                    fill="currentColor" opacity="0.55" />
            </svg>
            {{ pageCount }}&nbsp;{{ pageCount === 1 ? 'page' : 'pages' }}
        </span>

        <!-- Split button -->
        <button type="button" class="btn-primary btn-lg split-btn" :disabled="busy" @click="onSplit">
            <span class="split-btn__prompt" aria-hidden="true">$</span>
            split-pdf
            <svg viewBox="0 0 16 16" fill="none" xmlns="http://www.w3.org/2000/svg" class="icon-sm split-btn__arrow"
                aria-hidden="true">
                <path fill-rule="evenodd" clip-rule="evenodd"
                    d="M3.75 8a.75.75 0 0 1 .75-.75h5.19L7.22 4.78a.75.75 0 0 1 1.06-1.06l3.5 3.5a.75.75 0 0 1 0 1.06l-3.5 3.5a.75.75 0 0 1-1.06-1.06l2.47-2.47H4.5A.75.75 0 0 1 3.75 8Z"
                    fill="currentColor" />
            </svg>
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
    border: 1px solid var(--color-border);
    border-radius: var(--radius-xl);
    box-shadow: var(--shadow-md);
    overflow: hidden;
}

/* ── File info row ────────────────────────────────────────────────────────────── */

.file-info {
    display: flex;
    align-items: center;
    gap: var(--space-3);
    padding: var(--space-4) var(--space-5);
}

/* ── PDF icon ─────────────────────────────────────────────────────────────────── */

.file-icon {
    flex-shrink: 0;
    color: var(--color-accent);
    filter: drop-shadow(0 0 6px rgba(57, 211, 83, 0.2));
}

.doc-body {
    opacity: 0.12;
}

.doc-fold {
    opacity: 0.07;
}

.doc-band {
    opacity: 0.75;
}

.doc-label {
    opacity: 0.9;
    fill: var(--color-text-on-accent);
}

.doc-line {
    opacity: 0.14;
}

/* ── Prompt character ─────────────────────────────────────────────────────────── */

.file-prompt {
    font-family: var(--font-mono);
    font-size: var(--text-lg);
    font-weight: var(--weight-bold);
    color: var(--color-accent);
    text-shadow: 0 0 8px var(--color-accent-glow);
    flex-shrink: 0;
    line-height: 1;
}

/* ── Metadata ─────────────────────────────────────────────────────────────────── */

.file-meta {
    flex: 1;
    min-width: 0;
    display: flex;
    align-items: center;
    gap: var(--space-3);
}

.file-meta__text {
    display: flex;
    flex-direction: column;
    gap: 3px;
    min-width: 0;
    flex: 1;
}

.file-name {
    font-size: var(--text-md);
    font-weight: var(--weight-semibold);
    color: var(--color-text-primary);
    font-family: var(--font-mono);
    display: block;
    max-width: 340px;
}

.file-details {
    display: flex;
    align-items: center;
    gap: var(--space-2);
}

.file-detail {
    display: inline-flex;
    align-items: center;
    gap: 2px;
    font-size: var(--text-xs);
    font-family: var(--font-mono);
}

.file-detail__key {
    color: var(--color-text-tertiary);
}

.file-detail__sep {
    color: var(--color-border-strong);
}

.file-detail__val {
    color: var(--color-text-secondary);
    font-weight: var(--weight-medium);
}

.file-detail__dot {
    font-size: var(--text-xs);
    color: var(--color-text-quaternary);
    line-height: 1;
}

/* ── Dismiss button ───────────────────────────────────────────────────────────── */

.dismiss-btn {
    flex-shrink: 0;
    width: 28px;
    height: 28px;
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
    padding: var(--space-3) var(--space-5);
    background: var(--color-surface-inset);
}

.output-row__label-group {
    display: flex;
    align-items: center;
    gap: var(--space-3);
    min-width: 0;
    flex: 1;
}

/* Terminal > prompt */
.output-row__prompt {
    font-family: var(--font-mono);
    font-size: var(--text-md);
    font-weight: var(--weight-bold);
    color: var(--color-text-tertiary);
    flex-shrink: 0;
    line-height: 1;
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
    color: var(--color-text-quaternary);
    font-family: var(--font-mono);
    line-height: 1;
}

.output-row__path {
    font-size: var(--text-sm);
    color: var(--color-text-secondary);
    font-family: var(--font-mono);
    display: block;
    max-width: 320px;
}

.output-change-btn {
    flex-shrink: 0;
    font-size: var(--text-xs);
    font-family: var(--font-mono);
    letter-spacing: 0.02em;
    padding: var(--space-1) var(--space-3);
    border-radius: var(--radius-sm);
    height: 26px;
    color: var(--color-text-tertiary);
    border-color: var(--color-border-subtle);
}

.output-change-btn:hover:not(:disabled) {
    color: var(--color-accent);
    border-color: rgba(57, 211, 83, 0.35);
    box-shadow: 0 0 8px rgba(57, 211, 83, 0.1);
}

/* ── Action row ───────────────────────────────────────────────────────────────── */

.action-row {
    display: flex;
    align-items: center;
    justify-content: space-between;
    padding: var(--space-4) var(--space-5);
    gap: var(--space-4);
}

.page-badge {
    font-size: var(--text-xs);
    font-family: var(--font-mono);
}

/* ── Split button ─────────────────────────────────────────────────────────────── */

.split-btn {
    min-width: 148px;
    height: 40px;
    font-size: var(--text-sm);
    border-radius: var(--radius-md);
    padding: var(--space-3) var(--space-5);
    font-family: var(--font-mono);
    letter-spacing: 0;
    gap: var(--space-2);
    box-shadow:
        var(--shadow-sm),
        0 0 16px rgba(57, 211, 83, 0.15);
    transition:
        background var(--duration-fast) var(--ease-out),
        box-shadow var(--duration-fast) var(--ease-out),
        transform var(--duration-fast) var(--ease-spring),
        opacity var(--duration-fast) var(--ease-out);
}

.split-btn:hover:not(:disabled) {
    box-shadow:
        var(--shadow-md),
        0 0 24px rgba(57, 211, 83, 0.28);
    transform: translateY(-1px);
}

.split-btn:active:not(:disabled) {
    transform: scale(0.97) translateY(0);
    box-shadow: var(--shadow-sm);
}

.split-btn__prompt {
    color: rgba(13, 17, 23, 0.6);
    font-weight: var(--weight-bold);
}

.split-btn__arrow {
    margin-left: var(--space-1);
    opacity: 0.75;
}
</style>
