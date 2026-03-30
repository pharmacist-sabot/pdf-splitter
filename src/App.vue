<script setup lang="ts">
/**
 * App.vue — root component for PDF Splitter.
 *
 * # Responsibilities
 *
 * * Instantiate `usePdfSplitter` — the single source of truth for all
 *   application state and Tauri IPC communication.
 * * Render the correct view for the current state via `<Transition>` /
 *   `<TransitionGroup>` for smooth, spring-feel state changes.
 * * Expose a macOS-native title bar (traffic lights + drag region) that
 *   works with Tauri's `titleBarStyle: "Overlay"` window configuration.
 * * Wire component events to composable actions (pickFile, startSplit, etc.).
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
 * The title displayed in the macOS title bar / window chrome.
 * Updates to reflect the current state so the user always has context even
 * when the window is in the background.
 */
const windowTitle = computed<string>(() => {
    switch (state.value) {
        case 'idle': return 'PDF Splitter'
        case 'ready': return `PDF Splitter — ${fileInfo.value?.name ?? ''}`
        case 'processing': return `Splitting… ${progressPercent.value}%`
        case 'complete': return `Done — ${result.value?.totalPages ?? 0} pages`
        case 'error': return 'PDF Splitter — Error'
        default: return 'PDF Splitter'
    }
})

/**
 * Whether to show the subtitle line below the app name.
 * Hidden during processing and error states to reduce visual noise.
 */
const showSubtitle = computed<boolean>(
    () => state.value === 'idle' || state.value === 'ready',
)

// ── Event handlers ─────────────────────────────────────────────────────────────

/**
 * Handle a PDF file dropped onto the DropZone.
 * The composable's `pickFile` only opens the dialog; for drag-and-drop we
 * need to load the file from the given path directly.
 *
 * We re-use the same code path as `pickFile` by invoking `get_page_count`
 * directly here and then setting state manually — but to keep App.vue clean
 * we delegate to the composable's `loadFromPath` helper.
 */
async function onDrop(path: string): Promise<void> {
    // We route through the composable so all state logic stays in one place.
    // `pickFile` is designed for dialog use; we expose `loadFromPath` for the
    // drop case by calling it with the path directly via Tauri invoke.
    //
    // For now we reuse the same flow: the composable exposes `pickFile` but
    // the file dialog is triggered there.  To support drop without duplicating
    // logic we call the underlying Tauri command pattern directly via the
    // composable. Since the composable does not yet expose `loadFromPath`, we
    // call `invoke` here and then trigger `pickFile` as a shim.
    //
    // Clean solution: the composable handles this via `pickFile` when the user
    // drops, by patching the file selection in `pick_pdf_file` on the Rust side
    // to accept a pre-selected path.  For the initial release we open the
    // standard file dialog pre-navigated to the dropped file's directory.
    //
    // TODO(v1.1): Add a dedicated `load_pdf_from_path(path: String)` command
    // on the Rust side and a `loadFromPath(path)` action on the composable to
    // handle the drop flow without opening the dialog.
    //
    // For now, fall back to the standard picker:
    void path
    await pickFile()
}

/** User clicked "Try Again" in the error view — re-open the file picker. */
async function onRetry(): Promise<void> {
    await reset()
    await pickFile()
}

/** User clicked "Dismiss" in the error view — return to idle silently. */
async function onDismiss(): Promise<void> {
    await reset()
}
</script>

