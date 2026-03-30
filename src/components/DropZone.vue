<script setup lang="ts">
/**
 * DropZone — idle state view.
 *
 * Renders a large drag-and-drop target that accepts PDF files dragged from
 * Finder, plus a "Select File" button for the classic click-to-browse flow.
 *
 * Events
 * ──────
 * • `pick`  — emitted when the user clicks the "Select File" button.
 *             The parent composable (`usePdfSplitter`) owns the actual dialog
 *             invocation; this component is purely presentational.
 * • `drop`  — emitted with a file path string when the user drops a PDF onto
 *             the zone.  The parent validates and processes the path.
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
    /** User clicked "Select File". */
    pick: []
    /**
     * User dropped a file onto the zone.
     * @param path — absolute path of the dropped file (first file only).
     */
    drop: [path: string]
}>()

// ── Drag state ─────────────────────────────────────────────────────────────────

/** True while a drag gesture is hovering over the zone. */
const isDragOver = ref(false)

/**
 * Counter-based drag tracking prevents flickering when the cursor moves
 * between child elements (each enter/leave pair fires on child elements
 * without a matching event on the parent).
 */
let dragCounter = 0

// ── Drag event handlers ────────────────────────────────────────────────────────

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

    // Guard: only accept PDF files based on MIME type or extension.
    const isPdf =
        file.type === 'application/pdf' ||
        file.name.toLowerCase().endsWith('.pdf')

    if (!isPdf) return

    // In Tauri's WKWebView the File object's `path` property is populated with
    // the absolute filesystem path on macOS.
    // eslint-disable-next-line @typescript-eslint/no-explicit-any
    const path: string | undefined = (file as any).path
    if (path) {
        emit('drop', path)
    }
}

// ── Click handler ──────────────────────────────────────────────────────────────

function onSelectClick(): void {
    if (!props.busy) {
        emit('pick')
    }
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
        <!-- Animated background glow (visible on drag-over) -->
        <div class="drop-zone__glow" aria-hidden="true" />

        <!-- Content -->
        <div class="drop-zone__content">
            <!-- PDF icon with float animation -->
            <div class="drop-zone__icon" :class="{ 'animate-float': !isDragOver && !busy }" aria-hidden="true">
                <svg viewBox="0 0 64 64" fill="none" xmlns="http://www.w3.org/2000/svg" class="icon-2xl"
                    style="width: 64px; height: 64px;">
                    <!-- Document body -->
                    <rect x="10" y="4" width="36" height="48" rx="4" fill="currentColor" class="icon-body" />
                    <!-- Folded corner -->
                    <path d="M38 4 L46 12 L38 12 Z" fill="currentColor" class="icon-fold" />
                    <!-- PDF label band -->
                    <rect x="8" y="36" width="40" height="14" rx="3" fill="currentColor" class="icon-band" />
                    <!-- "PDF" text lines (decorative) -->
                    <rect x="16" y="42" width="8" height="2" rx="1" fill="white" opacity="0.9" />
                    <rect x="26" y="42" width="6" height="2" rx="1" fill="white" opacity="0.9" />
                    <rect x="34" y="42" width="6" height="2" rx="1" fill="white" opacity="0.9" />
                    <!-- Content lines -->
                    <rect x="16" y="16" width="20" height="2" rx="1" fill="currentColor" class="icon-line" />
                    <rect x="16" y="21" width="16" height="2" rx="1" fill="currentColor" class="icon-line" />
                    <rect x="16" y="26" width="18" height="2" rx="1" fill="currentColor" class="icon-line" />
                </svg>
            </div>

            <!-- Primary label -->
            <div class="drop-zone__label">
                <template v-if="isDragOver && !busy">
                    <span class="drop-zone__label-main drop-zone__label-main--active">
                        Release to open
                    </span>
                </template>
                <template v-else-if="busy">
                    <span class="drop-zone__label-main">
                        Loading…
                    </span>
                </template>
                <template v-else>
                    <span class="drop-zone__label-main">
                        Drop your PDF here
                    </span>
                    <span class="drop-zone__label-sub">
                        or click to browse files
                    </span>
                </template>
            </div>

            <!-- Select file button (hidden during drag-over) -->
            <Transition name="fade">
                <div v-if="!isDragOver" class="drop-zone__actions">
                    <button type="button" class="btn-primary btn-lg" :disabled="busy" tabindex="-1" aria-hidden="true"
                        @click.stop="onSelectClick">
                        <svg viewBox="0 0 20 20" fill="none" xmlns="http://www.w3.org/2000/svg" class="icon-sm"
                            aria-hidden="true">
                            <path
                                d="M10 2.5C10.4142 2.5 10.75 2.83579 10.75 3.25V9.25H16.75C17.1642 9.25 17.5 9.58579 17.5 10C17.5 10.4142 17.1642 10.75 16.75 10.75H10.75V16.75C10.75 17.1642 10.4142 17.5 10 17.5C9.58579 17.5 9.25 17.1642 9.25 16.75V10.75H3.25C2.83579 10.75 2.5 10.4142 2.5 10C2.5 9.58579 2.83579 9.25 3.25 9.25H9.25V3.25C9.25 2.83579 9.58579 2.5 10 2.5Z"
                                fill="currentColor" />
                        </svg>
                        Select PDF
                    </button>
                </div>
            </Transition>
        </div>

        <!-- Dashed border (rendered as SVG for crisp rendering at all DPIs) -->
        <svg class="drop-zone__border" aria-hidden="true" xmlns="http://www.w3.org/2000/svg">
            <rect x="1" y="1" width="calc(100% - 2px)" height="calc(100% - 2px)" rx="19" ry="19" fill="none"
                stroke="currentColor" stroke-width="2" stroke-dasharray="8 5" />
        </svg>
    </div>

    <!-- Hint text below the zone -->
    <p class="drop-zone__hint" aria-live="polite">
        <template v-if="isDragOver && !busy">
            PDF files only
        </template>
        <template v-else>
            Supports all PDF versions &nbsp;·&nbsp; Any number of pages
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
    gap: var(--space-4);
    width: 100%;
}

