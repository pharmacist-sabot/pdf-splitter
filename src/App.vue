<script setup lang="ts">
import { computed } from 'vue'
import { usePdfSplitter } from '@/composables/usePdfSplitter'
import DropZone from '@/components/DropZone.vue'
import FileCard from '@/components/FileCard.vue'
import ProgressView from '@/components/ProgressView.vue'
import ResultView from '@/components/ResultView.vue'
import ErrorView from '@/components/ErrorView.vue'

const {
    state,
    fileInfo,
    outputDir,
    operation,
    result,
    error,
    isBusy,
    fileSizeFormatted,
    progressPercent,
    elapsedFormatted,
    outputDirShort,
    pickFile,
    pickOutputDir,
    startSplit,
    revealOutput,
    reset,
} = usePdfSplitter()


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

const showSubtitle = computed<boolean>(
    () => state.value === 'idle' || state.value === 'ready',
)


async function onDrop(path: string): Promise<void> {
    void path
    await pickFile()
}

async function onRetry(): Promise<void> {
    await reset()
    await pickFile()
}

async function onDismiss(): Promise<void> {
    await reset()
}
</script>

<template>
<div class="app" :data-state="state">

    <!-- Atmospheric radial glow at top -->
    <div class="app__atmosphere" aria-hidden="true" />

    <!-- macOS-style title bar -->
    <header class="titlebar" data-tauri-drag-region aria-hidden="true">
        <div class="titlebar__traffic-lights" data-no-drag />
        <span class="titlebar__title" aria-live="polite">{{ windowTitle }}</span>
    </header>

    <main class="app__main" role="main">

        <!-- Wordmark/header that shows on idle + ready states -->
        <div class="app__wordmark-wrapper" :class="{ 'app__wordmark-wrapper--hidden': !showSubtitle }">
            <div class="app__wordmark">

                <div class="app__heading-group">
                    <h1 class="app__title">PDF Splitter</h1>
                    <p class="app__subtitle">Extract every page into its own file</p>
                </div>

            </div>
        </div>

        <div class="app__content">
            <Transition name="view" mode="out-in">

                <!-- idle -->
                <div v-if="state === 'idle'" key="idle" class="view view--idle">
                    <DropZone :busy="isBusy" @pick="pickFile" @drop="onDrop" />
                </div>

                <!-- ready -->
                <div v-else-if="state === 'ready' && fileInfo" key="ready" class="view view--ready">
                    <FileCard
                        :file-name="fileInfo.name"
                        :page-count="fileInfo.pageCount"
                        :file-size-formatted="fileSizeFormatted"
                        :output-dir-short="outputDirShort"
                        :busy="isBusy"
                        @split="startSplit"
                        @change-file="pickFile"
                        @change-output="pickOutputDir"
                    />
                </div>

                <!-- processing -->
                <div v-else-if="state === 'processing'" key="processing" class="view view--processing">
                    <div class="processing-card card">
                        <ProgressView
                            :percent="progressPercent"
                            :current="operation?.progress?.current ?? 0"
                            :total="operation?.progress?.total ?? (fileInfo?.pageCount ?? 0)"
                            :current-file="operation?.progress?.fileName ?? ''"
                            :file-name="fileInfo?.name ?? ''"
                        />
                    </div>
                </div>

                <!-- complete -->
                <div v-else-if="state === 'complete' && result" key="complete" class="view view--complete">
                    <div class="result-wrapper">
                        <ResultView
                            :total-pages="result.totalPages"
                            :output-files="result.outputFiles"
                            :elapsed-formatted="elapsedFormatted"
                            :output-dir="outputDir"
                            @reveal="revealOutput"
                            @reset="reset"
                        />
                    </div>
                </div>

                <!-- error -->
                <div v-else-if="state === 'error'" key="error" class="view view--error">
                    <div class="error-card card">
                        <ErrorView
                            :message="error ?? 'An unexpected error occurred.'"
                            @retry="onRetry"
                            @dismiss="onDismiss"
                        />
                    </div>
                </div>

            </Transition>
        </div>

    </main>

    <footer class="app__footer" role="contentinfo">
        <span class="app__footer-text">pdf-splitter · open source · MIT</span>
    </footer>

</div>
</template>

<style scoped>
/* =============================================================================
   App shell
   ============================================================================= */

.app {
    width: 720px;
    height: 560px;
    display: flex;
    flex-direction: column;
    overflow: hidden;
    position: relative;
    background: var(--color-bg);
}

/* Atmospheric radial glow — subtle forest-green halo at the top of the window */

.app__atmosphere {
    position: absolute;
    inset: 0;
    pointer-events: none;
    z-index: 0;
    background: radial-gradient(
        ellipse 80% 40% at 50% 0%,
        rgba(16, 38, 32, 0.8) 0%,
        transparent 70%
    );
}

/* =============================================================================
   macOS-style title bar
   ============================================================================= */

