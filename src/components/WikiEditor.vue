<script setup lang="ts">
import { ref, watch, onMounted, nextTick } from 'vue';
import { wrapSelection, insertAtCursor, markdownFormats, renderMarkdown, sanitizeHtml } from '../utils/markdown';
import { EditorHistory } from '../utils/editorHistory';
import TurndownService from 'turndown';
import { gfm } from 'turndown-plugin-gfm';
import mermaid from 'mermaid';

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
const wysiwygRef = ref<HTMLDivElement | null>(null);
const localValue = ref(props.modelValue);
const fileInputRef = ref<HTMLInputElement | null>(null);
const history = new EditorHistory(props.modelValue);
const isUndoRedoing = ref(false);

// Initialize Turndown for HTML to Markdown conversion
const turndownService = new TurndownService({
  headingStyle: 'atx',
  codeBlockStyle: 'fenced',
  bulletListMarker: '-',
  emDelimiter: '*',
  strongDelimiter: '**',
});
turndownService.use(gfm);

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

// Render Mermaid diagrams in WYSIWYG editor
async function renderMermaidInWysiwyg() {
  if (mermaidRenderingInProgress || !wysiwygRef.value) {
    return;
  }

  mermaidRenderingInProgress = true;
  await nextTick();

  try {
    const mermaidElements = wysiwygRef.value.querySelectorAll('.mermaid:not([data-processed="rendered"])');

    if (mermaidElements.length > 0) {
      for (const element of Array.from(mermaidElements)) {
        try {
          element.setAttribute('data-processed', 'processing');
          const id = `mermaid-${Date.now()}-${Math.random().toString(36).substr(2, 9)}`;
          element.id = id;

          await mermaid.run({
            nodes: [element as HTMLElement],
          });

          element.setAttribute('data-processed', 'rendered');
        } catch (err) {
          console.error('Mermaid rendering error for element:', err);
          element.setAttribute('data-processed', 'error');
          element.innerHTML = `<div class="diagram-error">Mermaid 渲染错误: ${err instanceof Error ? err.message : String(err)}</div>`;
        }
      }
    }
  } catch (error) {
    console.error('Mermaid batch rendering error:', error);
  } finally {
    mermaidRenderingInProgress = false;
  }
}

watch(() => props.modelValue, async (newVal) => {
  if (localValue.value !== newVal) {
    localValue.value = newVal;
    history.reset(newVal);
    // Update WYSIWYG content when modelValue changes
    // renderMarkdown includes sanitizeHtml to prevent XSS
    if (editorMode.value === 'wysiwyg' && wysiwygRef.value) {
      wysiwygRef.value.innerHTML = renderMarkdown(newVal);
      // Render Mermaid diagrams after setting HTML content
      await renderMermaidInWysiwyg();
    }
  }
});

watch(localValue, (newVal) => {
  if (!isUndoRedoing.value) {
    history.save(newVal);
  }
  emit('update:modelValue', newVal);
});

// Watch for mode changes
watch(editorMode, async (newMode, oldMode) => {
  if (newMode === 'wysiwyg' && wysiwygRef.value) {
    // Convert markdown to HTML for WYSIWYG
    // renderMarkdown includes sanitizeHtml for XSS prevention
    wysiwygRef.value.innerHTML = renderMarkdown(localValue.value);
    // Render Mermaid diagrams after setting HTML content
    await renderMermaidInWysiwyg();
  } else if (newMode === 'markdown' && oldMode === 'wysiwyg' && wysiwygRef.value) {
    // Convert HTML back to markdown
    const html = wysiwygRef.value.innerHTML;
    localValue.value = turndownService.turndown(html);
  }
});

function applyFormat(formatKey: keyof typeof markdownFormats) {
  if (editorMode.value === 'markdown') {
    if (!textareaRef.value) return;
    const format = markdownFormats[formatKey];
    localValue.value = wrapSelection(textareaRef.value, format.before, format.after);
  } else {
    // WYSIWYG mode - use execCommand
    applyWysiwygFormat(formatKey);
  }
}

