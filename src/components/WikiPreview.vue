<script setup lang="ts">
import { computed } from 'vue';
import { renderMarkdown } from '../utils/markdown';

const props = defineProps<{
  content: string;
}>();

const htmlContent = computed(() => renderMarkdown(props.content));
</script>

<template>
  <div class="wiki-preview">
    <div class="preview-content" v-html="htmlContent"></div>
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
  }
}
</style>