.titlebar {
    position: relative;
    height: var(--titlebar-height);
    display: flex;
    align-items: center;
    justify-content: center;
    flex-shrink: 0;
    z-index: var(--z-overlay);
    border-bottom: 1px solid var(--color-border-subtle);
    background: rgba(0, 0, 0, 0.4);
}

/* Spacer that matches the macOS traffic-light cluster width (~72 px) */
.titlebar__traffic-lights {
    position: absolute;
    left: 0;
    width: 72px;
    height: 100%;
}

.titlebar__title {
    font-size: 11px;
    font-weight: var(--weight-medium);
    color: var(--color-text-quaternary);
    font-family: var(--font-mono);
    letter-spacing: 0.02em;
    pointer-events: none;
    max-width: 400px;
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
    transition: color 200ms var(--ease-out);
}

/* =============================================================================
   Main content area
   ============================================================================= */

.app__main {
    flex: 1;
    display: flex;
    flex-direction: column;
    padding: 20px 28px 16px;
    overflow: hidden;
    min-height: 0;
    position: relative;
    z-index: 1;
}

/* =============================================================================
   Wordmark — smooth show/hide without layout jump
   ============================================================================= */

.app__wordmark-wrapper {
    flex-shrink: 0;
    overflow: hidden;
    /* Generous upper bound; actual content is ~52 px */
    max-height: 90px;
    /* padding-bottom acts as the gap between wordmark and content area,
       and collapses to 0 together with the wordmark height */
    padding-bottom: 20px;
    opacity: 1;
    transition:
        max-height 280ms var(--ease-out),
        padding-bottom 280ms var(--ease-out),
        opacity 180ms var(--ease-out);
}

.app__wordmark-wrapper--hidden {
    max-height: 0;
    padding-bottom: 0;
    opacity: 0;
    pointer-events: none;
}

.app__wordmark {
    display: flex;
    align-items: center;
    gap: 14px;
    padding-bottom: 18px;
    border-bottom: 1px solid var(--color-border-subtle);
}

.app__heading-group {
    display: flex;
    flex-direction: column;
    gap: 4px;
    min-width: 0;
}

.app__title {
    font-family: var(--font-display);
    font-size: var(--text-3xl);
    font-weight: var(--weight-light);
    color: var(--color-text-primary);
    line-height: 1.0;
    letter-spacing: -0.02em;
}

.app__subtitle {
    font-family: var(--font-sans);
    font-size: var(--text-sm);
    font-weight: var(--weight-regular);
    color: var(--color-text-tertiary);
    line-height: 1.5;
}

/* =============================================================================
   Content container
   ============================================================================= */

.app__content {
    flex: 1;
    display: flex;
    flex-direction: column;
    min-height: 0;
    position: relative;
    /* Clips absolutely-positioned views during transitions so they
       never bleed outside the content area */
    overflow: hidden;
}

/* Individual state views */
.view {
    display: flex;
    flex-direction: column;
    width: 100%;
    flex: 1;
}

/* Processing card */
.processing-card {
    padding: var(--space-7);
    flex: 1;
    display: flex;
    flex-direction: column;
    justify-content: center;
}

/* Result wrapper */
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

/* Error card */
.error-card {
    padding: var(--space-7);
    flex: 1;
    display: flex;
    flex-direction: column;
    justify-content: center;
}

/* =============================================================================
   Footer
   ============================================================================= */

.app__footer {
    height: 28px;
    display: flex;
    align-items: center;
    justify-content: center;
    flex-shrink: 0;
    border-top: 1px solid var(--color-border-subtle);
    background: rgba(0, 0, 0, 0.3);
}

.app__footer-text {
    font-size: 10px;
    color: var(--color-text-quaternary);
    letter-spacing: 0.06em;
    white-space: nowrap;
    font-family: var(--font-sans);
}

/* =============================================================================
   State-based background tints
   Very subtle radial glows from the top, colour-coded by app state.
   ============================================================================= */

.app[data-state="processing"] {
    background-image: radial-gradient(
        ellipse 60% 30% at 50% 0%,
        rgba(54, 244, 164, 0.05) 0%,
        transparent 100%
    );
}

.app[data-state="complete"] {
    background-image: radial-gradient(
        ellipse 60% 30% at 50% 0%,
        rgba(54, 244, 164, 0.07) 0%,
        transparent 100%
    );
}

.app[data-state="error"] {
    background-image: radial-gradient(
        ellipse 60% 30% at 50% 0%,
        rgba(248, 81, 73, 0.05) 0%,
        transparent 100%
    );
}

/* Title text picks up a colour hint during active operations */

.app[data-state="processing"] .titlebar__title,
.app[data-state="complete"] .titlebar__title {
    color: rgba(54, 244, 164, 0.6);
}

.app[data-state="error"] .titlebar__title {
    color: rgba(248, 81, 73, 0.6);
}
</style>