<script setup lang="ts">
import { ref, watch, onMounted, onBeforeUnmount, nextTick, shallowRef, computed } from 'vue';
import { wrapSelection, insertAtCursor, markdownFormats, renderMarkdown, generateDiagramId } from '../utils/markdown';
import { EditorHistory } from '../utils/editorHistory';
import mermaid from 'mermaid';
// Milkdown imports
import { Editor, rootCtx, defaultValueCtx, editorViewCtx } from '@milkdown/core';
import { commonmark } from '@milkdown/preset-commonmark';
import { gfm } from '@milkdown/preset-gfm';
import { history as milkdownHistory } from '@milkdown/plugin-history';
import { listener, listenerCtx } from '@milkdown/plugin-listener';
import type { Ctx } from '@milkdown/ctx';

const props = defineProps<{
  modelValue: string;
  placeholder?: string;
}>();

const emit = defineEmits<{
  'update:modelValue': [value: string];
  'insertTable': [];
  'insertReminder': [];
}>();

// Editor mode: 'markdown' or 'wysiwyg'
const editorMode = ref<'markdown' | 'wysiwyg'>('markdown');
const textareaRef = ref<HTMLTextAreaElement | null>(null);
const milkdownContainerRef = ref<HTMLDivElement | null>(null);
const previewRef = ref<HTMLDivElement | null>(null);
const localValue = ref(props.modelValue);
const fileInputRef = ref<HTMLInputElement | null>(null);
const history = new EditorHistory(props.modelValue);
const isUndoRedoing = ref(false);

// Milkdown editor instance
const milkdownEditor = shallowRef<Editor | null>(null);
const isMilkdownReady = ref(false);
// Track if we're updating content to prevent loops
let isUpdatingMilkdown = false;

// Track mermaid rendering state
let mermaidRenderingInProgress = false;

// Detect if dark mode is enabled
const isDarkMode = () => {
  return window.matchMedia && window.matchMedia('(prefers-color-scheme: dark)').matches;
};

// Initialize Mermaid with configuration
const initMermaid = () => {
  const darkMode = isDarkMode();
  mermaid.initialize({
    startOnLoad: false,
    theme: darkMode ? 'dark' : 'default',
    securityLevel: 'loose',
    fontFamily: 'system-ui, -apple-system, sans-serif',
    flowchart: {
      htmlLabels: true,
      curve: 'basis',
    },
    sequence: {
      diagramMarginX: 50,
      diagramMarginY: 10,
      actorMargin: 50,
      width: 150,
      height: 65,
      boxMargin: 10,
      boxTextMargin: 5,
      noteMargin: 10,
      messageMargin: 35,
    },
  });
};

// Render Mermaid diagrams in preview
async function renderMermaidInPreview() {
  if (mermaidRenderingInProgress || !previewRef.value) {
    return;
  }

  mermaidRenderingInProgress = true;
  await nextTick();

  try {
    const mermaidElements = previewRef.value.querySelectorAll('.mermaid:not([data-processed="rendered"])');

    if (mermaidElements.length > 0) {
      for (const element of Array.from(mermaidElements)) {
        try {
          element.setAttribute('data-processed', 'processing');
          const id = generateDiagramId('mermaid');
          element.id = id;

          await mermaid.run({
            nodes: [element as HTMLElement],
          });

          element.setAttribute('data-processed', 'rendered');
        } catch (err) {
          console.error('Mermaid rendering error for element:', err);
          element.setAttribute('data-processed', 'error');
          element.innerHTML = `<div class="diagram-error">Mermaid rendering error: ${err instanceof Error ? err.message : String(err)}</div>`;
        }
      }
    }
  } catch (error) {
    console.error('Mermaid batch rendering error:', error);
  } finally {
    mermaidRenderingInProgress = false;
  }
}

