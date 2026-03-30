<script setup lang="ts">
/**
 * App.vue — root component for PDF Splitter.
 *
 * # Terminal Design Language
 *
 * * Window chrome mimics a terminal emulator title bar.
 * * Wordmark uses a `$ pdf-splitter` prompt with a blinking cursor.
 * * State transitions are quick and crisp, matching terminal responsiveness.
 * * Subtle scanline overlay in `::before` evokes a classic CRT phosphor
 *   display without looking retro or ugly.
 *
 * # State machine
 *
 * ```
 * idle ──pick/drop──► ready ──split──► processing ──► complete
 *  ▲                    │                   │              │
 *  │                    │                  error           │
 *  │                    └──────────────────► error         │
 *  └──────────────────────────────────── reset ────────────┘
 * ```
 */

import { computed } from 'vue'
import { usePdfSplitter } from '@/composables/usePdfSplitter'
import DropZone from '@/components/DropZone.vue'
import FileCard from '@/components/FileCard.vue'
import ProgressView from '@/components/ProgressView.vue'
import ResultView from '@/components/ResultView.vue'
import ErrorView from '@/components/ErrorView.vue'

// ── Composable ─────────────────────────────────────────────────────────────────

const {
    // State
    state,
    fileInfo,
    outputDir,
    operation,
    result,
    error,
    isBusy,
    // Computed
    fileSizeFormatted,
    progressPercent,
    elapsedFormatted,
    outputDirShort,
    // Actions
    pickFile,
    pickOutputDir,
    startSplit,
    revealOutput,
    reset,
} = usePdfSplitter()

// ── Derived ────────────────────────────────────────────────────────────────────

/**
 * Terminal-style window title that updates with each state transition.
 * Mirrors the active operation so the window is identifiable in the Dock.
 */
const windowTitle = computed<string>(() => {
    switch (state.value) {
        case 'idle': return '~/pdf-splitter'
        case 'ready': return `~/pdf-splitter — ${fileInfo.value?.name ?? ''}`
        case 'processing': return `splitting… ${progressPercent.value}%`
        case 'complete': return `done — ${result.value?.totalPages ?? 0} pages`
        case 'error': return '~/pdf-splitter — error'
        default: return '~/pdf-splitter'
    }
})

/**
 * Show the wordmark (prompt + subtitle) only in idle and ready states.
 * Hidden during active operations to maximise content space.
 */
const showSubtitle = computed<boolean>(
    () => state.value === 'idle' || state.value === 'ready',
)

// ── Event handlers ─────────────────────────────────────────────────────────────

/**
 * Handle a PDF dropped onto the DropZone.
 *
 * TODO(v1.1): Add `load_pdf_from_path(path)` command on the Rust side so we
 * can load the dropped file directly without re-opening the dialog.
 */
async function onDrop(path: string): Promise<void> {
    void path
    await pickFile()
}

/** User clicked "Try Again" in the error view. */
async function onRetry(): Promise<void> {
    await reset()
    await pickFile()
}

/** User clicked "Dismiss" in the error view. */
async function onDismiss(): Promise<void> {
    await reset()
}
</script>

