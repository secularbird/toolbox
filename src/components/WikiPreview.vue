<script setup lang="ts">
import { watch, nextTick, onMounted, ref } from 'vue';
import { renderMarkdown } from '../utils/markdown';
import mermaid from 'mermaid';

const props = defineProps<{
  content: string;
}>();

const renderedContent = ref('');
let debounceTimer: number | null = null;

function scheduleRender(markdown: string) {
  if (debounceTimer) {
    clearTimeout(debounceTimer);
  }
  debounceTimer = window.setTimeout(() => {
    renderedContent.value = renderMarkdown(markdown);
    debounceTimer = null;
  }, 80);
}

watch(
  () => props.content,
  (val) => scheduleRender(val),
  { immediate: true }
);

const previewContentRef = ref<HTMLElement | null>(null);
let renderingInProgress = false;

// Initialize Mermaid with configuration
onMounted(() => {
  mermaid.initialize({
    startOnLoad: false,
    theme: 'default',
    securityLevel: 'strict',
    fontFamily: 'system-ui, -apple-system, sans-serif',
  });
});

// Re-render Mermaid diagrams when content changes
watch(renderedContent, async () => {
  // Prevent race conditions by checking if rendering is already in progress
  if (renderingInProgress) {
    return;
  }
  
  renderingInProgress = true;
  await nextTick();
  
  if (!previewContentRef.value) {
    renderingInProgress = false;
    return;
  }
  
  try {
    // Find all mermaid elements within the preview content that haven't been processed
    const mermaidElements = previewContentRef.value.querySelectorAll('.mermaid:not([data-processed])');
    
    if (mermaidElements.length > 0) {
      // Mark elements as being processed
      mermaidElements.forEach((element) => {
        element.setAttribute('data-processed', 'processing');
      });
      
      // Run Mermaid rendering on unprocessed elements
      await mermaid.run({
        querySelector: '.mermaid[data-processed="processing"]',
      });
      
      // Mark as fully rendered
      mermaidElements.forEach((element) => {
        element.setAttribute('data-processed', 'rendered');
      });
    }
  } catch (error) {
    console.error('Mermaid rendering error:', error);
  } finally {
    renderingInProgress = false;
  }
});
</script>

<template>
  <div class="wiki-preview">
    <div class="preview-content" ref="previewContentRef" v-html="renderedContent"></div>
  </div>
</template>

<style scoped>
.wiki-preview {
  height: 100%;
  overflow-y: auto;
  background: var(--preview-bg);
  padding: 16px;
}

.preview-content {
  max-width: 800px;
  margin: 0 auto;
  color: var(--text-primary);
  line-height: 1.6;
}

/* Markdown styles */
.preview-content :deep(h1),
.preview-content :deep(h2),
.preview-content :deep(h3),
.preview-content :deep(h4),
.preview-content :deep(h5),
.preview-content :deep(h6) {
  margin-top: 24px;
  margin-bottom: 16px;
  font-weight: 600;
  line-height: 1.25;
  color: var(--text-primary);
}

.preview-content :deep(h1) {
  font-size: 2em;
  border-bottom: 1px solid var(--border-color);
  padding-bottom: 0.3em;
}

.preview-content :deep(h2) {
  font-size: 1.5em;
  border-bottom: 1px solid var(--border-color);
  padding-bottom: 0.3em;
}

.preview-content :deep(h3) {
  font-size: 1.25em;
}

.preview-content :deep(p) {
  margin-bottom: 16px;
}

.preview-content :deep(a) {
  color: var(--link-color);
  text-decoration: none;
}

.preview-content :deep(a:hover) {
  text-decoration: underline;
}

.preview-content :deep(code) {
  padding: 0.2em 0.4em;
  margin: 0;
  font-size: 85%;
  background: var(--code-bg);
  border-radius: 6px;
  font-family: 'Monaco', 'Menlo', 'Consolas', monospace;
}

.preview-content :deep(pre) {
  padding: 16px;
  overflow: auto;
  font-size: 85%;
  line-height: 1.45;
  background: var(--code-block-bg);
  border-radius: 6px;
  margin-bottom: 16px;
}

.preview-content :deep(pre code) {
  padding: 0;
  background: transparent;
  border-radius: 0;
}

.preview-content :deep(blockquote) {
  padding: 0 1em;
  color: var(--text-secondary);
  border-left: 0.25em solid var(--border-color);
  margin-bottom: 16px;
}

.preview-content :deep(ul),
.preview-content :deep(ol) {
  padding-left: 2em;
  margin-bottom: 16px;
}

.preview-content :deep(li) {
  margin-bottom: 4px;
}

.preview-content :deep(table) {
  border-collapse: collapse;
  width: 100%;
  margin-bottom: 16px;
}

.preview-content :deep(table th),
.preview-content :deep(table td) {
  padding: 6px 13px;
  border: 1px solid var(--border-color);
}

.preview-content :deep(table th) {
  font-weight: 600;
  background: var(--table-header-bg);
}

.preview-content :deep(img) {
  max-width: 100%;
  height: auto;
  border-radius: 8px;
}

.preview-content :deep(hr) {
  height: 0.25em;
  padding: 0;
  margin: 24px 0;
  background-color: var(--border-color);
  border: 0;
}

/* Diagram styles */
.preview-content :deep(.diagram-container) {
  margin: 16px 0;
  padding: 16px;
  background: var(--diagram-bg);
  border-radius: 8px;
  border: 1px solid var(--border-color);
  overflow-x: auto;
}

.preview-content :deep(.diagram-image) {
  display: block;
  max-width: 100%;
  height: auto;
  margin: 0 auto;
}

.preview-content :deep(.mermaid) {
  display: flex;
  justify-content: center;
  align-items: center;
  background: transparent;
  margin: 0;
  padding: 0;
}

.preview-content :deep(.diagram-error) {
  color: var(--error-color);
  padding: 12px;
  background: var(--error-bg);
  border-radius: 6px;
  font-family: monospace;
}

/* Dark mode */
@media (prefers-color-scheme: dark) {
  .wiki-preview {
    --preview-bg: #1c1c1e;
    --text-primary: #f5f5f7;
    --text-secondary: #98989d;
    --border-color: #38383a;
    --link-color: #0a84ff;
    --code-bg: #2c2c2e;
    --code-block-bg: #2c2c2e;
    --table-header-bg: #2c2c2e;
    --diagram-bg: #2c2c2e;
    --error-color: #ff453a;
    --error-bg: rgba(255, 69, 58, 0.1);
  }
}

/* Light mode */
@media (prefers-color-scheme: light) {
  .wiki-preview {
    --preview-bg: #ffffff;
    --text-primary: #1d1d1f;
    --text-secondary: #86868b;
    --border-color: #e5e5ea;
    --link-color: #007aff;
    --code-bg: #f5f5f7;
    --code-block-bg: #f5f5f7;
    --table-header-bg: #f5f5f7;
    --diagram-bg: #fafafa;
    --error-color: #ff3b30;
    --error-bg: rgba(255, 59, 48, 0.1);
  }
}
</style>