function applyWysiwygFormat(formatKey: keyof typeof markdownFormats) {
  if (!wysiwygRef.value) return;
  
  wysiwygRef.value.focus();
  
  // Note: document.execCommand is deprecated but still widely supported
  // and provides the most reliable cross-browser WYSIWYG editing experience.
  // Modern alternatives like the Selection API require significantly more complex
  // implementation for the same functionality. We'll migrate when a stable
  // replacement API is broadly available.
  
  const commandMap: Record<string, string> = {
    bold: 'bold',
    italic: 'italic',
    code: 'insertHTML', // Special handling needed
    h1: 'formatBlock',
    h2: 'formatBlock',
    h3: 'formatBlock',
    quote: 'formatBlock',
    ul: 'insertUnorderedList',
    ol: 'insertOrderedList',
    link: 'createLink',
    codeBlock: 'insertHTML', // Special handling needed
  };
  
  const command = commandMap[formatKey];
  
  try {
    if (formatKey === 'code') {
      // Wrap selection in <code> tag
      const selection = window.getSelection();
      if (selection && selection.rangeCount > 0) {
        const range = selection.getRangeAt(0);
        const selectedText = range.toString();
        const code = document.createElement('code');
        code.textContent = selectedText || 'code';
        range.deleteContents();
        range.insertNode(code);
        
        // Move cursor after the code element
        range.setStartAfter(code);
        range.setEndAfter(code);
        selection.removeAllRanges();
        selection.addRange(range);
      }
    } else if (formatKey === 'codeBlock') {
      // Insert a code block
      const selection = window.getSelection();
      if (selection && selection.rangeCount > 0) {
        const range = selection.getRangeAt(0);
        const selectedText = range.toString() || 'code block';
        const pre = document.createElement('pre');
        const code = document.createElement('code');
        code.textContent = selectedText;
        pre.appendChild(code);
        range.deleteContents();
        range.insertNode(pre);
        
        // Add line break after pre
        const br = document.createElement('br');
        range.setStartAfter(pre);
        range.insertNode(br);
        range.setStartAfter(br);
        selection.removeAllRanges();
        selection.addRange(range);
      }
    } else if (formatKey === 'h1') {
      document.execCommand('formatBlock', false, '<h1>');
    } else if (formatKey === 'h2') {
      document.execCommand('formatBlock', false, '<h2>');
    } else if (formatKey === 'h3') {
      document.execCommand('formatBlock', false, '<h3>');
    } else if (formatKey === 'quote') {
      document.execCommand('formatBlock', false, '<blockquote>');
    } else if (formatKey === 'link') {
      const url = prompt('Enter URL:', 'https://');
      if (url) {
        // Validate URL to prevent javascript: and other dangerous protocols
        try {
          const urlObj = new URL(url);
          if (urlObj.protocol === 'http:' || urlObj.protocol === 'https:') {
            document.execCommand(command, false, url);
          } else {
            alert('Only http:// and https:// URLs are allowed');
          }
        } catch (e) {
          alert('Invalid URL');
        }
      }
    } else {
      document.execCommand(command, false);
    }
    
    // Update localValue from WYSIWYG content
    updateFromWysiwyg();
  } catch (error) {
    console.error('Error applying WYSIWYG format:', error);
  }
}

function updateFromWysiwyg() {
  if (!wysiwygRef.value) return;
  const html = wysiwygRef.value.innerHTML;
  localValue.value = turndownService.turndown(html);
}

function insertText(text: string) {
  if (editorMode.value === 'markdown') {
    if (!textareaRef.value) return;
    localValue.value = insertAtCursor(textareaRef.value, text);
  } else {
    // WYSIWYG mode
    if (!wysiwygRef.value) return;
    wysiwygRef.value.focus();
    document.execCommand('insertText', false, text);
    updateFromWysiwyg();
  }
}

