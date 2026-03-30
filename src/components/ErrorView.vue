<script setup lang="ts">
/**
 * ErrorView — "error" state view.
 *
 * Displayed whenever a Tauri command rejects with a `PdfError`.  Shows a
 * clear, human-readable error message together with contextual guidance and
 * two recovery actions:
 *
 * • "Try Again"      — re-opens the file picker so the user can select a
 *                      different (or the same) PDF.
 * • "Dismiss"        — returns to the idle drop-zone without opening the
 *                      picker immediately.
 *
 * Props
 * ─────
 * • `message`  — human-readable error string forwarded from `PdfError.message`.
 * • `kind`     — machine-readable discriminant from `PdfError.kind`, used to
 *                render a contextual hint below the main message.
 *
 * Events
 * ──────
 * • `retry`   — user clicked "Try Again" (open file picker).
 * • `dismiss` — user clicked "Dismiss" (return to idle, no picker).
 */

import { computed } from 'vue'
import type { PdfError } from '@/types'

// ── Props & emits ──────────────────────────────────────────────────────────────

const props = defineProps<{
    /** Human-readable error description, e.g. `"File not found: /tmp/x.pdf"`. */
    message: string
    /**
     * Machine-readable error kind — a variant name from `PdfError`.
     * Used to show contextual guidance below the main message.
     */
    kind?: PdfError['kind']
}>()

const emit = defineEmits<{
    /** Open the file picker so the user can select a new PDF. */
    retry: []
    /** Return to idle without opening the picker. */
    dismiss: []
}>()

// ── Computed ───────────────────────────────────────────────────────────────────

/**
 * Contextual hint text shown below the raw error message.
 * Maps each `PdfError` kind to actionable guidance.
 */
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

/**
 * Emoji / icon label for the error kind — subtle visual differentiation
 * without requiring additional icon assets.
 */