// Initialize Milkdown editor
async function initMilkdown(container: HTMLDivElement, content: string) {
  try {
    const editor = await Editor.make()
      .config((ctx: Ctx) => {
        ctx.set(rootCtx, container);
        ctx.set(defaultValueCtx, content);
        // Setup listener for content changes
        ctx.get(listenerCtx).markdownUpdated((_ctx: Ctx, markdown: string, _prevMarkdown: string) => {
          if (!isUpdatingMilkdown) {
            localValue.value = markdown;
          }
        });
      })
      .use(commonmark)
      .use(gfm)
      .use(milkdownHistory)
      .use(listener)
      .create();
    
    milkdownEditor.value = editor;
    isMilkdownReady.value = true;
    return editor;
  } catch (error) {
    console.error('Failed to initialize Milkdown:', error);
    return null;
  }
}

// Destroy Milkdown editor
async function destroyMilkdown() {
  if (milkdownEditor.value) {
    try {
      await milkdownEditor.value.destroy();
    } catch (error) {
      console.error('Failed to destroy Milkdown:', error);
    }
    milkdownEditor.value = null;
    isMilkdownReady.value = false;
  }
}

// Computed preview content (for wysiwyg preview mode)
const previewContent = computed(() => {
  return renderMarkdown(localValue.value);
});

watch(() => props.modelValue, async (newVal) => {
  if (localValue.value !== newVal) {
    localValue.value = newVal;
    history.reset(newVal);
    
    // Re-initialize Milkdown with new content when in WYSIWYG mode
    if (editorMode.value === 'wysiwyg' && milkdownContainerRef.value) {
      await destroyMilkdown();
      await initMilkdown(milkdownContainerRef.value, newVal);
    }
  }
});

watch(localValue, (newVal) => {
  if (!isUndoRedoing.value) {
    history.save(newVal);
  }
  emit('update:modelValue', newVal);
  
  // Update preview with mermaid diagrams
  if (editorMode.value === 'wysiwyg') {
    nextTick(() => {
      renderMermaidInPreview();
    });
  }
});

// Watch for mode changes - this is the key feature!
// Content is preserved because localValue always contains the markdown
watch(editorMode, async (newMode, oldMode) => {
  if (newMode === 'wysiwyg') {
    // Switch to WYSIWYG mode - initialize Milkdown with current markdown content
    await nextTick();
    if (milkdownContainerRef.value) {
      await initMilkdown(milkdownContainerRef.value, localValue.value);
    }
  } else if (newMode === 'markdown' && oldMode === 'wysiwyg') {
    // Switch back to markdown mode - content is already in localValue
    // Destroy Milkdown instance
    await destroyMilkdown();
    // Focus the textarea
    await nextTick();
    textareaRef.value?.focus();
  }
});

function applyFormat(formatKey: keyof typeof markdownFormats) {
  if (editorMode.value === 'markdown') {
    if (!textareaRef.value) return;
    const format = markdownFormats[formatKey];
    localValue.value = wrapSelection(textareaRef.value, format.before, format.after);
  } else {
    // WYSIWYG mode - apply format through Milkdown
    applyMilkdownFormat(formatKey);
  }
}

