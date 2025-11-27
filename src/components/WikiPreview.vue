<script setup lang="ts">
import { watch, onBeforeUnmount, ref } from 'vue';
import { renderMarkdown } from '../utils/markdown';

const props = defineProps<{
  content: string;
}>();

const emit = defineEmits<{
  'editTable': [tableIndex: number];
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

// Handle clicks on tables for editing
function handlePreviewClick(event: MouseEvent) {
  const target = event.target as HTMLElement;
  const table = target.closest('table.editable-table');
  if (table) {
    const tableIndex = parseInt(table.getAttribute('data-table-index') || '0', 10);
    emit('editTable', tableIndex);
  }
}

onBeforeUnmount(() => {
  if (debounceTimer) {
    clearTimeout(debounceTimer);
  }
});
</script>

<template>
  <div class="wiki-preview">
    <div class="preview-content" ref="previewContentRef" v-html="renderedContent" @click="handlePreviewClick"></div>
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

.preview-content :deep(table.editable-table) {
  cursor: pointer;
  transition: box-shadow 0.2s, transform 0.1s;
  position: relative;
}

.preview-content :deep(table.editable-table:hover) {
  box-shadow: 0 0 0 2px var(--primary-color, #007aff);
  transform: translateY(-1px);
}

.preview-content :deep(table.editable-table::after) {
  content: '✏️ Click to edit';
  position: absolute;
  top: -24px;
  right: 0;
  font-size: 11px;
  color: var(--primary-color, #007aff);
  background: var(--table-header-bg, #f5f5f7);
  padding: 2px 8px;
  border-radius: 4px;
  opacity: 0;
  transition: opacity 0.2s;
  pointer-events: none;
}

.preview-content :deep(table.editable-table:hover::after) {
  opacity: 1;
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
  display: flex;
  justify-content: center;
  align-items: center;
  min-height: 100px;
}

.preview-content :deep(.diagram-image) {
  display: block;
  max-width: 100%;
  height: auto;
  margin: 0 auto;
}

.preview-content :deep(.diagram-error) {
  color: var(--error-color);
  padding: 12px;
  background: var(--error-bg);
  border-radius: 6px;
  font-family: monospace;
}

/* Reminder Card styles */
.preview-content :deep(.reminder-card) {
  margin: 16px 0;
  padding: 0;
  background: var(--reminder-bg);
  border-radius: 12px;
  border: 1px solid var(--reminder-border);
  overflow: hidden;
  box-shadow: 0 2px 8px rgba(0, 0, 0, 0.08);
  transition: transform 0.2s, box-shadow 0.2s;
}

.preview-content :deep(.reminder-card:hover) {
  transform: translateY(-2px);
  box-shadow: 0 4px 16px rgba(0, 0, 0, 0.12);
}

.preview-content :deep(.reminder-card-header) {
  display: flex;
  align-items: center;
  gap: 8px;
  padding: 12px 16px;
  background: var(--reminder-header-bg);
  border-bottom: 1px solid var(--reminder-border);
}

.preview-content :deep(.reminder-icon) {
  font-size: 1.25rem;
}

.preview-content :deep(.reminder-title) {
  flex: 1;
  font-weight: 600;
  font-size: 1rem;
  color: var(--text-primary);
}

.preview-content :deep(.reminder-badge) {
  padding: 4px 10px;
  border-radius: 12px;
  font-size: 0.75rem;
  font-weight: 500;
  text-transform: uppercase;
  letter-spacing: 0.5px;
}

.preview-content :deep(.reminder-badge.upcoming) {
  background: var(--badge-upcoming-bg);
  color: var(--badge-upcoming-color);
}

.preview-content :deep(.reminder-badge.past) {
  background: var(--badge-past-bg);
  color: var(--badge-past-color);
}

.preview-content :deep(.reminder-card-body) {
  padding: 12px 16px;
  display: flex;
  flex-direction: column;
  gap: 8px;
}

.preview-content :deep(.reminder-meta-item) {
  display: flex;
  align-items: center;
  gap: 8px;
  font-size: 0.9rem;
}

.preview-content :deep(.reminder-meta-icon) {
  width: 20px;
  text-align: center;
}

.preview-content :deep(.reminder-meta-label) {
  color: var(--text-secondary);
  min-width: 80px;
}

.preview-content :deep(.reminder-meta-value) {
  font-weight: 500;
  color: var(--text-primary);
}

.preview-content :deep(.reminder-meta-value.past) {
  color: var(--text-secondary);
  text-decoration: line-through;
}

.preview-content :deep(.reminder-description) {
  display: flex;
  align-items: flex-start;
  gap: 8px;
  padding-top: 8px;
  border-top: 1px dashed var(--reminder-border);
  font-size: 0.875rem;
  color: var(--text-secondary);
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
    /* Reminder card variables */
    --reminder-bg: #2c2c2e;
    --reminder-border: #48484a;
    --reminder-header-bg: #38383a;
    --badge-upcoming-bg: rgba(10, 132, 255, 0.2);
    --badge-upcoming-color: #0a84ff;
    --badge-past-bg: rgba(152, 152, 157, 0.2);
    --badge-past-color: #98989d;
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
    /* Reminder card variables */
    --reminder-bg: #ffffff;
    --reminder-border: #e5e5ea;
    --reminder-header-bg: #f5f5f7;
    --badge-upcoming-bg: rgba(0, 122, 255, 0.1);
    --badge-upcoming-color: #007aff;
    --badge-past-bg: rgba(134, 134, 139, 0.1);
    --badge-past-color: #86868b;
  }
}
</style>
