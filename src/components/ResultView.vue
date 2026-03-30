<script setup lang="ts">
/**
 * ResultView — "complete" state view.
 *
 * Displays a summary of the completed split operation:
 * • Animated success badge with page count and elapsed time.
 * • Scrollable list of every output file, each with a "Reveal" action.
 * • Primary "Show in Finder" button to open the output directory.
 * • Secondary "Split Another File" button to reset the app.
 *
 * Props
 * ─────
 * • `totalPages`    — number of pages that were split.
 * • `outputFiles`   — sorted array of absolute output file paths.
 * • `elapsedFormatted` — human-readable elapsed time, e.g. `"1.2 s"`.
 * • `outputDir`     — absolute path of the output directory.
 *
 * Events
 * ──────
 * • `reveal`  — user clicked "Show in Finder" or the per-file reveal button.
 *               Payload: the path to reveal (directory or specific file).
 * • `reset`   — user clicked "Split Another File".
 */

import { ref, computed } from 'vue'
import { basename } from '@/types'

// ── Props & emits ──────────────────────────────────────────────────────────────

const props = defineProps<{
    /** Total pages that were successfully written to disk. */
    totalPages: number
    /** Sorted absolute paths of every output PDF file. */
    outputFiles: string[]
    /** Human-readable elapsed time, e.g. `"1.2 s"`. */
    elapsedFormatted: string
    /** Absolute path of the output directory. */
    outputDir: string
}>()

const emit = defineEmits<{
    /**
     * Request to reveal a path in Finder.
     * @param path — absolute path of the file or directory to reveal.
     */
    reveal: [path: string]
    /** Reset the application to the initial idle state. */
    reset: []
}>()

// ── Local state ────────────────────────────────────────────────────────────────

/** Index of the file row currently hovered — for per-row reveal button. */
const hoveredIndex = ref<number | null>(null)

// ── Computed ───────────────────────────────────────────────────────────────────

/** Display basenames for every output file. */
const fileNames = computed<string[]>(() =>
    props.outputFiles.map((p) => basename(p)),
)

/** Throughput label, e.g. `"48 pages in 1.2 s"`. */
const summaryLabel = computed<string>(() => {
    const pages = props.totalPages
    const unit = pages === 1 ? 'page' : 'pages'
    return props.elapsedFormatted
        ? `${pages} ${unit} in ${props.elapsedFormatted}`
        : `${pages} ${unit}`
})

// ── Actions ────────────────────────────────────────────────────────────────────

function onRevealDir(): void {
    emit('reveal', props.outputDir)
}

function onRevealFile(path: string): void {
    emit('reveal', path)
}

function onReset(): void {
    emit('reset')
}
</script>