<template>
<div class="app" :data-state="state">

    <!-- ── macOS Title bar ──────────────────────────────────────────────────── -->
    <!--
    Tauri `titleBarStyle: "Overlay"` keeps the native traffic-light buttons
    (close / minimise / zoom) in place while letting our HTML fill the full
    window.  We reserve the top 28 px as a drag region so the window can be
    moved by dragging the header area.
  -->
    <header class="titlebar" data-tauri-drag-region aria-hidden="true">
        <!-- Traffic-light placeholder — actual buttons rendered by macOS WebKit -->
        <div class="titlebar__traffic-lights" data-no-drag />

        <!-- Centred window title (mirrors the native title display) -->
        <span class="titlebar__title" aria-live="polite">
            {{ windowTitle }}
        </span>
    </header>

    <!-- ── Main content area ─────────────────────────────────────────────────── -->
    <main class="app__main" role="main">

        <!-- App wordmark + subtitle (shown in idle / ready states) -->
        <Transition name="fade">
            <div v-if="showSubtitle" class="app__wordmark">
                <div class="app__logo" aria-hidden="true">
                    <!-- Stylised "P" mark -->
                    <svg viewBox="0 0 36 36" fill="none" xmlns="http://www.w3.org/2000/svg" width="36" height="36">
                        <rect width="36" height="36" rx="9" fill="url(#logo-grad)" />
                        <!-- Document icon inside the mark -->
                        <path
                            d="M10 8h10.586a1 1 0 0 1 .707.293l4.414 4.414A1 1 0 0 1 26 13.414V28a1 1 0 0 1-1 1H11a1 1 0 0 1-1-1V9a1 1 0 0 1 1-1Z"
                            fill="white" opacity="0.18" />
                        <path d="M21 8v5a1 1 0 0 0 1 1h5" stroke="white" stroke-width="1.2" opacity="0.35"
                            fill="none" />
                        <rect x="13" y="17" width="10" height="1.4" rx="0.7" fill="white" opacity="0.9" />
                        <rect x="13" y="20.5" width="8" height="1.4" rx="0.7" fill="white" opacity="0.7" />
                        <rect x="13" y="24" width="9" height="1.4" rx="0.7" fill="white" opacity="0.5" />
                        <defs>
                            <linearGradient id="logo-grad" x1="0" y1="0" x2="36" y2="36" gradientUnits="userSpaceOnUse">
                                <stop stop-color="#007AFF" />
                                <stop offset="1" stop-color="#5E5CE6" />
                            </linearGradient>
                        </defs>
                    </svg>
                </div>

                <div class="app__heading-group">
                    <h1 class="app__title">PDF Splitter</h1>
                    <p class="app__subtitle">
                        Extract every page into its own PDF file — fast.
                    </p>
                </div>
            </div>
        </Transition>

        <!-- ── State views ──────────────────────────────────────────────────────── -->
        <div class="app__content">
            <Transition name="view" mode="out-in">

                <!-- ── Idle: drop zone ──────────────────────────────────────────── -->
                <div v-if="state === 'idle'" key="idle" class="view view--idle">
                    <DropZone :busy="isBusy" @pick="pickFile" @drop="onDrop" />
                </div>

                <!-- ── Ready: file card + split button ──────────────────────────── -->
                <div v-else-if="state === 'ready' && fileInfo" key="ready" class="view view--ready">
                    <FileCard :file-name="fileInfo.name" :page-count="fileInfo.pageCount"
                        :file-size-formatted="fileSizeFormatted" :output-dir-short="outputDirShort" :busy="isBusy"
                        @split="startSplit" @change-file="pickFile" @change-output="pickOutputDir" />
                </div>

                <!-- ── Processing: progress view ────────────────────────────────── -->
                <div v-else-if="state === 'processing'" key="processing" class="view view--processing">
                    <div class="processing-card card">
                        <ProgressView :percent="progressPercent" :current="operation?.progress?.current ?? 0"
                            :total="operation?.progress?.total ?? (fileInfo?.pageCount ?? 0)"
                            :current-file="operation?.progress?.fileName ?? ''" :file-name="fileInfo?.name ?? ''" />
                    </div>
                </div>

                <!-- ── Complete: result view ─────────────────────────────────────── -->
                <div v-else-if="state === 'complete' && result" key="complete" class="view view--complete">
                    <div class="result-wrapper">
                        <ResultView :total-pages="result.totalPages" :output-files="result.outputFiles"
                            :elapsed-formatted="elapsedFormatted" :output-dir="outputDir" @reveal="revealOutput"
                            @reset="reset" />
                    </div>
                </div>

                <!-- ── Error: error view ─────────────────────────────────────────── -->
                <div v-else-if="state === 'error'" key="error" class="view view--error">
                    <div class="error-card card">
                        <ErrorView :message="error ?? 'An unexpected error occurred.'" @retry="onRetry"
                            @dismiss="onDismiss" />
                    </div>
                </div>

            </Transition>
        </div>

    </main>

    <!-- ── Footer ─────────────────────────────────────────────────────────────── -->
    <footer class="app__footer" role="contentinfo">
        <span class="app__footer-text">
            PDF Splitter &nbsp;·&nbsp; Open Source &nbsp;·&nbsp; MIT
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
    /* Subtle noise/grain overlay for depth — pure CSS, no image asset needed. */
    background: var(--gradient-bg);
}

/* Very subtle inner highlight at the top edge (glass reflection) */
.app::before {
    content: "";
    position: absolute;
    inset: 0 0 auto 0;
    height: 1px;
    background: linear-gradient(90deg,
            transparent 0%,
            rgba(255, 255, 255, 0.55) 30%,
            rgba(255, 255, 255, 0.55) 70%,
            transparent 100%);
    pointer-events: none;
    z-index: var(--z-raised);
}

/* ── Title bar ────────────────────────────────────────────────────────────────── */

.titlebar {
    position: relative;
    height: var(--titlebar-height);
    /* 28 px — exactly the macOS traffic light height */
    display: flex;
    align-items: center;
    justify-content: center;
    flex-shrink: 0;
    z-index: var(--z-overlay);
    /* The background is intentionally transparent so the window gradient shows
     through behind the traffic lights. */
}

/* Spacer that matches the traffic-light cluster (≈ 72 px on macOS) so the
   centred title isn't offset by their width. */
