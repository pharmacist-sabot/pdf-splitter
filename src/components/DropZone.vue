<script setup lang="ts">
/**
 * DropZone — idle state view.
 *
 * Terminal aesthetic: looks like a file-drop prompt in a modern terminal
 * emulator.  The dashed border glows green on drag-over; text uses prompt
 * prefixes (`$`, `>`) to reinforce the terminal metaphor.
 *
 * Events
 * ──────
 * • `pick`  — user clicked the "select file" button.
 * • `drop`  — user dropped a PDF; payload is the absolute path string.
 *
 * Props
 * ─────
 * • `busy` — disables interaction while an async operation is in flight.
 */

import { ref } from 'vue'

// ── Props & emits ──────────────────────────────────────────────────────────────

const props = defineProps<{
    /** Disables the zone while the parent is performing an async operation. */
    busy: boolean
}>()

const emit = defineEmits<{
    pick: []
    drop: [path: string]
}>()

// ── Drag state ─────────────────────────────────────────────────────────────────

const isDragOver = ref(false)
let dragCounter = 0

// ── Drag handlers ──────────────────────────────────────────────────────────────

function onDragEnter(event: DragEvent): void {
    event.preventDefault()
    dragCounter++
    isDragOver.value = true
}

function onDragLeave(event: DragEvent): void {
    event.preventDefault()
    dragCounter--
    if (dragCounter <= 0) {
        dragCounter = 0
        isDragOver.value = false
    }
}

function onDragOver(event: DragEvent): void {
    event.preventDefault()
    if (event.dataTransfer) {
        event.dataTransfer.dropEffect = 'copy'
    }
}

function onDrop(event: DragEvent): void {
    event.preventDefault()
    dragCounter = 0
    isDragOver.value = false

    if (props.busy) return

    const files = event.dataTransfer?.files
    if (!files || files.length === 0) return

    const file = files[0]
    const isPdf =
        file.type === 'application/pdf' ||
        file.name.toLowerCase().endsWith('.pdf')
    if (!isPdf) return

    // eslint-disable-next-line @typescript-eslint/no-explicit-any
    const path: string | undefined = (file as any).path
    if (path) emit('drop', path)
}

// ── Click handler ──────────────────────────────────────────────────────────────

function onSelectClick(): void {
    if (!props.busy) emit('pick')
}
</script>