function applyMilkdownFormat(formatKey: keyof typeof markdownFormats) {
  if (!milkdownEditor.value || !isMilkdownReady.value) return;
  
  try {
    milkdownEditor.value.action((ctx: Ctx) => {
      const view = ctx.get(editorViewCtx);
      const { state, dispatch } = view;
      const { selection, schema, tr } = state;
      const { from, to } = selection;
      
      switch (formatKey) {
        case 'bold': {
          const markType = schema.marks['strong'];
          if (markType) {
            const hasMark = state.doc.rangeHasMark(from, to, markType);
            if (hasMark) {
              dispatch(tr.removeMark(from, to, markType));
            } else {
              dispatch(tr.addMark(from, to, markType.create()));
            }
          }
          break;
        }
        case 'italic': {
          const markType = schema.marks['emphasis'];
          if (markType) {
            const hasMark = state.doc.rangeHasMark(from, to, markType);
            if (hasMark) {
              dispatch(tr.removeMark(from, to, markType));
            } else {
              dispatch(tr.addMark(from, to, markType.create()));
            }
          }
          break;
        }
        case 'code': {
          const markType = schema.marks['inlineCode'];
          if (markType) {
            const hasMark = state.doc.rangeHasMark(from, to, markType);
            if (hasMark) {
              dispatch(tr.removeMark(from, to, markType));
            } else {
              dispatch(tr.addMark(from, to, markType.create()));
            }
          }
          break;
        }
        case 'link': {
          const url = prompt('Enter URL:', 'https://');
          if (url) {
            try {
              const urlObj = new URL(url);
              if (urlObj.protocol === 'http:' || urlObj.protocol === 'https:') {
                const markType = schema.marks['link'];
                if (markType) {
                  dispatch(tr.addMark(from, to, markType.create({ href: url })));
                }
              } else {
                alert('Only http:// and https:// URLs are allowed');
              }
            } catch (e) {
              alert('Invalid URL');
            }
          }
          break;
        }
        default:
          // For heading and other block formats, insert markdown directly
          break;
      }
    });
  } catch (error) {
    console.error('Error applying Milkdown format:', error);
  }
}

function insertText(text: string) {
  if (editorMode.value === 'markdown') {
    if (!textareaRef.value) return;
    localValue.value = insertAtCursor(textareaRef.value, text);
  } else {
    // WYSIWYG mode - insert through Milkdown
    if (!milkdownEditor.value || !isMilkdownReady.value) return;
    
    try {
      milkdownEditor.value.action((ctx: Ctx) => {
        const view = ctx.get(editorViewCtx);
        const { state, dispatch } = view;
        const { selection } = state;
        const tr = state.tr.insertText(text, selection.from, selection.to);
        dispatch(tr);
      });
    } catch (error) {
      console.error('Error inserting text:', error);
    }
  }
}

function insertContentBlock(text: string) {
  if (editorMode.value === 'markdown') {
    if (!textareaRef.value) return;
    const content = localValue.value ? `${text}` : text;
    localValue.value = insertAtCursor(textareaRef.value, content);
  } else {
    // WYSIWYG mode - insert content
    insertText(text);
  }
}

async function handleFileSelect(e: Event) {
  const input = e.target as HTMLInputElement;
  if (!input.files || !input.files[0]) return;
  const file = input.files[0];

  const MAX_BYTES = 5 * 1024 * 1024; // 5MB inline limit
  if (file.size > MAX_BYTES) {
    alert('File too large to embed inline (max 5MB).');
    input.value = '';
    return;
  }

  const reader = new FileReader();
  reader.onload = () => {
    if (typeof reader.result !== 'string') return;
    const isImage = file.type.startsWith('image/');
    const markdown = isImage
      ? `![${file.name}](${reader.result})`
      : `[${file.name}](${reader.result})`;
    insertContentBlock(markdown);
  };
  reader.readAsDataURL(file);
  input.value = '';
}

function triggerFilePicker() {
  fileInputRef.value?.click();
}

function handleTab(e: KeyboardEvent) {
  e.preventDefault();
  insertText('  ');
}

function handleUndo() {
  const prevContent = history.undo();
  if (prevContent !== null) {
    isUndoRedoing.value = true;
    localValue.value = prevContent;
    setTimeout(() => {
      isUndoRedoing.value = false;
    }, 0);
  }
}

function handleRedo() {
  const nextContent = history.redo();
  if (nextContent !== null) {
    isUndoRedoing.value = true;
    localValue.value = nextContent;
    setTimeout(() => {
      isUndoRedoing.value = false;
    }, 0);
  }
}

function handleKeydown(e: KeyboardEvent) {
  if (editorMode.value === 'markdown') {
    handleMarkdownKeydown(e);
  }
  // Milkdown handles its own keyboard shortcuts
}