<template>
<div class="result-view">

    <!-- ── Success header ─────────────────────────────────────────────────── -->
    <div class="result-header">
        <!-- Animated check badge -->
        <div class="success-badge animate-bounce-in" aria-hidden="true">
            <svg viewBox="0 0 32 32" fill="none" xmlns="http://www.w3.org/2000/svg" width="32" height="32">
                <circle cx="16" cy="16" r="16" fill="currentColor" class="badge-circle" />
                <path fill-rule="evenodd" clip-rule="evenodd"
                    d="M22.78 10.72a.75.75 0 0 1 0 1.06l-8.5 8.5a.75.75 0 0 1-1.06 0l-3.5-3.5a.75.75 0 1 1 1.06-1.06l2.97 2.97 7.97-7.97a.75.75 0 0 1 1.06 0Z"
                    fill="white" />
            </svg>
        </div>

        <!-- Title & summary -->
        <div class="result-header__text">
            <h2 class="result-header__title">
                Split complete!
            </h2>
            <p class="result-header__summary">
                {{ summaryLabel }}
            </p>
        </div>

        <!-- Stats chips -->
        <div class="result-stats" aria-hidden="true">
            <span class="stat-chip">
                <svg viewBox="0 0 14 14" fill="none" xmlns="http://www.w3.org/2000/svg" width="11" height="11"
                    aria-hidden="true">
                    <path
                        d="M2.5 1.5A1 1 0 0 1 3.5.5h5.086a1 1 0 0 1 .707.293l2.414 2.414a1 1 0 0 1 .293.707V12.5a1 1 0 0 1-1 1h-7.5a1 1 0 0 1-1-1v-11Z"
                        fill="currentColor" opacity="0.6" />
                </svg>
                {{ totalPages }} {{ totalPages === 1 ? 'file' : 'files' }}
            </span>
            <span v-if="elapsedFormatted" class="stat-chip stat-chip--time">
                <svg viewBox="0 0 14 14" fill="none" xmlns="http://www.w3.org/2000/svg" width="11" height="11"
                    aria-hidden="true">
                    <circle cx="7" cy="7" r="6" stroke="currentColor" stroke-width="1.4" fill="none" />
                    <path d="M7 4v3l2 1.5" stroke="currentColor" stroke-width="1.4" stroke-linecap="round" />
                </svg>
                {{ elapsedFormatted }}
            </span>
        </div>
    </div>

    <!-- ── File list ──────────────────────────────────────────────────────── -->
    <div class="file-list-container">
        <div class="file-list" role="list" :aria-label="`${totalPages} output files`">
            <TransitionGroup name="list-item" tag="div" class="file-list__inner">
                <div v-for="(path, index) in outputFiles" :key="path" class="file-row" role="listitem"
                    :style="{ transitionDelay: `${Math.min(index * 18, 300)}ms` }" @mouseenter="hoveredIndex = index"
                    @mouseleave="hoveredIndex = null">
                    <!-- File icon -->
                    <div class="file-row__icon" aria-hidden="true">
                        <svg viewBox="0 0 16 20" fill="none" xmlns="http://www.w3.org/2000/svg" width="12" height="16">
                            <path
                                d="M2 1a1 1 0 0 0-1 1v16a1 1 0 0 0 1 1h12a1 1 0 0 0 1-1V6.414a1 1 0 0 0-.293-.707l-4.414-4.414A1 1 0 0 0 9.586 1H2Z"
                                fill="currentColor" class="file-row__doc" />
                            <path d="M9 1.5V5a1 1 0 0 0 1 1h3.5" stroke="currentColor" stroke-width="1"
                                class="file-row__fold" fill="none" opacity="0.4" />
                        </svg>
                    </div>

                    <!-- Filename -->
                    <span class="file-row__name">
                        {{ fileNames[index] }}
                    </span>

                    <!-- Page index badge -->
                    <span class="file-row__index" aria-hidden="true">
                        {{ index + 1 }}
                    </span>

                    <!-- Per-row reveal button (shown on hover) -->
                    <Transition name="fade">
                        <button v-if="hoveredIndex === index" type="button" class="file-row__reveal btn-icon"
                            :aria-label="`Reveal ${fileNames[index]} in Finder`" @click.stop="onRevealFile(path)">
                            <svg viewBox="0 0 16 16" fill="none" xmlns="http://www.w3.org/2000/svg" width="13"
                                height="13" aria-hidden="true">
                                <path fill-rule="evenodd" clip-rule="evenodd"
                                    d="M2 2.75A.75.75 0 0 1 2.75 2h10.5a.75.75 0 0 1 .75.75v10.5a.75.75 0 0 1-.75.75H2.75a.75.75 0 0 1-.75-.75V2.75ZM3.5 3.5v9h9v-9h-9ZM8 5a.75.75 0 0 1 .75.75v1.5h1.5a.75.75 0 0 1 0 1.5h-1.5v1.5a.75.75 0 0 1-1.5 0v-1.5h-1.5a.75.75 0 0 1 0-1.5h1.5v-1.5A.75.75 0 0 1 8 5Z"
                                    fill="currentColor" />
                            </svg>
                        </button>
                    </Transition>
                </div>
            </TransitionGroup>
        </div>
    </div>

    <!-- ── Action row ─────────────────────────────────────────────────────── -->
    <div class="result-actions">
        <button type="button" class="btn-secondary result-actions__secondary" @click="onReset">
            <svg viewBox="0 0 20 20" fill="none" xmlns="http://www.w3.org/2000/svg" class="icon-sm" aria-hidden="true">
                <path fill-rule="evenodd" clip-rule="evenodd"
                    d="M4.293 4.293a1 1 0 0 1 1.414 0L10 8.586l4.293-4.293a1 1 0 1 1 1.414 1.414L11.414 10l4.293 4.293a1 1 0 0 1-1.414 1.414L10 11.414l-4.293 4.293a1 1 0 0 1-1.414-1.414L8.586 10 4.293 5.707a1 1 0 0 1 0-1.414Z"
                    fill="currentColor" />
            </svg>
            Split Another File
        </button>

        <button type="button" class="btn-primary result-actions__primary" @click="onRevealDir">
            <svg viewBox="0 0 20 20" fill="none" xmlns="http://www.w3.org/2000/svg" class="icon-md" aria-hidden="true">
                <path fill-rule="evenodd" clip-rule="evenodd"
                    d="M2 5a2 2 0 0 1 2-2h3.586A2 2 0 0 1 9 3.586L10.414 5H16a2 2 0 0 1 2 2v8a2 2 0 0 1-2 2H4a2 2 0 0 1-2-2V5Zm2-.5a.5.5 0 0 0-.5.5v10a.5.5 0 0 0 .5.5h12a.5.5 0 0 0 .5-.5V7a.5.5 0 0 0-.5-.5H10a.5.5 0 0 0-.354-.146L8.232 4.94A.5.5 0 0 0 7.879 4.5H4Z"
                    fill="currentColor" />
            </svg>
            Show in Finder
        </button>
    </div>