<template>
<div class="app" :data-state="state">

    <!-- ── Scanline overlay ────────────────────────────────────────────── -->
    <!-- Pure-CSS phosphor-CRT scanlines; pointer-events disabled so they  -->
    <!-- never interfere with interaction.                                  -->
    <div class="app__scanlines" aria-hidden="true" />

    <!-- ── Terminal title bar ─────────────────────────────────────────── -->
    <!--
        Tauri `titleBarStyle: "Overlay"` keeps native traffic lights in
        place while our HTML fills the full window.  We reserve the top
        28 px as a drag region.
    -->
    <header class="titlebar" data-tauri-drag-region aria-hidden="true">
        <!-- Traffic-light placeholder (native buttons rendered by WebKit) -->
        <div class="titlebar__traffic-lights" data-no-drag />

        <!-- Centred window title in terminal path format -->
        <span class="titlebar__title" aria-live="polite">
            {{ windowTitle }}
        </span>
    </header>

    <!-- ── Main content ───────────────────────────────────────────────── -->
    <main class="app__main" role="main">

        <!-- App wordmark — terminal prompt style -->
        <Transition name="fade">
            <div v-if="showSubtitle" class="app__wordmark">
                <!-- >_ terminal icon -->
                <div class="app__logo" aria-hidden="true">
                    <span class="app__logo-bracket">[</span><span class="app__logo-slash">/</span><span
                        class="app__logo-bracket">]</span>
                </div>

                <div class="app__heading-group">
                    <h1 class="app__title">
                        <span class="app__prompt" aria-hidden="true">$ </span>pdf-splitter<span
                            class="app__cursor cursor-blink" aria-hidden="true">▌</span>
                    </h1>
                    <p class="app__subtitle">
                        <span class="app__prompt-dim" aria-hidden="true">&gt; </span>extract every page into its own pdf
                        — fast
                    </p>
                </div>
            </div>
        </Transition>

        <!-- ── State views ──────────────────────────────────────────────── -->
        <div class="app__content">
            <Transition name="view" mode="out-in">

                <!-- idle: drop zone -->
                <div v-if="state === 'idle'" key="idle" class="view view--idle">
                    <DropZone :busy="isBusy" @pick="pickFile" @drop="onDrop" />
                </div>

                <!-- ready: file card + split button -->
                <div v-else-if="state === 'ready' && fileInfo" key="ready" class="view view--ready">
                    <FileCard :file-name="fileInfo.name" :page-count="fileInfo.pageCount"
                        :file-size-formatted="fileSizeFormatted" :output-dir-short="outputDirShort" :busy="isBusy"
                        @split="startSplit" @change-file="pickFile" @change-output="pickOutputDir" />
                </div>

                <!-- processing: progress view -->
                <div v-else-if="state === 'processing'" key="processing" class="view view--processing">
                    <div class="processing-card card">
                        <ProgressView :percent="progressPercent" :current="operation?.progress?.current ?? 0"
                            :total="operation?.progress?.total ?? (fileInfo?.pageCount ?? 0)"
                            :current-file="operation?.progress?.fileName ?? ''" :file-name="fileInfo?.name ?? ''" />
                    </div>
                </div>

                <!-- complete: result view -->
                <div v-else-if="state === 'complete' && result" key="complete" class="view view--complete">
                    <div class="result-wrapper">
                        <ResultView :total-pages="result.totalPages" :output-files="result.outputFiles"
                            :elapsed-formatted="elapsedFormatted" :output-dir="outputDir" @reveal="revealOutput"
                            @reset="reset" />
                    </div>
                </div>

                <!-- error: error view -->
                <div v-else-if="state === 'error'" key="error" class="view view--error">
                    <div class="error-card card">
                        <ErrorView :message="error ?? 'An unexpected error occurred.'" @retry="onRetry"
                            @dismiss="onDismiss" />
                    </div>
                </div>

            </Transition>
        </div>

    </main>

    <!-- ── Footer ─────────────────────────────────────────────────────── -->
    <footer class="app__footer" role="contentinfo">
        <span class="app__footer-text">
            <span class="app__footer-comment" aria-hidden="true">#</span>
            pdf-splitter &nbsp;·&nbsp; open source &nbsp;·&nbsp; MIT
        </span>
    </footer>

</div>
</template>

<style scoped>
/* ── App shell ────────────────────────────────────────────────────────────────── */

.app {
    width: 720px;
    height: 560px;
    display: flex;
    flex-direction: column;
    overflow: hidden;
    position: relative;
    background: var(--color-bg);
}

/* ── CRT scanline overlay ─────────────────────────────────────────────────────── */

.app__scanlines {
    position: absolute;
    inset: 0;
    background: repeating-linear-gradient(0deg,
            transparent,
            transparent 3px,
            rgba(0, 0, 0, 0.07) 3px,
            rgba(0, 0, 0, 0.07) 4px);
    pointer-events: none;
    z-index: var(--z-overlay);
    /* Subtle — visible only at close range */
    opacity: 0.55;
}

/* ── Terminal title bar ───────────────────────────────────────────────────────── */

.titlebar {
    position: relative;
    height: var(--titlebar-height);
    display: flex;
    align-items: center;
    justify-content: center;
    flex-shrink: 0;
    z-index: var(--z-overlay);
    /* Fine separator line below the chrome bar */
    border-bottom: 1px solid var(--color-border-subtle);
}

/* Spacer matching the traffic-light cluster width (~72 px on macOS) */
.titlebar__traffic-lights {
    position: absolute;
    left: 0;
    width: 72px;
    height: 100%;
}

.titlebar__title {
    font-size: 11px;
    font-weight: var(--weight-medium);
    letter-spacing: 0.02em;
    color: var(--color-text-quaternary);
    font-family: var(--font-mono);
    pointer-events: none;
    max-width: 400px;
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
    /* Subtle green tint on active operations */
    transition: color var(--duration-normal) var(--ease-out);
}

/* ── Main area ────────────────────────────────────────────────────────────────── */

.app__main {
    flex: 1;
    display: flex;
    flex-direction: column;
    padding: var(--space-4) var(--space-8) var(--space-3);
    gap: var(--space-4);
    overflow: hidden;
    min-height: 0;
}

/* ── Wordmark — terminal prompt ───────────────────────────────────────────────── */

