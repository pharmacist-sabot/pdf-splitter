<script setup lang="ts">
/**
 * ErrorView — "error" state view.
 *
 * Terminal aesthetic: the error output mimics stderr from a CLI command.
 * The header shows `✗ <kind>` in red, the message is displayed in a
 * terminal code block, and the hint reads like a man-page note.
 *
 * Props / computed logic unchanged — only the visual presentation is redesigned.
 */

import { computed } from 'vue'
import type { PdfError } from '@/types'

// ── Props & emits ──────────────────────────────────────────────────────────────

const props = defineProps<{
    /** Human-readable error description forwarded from `PdfError.message`. */
    message: string
    /** Machine-readable error kind — used to render contextual guidance. */
    kind?: PdfError['kind']
}>()

const emit = defineEmits<{
    retry: []
    dismiss: []
}>()

// ── Computed ───────────────────────────────────────────────────────────────────

const hint = computed<string>(() => {
    switch (props.kind) {
        case 'FileNotFound':
            return 'The file may have been moved, renamed, or deleted. Please select a valid PDF file.'
        case 'InvalidPdf':
            return 'The selected file could not be parsed as a PDF. Make sure the file is not corrupted or password-protected.'
        case 'NoPages':
            return 'The PDF document appears to have no pages. Please select a different file.'
        case 'Io':
            return 'A filesystem error occurred. Check that the output directory is accessible and that you have write permissions.'
        case 'Internal':
            return 'An unexpected internal error occurred. Please try again or report the issue if it persists.'
        default:
            return 'Please try again with a different file or output directory.'
    }
})

const kindLabel = computed<string>(() => {
    switch (props.kind) {
        case 'FileNotFound': return 'FileNotFound'
        case 'InvalidPdf': return 'InvalidPdf'
        case 'NoPages': return 'NoPages'
        case 'Io': return 'IOError'
        case 'Internal': return 'InternalError'
        default: return 'Error'
    }
})

// ── Handlers ───────────────────────────────────────────────────────────────────

function onRetry(): void {
    emit('retry')
}

function onDismiss(): void {
    emit('dismiss')
}
</script>

<template>
<div class="error-view" role="alert" aria-live="assertive">

    <!-- ── Error header (stderr line) ────────────────────────────────────── -->
    <div class="error-header">
        <span class="error-header__x" aria-hidden="true">✗</span>
        <div class="error-header__text">
            <span class="error-header__kind">{{ kindLabel }}</span>
            <span class="error-header__sep" aria-hidden="true">:</span>
            <span class="error-header__label">process exited with error</span>
        </div>
    </div>

    <!-- ── Error output block ─────────────────────────────────────────────── -->
    <div class="error-block">
        <!-- stderr label -->
        <div class="error-block__label" aria-hidden="true">
            <span class="error-block__stream">stderr</span>
        </div>

        <!-- Error message (selectable) -->
        <p class="error-message" data-selectable>
            <span class="error-message__prefix" aria-hidden="true">error:</span>
            {{ message }}
        </p>
    </div>

    <!-- ── Hint / note ────────────────────────────────────────────────────── -->
    <div class="error-note">
        <span class="error-note__prefix" aria-hidden="true">#</span>
        <p class="error-note__text">{{ hint }}</p>
    </div>

    <!-- ── Separator ──────────────────────────────────────────────────────── -->
    <div class="separator error-separator" role="separator" />

    <!-- ── Actions ────────────────────────────────────────────────────────── -->
    <div class="error-actions">
        <button type="button" class="btn-ghost error-actions__dismiss" @click="onDismiss">
            [dismiss]
        </button>

        <button type="button" class="btn-primary error-actions__retry" @click="onRetry">
            <span class="error-actions__prompt" aria-hidden="true">$</span>
            try-again
            <svg viewBox="0 0 16 16" fill="none" xmlns="http://www.w3.org/2000/svg" class="icon-sm" aria-hidden="true">
                <path fill-rule="evenodd" clip-rule="evenodd"
                    d="M3.5 8a4.5 4.5 0 0 1 7.854-3H9.25a.75.75 0 0 0 0 1.5h3.25a.75.75 0 0 0 .75-.75V2.5a.75.75 0 0 0-1.5 0v1.386A6 6 0 1 0 14 8a.75.75 0 0 0-1.5 0A4.5 4.5 0 1 1 3.5 8Z"
                    fill="currentColor" />
            </svg>
        </button>
    </div>

</div>
</template>

<style scoped>
/* ── View wrapper ─────────────────────────────────────────────────────────────── */

.error-view {
    display: flex;
    flex-direction: column;
    gap: var(--space-4);
    width: 100%;
    padding: var(--space-2) 0;
}

/* ── Error header ─────────────────────────────────────────────────────────────── */

.error-header {
    display: flex;
    align-items: center;
    gap: var(--space-3);
}