function handleMarkdownKeydown(e: KeyboardEvent) {
  if (e.key === 'Tab') {
    handleTab(e);
  } else if (e.ctrlKey || e.metaKey) {
    switch (e.key) {
      case 'b':
        e.preventDefault();
        applyFormat('bold');
        break;
      case 'i':
        e.preventDefault();
        applyFormat('italic');
        break;
      case 'k':
        e.preventDefault();
        applyFormat('link');
        break;
      case 't':
        if (e.shiftKey) {
          e.preventDefault();
          emit('insertTable');
        }
        break;
      case 'r':
        if (e.shiftKey) {
          e.preventDefault();
          emit('insertReminder');
        }
        break;
      case 'z':
        e.preventDefault();
        if (e.shiftKey) {
          handleRedo();
        } else {
          handleUndo();
        }
        break;
      case 'y':
        e.preventDefault();
        handleRedo();
        break;
    }
  }
}

function handlePaste(e: ClipboardEvent) {
  if (editorMode.value !== 'markdown') return; // Milkdown handles paste in WYSIWYG mode
  
  const clipboard = e.clipboardData;
  if (!clipboard) return;

  if (!textareaRef.value) return;

  // Paste image blobs as data URLs to preserve inline content
  const imageFile = Array.from(clipboard.files || []).find((file) =>
    file.type.startsWith('image/')
  );
  if (imageFile) {
    e.preventDefault();
    const reader = new FileReader();
    reader.onload = () => {
      if (typeof reader.result === 'string') {
        insertContentBlock(`![pasted-image](${reader.result})`);
      }
    };
    reader.readAsDataURL(imageFile);
    return;
  }

  // For HTML content, convert to plain text to avoid XSS - Milkdown handles
  // proper markdown conversion in WYSIWYG mode
  const htmlContent = clipboard.getData('text/html');
  if (htmlContent) {
    e.preventDefault();
    // Insert as plain text for safety in markdown mode
    const plainText = clipboard.getData('text/plain');
    if (plainText) {
      insertContentBlock(plainText.trim());
    }
    return;
  }
}

onMounted(() => {
  // Initialize Mermaid
  initMermaid();
  
  // Re-initialize Mermaid when color scheme changes
  const mediaQuery = window.matchMedia('(prefers-color-scheme: dark)');
  mediaQuery.addEventListener('change', async () => {
    initMermaid();
    // Re-render all diagrams in preview
    if (previewRef.value) {
      const elements = previewRef.value.querySelectorAll('.mermaid[data-processed="rendered"]');
      elements.forEach(el => {
        el.removeAttribute('data-processed');
        el.removeAttribute('id');
      });
      mermaidRenderingInProgress = false;
      await renderMermaidInPreview();
    }
  });
  
  if (editorMode.value === 'markdown') {
    textareaRef.value?.focus();
  }
});

onBeforeUnmount(async () => {
  await destroyMilkdown();
});

defineExpose({ applyFormat, insertText, editorMode });
</script>