</div>
</template>

<style scoped>
/* ── View wrapper ─────────────────────────────────────────────────────────────── */

.result-view {
    display: flex;
    flex-direction: column;
    gap: var(--space-5);
    width: 100%;
}

/* ── Success header ───────────────────────────────────────────────────────────── */

.result-header {
    display: flex;
    align-items: center;
    gap: var(--space-4);
}

/* ── Animated check badge ─────────────────────────────────────────────────────── */

.success-badge {
    flex-shrink: 0;
    width: 40px;
    height: 40px;
    color: var(--color-success);
    filter: drop-shadow(0 4px 12px rgba(52, 199, 89, 0.35));
}

.badge-circle {
    opacity: 0.92;
}

/* ── Header text ──────────────────────────────────────────────────────────────── */

.result-header__text {
    flex: 1;
    min-width: 0;
    display: flex;
    flex-direction: column;
    gap: 2px;
}

.result-header__title {
    font-size: var(--text-xl);
    font-weight: var(--weight-semibold);
    letter-spacing: var(--tracking-tight);
    color: var(--color-text-primary);
    line-height: var(--leading-tight);
}

.result-header__summary {
    font-size: var(--text-sm);
    color: var(--color-text-tertiary);
    font-weight: var(--weight-regular);
    line-height: var(--leading-normal);
}

/* ── Stats chips ──────────────────────────────────────────────────────────────── */

.result-stats {
    display: flex;
    align-items: center;
    gap: var(--space-2);
    flex-shrink: 0;
}

.stat-chip {
    display: inline-flex;
    align-items: center;
    gap: 4px;
    font-size: var(--text-xs);
    font-weight: var(--weight-semibold);
    letter-spacing: var(--tracking-wide);
    padding: 4px var(--space-3);
    border-radius: var(--radius-full);
    background: var(--color-success-subtle);
    color: var(--color-success-text);
    line-height: 1;
}

.stat-chip--time {
    background: var(--color-accent-subtle);
    color: var(--color-accent);
}

/* ── File list container ──────────────────────────────────────────────────────── */

