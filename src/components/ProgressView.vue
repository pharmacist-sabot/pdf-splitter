<script setup lang="ts">
/**
 * ProgressView — "processing" state view.
 *
 * Displays a live progress bar and per-page status while the Rust backend
 * is splitting the PDF.  All data flows in via props; this component owns
 * no state of its own — keeping it purely presentational and easy to test.
 *
 * Props
 * ─────
 * • `percent`      — 0–100 integer driving the progress bar fill width.
 * • `current`      — 1-based count of pages completed so far.
 * • `total`        — total number of pages to process.
 * • `currentFile`  — filename of the most-recently completed page,
 *                    e.g. `"page_0032.pdf"`.  Empty string before first event.
 * • `fileName`     — basename of the source PDF being processed.
 */

import { computed } from 'vue'

// ── Props ──────────────────────────────────────────────────────────────────────

const props = defineProps<{
    /** Progress percentage (0–100). */
    percent: number
    /** Pages completed so far (1-based). */
    current: number
    /** Total pages to process. */
    total: number
    /** Filename of the most-recently written output page. */
    currentFile: string
    /** Basename of the source PDF file. */
    fileName: string
}>()

// ── Computed ───────────────────────────────────────────────────────────────────

/**
 * Clamp percent to [0, 100] so CSS `width` is always valid even if the
 * backend emits an out-of-range value.
 */
const clampedPercent = computed<number>(() =>
    Math.min(100, Math.max(0, props.percent)),
)

/** CSS width string for the progress fill element. */
const fillWidth = computed<string>(() => `${clampedPercent.value}%`)

/** Human-readable fraction label, e.g. `"32 / 48"`. */
const fractionLabel = computed<string>(() =>
    props.total > 0 ? `${props.current} / ${props.total}` : '…',
)

/**
 * Array of "step" indicators rendered beneath the progress bar.
 * We show at most MAX_DOTS evenly-spaced markers so the user can visually
 * gauge where they are in a large document without cluttering the UI.
 */
const MAX_DOTS = 20
const stepDots = computed<boolean[]>(() => {
    if (props.total <= 0) return []
    const count = Math.min(props.total, MAX_DOTS)
    return Array.from({ length: count }, (_, i) => {
        const threshold = Math.round(((i + 1) / count) * props.total)
        return props.current >= threshold
    })
})

/** Estimated pages per second (rough throughput indicator). */
const isStarting = computed<boolean>(() => props.current === 0)
</script>

<template>
<div class="progress-view" role="status" aria-live="polite" aria-label="Splitting PDF…">

    <!-- ── Header ─────────────────────────────────────────────────────────── -->
    <div class="progress-view__header">
        <!-- Animated spinner -->
        <div class="spinner-ring" aria-hidden="true">
            <svg viewBox="0 0 40 40" fill="none" xmlns="http://www.w3.org/2000/svg" width="40" height="40">
                <!-- Track ring -->
                <circle cx="20" cy="20" r="16" stroke="currentColor" stroke-width="3" class="spinner-track" />
                <!-- Animated arc -->
                <circle cx="20" cy="20" r="16" stroke="currentColor" stroke-width="3" stroke-linecap="round"
                    class="spinner-arc" :style="{
                        strokeDashoffset: `${100 - clampedPercent}px`,
                    }" />
            </svg>

            <!-- Percent label inside the ring -->
            <span class="spinner-percent" aria-hidden="true">
                {{ isStarting ? '…' : `${clampedPercent}%` }}
            </span>
        </div>

        <div class="progress-view__title-group">
            <h2 class="progress-view__title">
                Splitting PDF
            </h2>
            <p class="progress-view__subtitle truncate" :title="fileName">
                {{ fileName }}
            </p>
        </div>
    </div>

    <!-- ── Progress bar ──────────────────────────────────────────────────── -->
    <div class="progress-section">
        <!-- Bar -->
        <div class="progress-track" role="progressbar" :aria-valuenow="clampedPercent" aria-valuemin="0"
            aria-valuemax="100" :aria-label="`${clampedPercent}% complete`">
            <div class="progress-fill" :style="{ width: fillWidth }" />
        </div>

        <!-- Step dots -->
        <div v-if="stepDots.length > 1" class="step-dots" aria-hidden="true">
            <span v-for="(done, i) in stepDots" :key="i" class="step-dot" :class="{ 'step-dot--done': done }" />
        </div>

        <!-- Fraction + file label -->
        <div class="progress-labels">
            <span class="progress-labels__fraction" aria-hidden="true">
                {{ fractionLabel }}
                <span class="progress-labels__unit"> pages</span>
            </span>

            <Transition name="fade">
                <span v-if="currentFile" :key="currentFile" class="progress-labels__file truncate" aria-hidden="true">
                    <svg viewBox="0 0 14 14" fill="none" xmlns="http://www.w3.org/2000/svg" width="11" height="11"
                        aria-hidden="true">
                        <path fill-rule="evenodd" clip-rule="evenodd"
                            d="M11.78 3.22a.75.75 0 0 1 0 1.06l-6 6a.75.75 0 0 1-1.06 0l-2.5-2.5a.75.75 0 1 1 1.06-1.06L5.25 8.69l5.47-5.47a.75.75 0 0 1 1.06 0Z"
                            fill="currentColor" />
                    </svg>
                    {{ currentFile }}
                </span>
            </Transition>
        </div>
    </div>

    <!-- ── Status message ────────────────────────────────────────────────── -->
    <div class="status-message">
        <template v-if="isStarting">
            <span class="status-message__text animate-pulse">
                Preparing…
            </span>
        </template>
        <template v-else-if="clampedPercent < 100">
            <span class="status-message__text">
                Processing page
                <strong>{{ current }}</strong>
                of
                <strong>{{ total }}</strong>
                <span class="status-message__ellipsis" aria-hidden="true">…</span>
            </span>
        </template>
        <template v-else>
            <span class="status-message__text status-message__text--finalising">
                Finalising
                <span class="animate-pulse">…</span>
            </span>
        </template>
    </div>