<template>
  <div class="wiki-editor">
    <div class="editor-toolbar">
      <div class="toolbar-group">
        <button 
          @click="editorMode = 'markdown'" 
          :class="['mode-btn', { active: editorMode === 'markdown' }]"
          title="Markdown Mode"
        >
          📝 Markdown
        </button>
        <button 
          @click="editorMode = 'wysiwyg'" 
          :class="['mode-btn', { active: editorMode === 'wysiwyg' }]"
          title="WYSIWYG Mode"
        >
          👁️ WYSIWYG
        </button>
      </div>

      <div class="toolbar-divider"></div>

      <div class="toolbar-group">
        <button @click="applyFormat('bold')" title="Bold (Ctrl+B)" class="toolbar-btn">
          <strong>B</strong>
        </button>
        <button @click="applyFormat('italic')" title="Italic (Ctrl+I)" class="toolbar-btn">
          <em>I</em>
        </button>
        <button @click="applyFormat('code')" title="Inline Code" class="toolbar-btn">
          &lt;/&gt;
        </button>
      </div>

      <div class="toolbar-divider"></div>

      <div class="toolbar-group">
        <button @click="applyFormat('h1')" title="Heading 1" class="toolbar-btn">H1</button>
        <button @click="applyFormat('h2')" title="Heading 2" class="toolbar-btn">H2</button>
        <button @click="applyFormat('h3')" title="Heading 3" class="toolbar-btn">H3</button>
      </div>

      <div class="toolbar-divider"></div>

    <div class="toolbar-group">
      <button @click="applyFormat('link')" title="Link (Ctrl+K)" class="toolbar-btn">🔗</button>
      <button @click="applyFormat('quote')" title="Quote" class="toolbar-btn">"</button>
      <button @click="applyFormat('ul')" title="Bullet List" class="toolbar-btn">•</button>
      <button @click="applyFormat('ol')" title="Numbered List" class="toolbar-btn">1.</button>
      <button @click="applyFormat('codeBlock')" title="Code Block" class="toolbar-btn">{ }</button>
      <button @click="emit('insertTable')" title="Insert Table (Ctrl+Shift+T)" class="toolbar-btn">⊞</button>
      <button @click="emit('insertReminder')" title="Insert Reminder (Ctrl+Shift+R)" class="toolbar-btn">🔔</button>
    </div>

      <div class="toolbar-divider"></div>

      <div class="toolbar-group">
        <input
          ref="fileInputRef"
          type="file"
          accept="image/*,application/pdf,text/*,application/json"
          class="file-input"
          @change="handleFileSelect"
        />
        <button @click="triggerFilePicker" title="Embed file or image" class="toolbar-btn">📎</button>
      </div>

      <div class="toolbar-divider"></div>

      <div class="toolbar-group">
        <button 
          @click="handleUndo" 
          :disabled="!history.canUndo()" 
          title="Undo (Ctrl+Z)" 
          class="toolbar-btn"
        >↶</button>
        <button 
          @click="handleRedo" 
          :disabled="!history.canRedo()" 
          title="Redo (Ctrl+Shift+Z / Ctrl+Y)" 
          class="toolbar-btn"
        >↷</button>
      </div>
    </div>

    <textarea
      v-if="editorMode === 'markdown'"
      ref="textareaRef"
      v-model="localValue"
      :placeholder="placeholder || 'Write your markdown here...'"
      class="editor-textarea"
      @keydown="handleKeydown"
      @paste="handlePaste"
      spellcheck="false"
    ></textarea>

    <!-- Milkdown WYSIWYG editor -->
    <div
      v-else
      class="milkdown-wysiwyg-container"
    >
      <!-- Milkdown editor container -->
      <div 
        ref="milkdownContainerRef" 
        class="milkdown-editor"
        :class="{ 'is-loading': !isMilkdownReady }"
      ></div>
      <!-- Preview panel for mermaid/plantuml diagrams -->
      <div 
        ref="previewRef" 
        class="milkdown-preview"
        v-html="previewContent"
      ></div>
    </div>
  </div>
</template>

<style scoped>
.wiki-editor {
  display: flex;
  flex-direction: column;
  height: 100%;
  background: var(--editor-bg);
}

.editor-toolbar {
  display: flex;
  align-items: center;
  gap: 8px;
  padding: 8px 12px;
  background: var(--toolbar-bg);
  border-bottom: 1px solid var(--border-color);
  flex-shrink: 0;
}

.toolbar-group {
  display: flex;
  gap: 4px;
}

.file-input {
  display: none;
}

.mode-btn {
  padding: 6px 12px;
  background: var(--btn-bg);
  border: 1px solid var(--border-color);
  border-radius: 6px;
  color: var(--text-primary);
  font-size: 12px;
  cursor: pointer;
  transition: all 0.15s;
  font-family: inherit;
  font-weight: 500;
}

.mode-btn:hover {
  background: var(--btn-hover-bg);
  border-color: var(--primary-color);
}