<template>
<div class="drop-zone-wrapper">

    <!-- ── Drop target ──────────────────────────────────────────────────────── -->
    <div class="drop-zone" :class="{
        'drop-zone--active': isDragOver && !busy,
        'drop-zone--busy': busy,
    }" role="button" tabindex="0" aria-label="Drop a PDF file here or press Space to open the file picker"
        @dragenter="onDragEnter" @dragleave="onDragLeave" @dragover="onDragOver" @drop="onDrop" @click="onSelectClick"
        @keydown.space.prevent="onSelectClick" @keydown.enter.prevent="onSelectClick">
        <!-- Green glow overlay on drag-over -->
        <div class="drop-zone__glow" aria-hidden="true" />

        <!-- Content -->
        <div class="drop-zone__content">

            <!-- Terminal PDF icon -->
            <div class="drop-zone__icon" :class="{ 'animate-float': !isDragOver && !busy }" aria-hidden="true">
                <div class="drop-zone__icon-box">
                    <!-- ASCII-style document icon rendered as SVG -->
                    <svg viewBox="0 0 56 64" fill="none" xmlns="http://www.w3.org/2000/svg" width="56" height="64">
                        <!-- Document body -->
                        <rect x="2" y="2" width="44" height="56" rx="3" fill="currentColor" class="icon-body" />
                        <!-- Folded corner -->
                        <path d="M32 2 L46 16 L32 16 Z" fill="currentColor" class="icon-fold" />
                        <!-- Corner fold line -->
                        <path d="M32 2 L46 16 L32 16 Z" stroke="currentColor" stroke-width="1" class="icon-fold-stroke"
                            fill="none" />
                        <!-- Label band -->
                        <rect x="0" y="40" width="48" height="16" rx="2" fill="currentColor" class="icon-band" />
                        <!-- "PDF" label lines -->
                        <rect x="6" y="45.5" width="7" height="1.8" rx="0.9" fill="currentColor" class="icon-label" />
                        <rect x="15" y="45.5" width="5" height="1.8" rx="0.9" fill="currentColor" class="icon-label" />
                        <rect x="22" y="45.5" width="5" height="1.8" rx="0.9" fill="currentColor" class="icon-label" />
                        <!-- Content lines -->
                        <rect x="7" y="18" width="22" height="1.5" rx="0.75" fill="currentColor" class="icon-line" />
                        <rect x="7" y="22" width="18" height="1.5" rx="0.75" fill="currentColor" class="icon-line" />
                        <rect x="7" y="26" width="20" height="1.5" rx="0.75" fill="currentColor" class="icon-line" />
                        <rect x="7" y="30" width="14" height="1.5" rx="0.75" fill="currentColor" class="icon-line" />
                    </svg>
                </div>
            </div>

            <!-- Labels -->
            <div class="drop-zone__label">
                <template v-if="isDragOver && !busy">
                    <span class="drop-zone__label-prompt" aria-hidden="true">&gt;</span>
                    <span class="drop-zone__label-main drop-zone__label-main--active">
                        release to load
                    </span>
                </template>
                <template v-else-if="busy">
                    <span class="drop-zone__label-prompt" aria-hidden="true">$</span>
                    <span class="drop-zone__label-main animate-pulse">
                        loading<span class="drop-zone__ellipsis">...</span>
                    </span>
                </template>
                <template v-else>
                    <span class="drop-zone__label-prompt" aria-hidden="true">$</span>
                    <div class="drop-zone__label-stack">
                        <span class="drop-zone__label-main">
                            drop .pdf file here
                        </span>
                        <span class="drop-zone__label-sub">
                            or click to browse
                        </span>
                    </div>
                </template>
            </div>

            <!-- Select-file button -->
            <Transition name="fade">
                <div v-if="!isDragOver" class="drop-zone__actions">
                    <button type="button" class="btn-primary btn-lg drop-zone__btn" :disabled="busy" tabindex="-1"
                        aria-hidden="true" @click.stop="onSelectClick">
                        <span class="drop-zone__btn-prompt" aria-hidden="true">&gt;</span>
                        select-file
                    </button>
                </div>
            </Transition>
        </div>

        <!-- SVG dashed border (terminal green on drag-over) -->
        <svg class="drop-zone__border" aria-hidden="true" xmlns="http://www.w3.org/2000/svg">
            <rect x="1" y="1" width="calc(100% - 2px)" height="calc(100% - 2px)" rx="9" ry="9" fill="none"
                stroke="currentColor" stroke-width="1.5" stroke-dasharray="6 4" />
        </svg>
    </div>

    <!-- Hint text -->
    <p class="drop-zone__hint" aria-live="polite">
        <template v-if="isDragOver && !busy">
            <span class="drop-zone__hint-prompt" aria-hidden="true">#</span>
            PDF files only
        </template>
        <template v-else>
            <span class="drop-zone__hint-prompt" aria-hidden="true">#</span>
            accepts all pdf versions &nbsp;·&nbsp; any number of pages
        </template>
    </p>
</div>
</template>

<style scoped>
/* ── Wrapper ──────────────────────────────────────────────────────────────────── */

.drop-zone-wrapper {
    display: flex;
    flex-direction: column;
    align-items: center;
    gap: var(--space-3);
    width: 100%;
}

/* ── Drop zone container ──────────────────────────────────────────────────────── */

.drop-zone {
    position: relative;
    width: 100%;
    min-height: 260px;
    display: flex;
    align-items: center;
    justify-content: center;
    border-radius: var(--radius-lg);
    background: var(--color-surface-inset);
    cursor: pointer;
    overflow: hidden;
    transition:
        background-color var(--duration-normal) var(--ease-out),
        box-shadow var(--duration-normal) var(--ease-out),
        transform var(--duration-normal) var(--ease-spring);
    outline: none;
}

.drop-zone:focus-visible {
    box-shadow: var(--shadow-focus);
}

/* Drag-over state — phosphor glow */
.drop-zone--active {
    background: rgba(57, 211, 83, 0.04);
    transform: scale(1.004);
    box-shadow:
        0 0 0 1px var(--color-accent),
        0 0 28px rgba(57, 211, 83, 0.18);
}

/* Busy state */
.drop-zone--busy {
    cursor: default;
    opacity: 0.65;
}

/* ── Glow overlay (drag-over) ────────────────────────────────────────────────── */

.drop-zone__glow {
    position: absolute;
    inset: -60px;
    background: radial-gradient(ellipse at center,
            rgba(57, 211, 83, 0.12) 0%,
            transparent 65%);
    opacity: 0;
    pointer-events: none;
    transition: opacity var(--duration-normal) var(--ease-out);
}