/* ✗ symbol */
.error-header__x {
    font-family: var(--font-mono);
    font-size: 22px;
    font-weight: var(--weight-bold);
    color: var(--color-error);
    text-shadow: 0 0 10px rgba(248, 81, 73, 0.45);
    flex-shrink: 0;
    line-height: 1;
    animation: error-icon-in var(--duration-slow) var(--ease-spring) forwards;
}

@keyframes error-icon-in {
    0% {
        transform: scale(0.5);
        opacity: 0;
    }

    70% {
        transform: scale(1.15);
        opacity: 1;
    }

    100% {
        transform: scale(1);
        opacity: 1;
    }
}

.error-header__text {
    display: flex;
    align-items: baseline;
    gap: 0;
    font-family: var(--font-mono);
    font-size: var(--text-base);
    flex-wrap: wrap;
    gap: 2px;
}

.error-header__kind {
    font-weight: var(--weight-bold);
    color: var(--color-error-text);
}

.error-header__sep {
    color: var(--color-text-quaternary);
    margin: 0 2px;
}

.error-header__label {
    color: var(--color-text-tertiary);
    font-weight: var(--weight-regular);
}

/* ── Error block (stderr output) ──────────────────────────────────────────────── */

.error-block {
    border: 1px solid rgba(248, 81, 73, 0.25);
    border-radius: var(--radius-md);
    background: rgba(248, 81, 73, 0.05);
    overflow: hidden;
}

/* "stderr" label bar */
.error-block__label {
    display: flex;
    align-items: center;
    gap: var(--space-2);
    padding: var(--space-1) var(--space-3);
    background: rgba(248, 81, 73, 0.08);
    border-bottom: 1px solid rgba(248, 81, 73, 0.15);
}

.error-block__stream {
    font-family: var(--font-mono);
    font-size: var(--text-xs);
    color: var(--color-error-text);
    opacity: 0.65;
    letter-spacing: var(--tracking-wide);
    text-transform: uppercase;
    font-weight: var(--weight-semibold);
}

/* Message text */
.error-message {
    font-family: var(--font-mono);
    font-size: var(--text-sm);
    color: var(--color-error-text);
    line-height: var(--leading-snug);
    padding: var(--space-3) var(--space-4);
    /* Allow user to copy error for bug reports */
    user-select: text;
    cursor: text;
    word-break: break-all;
    display: flex;
    align-items: flex-start;
    gap: var(--space-2);
    flex-wrap: wrap;
}

.error-message__prefix {
    color: var(--color-error);
    font-weight: var(--weight-bold);
    flex-shrink: 0;
    opacity: 0.75;
}

/* ── Hint / note ──────────────────────────────────────────────────────────────── */

.error-note {
    display: flex;
    align-items: flex-start;
    gap: var(--space-2);
}

.error-note__prefix {
    font-family: var(--font-mono);
    font-size: var(--text-sm);
    color: var(--color-text-quaternary);
    flex-shrink: 0;
    line-height: var(--leading-relaxed);
    opacity: 0.6;
}

.error-note__text {
    font-family: var(--font-mono);
    font-size: var(--text-sm);
    color: var(--color-text-tertiary);
    line-height: var(--leading-relaxed);
    flex: 1;
}

/* ── Separator ────────────────────────────────────────────────────────────────── */

.error-separator {
    background: var(--color-separator);
}

/* ── Actions ──────────────────────────────────────────────────────────────────── */

.error-actions {
    display: flex;
    align-items: center;
    justify-content: flex-end;
    gap: var(--space-3);
}

.error-actions__dismiss {
    font-family: var(--font-mono);
    font-size: var(--text-sm);
    color: var(--color-text-quaternary);
    padding: var(--space-2) var(--space-4);
    border-radius: var(--radius-md);
    height: 36px;
    letter-spacing: 0.02em;
}

.error-actions__dismiss:hover:not(:disabled) {
    color: var(--color-text-secondary);
    background: var(--color-surface-hover);
}

.error-actions__retry {
    min-width: 148px;
    height: 38px;
    font-size: var(--text-sm);
    font-family: var(--font-mono);
    border-radius: var(--radius-md);
    padding: var(--space-2) var(--space-5);
    gap: var(--space-2);
    box-shadow:
        var(--shadow-sm),
        0 0 14px rgba(57, 211, 83, 0.15);
    transition:
        background var(--duration-fast) var(--ease-out),
        box-shadow var(--duration-fast) var(--ease-out),
        transform var(--duration-fast) var(--ease-spring);
}

.error-actions__retry:hover:not(:disabled) {
    box-shadow:
        var(--shadow-md),
        0 0 22px rgba(57, 211, 83, 0.28);
    transform: translateY(-1px);
}

.error-actions__retry:active:not(:disabled) {
    transform: scale(0.97) translateY(0);
    box-shadow: var(--shadow-sm);
}

.error-actions__prompt {
    color: rgba(13, 17, 23, 0.6);
    font-weight: var(--weight-bold);
}

/* ── Reduced motion ───────────────────────────────────────────────────────────── */

@media (prefers-reduced-motion: reduce) {
    .error-header__x {
        animation: none;
    }
}
</style>