.mode-btn.active {
  background: var(--primary-color);
  border-color: var(--primary-color);
  color: white;
  font-weight: 600;
}

.toolbar-btn {
  padding: 6px 10px;
  background: var(--btn-bg);
  border: 1px solid var(--border-color);
  border-radius: 6px;
  color: var(--text-primary);
  font-size: 13px;
  cursor: pointer;
  transition: all 0.15s;
  font-family: inherit;
}

.toolbar-btn:hover {
  background: var(--btn-hover-bg);
  border-color: var(--primary-color);
}

.toolbar-btn:active {
  transform: scale(0.95);
}

.toolbar-btn:disabled {
  opacity: 0.4;
  cursor: not-allowed;
}

.toolbar-btn:disabled:hover {
  background: var(--btn-bg);
  border-color: var(--border-color);
  transform: none;
}

.toolbar-divider {
  width: 1px;
  height: 20px;
  background: var(--border-color);
}

.editor-textarea {
  flex: 1;
  width: 100%;
  padding: 16px;
  border: none;
  background: var(--editor-bg);
  color: var(--text-primary);
  font-family: 'Monaco', 'Menlo', 'Consolas', monospace;
  font-size: 14px;
  line-height: 1.6;
  resize: none;
  outline: none;
  overflow-y: auto;
}

.editor-textarea::placeholder {
  color: var(--text-tertiary);
}

/* Milkdown WYSIWYG container */
.milkdown-wysiwyg-container {
  flex: 1;
  display: flex;
  min-height: 0;
  overflow: hidden;
}

.milkdown-editor {
  flex: 1;
  min-width: 0;
  overflow-y: auto;
  border-right: 1px solid var(--border-color);
}

.milkdown-editor.is-loading {
  opacity: 0.6;
}

.milkdown-preview {
  flex: 1;
  min-width: 0;
  overflow-y: auto;
  padding: 16px;
  background: var(--preview-bg, var(--editor-bg));
}

/* Milkdown editor styling */
.milkdown-editor :deep(.milkdown) {
  padding: 16px;
  outline: none;
}

.milkdown-editor :deep(.ProseMirror) {
  outline: none;
  min-height: 200px;
  font-family: system-ui, -apple-system, sans-serif;
  font-size: 14px;
  line-height: 1.6;
  color: var(--text-primary);
}

.milkdown-editor :deep(.ProseMirror-focused) {
  outline: none;
}

/* Milkdown content styling */
.milkdown-editor :deep(h1),
.milkdown-editor :deep(h2),
.milkdown-editor :deep(h3),
.milkdown-editor :deep(h4),
.milkdown-editor :deep(h5),
.milkdown-editor :deep(h6) {
  margin-top: 24px;
  margin-bottom: 16px;
  font-weight: 600;
  line-height: 1.25;
}

.milkdown-editor :deep(h1) {
  font-size: 2em;
  border-bottom: 1px solid var(--border-color);
  padding-bottom: 0.3em;
}

.milkdown-editor :deep(h2) {
  font-size: 1.5em;
  border-bottom: 1px solid var(--border-color);
  padding-bottom: 0.3em;
}

.milkdown-editor :deep(h3) {
  font-size: 1.25em;
}

.milkdown-editor :deep(p) {
  margin-bottom: 16px;
}

.milkdown-editor :deep(a) {
  color: var(--link-color);
  text-decoration: none;
}

.milkdown-editor :deep(a:hover) {
  text-decoration: underline;
}

.milkdown-editor :deep(code) {
  padding: 0.2em 0.4em;
  margin: 0;
  font-size: 85%;
  background: var(--code-bg);
  border-radius: 6px;
  font-family: 'Monaco', 'Menlo', 'Consolas', monospace;
}

.milkdown-editor :deep(pre) {
  padding: 16px;
  overflow: auto;
  font-size: 85%;
  line-height: 1.45;
  background: var(--code-block-bg);
  border-radius: 6px;
  margin-bottom: 16px;
}