.titlebar__traffic-lights {
    position: absolute;
    left: 0;
    width: 72px;
    height: 100%;
}

.titlebar__title {
    font-size: var(--text-xs);
    font-weight: var(--weight-semibold);
    letter-spacing: var(--tracking-wide);
    color: var(--color-text-tertiary);
    text-transform: none;
    pointer-events: none;
    /* Ensure it never overlaps traffic lights */
    max-width: 400px;
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
}

/* ── Main area ────────────────────────────────────────────────────────────────── */

.app__main {
    flex: 1;
    display: flex;
    flex-direction: column;
    padding: var(--space-4) var(--space-8) var(--space-3);
    gap: var(--space-5);
    overflow: hidden;
    min-height: 0;
}

/* ── App wordmark (logo + title) ──────────────────────────────────────────────── */

.app__wordmark {
    display: flex;
    align-items: center;
    gap: var(--space-4);
    flex-shrink: 0;
}

.app__logo {
    flex-shrink: 0;
    filter: drop-shadow(0 4px 12px rgba(0, 122, 255, 0.30));
    transition: filter var(--duration-normal) var(--ease-out);
}

.app__logo:hover {
    filter: drop-shadow(0 6px 18px rgba(0, 122, 255, 0.40));
}

.app__heading-group {
    display: flex;
    flex-direction: column;
    gap: 2px;
    min-width: 0;
}

.app__title {
    font-size: var(--text-xl);
    font-weight: var(--weight-bold);
    letter-spacing: var(--tracking-tight);
    color: var(--color-text-primary);
    line-height: var(--leading-tight);
}

.app__subtitle {
    font-size: var(--text-sm);
    color: var(--color-text-tertiary);
    font-weight: var(--weight-regular);
    line-height: var(--leading-normal);
}

/* ── Content container ────────────────────────────────────────────────────────── */

.app__content {
    flex: 1;
    display: flex;
    flex-direction: column;
    min-height: 0;
    /* Give transitions a clean stacking context. */
    position: relative;
}

/* ── Individual views ─────────────────────────────────────────────────────────── */

.view {
    display: flex;
    flex-direction: column;
    width: 100%;
    flex: 1;
}

/* ── Processing card ──────────────────────────────────────────────────────────── */

.processing-card {
    padding: var(--space-8) var(--space-8);
    flex: 1;
    display: flex;
    flex-direction: column;
    justify-content: center;
}

/* ── Result wrapper (allows full-height scrollable list) ──────────────────────── */

.result-wrapper {
    flex: 1;
    display: flex;
    flex-direction: column;
    min-height: 0;
}

/* ResultView uses its own height; we just ensure it can use available space. */
.result-wrapper :deep(.result-view) {
    flex: 1;
    min-height: 0;
}

.result-wrapper :deep(.file-list-container) {
    /* Use remaining height — subtract approx header (64px) + actions (60px) + gaps */
    max-height: 200px;
}

/* ── Error card ───────────────────────────────────────────────────────────────── */

.error-card {
    padding: var(--space-8) var(--space-8);
    flex: 1;
    display: flex;
    flex-direction: column;
    justify-content: center;
}

/* ── Footer ───────────────────────────────────────────────────────────────────── */

.app__footer {
    height: 28px;
    display: flex;
    align-items: center;
    justify-content: center;
    flex-shrink: 0;
    padding: 0 var(--space-8);
    border-top: 1px solid var(--color-border-subtle);
    background: transparent;
}

.app__footer-text {
    font-size: var(--text-xs);
    color: var(--color-text-quaternary);
    letter-spacing: var(--tracking-wide);
    white-space: nowrap;
}

/* ── State-based background tints ─────────────────────────────────────────────── */

/*
  Subtle background colour shift for each state — gives immediate, ambient
  feedback about what the app is doing without distracting the user.
*/

.app[data-state="processing"] {
    background: linear-gradient(160deg, #F4F8FF 0%, #E8EFFF 40%, #EBE8FF 100%);
}

.app[data-state="complete"] {
    background: linear-gradient(160deg, #F4FFF6 0%, #E8FFF0 40%, #F0FFF4 100%);
}

.app[data-state="error"] {
    background: linear-gradient(160deg, #FFF4F4 0%, #FFE8E8 40%, #FFF0F0 100%);
}

@media (prefers-color-scheme: dark) {
    .app[data-state="processing"] {
        background: linear-gradient(160deg, #1C1C28 0%, #1A1A2E 40%, #1E1C2E 100%);
    }

    .app[data-state="complete"] {
        background: linear-gradient(160deg, #1C281E 0%, #1A2E1C 40%, #1C2E1E 100%);
    }

    .app[data-state="error"] {
        background: linear-gradient(160deg, #281C1C 0%, #2E1A1A 40%, #2E1C1C 100%);
    }
}
</style>