.file-list-container {
    flex: 1;
    min-height: 0;
    background: var(--color-surface);
    backdrop-filter: var(--blur-md);
    -webkit-backdrop-filter: var(--blur-md);
    border: 1px solid var(--color-border);
    border-radius: var(--radius-lg);
    overflow: hidden;
    box-shadow: var(--shadow-sm);
}

/* ── File list ────────────────────────────────────────────────────────────────── */

.file-list {
    height: 100%;
    overflow-y: auto;
    overscroll-behavior: contain;
}

.file-list__inner {
    display: flex;
    flex-direction: column;
    padding: var(--space-2) 0;
}

/* ── File row ─────────────────────────────────────────────────────────────────── */

.file-row {
    display: flex;
    align-items: center;
    gap: var(--space-3);
    padding: var(--space-2) var(--space-4);
    min-height: 34px;
    cursor: default;
    transition: background-color var(--duration-fast) var(--ease-out);
    position: relative;
}

.file-row:hover {
    background: var(--color-surface-inset);
}

/* ── File row icon ────────────────────────────────────────────────────────────── */

.file-row__icon {
    flex-shrink: 0;
    color: var(--color-accent);
    opacity: 0.7;
    display: flex;
    align-items: center;
    justify-content: center;
    width: 16px;
}

.file-row__doc {
    opacity: 0.65;
}

.file-row:hover .file-row__icon {
    opacity: 1;
}

/* ── Filename ─────────────────────────────────────────────────────────────────── */

.file-row__name {
    flex: 1;
    font-size: var(--text-sm);
    font-family: var(--font-mono);
    font-weight: var(--weight-regular);
    color: var(--color-text-secondary);
    letter-spacing: 0;
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
    transition: color var(--duration-fast) var(--ease-out);
}

.file-row:hover .file-row__name {
    color: var(--color-text-primary);
}

/* ── Page index badge ─────────────────────────────────────────────────────────── */

.file-row__index {
    flex-shrink: 0;
    font-size: var(--text-xs);
    color: var(--color-text-quaternary);
    font-family: var(--font-mono);
    font-weight: var(--weight-regular);
    min-width: 24px;
    text-align: right;
    line-height: 1;
    transition: opacity var(--duration-fast) var(--ease-out);
}

.file-row:hover .file-row__index {
    opacity: 0;
}

/* ── Per-row reveal button ────────────────────────────────────────────────────── */

.file-row__reveal {
    position: absolute;
    right: var(--space-3);
    width: 24px;
    height: 24px;
    border-radius: var(--radius-sm);
    color: var(--color-accent);
    background: var(--color-accent-subtle);
    display: flex;
    align-items: center;
    justify-content: center;
    cursor: pointer;
    transition:
        background-color var(--duration-fast) var(--ease-out),
        color var(--duration-fast) var(--ease-out),
        transform var(--duration-fast) var(--ease-spring);
}

.file-row__reveal:hover {
    background: var(--color-accent);
    color: #fff;
    transform: scale(1.08);
}

.file-row__reveal:active {
    transform: scale(0.94);
}

/* ── Action row ───────────────────────────────────────────────────────────────── */

.result-actions {
    display: flex;
    align-items: center;
    justify-content: space-between;
    gap: var(--space-3);
}

.result-actions__secondary {
    font-size: var(--text-sm);
    padding: var(--space-3) var(--space-5);
    height: 38px;
    color: var(--color-text-secondary);
}

.result-actions__primary {
    min-width: 160px;
    height: 44px;
    font-size: var(--text-base);
    border-radius: var(--radius-md);
    padding: var(--space-3) var(--space-6);
    box-shadow:
        var(--shadow-sm),
        0 4px 20px var(--color-accent-glow);
}

.result-actions__primary:hover:not(:disabled) {
    box-shadow:
        var(--shadow-md),
        0 8px 28px var(--color-accent-glow);
    transform: translateY(-1px);
}

.result-actions__primary:active:not(:disabled) {
    transform: scale(0.97) translateY(0);
    box-shadow: var(--shadow-sm);
}
</style>
