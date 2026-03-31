<script setup lang="ts">
import { computed } from 'vue'

const props = defineProps<{
    /* Progress percentage (0–100) */
    percent: number
    /* Pages completed so far (1-based) */
    current: number
    /* Total pages to process */
    total: number
    /* Filename of the most-recently written output page */
    currentFile: string
    /* Basename of the source PDF file */
    fileName: string
}>()

const clampedPercent = computed<number>(() =>
    Math.min(100, Math.max(0, props.percent)),
)

const fillTransform = computed<string>(() =>
    `scaleX(${clampedPercent.value / 100})`,
)


/* Human-readable fraction label, e.g. `"32 / 48"` */
const fractionLabel = computed<string>(() =>
    props.total > 0 ? `${props.current} / ${props.total}` : '…',
)

/* True before the first progress event arrives */
const isStarting = computed<boolean>(() => props.current === 0)

/* True once all pages have been processed (finalising phase) */
const isFinalising = computed<boolean>(
    () => clampedPercent.value >= 100 && !isStarting.value,
)

/*
 * ASCII block progress bar, e.g. `[████████████░░░░░░░░░░░░]`
 *
 * Uses U+2588 FULL BLOCK (█) for filled segments and U+2591 LIGHT SHADE (░)
 * for empty segments.  The bar is 24 characters wide inside the brackets
 */
const BAR_LENGTH = 24
const asciiBar = computed<string>(() => {
    const filled = Math.round((clampedPercent.value / 100) * BAR_LENGTH)
    const empty = BAR_LENGTH - filled
    return '█'.repeat(filled) + '░'.repeat(empty)
})

const MAX_DOTS = 20
const stepDots = computed<boolean[]>(() => {
    if (props.total <= 0) return []
    const count = Math.min(props.total, MAX_DOTS)
    return Array.from({ length: count }, (_, i) => {
        const threshold = Math.round(((i + 1) / count) * props.total)
        return props.current >= threshold
    })
})
</script>

<template>
<div class="progress-view" role="status" aria-live="polite" aria-label="Splitting PDF…">

    <div class="progress-view__header">
        <span class="progress-view__prompt" aria-hidden="true">$</span>
        <div class="progress-view__title-group">
            <h2 class="progress-view__title">
                split-pdf
                <span class="progress-view__flag">--output</span>
                <span class="progress-view__filename truncate" :title="fileName">{{ fileName }}</span>
            </h2>
            <p class="progress-view__running">
                <span class="progress-view__dot animate-pulse" aria-hidden="true">●</span>
                <template v-if="isStarting">
                    <span class="animate-pulse">preparing...</span>
                </template>
                <template v-else-if="isFinalising">
                    <span>finalising<span class="animate-pulse">...</span></span>
                </template>
                <template v-else>
                    <span>
                        processing page
                        <strong>{{ current }}</strong>
                        of
                        <strong>{{ total }}</strong>
                    </span>
                </template>
            </p>
        </div>

        <!-- Percent badge -->
        <span class="progress-view__pct" :class="{ 'progress-view__pct--finalising': isFinalising }" aria-hidden="true">
            {{ isStarting ? '…' : `${clampedPercent}%` }}
        </span>
    </div>

    <div class="progress-section">
        <!-- ASCII bar label -->
        <div class="ascii-bar-row" aria-hidden="true">
            <span class="ascii-bar-bracket">[</span>
            <span class="ascii-bar-fill" :class="{ 'ascii-bar-fill--glow': !isStarting }">{{ asciiBar }}</span>
            <span class="ascii-bar-bracket">]</span>
            <span class="ascii-bar-stats">{{ fractionLabel }}<span class="ascii-bar-unit"> pages</span></span>
        </div>

        <!-- Native progress element (sr + CSS bar) -->
        <div class="progress-track" role="progressbar" :aria-valuenow="clampedPercent" aria-valuemin="0"
            aria-valuemax="100" :aria-label="`${clampedPercent}% complete`">
            <div class="progress-fill" :style="{ transform: fillTransform }" />
        </div>

        <!-- Step dots -->
        <div v-if="stepDots.length > 1" class="step-dots" aria-hidden="true">
            <span v-for="(done, i) in stepDots" :key="i" class="step-dot" :class="{ 'step-dot--done': done }" />
        </div>
    </div>

    <div class="output-stream">
        <!-- Most-recently completed file (no Vue Transition — avoids costly
             DOM mount/unmount at high event rates; CSS handles the visual) -->
        <div v-if="currentFile" class="output-line output-line--ok">
            <span class="output-line__check" aria-hidden="true">✓</span>
            <span class="output-line__text truncate">{{ currentFile }}</span>
        </div>

        <!-- Idle / waiting message when no file yet -->
        <div v-if="isStarting" class="output-line output-line--pending">
            <span class="output-line__check animate-pulse" aria-hidden="true">…</span>
            <span class="output-line__text">waiting for first page...</span>
        </div>
    </div>

</div>
</template>

<style scoped>
.progress-view {
    display: flex;
    flex-direction: column;
    align-items: stretch;
    gap: var(--space-6);
    width: 100%;
    padding: var(--space-2) 0;
}