.milkdown-editor :deep(pre code) {
  padding: 0;
  background: transparent;
  border-radius: 0;
}

.milkdown-editor :deep(blockquote) {
  padding: 0 1em;
  color: var(--text-secondary);
  border-left: 0.25em solid var(--border-color);
  margin-bottom: 16px;
}

.milkdown-editor :deep(ul),
.milkdown-editor :deep(ol) {
  padding-left: 2em;
  margin-bottom: 16px;
}

.milkdown-editor :deep(li) {
  margin-bottom: 4px;
}

.milkdown-editor :deep(strong) {
  font-weight: 600;
}

.milkdown-editor :deep(em) {
  font-style: italic;
}

.milkdown-editor :deep(img) {
  max-width: 100%;
  height: auto;
  border-radius: 8px;
}

.milkdown-editor :deep(table) {
  border-collapse: collapse;
  width: 100%;
  margin-bottom: 16px;
}

.milkdown-editor :deep(th),
.milkdown-editor :deep(td) {
  padding: 6px 13px;
  border: 1px solid var(--border-color);
}

.milkdown-editor :deep(th) {
  font-weight: 600;
  background: var(--table-header-bg, var(--code-bg));
}

.milkdown-editor :deep(hr) {
  height: 0.25em;
  padding: 0;
  margin: 24px 0;
  background-color: var(--border-color);
  border: 0;
}

/* Preview pane styling (for diagrams) */
.milkdown-preview :deep(.diagram-container) {
  margin: 16px 0;
  padding: 16px;
  background: var(--diagram-bg);
  border-radius: 8px;
  border: 1px solid var(--border-color);
  overflow-x: auto;
}

.milkdown-preview :deep(.diagram-image) {
  display: block;
  max-width: 100%;
  height: auto;
  margin: 0 auto;
}

.milkdown-preview :deep(.mermaid) {
  display: flex;
  justify-content: center;
  align-items: center;
  background: var(--diagram-bg);
  margin: 16px 0;
  padding: 16px;
  border-radius: 8px;
  border: 1px solid var(--border-color);
  overflow-x: auto;
  min-height: 100px;
}

.milkdown-preview :deep(.mermaid svg) {
  max-width: 100%;
  height: auto;
}

.milkdown-preview :deep(.diagram-error) {
  color: var(--error-color);
  padding: 12px;
  background: var(--error-bg);
  border-radius: 6px;
  font-family: monospace;
}

/* Dark mode */
@media (prefers-color-scheme: dark) {
  .wiki-editor {
    --editor-bg: #1c1c1e;
    --toolbar-bg: #2c2c2e;
    --border-color: #38383a;
    --text-primary: #f5f5f7;
    --text-secondary: #98989d;
    --text-tertiary: #636366;
    --primary-color: #0a84ff;
    --btn-bg: #3a3a3c;
    --btn-hover-bg: #48484a;
    --link-color: #0a84ff;
    --code-bg: #2c2c2e;
    --code-block-bg: #2c2c2e;
    --diagram-bg: #2c2c2e;
    --error-color: #ff453a;
    --error-bg: rgba(255, 69, 58, 0.1);
    --preview-bg: #0f0f0f;
  }
}

/* Light mode */
@media (prefers-color-scheme: light) {
  .wiki-editor {
    --editor-bg: #ffffff;
    --toolbar-bg: #f5f5f7;
    --border-color: #e5e5ea;
    --text-primary: #1d1d1f;
    --text-secondary: #86868b;
    --text-tertiary: #98989d;
    --primary-color: #007aff;
    --btn-bg: #ffffff;
    --btn-hover-bg: #e8e8ed;
    --link-color: #007aff;
    --code-bg: #f5f5f7;
    --code-block-bg: #f5f5f7;
    --diagram-bg: #fafafa;
    --error-color: #ff3b30;
    --error-bg: rgba(255, 59, 48, 0.1);
    --preview-bg: #fafafa;
  }
}
</style>