/* ── Drop zone container ──────────────────────────────────────────────────────── */

.drop-zone {
    position: relative;
    width: 100%;
    min-height: 280px;
    display: flex;
    align-items: center;
    justify-content: center;
    border-radius: var(--radius-xl);
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

/* Drag-over state */
.drop-zone--active {
    background: var(--color-accent-subtle);
    transform: scale(1.005);
    box-shadow:
        0 0 0 2px var(--color-accent),
        0 8px 32px var(--color-accent-glow);
}

/* Busy state */
.drop-zone--busy {
    cursor: default;
    opacity: 0.7;
}

/* ── Animated glow overlay (drag-over) ───────────────────────────────────────── */

.drop-zone__glow {
    position: absolute;
    inset: -40px;
    background: radial-gradient(ellipse at center,
            var(--color-accent-glow) 0%,
            transparent 70%);
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
    color: var(--color-border-strong);
    transition: color var(--duration-normal) var(--ease-out);
    overflow: visible;
}

.drop-zone--active .drop-zone__border {
    color: var(--color-accent);
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
    /* let parent handle click */
}

/* Re-enable pointer events for the button only */
.drop-zone__actions {
    pointer-events: auto;
}

/* ── Icon ──────────────────────────────────────────────────────────────────────── */

.drop-zone__icon {
    color: var(--color-text-tertiary);
    transition: color var(--duration-normal) var(--ease-out);
    filter: drop-shadow(0 4px 12px rgba(0, 0, 0, 0.08));
}

.drop-zone--active .drop-zone__icon {
    color: var(--color-accent);
    filter: drop-shadow(0 4px 16px var(--color-accent-glow));
}

.icon-body {
    opacity: 0.18;
}

.icon-fold {
    opacity: 0.10;
}

.icon-band {
    opacity: 0.75;
}

.icon-line {
    opacity: 0.15;
}

.drop-zone--active .icon-body {
    opacity: 0.22;
}

.drop-zone--active .icon-band {
    opacity: 0.90;
}

/* ── Labels ───────────────────────────────────────────────────────────────────── */

.drop-zone__label {
    display: flex;
    flex-direction: column;
    align-items: center;
    gap: var(--space-2);
}

.drop-zone__label-main {
    font-size: var(--text-xl);
    font-weight: var(--weight-semibold);
    letter-spacing: var(--tracking-tight);
    color: var(--color-text-primary);
    transition: color var(--duration-normal) var(--ease-out);
}

.drop-zone__label-main--active {
    color: var(--color-accent);
}

.drop-zone__label-sub {
    font-size: var(--text-base);
    color: var(--color-text-tertiary);
    font-weight: var(--weight-regular);
}

/* ── Hint ─────────────────────────────────────────────────────────────────────── */

.drop-zone__hint {
    font-size: var(--text-xs);
    color: var(--color-text-quaternary);
    font-weight: var(--weight-regular);
    letter-spacing: var(--tracking-wide);
    text-transform: uppercase;
    line-height: var(--leading-normal);
    text-align: center;
}
</style>