.drop-zone--active .drop-zone__glow {
    opacity: 1;
}

/* ── SVG dashed border ────────────────────────────────────────────────────────── */

.drop-zone__border {
    position: absolute;
    inset: 0;
    width: 100%;
    height: 100%;
    pointer-events: none;
    color: var(--color-border);
    transition: color var(--duration-normal) var(--ease-out);
    overflow: visible;
}

.drop-zone--active .drop-zone__border {
    color: var(--color-accent);
    filter: drop-shadow(0 0 4px rgba(57, 211, 83, 0.5));
}

/* ── Content ──────────────────────────────────────────────────────────────────── */

.drop-zone__content {
    position: relative;
    z-index: 1;
    display: flex;
    flex-direction: column;
    align-items: center;
    gap: var(--space-5);
    padding: var(--space-8) var(--space-6);
    text-align: center;
    pointer-events: none;
}

.drop-zone__actions {
    pointer-events: auto;
}

/* ── Icon ─────────────────────────────────────────────────────────────────────── */

.drop-zone__icon {
    color: var(--color-text-tertiary);
    transition: color var(--duration-normal) var(--ease-out);
}

.drop-zone--active .drop-zone__icon {
    color: var(--color-accent);
}

/* Terminal box around the icon */
.drop-zone__icon-box {
    position: relative;
    display: inline-flex;
    padding: var(--space-3);
    border: 1px solid var(--color-border);
    border-radius: var(--radius-md);
    background: var(--color-surface);
    transition:
        border-color var(--duration-normal) var(--ease-out),
        box-shadow var(--duration-normal) var(--ease-out);
}

.drop-zone--active .drop-zone__icon-box {
    border-color: rgba(57, 211, 83, 0.35);
    box-shadow: 0 0 12px rgba(57, 211, 83, 0.15);
}

.icon-body {
    opacity: 0.12;
}

.icon-fold {
    opacity: 0.06;
}

.icon-fold-stroke {
    opacity: 0.15;
}

.icon-band {
    opacity: 0.70;
}

.icon-label {
    opacity: 0.85;
    fill: currentColor;
}

.icon-line {
    opacity: 0.14;
}

.drop-zone--active .icon-body {
    opacity: 0.18;
}

.drop-zone--active .icon-band {
    opacity: 0.88;
}

/* ── Labels ───────────────────────────────────────────────────────────────────── */

.drop-zone__label {
    display: flex;
    align-items: center;
    gap: var(--space-2);
}

.drop-zone__label-stack {
    display: flex;
    flex-direction: column;
    align-items: flex-start;
    gap: 3px;
}

/* Terminal prompt character: $ or > */
.drop-zone__label-prompt {
    font-size: var(--text-xl);
    font-weight: var(--weight-bold);
    color: var(--color-accent);
    font-family: var(--font-mono);
    line-height: 1;
    text-shadow: 0 0 8px var(--color-accent-glow);
    flex-shrink: 0;
}

.drop-zone__label-main {
    font-size: var(--text-lg);
    font-weight: var(--weight-semibold);
    letter-spacing: -0.01em;
    color: var(--color-text-primary);
    transition: color var(--duration-normal) var(--ease-out);
    font-family: var(--font-mono);
}

.drop-zone__label-main--active {
    color: var(--color-accent);
    text-shadow: 0 0 10px rgba(57, 211, 83, 0.3);
}

.drop-zone__label-sub {
    font-size: var(--text-sm);
    color: var(--color-text-tertiary);
    font-weight: var(--weight-regular);
    font-family: var(--font-mono);
}

.drop-zone__ellipsis {
    display: inline-block;
    letter-spacing: 0.05em;
}

/* ── Button ───────────────────────────────────────────────────────────────────── */

.drop-zone__btn {
    gap: var(--space-2);
}

.drop-zone__btn-prompt {
    font-weight: var(--weight-bold);
    opacity: 0.8;
}

/* ── Hint ─────────────────────────────────────────────────────────────────────── */

.drop-zone__hint {
    display: flex;
    align-items: center;
    gap: var(--space-2);
    font-size: var(--text-xs);
    color: var(--color-text-quaternary);
    font-family: var(--font-mono);
    letter-spacing: 0.04em;
    text-align: center;
    line-height: var(--leading-normal);
}

.drop-zone__hint-prompt {
    color: var(--color-text-quaternary);
    opacity: 0.6;
}
</style>