.progress-view__header {
    display: flex;
    align-items: flex-start;
    gap: var(--space-3);
}

/* $ prompt */
.progress-view__prompt {
    font-family: var(--font-mono);
    font-size: var(--text-lg);
    font-weight: var(--weight-bold);
    color: var(--color-accent);
    text-shadow: 0 0 8px var(--color-accent-glow);
    flex-shrink: 0;
    line-height: 1.6;
}

/* Title group */
.progress-view__title-group {
    display: flex;
    flex-direction: column;
    gap: var(--space-1);
    min-width: 0;
    flex: 1;
}

.progress-view__title {
    font-size: var(--text-base);
    font-weight: var(--weight-semibold);
    font-family: var(--font-mono);
    color: var(--color-text-primary);
    line-height: var(--leading-snug);
    display: flex;
    align-items: baseline;
    flex-wrap: wrap;
    gap: 0.4em;
}

.progress-view__flag {
    color: var(--color-text-tertiary);
    font-weight: var(--weight-regular);
}

.progress-view__filename {
    color: var(--color-accent);
    max-width: 280px;
}

/* Running status */
.progress-view__running {
    font-size: var(--text-sm);
    color: var(--color-text-tertiary);
    font-family: var(--font-mono);
    line-height: var(--leading-normal);
    display: flex;
    align-items: center;
    gap: var(--space-2);
}

.progress-view__running strong {
    color: var(--color-text-secondary);
    font-weight: var(--weight-semibold);
}

.progress-view__dot {
    font-size: 8px;
    color: var(--color-accent);
}

/* Percent badge */
.progress-view__pct {
    font-family: var(--font-mono);
    font-size: var(--text-sm);
    font-weight: var(--weight-bold);
    color: var(--color-text-secondary);
    background: var(--color-surface-inset);
    border: 1px solid var(--color-border);
    border-radius: var(--radius-sm);
    padding: 2px var(--space-2);
    flex-shrink: 0;
    min-width: 48px;
    text-align: center;
    transition: color var(--duration-normal) var(--ease-out);
    letter-spacing: 0.02em;
}

.progress-view__pct--finalising {
    color: var(--color-accent);
    border-color: rgba(57, 211, 83, 0.3);
    text-shadow: 0 0 8px rgba(57, 211, 83, 0.3);
}

.progress-section {
    display: flex;
    flex-direction: column;
    gap: var(--space-2);
}

.ascii-bar-row {
    display: flex;
    align-items: baseline;
    gap: 0;
    font-family: var(--font-mono);
    font-size: var(--text-sm);
    line-height: 1;
    white-space: nowrap;
    overflow: hidden;
}

.ascii-bar-bracket {
    color: var(--color-border-strong);
    font-weight: var(--weight-bold);
}

.ascii-bar-fill {
    color: var(--color-text-quaternary);
    letter-spacing: -0.02em;
    transition: color var(--duration-slow) var(--ease-out);
}

.ascii-bar-fill--glow {
    /* Use a text gradient for the filled part — done via clip */
    color: var(--color-accent);
    text-shadow: 0 0 6px rgba(57, 211, 83, 0.25);
}

.ascii-bar-stats {
    margin-left: var(--space-3);
    color: var(--color-text-secondary);
    font-size: var(--text-xs);
    white-space: nowrap;
    flex-shrink: 0;
}

.ascii-bar-unit {
    color: var(--color-text-quaternary);
    font-weight: var(--weight-regular);
}

/* Native progress bar (below the ASCII bar, acts as a precise visual fill) */
.progress-section .progress-track {
    height: 3px;
    border-radius: 0;
    background: var(--color-border-subtle);
    border: none;
    /* Promote to own GPU layer for smoother updates during rapid progress */
    will-change: contents;
}

.step-dots {
    display: flex;
    align-items: center;
    gap: 3px;
    margin-top: var(--space-1);
}

.step-dot {
    flex: 1;
    height: 2px;
    border-radius: 1px;
    background: var(--color-surface-inset);
    transition: background-color var(--duration-normal) var(--ease-out);
}

.step-dot--done {
    background: var(--color-accent);
    opacity: 0.55;
    box-shadow: 0 0 4px rgba(57, 211, 83, 0.3);
}

.output-stream {
    min-height: 32px;
    display: flex;
    flex-direction: column;
    gap: var(--space-1);
}

.output-line {
    display: flex;
    align-items: center;
    gap: var(--space-2);
    font-family: var(--font-mono);
    font-size: var(--text-sm);
    line-height: var(--leading-normal);
    padding: var(--space-1) var(--space-3);
    border-radius: var(--radius-sm);
    border-left: 2px solid transparent;
}

.output-line--ok {
    color: var(--color-success-text);
    background: var(--color-success-subtle);
    border-left-color: var(--color-accent);
}

.output-line--pending {
    color: var(--color-text-quaternary);
    border-left-color: var(--color-border);
}

.output-line__check {
    flex-shrink: 0;
    font-weight: var(--weight-bold);
    font-size: 11px;
}

.output-line__text {
    flex: 1;
    min-width: 0;
}
</style>
