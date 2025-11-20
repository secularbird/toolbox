<script setup lang="ts">
import { ref, watch, onMounted } from 'vue';
import { wrapSelection, insertAtCursor, markdownFormats } from '../utils/markdown';

const props = defineProps<{
  modelValue: string;
  placeholder?: string;
}>();

const emit = defineEmits<{
  'update:modelValue': [value: string];
}>();

const textareaRef = ref<HTMLTextAreaElement | null>(null);
const localValue = ref(props.modelValue);
const fileInputRef = ref<HTMLInputElement | null>(null);

watch(() => props.modelValue, (newVal) => {
  localValue.value = newVal;
});

watch(localValue, (newVal) => {
  emit('update:modelValue', newVal);
});

function applyFormat(formatKey: keyof typeof markdownFormats) {
  if (!textareaRef.value) return;
  const format = markdownFormats[formatKey];
  localValue.value = wrapSelection(textareaRef.value, format.before, format.after);
}

function insertText(text: string) {
  if (!textareaRef.value) return;
  localValue.value = insertAtCursor(textareaRef.value, text);
}

function insertContentBlock(text: string) {
  if (!textareaRef.value) return;
  const content = localValue.value ? `${text}` : text;
  localValue.value = insertAtCursor(textareaRef.value, content);
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

function handleKeydown(e: KeyboardEvent) {
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
    }
  }
}

function handlePaste(e: ClipboardEvent) {
  const clipboard = e.clipboardData;
  if (!clipboard || !textareaRef.value) return;

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
}

onMounted(() => {
  textareaRef.value?.focus();
});

defineExpose({ applyFormat, insertText });
</script>

<template>
  <div class="wiki-editor">
    <div class="editor-toolbar">
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
    </div>

    <textarea
      ref="textareaRef"
      v-model="localValue"
      :placeholder="placeholder || 'Write your markdown here...'"
      class="editor-textarea"
      @keydown="handleKeydown"
      @paste="handlePaste"
      spellcheck="false"
    ></textarea>
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
}

.editor-textarea::placeholder {
  color: var(--text-tertiary);
}

/* Dark mode */
@media (prefers-color-scheme: dark) {
  .wiki-editor {
    --editor-bg: #1c1c1e;
    --toolbar-bg: #2c2c2e;
    --border-color: #38383a;
    --text-primary: #f5f5f7;
    --text-tertiary: #636366;
    --primary-color: #0a84ff;
    --btn-bg: #3a3a3c;
    --btn-hover-bg: #48484a;
  }
}

/* Light mode */
@media (prefers-color-scheme: light) {
  .wiki-editor {
    --editor-bg: #ffffff;
    --toolbar-bg: #f5f5f7;
    --border-color: #e5e5ea;
    --text-primary: #1d1d1f;
    --text-tertiary: #98989d;
    --primary-color: #007aff;
    --btn-bg: #ffffff;
    --btn-hover-bg: #e8e8ed;
  }
}
</style>