const severityLabel = computed<string>(() => {
    switch (props.kind) {
        case 'FileNotFound':
            return 'File not found'
        case 'InvalidPdf':
            return 'Invalid PDF'
        case 'NoPages':
            return 'Empty document'
        case 'Io':
            return 'Filesystem error'
        default:
            return 'Error'
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

    <!-- ── Error icon ──────────────────────────────────────────────────────── -->
    <div class="error-icon-wrap" aria-hidden="true">
        <!-- Outer glow ring -->
        <div class="error-icon-glow" />

        <!-- Icon circle -->
        <div class="error-icon-circle">
            <svg viewBox="0 0 32 32" fill="none" xmlns="http://www.w3.org/2000/svg" width="32" height="32">
                <!-- Background circle -->
                <circle cx="16" cy="16" r="16" fill="currentColor" class="error-circle" />
                <!-- Exclamation mark — vertical bar -->
                <rect x="14.75" y="8" width="2.5" height="10" rx="1.25" fill="white" />
                <!-- Exclamation mark — dot -->
                <circle cx="16" cy="22.5" r="1.5" fill="white" />
            </svg>
        </div>
    </div>

    <!-- ── Text block ──────────────────────────────────────────────────────── -->
    <div class="error-text">
        <!-- Kind badge -->
        <span v-if="kind" class="badge badge-error error-kind-badge" aria-label="Error type">
            {{ severityLabel }}
        </span>

        <!-- Main message -->
        <p class="error-message" data-selectable>
            {{ message }}
        </p>

        <!-- Contextual hint -->
        <p class="error-hint">
            {{ hint }}
        </p>
    </div>

    <!-- ── Separator ──────────────────────────────────────────────────────── -->
    <div class="separator error-separator" role="separator" />

    <!-- ── Action row ─────────────────────────────────────────────────────── -->
    <div class="error-actions">
        <!-- Dismiss — ghost action -->
        <button type="button" class="btn-ghost error-actions__dismiss" @click="onDismiss">
            Dismiss
        </button>

        <!-- Try again — primary action -->
        <button type="button" class="btn-primary error-actions__retry" @click="onRetry">
            <svg viewBox="0 0 20 20" fill="none" xmlns="http://www.w3.org/2000/svg" class="icon-md" aria-hidden="true">
                <path fill-rule="evenodd" clip-rule="evenodd"
                    d="M4 10a6 6 0 0 1 10.472-4H12.5a.75.75 0 0 0 0 1.5h3.25A.75.75 0 0 0 16.5 6.75V3.5a.75.75 0 0 0-1.5 0v1.848A7.5 7.5 0 1 0 17.5 10a.75.75 0 0 0-1.5 0A6 6 0 1 1 4 10Z"
                    fill="currentColor" />
            </svg>
            Try Again
        </button>
    </div>

</div>
</template>

<style scoped>
/* ── View wrapper ─────────────────────────────────────────────────────────────── */

.error-view {
    display: flex;
    flex-direction: column;
    align-items: center;
    gap: var(--space-6);
    width: 100%;
    padding: var(--space-4) 0;
    text-align: center;
}

/* ── Icon wrap ────────────────────────────────────────────────────────────────── */

.error-icon-wrap {
    position: relative;
    width: 72px;
    height: 72px;
    display: flex;
    align-items: center;
    justify-content: center;
    flex-shrink: 0;
}

/* Blurred outer glow */
.error-icon-glow {
    position: absolute;
    inset: -10px;
    border-radius: var(--radius-full);
    background: radial-gradient(ellipse at center,
            rgba(255, 59, 48, 0.22) 0%,
            transparent 70%);
    pointer-events: none;
}

/* Frosted circle behind the SVG */
.error-icon-circle {
    position: relative;
    width: 64px;
    height: 64px;
    border-radius: var(--radius-full);
    background: var(--color-error-subtle);
    border: 1.5px solid rgba(255, 59, 48, 0.18);
    display: flex;
    align-items: center;
    justify-content: center;
    box-shadow:
        0 4px 16px rgba(255, 59, 48, 0.18),
        inset 0 1px 0 rgba(255, 255, 255, 0.12);
    /* Entrance animation */
    animation: error-icon-in var(--duration-slow) var(--ease-spring) forwards;
}

@keyframes error-icon-in {
    0% {
        transform: scale(0.6);
        opacity: 0;
    }

    70% {
        transform: scale(1.08);
        opacity: 1;
    }

    100% {
        transform: scale(1);
        opacity: 1;
    }
}

/* SVG icon colour */
.error-circle {
    opacity: 0.88;
    color: var(--color-error);
}

/* ── Text block ───────────────────────────────────────────────────────────────── */

.error-text {
    display: flex;
    flex-direction: column;
    align-items: center;
    gap: var(--space-3);
    max-width: 440px;
    width: 100%;
}

/* ── Error kind badge ─────────────────────────────────────────────────────────── */

.error-kind-badge {
    font-size: var(--text-xs);
    letter-spacing: var(--tracking-wider);
    text-transform: uppercase;
}

/* ── Main message ─────────────────────────────────────────────────────────────── */

.error-message {
    font-size: var(--text-base);
    font-weight: var(--weight-medium);
    color: var(--color-error-text);
    line-height: var(--leading-snug);
    letter-spacing: var(--tracking-tight);
    /* Allow selection so the user can copy the error for bug reports */
    user-select: text;
    cursor: text;
    padding: var(--space-3) var(--space-5);
    background: var(--color-error-subtle);
    border: 1px solid rgba(255, 59, 48, 0.14);
    border-radius: var(--radius-md);
    width: 100%;
    text-align: left;
    /* Truncate extremely long paths */
    word-break: break-all;
}

/* ── Contextual hint ──────────────────────────────────────────────────────────── */

.error-hint {
    font-size: var(--text-sm);
    color: var(--color-text-tertiary);
    font-weight: var(--weight-regular);
    line-height: var(--leading-relaxed);
    text-align: center;
    max-width: 380px;
}

/* ── Separator ────────────────────────────────────────────────────────────────── */

.error-separator {
    width: 100%;
    background: var(--color-separator);
}

/* ── Action row ───────────────────────────────────────────────────────────────── */

.error-actions {
    display: flex;
    align-items: center;
    justify-content: center;
    gap: var(--space-3);
    width: 100%;
}

.error-actions__dismiss {
    font-size: var(--text-base);
    color: var(--color-text-tertiary);
    padding: var(--space-3) var(--space-5);
    border-radius: var(--radius-md);
    height: 44px;
}

.error-actions__dismiss:hover:not(:disabled) {
    background: var(--color-surface-inset);
    color: var(--color-text-primary);
}

.error-actions__retry {
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

.error-actions__retry:hover:not(:disabled) {
    box-shadow:
        var(--shadow-md),
        0 8px 28px var(--color-accent-glow);
    transform: translateY(-1px);
}

.error-actions__retry:active:not(:disabled) {
    transform: scale(0.97) translateY(0);
    box-shadow: var(--shadow-sm);
}

/* ── Reduced motion ───────────────────────────────────────────────────────────── */

@media (prefers-reduced-motion: reduce) {
    .error-icon-circle {
        animation: none;
    }
}
</style>