function insertContentBlock(text: string) {
  if (editorMode.value === 'markdown') {
    if (!textareaRef.value) return;
    const content = localValue.value ? `${text}` : text;
    localValue.value = insertAtCursor(textareaRef.value, content);
  } else {
    // WYSIWYG mode - sanitize HTML before insertion
    if (!wysiwygRef.value) return;
    wysiwygRef.value.focus();
    
    // Check if text is HTML (contains tags)
    const isHTML = /<[^>]+>/.test(text);
    if (isHTML) {
      // Sanitize HTML content before insertion
      const parser = new DOMParser();
      const doc = parser.parseFromString(text, 'text/html');
      const sanitized = doc.body.textContent || '';
      document.execCommand('insertText', false, sanitized);
    } else {
      document.execCommand('insertText', false, text);
    }
    updateFromWysiwyg();
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
  } else {
    handleWysiwygKeydown(e);
  }
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

function handleWysiwygKeydown(e: KeyboardEvent) {
  if (e.ctrlKey || e.metaKey) {
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

function handleWysiwygInput() {
  updateFromWysiwyg();
}

function handlePaste(e: ClipboardEvent) {
  const clipboard = e.clipboardData;
  if (!clipboard) return;

  if (editorMode.value === 'markdown') {
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

    // Prefer HTML content to preserve formatting
    const htmlContent = clipboard.getData('text/html');
    if (htmlContent) {
      e.preventDefault();
      insertContentBlock(htmlContent.trim());
      return;
    }
  } else {
    // WYSIWYG mode - sanitize HTML content before allowing paste
    if (!wysiwygRef.value) return;
    
    // For images, convert to data URLs
    const imageFile = Array.from(clipboard.files || []).find((file) =>
      file.type.startsWith('image/')
    );
    if (imageFile) {
      e.preventDefault();
      const reader = new FileReader();
      reader.onload = () => {
        if (typeof reader.result === 'string') {
          const img = document.createElement('img');
          img.src = reader.result;
          img.alt = 'pasted-image';
          const selection = window.getSelection();
          if (selection && selection.rangeCount > 0) {
            const range = selection.getRangeAt(0);
            range.deleteContents();
            range.insertNode(img);
          }
          updateFromWysiwyg();
        }
      };
      reader.readAsDataURL(imageFile);
      return;
    }
    
    // For HTML content, sanitize it before paste
    const htmlContent = clipboard.getData('text/html');
    if (htmlContent) {
      e.preventDefault();
      const sanitized = sanitizeHtml(htmlContent);
      const selection = window.getSelection();
      if (selection && selection.rangeCount > 0) {
        const range = selection.getRangeAt(0);
        range.deleteContents();
        const template = document.createElement('template');
        template.innerHTML = sanitized;
        const fragment = template.content;
        range.insertNode(fragment);
      }
      updateFromWysiwyg();
    }
  }
}

onMounted(() => {
  // Initialize Mermaid
  initMermaid();
  
  // Re-initialize Mermaid when color scheme changes
  const mediaQuery = window.matchMedia('(prefers-color-scheme: dark)');
  mediaQuery.addEventListener('change', () => {
    initMermaid();
    // Re-render all diagrams in WYSIWYG mode
    if (editorMode.value === 'wysiwyg' && wysiwygRef.value) {
      const elements = wysiwygRef.value.querySelectorAll('.mermaid[data-processed="rendered"]');
      elements.forEach(el => {
        el.removeAttribute('data-processed');
        el.removeAttribute('id');
      });
      mermaidRenderingInProgress = false;
      renderMermaidInWysiwyg();
    }
  });
  
  if (editorMode.value === 'markdown') {
    textareaRef.value?.focus();
  } else {
    wysiwygRef.value?.focus();
  }
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

    <div
      v-else
      ref="wysiwygRef"
      contenteditable="true"
      :placeholder="placeholder || 'Write your content here...'"
      class="editor-wysiwyg"
      @input="handleWysiwygInput"
      @keydown="handleKeydown"
      @paste="handlePaste"
    ></div>
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

.editor-wysiwyg {
  flex: 1;
  width: 100%;
  padding: 16px;
  border: none;
  background: var(--editor-bg);
  color: var(--text-primary);
  font-family: system-ui, -apple-system, sans-serif;
  font-size: 14px;
  line-height: 1.6;
  outline: none;
  overflow-y: auto;
  cursor: text;
}

.editor-wysiwyg:empty:before {
  content: attr(placeholder);
  color: var(--text-tertiary);
  pointer-events: none;
}

/* WYSIWYG content styling */
.editor-wysiwyg :deep(h1),
.editor-wysiwyg :deep(h2),
.editor-wysiwyg :deep(h3),
.editor-wysiwyg :deep(h4),
.editor-wysiwyg :deep(h5),
.editor-wysiwyg :deep(h6) {
  margin-top: 24px;
  margin-bottom: 16px;
  font-weight: 600;
  line-height: 1.25;
}

.editor-wysiwyg :deep(h1) {
  font-size: 2em;
  border-bottom: 1px solid var(--border-color);
  padding-bottom: 0.3em;
}

.editor-wysiwyg :deep(h2) {
  font-size: 1.5em;
  border-bottom: 1px solid var(--border-color);
  padding-bottom: 0.3em;
}

.editor-wysiwyg :deep(h3) {
  font-size: 1.25em;
}

.editor-wysiwyg :deep(p) {
  margin-bottom: 16px;
}

.editor-wysiwyg :deep(a) {
  color: var(--link-color);
  text-decoration: none;
}

.editor-wysiwyg :deep(a:hover) {
  text-decoration: underline;
}

.editor-wysiwyg :deep(code) {
  padding: 0.2em 0.4em;
  margin: 0;
  font-size: 85%;
  background: var(--code-bg);
  border-radius: 6px;
  font-family: 'Monaco', 'Menlo', 'Consolas', monospace;
}

.editor-wysiwyg :deep(pre) {
  padding: 16px;
  overflow: auto;
  font-size: 85%;
  line-height: 1.45;
  background: var(--code-block-bg);
  border-radius: 6px;
  margin-bottom: 16px;
}

.editor-wysiwyg :deep(pre code) {
  padding: 0;
  background: transparent;
  border-radius: 0;
}

.editor-wysiwyg :deep(blockquote) {
  padding: 0 1em;
  color: var(--text-secondary);
  border-left: 0.25em solid var(--border-color);
  margin-bottom: 16px;
}

.editor-wysiwyg :deep(ul),
.editor-wysiwyg :deep(ol) {
  padding-left: 2em;
  margin-bottom: 16px;
}

.editor-wysiwyg :deep(li) {
  margin-bottom: 4px;
}

.editor-wysiwyg :deep(strong) {
  font-weight: 600;
}

.editor-wysiwyg :deep(em) {
  font-style: italic;
}

.editor-wysiwyg :deep(img) {
  max-width: 100%;
  height: auto;
  border-radius: 8px;
}

/* Diagram styles for WYSIWYG mode */
.editor-wysiwyg :deep(.diagram-container) {
  margin: 16px 0;
  padding: 16px;
  background: var(--diagram-bg);
  border-radius: 8px;
  border: 1px solid var(--border-color);
  overflow-x: auto;
}

.editor-wysiwyg :deep(.diagram-image) {
  display: block;
  max-width: 100%;
  height: auto;
  margin: 0 auto;
}

.editor-wysiwyg :deep(.mermaid) {
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

.editor-wysiwyg :deep(.mermaid svg) {
  max-width: 100%;
  height: auto;
}

.editor-wysiwyg :deep(.diagram-error) {
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
  }
}
</style>