.app__wordmark {
    display: flex;
    align-items: center;
    gap: var(--space-4);
    flex-shrink: 0;
    padding-bottom: var(--space-1);
    border-bottom: 1px solid var(--color-border-subtle);
}

/* Terminal [/] bracket icon */
.app__logo {
    flex-shrink: 0;
    font-family: var(--font-mono);
    font-size: 20px;
    font-weight: var(--weight-bold);
    line-height: 1;
    letter-spacing: -0.04em;
    color: var(--color-accent);
    text-shadow: 0 0 14px var(--color-accent-glow);
    /* Subtle entrance */
    transition: text-shadow var(--duration-normal) var(--ease-out);
}

.app__logo:hover {
    text-shadow: 0 0 20px rgba(57, 211, 83, 0.4);
}

.app__logo-bracket {
    color: var(--color-text-tertiary);
    opacity: 0.8;
}

.app__logo-slash {
    color: var(--color-accent);
}

.app__heading-group {
    display: flex;
    flex-direction: column;
    gap: 3px;
    min-width: 0;
}

/* $ pdf-splitter▌ */
.app__title {
    font-size: var(--text-xl);
    font-weight: var(--weight-bold);
    letter-spacing: -0.015em;
    color: var(--color-text-primary);
    line-height: var(--leading-tight);
    display: flex;
    align-items: baseline;
    gap: 0;
}

.app__prompt {
    color: var(--color-accent);
    font-weight: var(--weight-regular);
}

.app__cursor {
    color: var(--color-accent);
    font-size: 0.9em;
    margin-left: 2px;
    text-shadow: 0 0 8px var(--color-accent-glow);
}

/* > extract every page... */
.app__subtitle {
    font-size: var(--text-sm);
    color: var(--color-text-tertiary);
    font-weight: var(--weight-regular);
    line-height: var(--leading-normal);
    display: flex;
    align-items: baseline;
    gap: 0;
}

.app__prompt-dim {
    color: var(--color-text-quaternary);
    font-size: var(--text-xs);
}

/* ── Content container ────────────────────────────────────────────────────────── */

.app__content {
    flex: 1;
    display: flex;
    flex-direction: column;
    min-height: 0;
    position: relative;
}

/* ── Individual state views ───────────────────────────────────────────────────── */

.view {
    display: flex;
    flex-direction: column;
    width: 100%;
    flex: 1;
}

/* ── Processing card ──────────────────────────────────────────────────────────── */

.processing-card {
    padding: var(--space-8);
    flex: 1;
    display: flex;
    flex-direction: column;
    justify-content: center;
}

/* ── Result wrapper ───────────────────────────────────────────────────────────── */

.result-wrapper {
    flex: 1;
    display: flex;
    flex-direction: column;
    min-height: 0;
}

.result-wrapper :deep(.result-view) {
    flex: 1;
    min-height: 0;
}

.result-wrapper :deep(.file-list-container) {
    max-height: 200px;
}

/* ── Error card ───────────────────────────────────────────────────────────────── */

.error-card {
    padding: var(--space-8);
    flex: 1;
    display: flex;
    flex-direction: column;
    justify-content: center;
}

/* ── Footer ───────────────────────────────────────────────────────────────────── */

.app__footer {
    height: 26px;
    display: flex;
    align-items: center;
    justify-content: center;
    flex-shrink: 0;
    padding: 0 var(--space-8);
    border-top: 1px solid var(--color-border-subtle);
}

.app__footer-text {
    font-size: 10px;
    color: var(--color-text-quaternary);
    letter-spacing: 0.06em;
    white-space: nowrap;
    font-family: var(--font-mono);
}

.app__footer-comment {
    color: var(--color-text-quaternary);
    margin-right: var(--space-1);
    opacity: 0.6;
}

/* ── State-based background tints ─────────────────────────────────────────────── */

/* Very subtle radial glow from the top, colour-coded by state. */

.app[data-state="processing"] {
    background-image: radial-gradient(ellipse 60% 30% at 50% 0%,
            rgba(57, 211, 83, 0.04) 0%,
            transparent 100%);
}

.app[data-state="complete"] {
    background-image: radial-gradient(ellipse 60% 30% at 50% 0%,
            rgba(63, 185, 80, 0.06) 0%,
            transparent 100%);
}

.app[data-state="error"] {
    background-image: radial-gradient(ellipse 60% 30% at 50% 0%,
            rgba(248, 81, 73, 0.05) 0%,
            transparent 100%);
}

/* Title text picks up a hint of colour during active operations */
.app[data-state="processing"] .titlebar__title,
.app[data-state="complete"] .titlebar__title {
    color: rgba(57, 211, 83, 0.55);
}

.app[data-state="error"] .titlebar__title {
    color: rgba(248, 81, 73, 0.55);
}
</style>