</div>
</template>

<style scoped>
/* ── View wrapper ─────────────────────────────────────────────────────────────── */

.progress-view {
    display: flex;
    flex-direction: column;
    align-items: stretch;
    gap: var(--space-8);
    width: 100%;
    padding: var(--space-4) 0;
}

/* ── Header ───────────────────────────────────────────────────────────────────── */

.progress-view__header {
    display: flex;
    align-items: center;
    gap: var(--space-5);
}

/* ── Spinner ring ─────────────────────────────────────────────────────────────── */

.spinner-ring {
    position: relative;
    flex-shrink: 0;
    width: 40px;
    height: 40px;
    color: var(--color-accent);
}

.spinner-track {
    opacity: 0.12;
    color: var(--color-accent);
}

.spinner-arc {
    stroke-dasharray: 100px;
    /* strokeDashoffset bound via :style */
    transform-origin: 50% 50%;
    transform: rotate(-90deg);
    transition: stroke-dashoffset var(--duration-slow) var(--ease-out);
    color: var(--color-accent);
    filter: drop-shadow(0 0 4px var(--color-accent-glow));
}

.spinner-percent {
    position: absolute;
    inset: 0;
    display: flex;
    align-items: center;
    justify-content: center;
    font-size: 9px;
    font-weight: var(--weight-bold);
    color: var(--color-accent);
    letter-spacing: -0.02em;
    pointer-events: none;
}

/* ── Title group ──────────────────────────────────────────────────────────────── */

.progress-view__title-group {
    display: flex;
    flex-direction: column;
    gap: var(--space-1);
    min-width: 0;
    flex: 1;
}

.progress-view__title {
    font-size: var(--text-xl);
    font-weight: var(--weight-semibold);
    letter-spacing: var(--tracking-tight);
    color: var(--color-text-primary);
    line-height: var(--leading-tight);
}

.progress-view__subtitle {
    font-size: var(--text-sm);
    color: var(--color-text-tertiary);
    font-weight: var(--weight-regular);
    max-width: 360px;
}

/* ── Progress section ─────────────────────────────────────────────────────────── */

.progress-section {
    display: flex;
    flex-direction: column;
    gap: var(--space-3);
}

/* ── Step dots ────────────────────────────────────────────────────────────────── */

.step-dots {
    display: flex;
    align-items: center;
    gap: 4px;
}

.step-dot {
    flex: 1;
    height: 3px;
    border-radius: var(--radius-full);
    background: var(--color-bg-secondary);
    transition: background-color var(--duration-normal) var(--ease-out);
}

.step-dot--done {
    background: var(--color-accent);
    opacity: 0.6;
}

/* ── Progress labels ──────────────────────────────────────────────────────────── */

.progress-labels {
    display: flex;
    align-items: center;
    justify-content: space-between;
    gap: var(--space-4);
}

.progress-labels__fraction {
    font-size: var(--text-sm);
    font-weight: var(--weight-semibold);
    color: var(--color-text-primary);
    letter-spacing: var(--tracking-tight);
    white-space: nowrap;
    flex-shrink: 0;
}

.progress-labels__unit {
    font-weight: var(--weight-regular);
    color: var(--color-text-tertiary);
}

.progress-labels__file {
    display: inline-flex;
    align-items: center;
    gap: 4px;
    font-size: var(--text-xs);
    color: var(--color-success-text);
    font-family: var(--font-mono);
    letter-spacing: 0;
    max-width: 260px;
}

/* ── Status message ───────────────────────────────────────────────────────────── */

.status-message {
    display: flex;
    align-items: center;
    justify-content: center;
    min-height: 24px;
}

.status-message__text {
    font-size: var(--text-sm);
    color: var(--color-text-tertiary);
    font-weight: var(--weight-regular);
    letter-spacing: var(--tracking-tight);
    text-align: center;
}

.status-message__text strong {
    color: var(--color-text-secondary);
    font-weight: var(--weight-semibold);
}

.status-message__ellipsis {
    color: var(--color-text-quaternary);
}

.status-message__text--finalising {
    color: var(--color-accent);
    font-weight: var(--weight-medium);
}
</style>
