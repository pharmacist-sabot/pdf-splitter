<script setup lang="ts">
/**
 * ResultView — "complete" state view.
 *
 * Terminal aesthetic: the success header reads like a CLI exit message
 * (`✓ done — 48 files · 1.2 s`), the file list mimics `ls -1` output
 * with monospace filenames and per-row reveal actions.
 *
 * Props / computed logic unchanged — only the visual presentation is redesigned.
 */

import { ref, computed } from 'vue'
import { basename } from '@/types'

// ── Props & emits ──────────────────────────────────────────────────────────────

const props = defineProps<{
    totalPages: number
    outputFiles: string[]
    elapsedFormatted: string
    outputDir: string
}>()

const emit = defineEmits<{
    reveal: [path: string]
    reset: []
}>()

// ── Local state ────────────────────────────────────────────────────────────────

const hoveredIndex = ref<number | null>(null)

// ── Computed ───────────────────────────────────────────────────────────────────

const fileNames = computed<string[]>(() =>
    props.outputFiles.map((p) => basename(p)),
)

const summaryLabel = computed<string>(() => {
    const pages = props.totalPages
    const unit = pages === 1 ? 'file' : 'files'
    return props.elapsedFormatted
        ? `${pages} ${unit} · ${props.elapsedFormatted}`
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
        <!-- ✓ check icon -->
        <div class="success-badge animate-bounce-in" aria-hidden="true">
            <svg viewBox="0 0 32 32" fill="none" xmlns="http://www.w3.org/2000/svg" width="30" height="30">
                <circle cx="16" cy="16" r="15" fill="currentColor" class="badge-circle" />
                <path fill-rule="evenodd" clip-rule="evenodd"
                    d="M22.78 10.72a.75.75 0 0 1 0 1.06l-8.5 8.5a.75.75 0 0 1-1.06 0l-3.5-3.5a.75.75 0 1 1 1.06-1.06l2.97 2.97 7.97-7.97a.75.75 0 0 1 1.06 0Z"
                    fill="currentColor" class="badge-check" />
            </svg>
        </div>

        <!-- Title & summary -->
        <div class="result-header__text">
            <h2 class="result-header__title">
                <span class="result-header__ok" aria-hidden="true">✓</span>
                done
            </h2>
            <p class="result-header__summary">
                {{ summaryLabel }}
            </p>
        </div>

        <!-- Stats chips -->
        <div class="result-stats" aria-label="Split statistics">
            <span class="stat-chip stat-chip--files">
                <svg viewBox="0 0 12 14" fill="none" xmlns="http://www.w3.org/2000/svg" width="10" height="12"
                    aria-hidden="true">
                    <path
                        d="M1.5 1A.5.5 0 0 1 2 .5h6.086a.5.5 0 0 1 .353.146l2.414 2.415A.5.5 0 0 1 11 3.414V13a.5.5 0 0 1-.5.5h-8A.5.5 0 0 1 1.5 13V1Z"
                        fill="currentColor" opacity="0.55" />
                </svg>
                {{ totalPages }}&nbsp;{{ totalPages === 1 ? 'file' : 'files' }}
            </span>
            <span v-if="elapsedFormatted" class="stat-chip stat-chip--time">
                <svg viewBox="0 0 12 12" fill="none" xmlns="http://www.w3.org/2000/svg" width="10" height="10"
                    aria-hidden="true">
                    <circle cx="6" cy="6" r="5" stroke="currentColor" stroke-width="1.3" fill="none" />
                    <path d="M6 3.5v2.5l1.5 1" stroke="currentColor" stroke-width="1.3" stroke-linecap="round" />
                </svg>
                {{ elapsedFormatted }}
            </span>
        </div>
    </div>

    <!-- ── ls-style file list ─────────────────────────────────────────────── -->
    <div class="file-list-container">
        <!-- Directory label -->
        <div class="file-list__header" aria-hidden="true">
            <span class="file-list__cmd">$ ls -1</span>
            <span class="file-list__dir truncate">{{ outputDir }}</span>
        </div>
        <div class="file-list" role="list" :aria-label="`${totalPages} output files`">
            <TransitionGroup name="list-item" tag="div" class="file-list__inner">
                <div v-for="(path, index) in outputFiles" :key="path" class="file-row" role="listitem"
                    :style="{ transitionDelay: `${Math.min(index * 14, 280)}ms` }" @mouseenter="hoveredIndex = index"
                    @mouseleave="hoveredIndex = null">
                    <!-- Line number -->
                    <span class="file-row__lineno" aria-hidden="true">{{ String(index + 1).padStart(3, ' ') }}</span>

                    <!-- Filename -->
                    <span class="file-row__name">{{ fileNames[index] }}</span>

                    <!-- Reveal button (hover) -->
                    <Transition name="fade">
                        <button v-if="hoveredIndex === index" type="button" class="file-row__reveal"
                            :aria-label="`Reveal ${fileNames[index]} in Finder`" @click.stop="onRevealFile(path)">
                            <svg viewBox="0 0 14 14" fill="none" xmlns="http://www.w3.org/2000/svg" width="12"
                                height="12" aria-hidden="true">
                                <path fill-rule="evenodd" clip-rule="evenodd"
                                    d="M1.5 1.5A.5.5 0 0 1 2 1h10a.5.5 0 0 1 .5.5v10a.5.5 0 0 1-.5.5H2a.5.5 0 0 1-.5-.5v-10ZM3 2.5v9h8v-9H3ZM7 4a.5.5 0 0 1 .5.5v2h2a.5.5 0 0 1 0 1h-2v2a.5.5 0 0 1-1 0v-2h-2a.5.5 0 0 1 0-1h2v-2A.5.5 0 0 1 7 4Z"
                                    fill="currentColor" />
                            </svg>
                            reveal
                        </button>
                    </Transition>
                </div>
            </TransitionGroup>
        </div>
    </div>

    <!-- ── Action row ─────────────────────────────────────────────────────── -->
    <div class="result-actions">
        <button type="button" class="btn-secondary result-actions__secondary" @click="onReset">
            <svg viewBox="0 0 16 16" fill="none" xmlns="http://www.w3.org/2000/svg" class="icon-sm" aria-hidden="true">
                <path fill-rule="evenodd" clip-rule="evenodd"
                    d="M3.293 3.293a1 1 0 0 1 1.414 0L8 6.586l3.293-3.293a1 1 0 1 1 1.414 1.414L9.414 8l3.293 3.293a1 1 0 0 1-1.414 1.414L8 9.414l-3.293 3.293a1 1 0 0 1-1.414-1.414L6.586 8 3.293 4.707a1 1 0 0 1 0-1.414Z"
                    fill="currentColor" />
            </svg>
            split another
        </button>

        <button type="button" class="btn-primary result-actions__primary" @click="onRevealDir">
            <svg viewBox="0 0 16 16" fill="none" xmlns="http://www.w3.org/2000/svg" class="icon-sm" aria-hidden="true">
                <path fill-rule="evenodd" clip-rule="evenodd"
                    d="M1.5 3.5A1.5 1.5 0 0 1 3 2h2.629A1.5 1.5 0 0 1 6.69 2.44L7.81 3.56A.5.5 0 0 0 8.164 3.7H13a1.5 1.5 0 0 1 1.5 1.5v7A1.5 1.5 0 0 1 13 13.7H3A1.5 1.5 0 0 1 1.5 12.2v-8.7Z"
                    fill="currentColor" opacity="0.8" />
            </svg>
            <span class="result-actions__primary-prompt" aria-hidden="true">&gt;</span>
            open folder
        </button>
    </div>

</div>
</template>

<style scoped>
/* ── View wrapper ─────────────────────────────────────────────────────────────── */

.result-view {
    display: flex;
    flex-direction: column;
    gap: var(--space-4);
    width: 100%;
}

/* ── Success header ───────────────────────────────────────────────────────────── */

.result-header {
    display: flex;
    align-items: center;
    gap: var(--space-3);
    padding-bottom: var(--space-3);
    border-bottom: 1px solid var(--color-separator);
}

/* Animated ✓ check badge */
.success-badge {
    flex-shrink: 0;
    width: 30px;
    height: 30px;
    color: var(--color-success);
    filter: drop-shadow(0 0 8px rgba(63, 185, 80, 0.4));
}

.badge-circle {
    opacity: 0.18;
}

.badge-check {
    color: var(--color-success);
}

/* Header text */
.result-header__text {
    flex: 1;
    min-width: 0;
    display: flex;
    flex-direction: column;
    gap: 2px;
}

.result-header__title {
    font-size: var(--text-xl);
    font-weight: var(--weight-bold);
    color: var(--color-text-primary);
    line-height: var(--leading-tight);
    font-family: var(--font-mono);
    display: flex;
    align-items: center;
    gap: var(--space-2);
}

.result-header__ok {
    color: var(--color-success);
    text-shadow: 0 0 10px rgba(63, 185, 80, 0.45);
}

.result-header__summary {
    font-size: var(--text-sm);
    color: var(--color-text-tertiary);
    font-family: var(--font-mono);
    line-height: var(--leading-normal);
}

/* ── Stat chips ───────────────────────────────────────────────────────────────── */

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
    font-family: var(--font-mono);
    letter-spacing: 0.04em;
    padding: 3px var(--space-2);
    border-radius: var(--radius-sm);
    border: 1px solid transparent;
    line-height: 1;
}

.stat-chip--files {
    background: var(--color-success-subtle);
    color: var(--color-success-text);
    border-color: rgba(63, 185, 80, 0.22);
}

.stat-chip--time {
    background: var(--color-accent-subtle);
    color: var(--color-accent);
    border-color: rgba(57, 211, 83, 0.22);
}

/* ── File list container ──────────────────────────────────────────────────────── */

.file-list-container {
    flex: 1;
    min-height: 0;
    background: var(--color-surface-inset);
    border: 1px solid var(--color-border);
    border-radius: var(--radius-lg);
    overflow: hidden;
}

/* `$ ls -1 <dir>` header */
.file-list__header {
    display: flex;
    align-items: center;
    gap: var(--space-3);
    padding: var(--space-2) var(--space-4);
    border-bottom: 1px solid var(--color-border-subtle);
    background: var(--color-surface);
    overflow: hidden;
}

.file-list__cmd {
    font-family: var(--font-mono);
    font-size: var(--text-xs);
    color: var(--color-accent);
    flex-shrink: 0;
    font-weight: var(--weight-semibold);
}

.file-list__dir {
    font-family: var(--font-mono);
    font-size: var(--text-xs);
    color: var(--color-text-quaternary);
    flex: 1;
    min-width: 0;
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
    padding: var(--space-1) 0;
}

/* ── File row ─────────────────────────────────────────────────────────────────── */

.file-row {
    display: flex;
    align-items: center;
    gap: var(--space-2);
    padding: 3px var(--space-4);
    min-height: 28px;
    cursor: default;
    transition: background-color var(--duration-fast) var(--ease-out);
    position: relative;
}

.file-row:hover {
    background: var(--color-surface-hover);
}

/* ── Line number ──────────────────────────────────────────────────────────────── */

.file-row__lineno {
    flex-shrink: 0;
    font-family: var(--font-mono);
    font-size: var(--text-xs);
    color: var(--color-text-quaternary);
    min-width: 28px;
    text-align: right;
    line-height: 1;
    /* Monospace space so the number column aligns */
    white-space: pre;
    user-select: none;
}

/* ── Filename ─────────────────────────────────────────────────────────────────── */

.file-row__name {
    flex: 1;
    font-size: var(--text-sm);
    font-family: var(--font-mono);
    font-weight: var(--weight-regular);
    color: var(--color-text-secondary);
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
    transition: color var(--duration-fast) var(--ease-out);
}

.file-row:hover .file-row__name {
    color: var(--color-text-primary);
}

/* ── Per-row reveal button ────────────────────────────────────────────────────── */

.file-row__reveal {
    position: absolute;
    right: var(--space-3);
    display: inline-flex;
    align-items: center;
    gap: 4px;
    padding: 2px var(--space-2);
    border: 1px solid rgba(57, 211, 83, 0.3);
    border-radius: var(--radius-sm);
    font-family: var(--font-mono);
    font-size: var(--text-xs);
    color: var(--color-accent);
    background: rgba(57, 211, 83, 0.07);
    cursor: pointer;
    transition:
        background-color var(--duration-fast) var(--ease-out),
        box-shadow var(--duration-fast) var(--ease-out);
    white-space: nowrap;
    line-height: 1;
    height: 22px;
}

.file-row__reveal:hover {
    background: rgba(57, 211, 83, 0.14);
    box-shadow: 0 0 8px rgba(57, 211, 83, 0.2);
}

.file-row__reveal:active {
    transform: scale(0.96);
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
    font-family: var(--font-mono);
    padding: var(--space-2) var(--space-4);
    height: 36px;
    color: var(--color-text-secondary);
    gap: var(--space-2);
}

.result-actions__primary {
    min-width: 160px;
    height: 40px;
    font-size: var(--text-sm);
    font-family: var(--font-mono);
    border-radius: var(--radius-md);
    padding: var(--space-2) var(--space-5);
    gap: var(--space-2);
    box-shadow:
        var(--shadow-sm),
        0 0 16px rgba(57, 211, 83, 0.15);
    transition:
        background var(--duration-fast) var(--ease-out),
        box-shadow var(--duration-fast) var(--ease-out),
        transform var(--duration-fast) var(--ease-spring);
}

.result-actions__primary:hover:not(:disabled) {
    box-shadow:
        var(--shadow-md),
        0 0 24px rgba(57, 211, 83, 0.28);
    transform: translateY(-1px);
}

.result-actions__primary:active:not(:disabled) {
    transform: scale(0.97) translateY(0);
    box-shadow: var(--shadow-sm);
}

.result-actions__primary-prompt {
    color: rgba(13, 17, 23, 0.55);
    font-weight: var(--weight-bold);
}
</style>
