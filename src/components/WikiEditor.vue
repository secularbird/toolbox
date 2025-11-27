<script setup lang="ts">
import { ref, watch, onMounted, onBeforeUnmount, nextTick, shallowRef } from 'vue';
import { wrapSelection, insertAtCursor, markdownFormats, extractMarkdownTables, getMermaidInkUrl, generateDiagramId } from '../utils/markdown';
import { EditorHistory } from '../utils/editorHistory';
import plantumlEncoder from 'plantuml-encoder';
// Milkdown imports
import { Editor, rootCtx, defaultValueCtx, editorViewCtx } from '@milkdown/core';
import { commonmark } from '@milkdown/preset-commonmark';
import { gfm } from '@milkdown/preset-gfm';
import { history as milkdownHistory } from '@milkdown/plugin-history';
import { listener, listenerCtx } from '@milkdown/plugin-listener';
import type { Ctx } from '@milkdown/ctx';
// ProseMirror table commands for WYSIWYG table editing
import {
  addRowBefore,
  addRowAfter,
  deleteRow,
  addColumnBefore,
  addColumnAfter,
  deleteColumn,
  isInTable
} from 'prosemirror-tables';

const props = defineProps<{
  modelValue: string;
  placeholder?: string;
}>();

const emit = defineEmits<{
  'update:modelValue': [value: string];
  'insertTable': [];
  'insertReminder': [];
  'editTable': [tableIndex: number];
}>();

// Editor mode: 'markdown' or 'wysiwyg'
const editorMode = ref<'markdown' | 'wysiwyg'>('wysiwyg');
const textareaRef = ref<HTMLTextAreaElement | null>(null);
const milkdownContainerRef = ref<HTMLDivElement | null>(null);
const localValue = ref(props.modelValue);
const fileInputRef = ref<HTMLInputElement | null>(null);
const history = new EditorHistory(props.modelValue);
const isUndoRedoing = ref(false);

// Milkdown editor instance
const milkdownEditor = shallowRef<Editor | null>(null);
const isMilkdownReady = ref(false);
// Track if we're updating content to prevent loops
let isUpdatingMilkdown = false;

// Table toolbar state for WYSIWYG mode
const showTableToolbar = ref(false);
const tableToolbarPosition = ref({ top: 0, left: 0 });
// Track last known state to avoid unnecessary updates
let lastInTableState = false;
let lastSelectionFrom = -1;

// Check if cursor is in a table and update toolbar visibility
function updateTableToolbarState() {
  if (!milkdownEditor.value || !isMilkdownReady.value || editorMode.value !== 'wysiwyg') {
    showTableToolbar.value = false;
    lastInTableState = false;
    return;
  }

  try {
    milkdownEditor.value.action((ctx: Ctx) => {
      const view = ctx.get(editorViewCtx);
      const { state } = view;
      const currentInTable = isInTable(state);
      const currentFrom = state.selection.from;
      
      // Only update if the table state or position has changed
      if (currentInTable === lastInTableState && currentFrom === lastSelectionFrom) {
        return;
      }
      
      lastInTableState = currentInTable;
      lastSelectionFrom = currentFrom;
      
      if (currentInTable) {
        showTableToolbar.value = true;
        
        // Position the toolbar near the selection
        const coords = view.coordsAtPos(currentFrom);
        const editorRect = milkdownContainerRef.value?.getBoundingClientRect();
        
        if (editorRect) {
          tableToolbarPosition.value = {
            top: coords.top - editorRect.top - 40,
            left: Math.max(0, coords.left - editorRect.left)
          };
        }
      } else {
        showTableToolbar.value = false;
      }
    });
  } catch {
    showTableToolbar.value = false;
    lastInTableState = false;
  }
}

// Handle click on WYSIWYG editor to detect table clicks for editing
function handleWysiwygClick(event: MouseEvent) {
  const target = event.target as HTMLElement;
  const table = target.closest('table');
  
  if (table && milkdownContainerRef.value?.contains(table)) {
    // Find the index of this table in the content
    const allTables = milkdownContainerRef.value.querySelectorAll('table');
    let tableIndex = -1;
    
    allTables.forEach((t, index) => {
      if (t === table) {
        tableIndex = index;
      }
    });
    
    if (tableIndex >= 0) {
      // Verify this matches a table in the markdown content
      const markdownTables = extractMarkdownTables(localValue.value);
      if (tableIndex < markdownTables.length) {
        emit('editTable', tableIndex);
      }
    }
  }
}

// Table manipulation functions for WYSIWYG mode
function handleAddRowAbove() {
  if (!milkdownEditor.value || !isMilkdownReady.value) return;
  
  try {
    milkdownEditor.value.action((ctx: Ctx) => {
      const view = ctx.get(editorViewCtx);
      addRowBefore(view.state, view.dispatch);
    });
  } catch (error) {
    console.error('Error adding row above:', error);
  }
}

function handleAddRowBelow() {
  if (!milkdownEditor.value || !isMilkdownReady.value) return;
  
  try {
    milkdownEditor.value.action((ctx: Ctx) => {
      const view = ctx.get(editorViewCtx);
      addRowAfter(view.state, view.dispatch);
    });
  } catch (error) {
    console.error('Error adding row below:', error);
  }
}

function handleDeleteRow() {
  if (!milkdownEditor.value || !isMilkdownReady.value) return;
  
  try {
    milkdownEditor.value.action((ctx: Ctx) => {
      const view = ctx.get(editorViewCtx);
      deleteRow(view.state, view.dispatch);
    });
  } catch (error) {
    console.error('Error deleting row:', error);
  }
}

function handleAddColumnLeft() {
  if (!milkdownEditor.value || !isMilkdownReady.value) return;
  
  try {
    milkdownEditor.value.action((ctx: Ctx) => {
      const view = ctx.get(editorViewCtx);
      addColumnBefore(view.state, view.dispatch);
    });
  } catch (error) {
    console.error('Error adding column left:', error);
  }
}

function handleAddColumnRight() {
  if (!milkdownEditor.value || !isMilkdownReady.value) return;
  
  try {
    milkdownEditor.value.action((ctx: Ctx) => {
      const view = ctx.get(editorViewCtx);
      addColumnAfter(view.state, view.dispatch);
    });
  } catch (error) {
    console.error('Error adding column right:', error);
  }
}

function handleDeleteColumn() {
  if (!milkdownEditor.value || !isMilkdownReady.value) return;
  
  try {
    milkdownEditor.value.action((ctx: Ctx) => {
      const view = ctx.get(editorViewCtx);
      deleteColumn(view.state, view.dispatch);
    });
  } catch (error) {
    console.error('Error deleting column:', error);
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
          // Render diagrams and style reminders after content changes
          nextTick(() => {
            renderDiagramsInWysiwyg();
            styleRemindersInWysiwyg();
          });
        });
        // Setup listener for selection changes to update table toolbar
        ctx.get(listenerCtx).updated(() => {
          updateTableToolbarState();
        });
      })
      .use(commonmark)
      .use(gfm)
      .use(milkdownHistory)
      .use(listener)
      .create();
    
    milkdownEditor.value = editor;
    isMilkdownReady.value = true;
    
    // Render diagrams and style reminders after initial load
    await nextTick();
    renderDiagramsInWysiwyg();
    styleRemindersInWysiwyg();
    
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

// Render diagram code blocks in WYSIWYG mode
// Uses data-diagram-processed attribute to skip already processed blocks
function renderDiagramsInWysiwyg() {
  if (!milkdownContainerRef.value || editorMode.value !== 'wysiwyg') return;
  
  // Only find unprocessed code blocks by using a more specific selector
  // This is more efficient than checking all code blocks
  const unprocessedPreBlocks = milkdownContainerRef.value.querySelectorAll(
    'pre:not([data-diagram-processed="true"]) code'
  );
  
  if (unprocessedPreBlocks.length === 0) return;
  
  unprocessedPreBlocks.forEach((codeBlock) => {
    const preElement = codeBlock.parentElement;
    if (!preElement) return;
    
    // Check language class for mermaid or plantuml
    const classList = codeBlock.className.split(' ');
    const langClass = classList.find(c => c.startsWith('language-'));
    if (!langClass) return;
    
    const lang = langClass.replace('language-', '').toLowerCase();
    
    // Only process diagram languages
    if (lang !== 'mermaid' && lang !== 'mermiad' && lang !== 'plantuml') {
      // Mark non-diagram code blocks as processed to skip in future iterations
      preElement.setAttribute('data-diagram-processed', 'skipped');
      return;
    }
    
    const code = codeBlock.textContent || '';
    
    if (lang === 'mermaid' || lang === 'mermiad') {
      try {
        const url = getMermaidInkUrl(code);
        const id = generateDiagramId('mermaid');
        const container = document.createElement('div');
        container.className = 'diagram-preview mermaid-preview';
        container.id = id;
        container.innerHTML = `<img src="${url}" alt="Mermaid Diagram" class="diagram-image" />`;
        
        // Insert after the code block
        preElement.setAttribute('data-diagram-processed', 'true');
        preElement.insertAdjacentElement('afterend', container);
      } catch (err) {
        console.error('Mermaid rendering error:', err);
      }
    } else if (lang === 'plantuml') {
      try {
        const encoded = plantumlEncoder.encode(code);
        const url = `https://www.plantuml.com/plantuml/svg/${encoded}`;
        const id = generateDiagramId('plantuml');
        const container = document.createElement('div');
        container.className = 'diagram-preview plantuml-preview';
        container.id = id;
        container.innerHTML = `<img src="${url}" alt="PlantUML Diagram" class="diagram-image" />`;
        
        // Insert after the code block
        preElement.setAttribute('data-diagram-processed', 'true');
        preElement.insertAdjacentElement('afterend', container);
      } catch (err) {
        console.error('PlantUML rendering error:', err);
      }
    }
  });
}

// Category icons mapping for reminder cards
const categoryIcons: Record<string, { icon: string; name: string; color: string }> = {
  work: { icon: '💼', name: 'Work', color: '#ff9800' },
  personal: { icon: '👤', name: 'Personal', color: '#4caf50' },
  shopping: { icon: '🛒', name: 'Shopping', color: '#e91e63' },
  health: { icon: '🏥', name: 'Health', color: '#00bcd4' },
  other: { icon: '📌', name: 'Other', color: '#9c27b0' },
};

// Frequency labels mapping for reminder cards
const frequencyLabels: Record<string, { icon: string; label: string }> = {
  once: { icon: '🔵', label: 'Once' },
  daily: { icon: '📅', label: 'Daily' },
  weekly: { icon: '📆', label: 'Weekly' },
  monthly: { icon: '🗓️', label: 'Monthly' },
  yearly: { icon: '📊', label: 'Yearly' },
};

// Style reminder blockquotes in WYSIWYG mode
// Converts blockquotes with reminder data into styled reminder cards
function styleRemindersInWysiwyg() {
  if (!milkdownContainerRef.value || editorMode.value !== 'wysiwyg') return;
  
  // Find all blockquotes that haven't been processed yet
  const blockquotes = milkdownContainerRef.value.querySelectorAll(
    'blockquote:not([data-reminder-processed])'
  );
  
  blockquotes.forEach((blockquote) => {
    const textContent = blockquote.textContent || '';
    
    // Check if this blockquote contains reminder content
    // The reminder pattern: starts with "🔔 Reminder:" and contains reminder-data comment
    if (!textContent.includes('🔔 Reminder:')) {
      blockquote.setAttribute('data-reminder-processed', 'false');
      return;
    }
    
    // Try to extract reminder data from the HTML content
    const innerHTML = blockquote.innerHTML;
    const commentMatch = innerHTML.match(/<!--\s*reminder-data:([\s\S]*?)-->/);
    
    if (!commentMatch) {
      blockquote.setAttribute('data-reminder-processed', 'false');
      return;
    }
    
    try {
      const data = JSON.parse(commentMatch[1].trim());
      const category = categoryIcons[data.category] || categoryIcons.other;
      const frequency = frequencyLabels[data.frequency] || frequencyLabels.once;
      const time = new Date(data.time).toLocaleString();
      const isPast = new Date(data.time) < new Date();
      
      // Create a styled reminder card
      const cardDiv = document.createElement('div');
      cardDiv.className = 'reminder-card-wysiwyg';
      cardDiv.setAttribute('data-reminder-id', String(data.id));
      cardDiv.innerHTML = `
        <div class="reminder-card-header">
          <span class="reminder-icon">🔔</span>
          <span class="reminder-title">${escapeHtmlText(data.title)}</span>
          ${isPast ? '<span class="reminder-badge past">Past</span>' : '<span class="reminder-badge upcoming">Upcoming</span>'}
        </div>
        <div class="reminder-card-body">
          <div class="reminder-meta-item">
            <span class="reminder-meta-icon">${category.icon}</span>
            <span class="reminder-meta-label">Category:</span>
            <span class="reminder-meta-value" style="color: ${category.color}">${category.name}</span>
          </div>
          <div class="reminder-meta-item">
            <span class="reminder-meta-icon">${frequency.icon}</span>
            <span class="reminder-meta-label">Frequency:</span>
            <span class="reminder-meta-value">${frequency.label}</span>
          </div>
          <div class="reminder-meta-item">
            <span class="reminder-meta-icon">⏰</span>
            <span class="reminder-meta-label">Time:</span>
            <span class="reminder-meta-value ${isPast ? 'past' : ''}">${time}</span>
          </div>
          ${data.description && data.description !== 'Created from Wiki' ? `
          <div class="reminder-description">
            <span class="reminder-meta-icon">📝</span>
            <span>${escapeHtmlText(data.description)}</span>
          </div>
          ` : ''}
        </div>
      `;
      
      // Insert the card after the blockquote and hide the original
      blockquote.setAttribute('data-reminder-processed', 'true');
      blockquote.setAttribute('style', 'display: none;');
      blockquote.insertAdjacentElement('afterend', cardDiv);
    } catch {
      blockquote.setAttribute('data-reminder-processed', 'false');
    }
  });
}

// Simple HTML text escaper for dynamic content
function escapeHtmlText(text: string): string {
  const div = document.createElement('div');
  div.textContent = text;
  return div.innerHTML;
}

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

// Diagram insertion state
const showDiagramMenu = ref(false);
const diagramMenuRef = ref<HTMLDivElement | null>(null);

// Close diagram menu when clicking outside
function handleClickOutside(event: MouseEvent) {
  const target = event.target;
  if (target && diagramMenuRef.value && !diagramMenuRef.value.contains(target as Node)) {
    showDiagramMenu.value = false;
  }
}

// Insert Mermaid diagram template
function insertMermaidDiagram(type: string) {
  let template = '';
  
  switch (type) {
    case 'flowchart':
      template = `\`\`\`mermaid
graph TD
    A[Start] --> B{Decision}
    B -->|Yes| C[Action 1]
    B -->|No| D[Action 2]
    C --> E[End]
    D --> E
\`\`\``;
      break;
    case 'sequence':
      template = `\`\`\`mermaid
sequenceDiagram
    participant A as User
    participant B as System
    A->>B: Request
    B-->>A: Response
\`\`\``;
      break;
    case 'class':
      template = `\`\`\`mermaid
classDiagram
    class MyClass {
        +String name
        +int id
        +method()
    }
\`\`\``;
      break;
    case 'state':
      template = `\`\`\`mermaid
stateDiagram-v2
    [*] --> State1
    State1 --> State2: Event
    State2 --> [*]
\`\`\``;
      break;
    case 'er':
      template = `\`\`\`mermaid
erDiagram
    CUSTOMER ||--o{ ORDER : places
    ORDER ||--|{ LINE-ITEM : contains
\`\`\``;
      break;
    case 'gantt':
      template = `\`\`\`mermaid
gantt
    title Project Timeline
    dateFormat YYYY-MM-DD
    section Phase 1
    Task 1 :a1, 2024-01-01, 7d
    Task 2 :a2, after a1, 5d
\`\`\``;
      break;
    case 'pie':
      template = `\`\`\`mermaid
pie title Distribution
    "Category A" : 40
    "Category B" : 30
    "Category C" : 30
\`\`\``;
      break;
    default:
      template = `\`\`\`mermaid
graph TD
    A[Start] --> B[End]
\`\`\``;
  }
  
  insertContentBlock('\n' + template + '\n');
  showDiagramMenu.value = false;
}

// Insert PlantUML diagram template
function insertPlantUMLDiagram(type: string) {
  let template = '';
  
  switch (type) {
    case 'sequence':
      template = `\`\`\`plantuml
@startuml
actor User
participant "System" as S

User -> S: Request
S --> User: Response
@enduml
\`\`\``;
      break;
    case 'class':
      template = `\`\`\`plantuml
@startuml
class MyClass {
  +String name
  +int id
  +method()
}
@enduml
\`\`\``;
      break;
    case 'usecase':
      template = `\`\`\`plantuml
@startuml
actor User
usecase "Use Case" as UC
User --> UC
@enduml
\`\`\``;
      break;
    case 'activity':
      template = `\`\`\`plantuml
@startuml
start
:Step 1;
if (Condition?) then (yes)
  :Step 2a;
else (no)
  :Step 2b;
endif
stop
@enduml
\`\`\``;
      break;
    case 'component':
      template = `\`\`\`plantuml
@startuml
package "My Package" {
  [Component A]
  [Component B]
}
[Component A] --> [Component B]
@enduml
\`\`\``;
      break;
    case 'state':
      template = `\`\`\`plantuml
@startuml
[*] --> State1
State1 --> State2 : event
State2 --> [*]
@enduml
\`\`\``;
      break;
    default:
      template = `\`\`\`plantuml
@startuml
Alice -> Bob: Hello
Bob --> Alice: Hi
@enduml
\`\`\``;
  }
  
  insertContentBlock('\n' + template + '\n');
  showDiagramMenu.value = false;
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

onMounted(async () => {
  if (editorMode.value === 'markdown') {
    textareaRef.value?.focus();
  } else if (editorMode.value === 'wysiwyg') {
    // Initialize Milkdown for default WYSIWYG mode
    await nextTick();
    if (milkdownContainerRef.value) {
      await initMilkdown(milkdownContainerRef.value, localValue.value);
    }
  }
  // Add click outside listener for diagram menu
  document.addEventListener('click', handleClickOutside);
});

onBeforeUnmount(async () => {
  await destroyMilkdown();
  // Remove click outside listener
  document.removeEventListener('click', handleClickOutside);
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

      <!-- Diagram insertion menu -->
      <div class="toolbar-group diagram-menu-container" ref="diagramMenuRef">
        <button 
          @click.stop="showDiagramMenu = !showDiagramMenu" 
          title="Insert Diagram" 
          class="toolbar-btn diagram-btn"
          aria-label="Insert Diagram"
          aria-haspopup="true"
          :aria-expanded="showDiagramMenu"
          id="diagram-menu-button"
        >
          📊 Diagram ▾
        </button>
        <div 
          v-if="showDiagramMenu" 
          class="diagram-dropdown"
          role="menu"
          aria-labelledby="diagram-menu-button"
        >
          <div class="diagram-section" role="group" aria-label="Mermaid Diagrams">
            <div class="diagram-section-title" id="mermaid-section-label">🌊 Mermaid Diagrams</div>
            <button @click="insertMermaidDiagram('flowchart')" class="diagram-option" role="menuitem">Flowchart</button>
            <button @click="insertMermaidDiagram('sequence')" class="diagram-option" role="menuitem">Sequence Diagram</button>
            <button @click="insertMermaidDiagram('class')" class="diagram-option" role="menuitem">Class Diagram</button>
            <button @click="insertMermaidDiagram('state')" class="diagram-option" role="menuitem">State Diagram</button>
            <button @click="insertMermaidDiagram('er')" class="diagram-option" role="menuitem">ER Diagram</button>
            <button @click="insertMermaidDiagram('gantt')" class="diagram-option" role="menuitem">Gantt Chart</button>
            <button @click="insertMermaidDiagram('pie')" class="diagram-option" role="menuitem">Pie Chart</button>
          </div>
          <div class="diagram-section-divider" role="separator"></div>
          <div class="diagram-section" role="group" aria-label="PlantUML Diagrams">
            <div class="diagram-section-title" id="plantuml-section-label">🌱 PlantUML Diagrams</div>
            <button @click="insertPlantUMLDiagram('sequence')" class="diagram-option" role="menuitem">Sequence Diagram</button>
            <button @click="insertPlantUMLDiagram('class')" class="diagram-option" role="menuitem">Class Diagram</button>
            <button @click="insertPlantUMLDiagram('usecase')" class="diagram-option" role="menuitem">Use Case Diagram</button>
            <button @click="insertPlantUMLDiagram('activity')" class="diagram-option" role="menuitem">Activity Diagram</button>
            <button @click="insertPlantUMLDiagram('component')" class="diagram-option" role="menuitem">Component Diagram</button>
            <button @click="insertPlantUMLDiagram('state')" class="diagram-option" role="menuitem">State Diagram</button>
          </div>
        </div>
      </div>

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
      <!-- Table toolbar for WYSIWYG mode -->
      <div 
        v-if="showTableToolbar"
        class="table-toolbar"
        :style="{ top: tableToolbarPosition.top + 'px', left: tableToolbarPosition.left + 'px' }"
      >
        <div class="table-toolbar-group">
          <button 
            class="table-toolbar-btn" 
            @click="handleAddRowAbove"
            title="Insert row above"
          >
            ⬆️ Row
          </button>
          <button 
            class="table-toolbar-btn" 
            @click="handleAddRowBelow"
            title="Insert row below"
          >
            ⬇️ Row
          </button>
          <button 
            class="table-toolbar-btn" 
            @click="handleAddColumnLeft"
            title="Insert column left"
          >
            ⬅️ Col
          </button>
          <button 
            class="table-toolbar-btn" 
            @click="handleAddColumnRight"
            title="Insert column right"
          >
            ➡️ Col
          </button>
        </div>
        <div class="table-toolbar-divider"></div>
        <div class="table-toolbar-group">
          <button 
            class="table-toolbar-btn danger" 
            @click="handleDeleteRow"
            title="Delete row"
          >
            🗑️ Row
          </button>
          <button 
            class="table-toolbar-btn danger" 
            @click="handleDeleteColumn"
            title="Delete column"
          >
            🗑️ Col
          </button>
        </div>
      </div>
      
      <!-- Milkdown editor container -->
      <div 
        ref="milkdownContainerRef" 
        class="milkdown-editor"
        :class="{ 'is-loading': !isMilkdownReady }"
        @dblclick="handleWysiwygClick"
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

/* Table toolbar for WYSIWYG mode */
.table-toolbar {
  position: absolute;
  z-index: 1000;
  display: flex;
  align-items: center;
  gap: 4px;
  padding: 6px 8px;
  background: var(--toolbar-bg);
  border: 1px solid var(--border-color);
  border-radius: 8px;
  box-shadow: 0 2px 8px rgba(0, 0, 0, 0.15);
}

.table-toolbar-group {
  display: flex;
  gap: 2px;
}

.table-toolbar-btn {
  padding: 4px 8px;
  font-size: 11px;
  background: var(--btn-bg);
  border: 1px solid var(--border-color);
  border-radius: 4px;
  cursor: pointer;
  transition: all 0.15s;
  white-space: nowrap;
  color: var(--text-primary);
}

.table-toolbar-btn:hover {
  background: var(--btn-hover-bg);
  border-color: var(--primary-color);
}

.table-toolbar-btn.danger:hover {
  background: var(--danger-bg);
  border-color: var(--danger-color);
  color: var(--danger-color);
}

.table-toolbar-divider {
  width: 1px;
  height: 20px;
  background: var(--border-color);
  margin: 0 4px;
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
  overflow: auto;
  position: relative;
}

.milkdown-editor {
  flex: 1;
  min-width: 0;
  overflow-y: auto;
}

.milkdown-editor.is-loading {
  opacity: 0.6;
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
  cursor: pointer;
  transition: box-shadow var(--transition-fast, 0.2s), transform var(--transition-fast, 0.2s);
  position: relative;
}

.milkdown-editor :deep(table:hover) {
  box-shadow: 0 0 0 2px var(--primary-color, #007aff);
  transform: translateY(-1px);
}

/* Decorative tooltip - accessibility note: screen readers will announce the table content,
   and keyboard users can interact with table cells directly. The double-click action
   provides an enhanced editing experience but is not the only way to edit tables. */
.milkdown-editor :deep(table::after) {
  content: '✏️ Double-click to edit';
  position: absolute;
  top: -24px;
  right: 0;
  font-size: 11px;
  color: var(--primary-color, #007aff);
  background: var(--table-header-bg, #f5f5f7);
  padding: 2px 8px;
  border-radius: 4px;
  opacity: 0;
  transition: opacity var(--transition-fast, 0.2s);
  pointer-events: none;
}

.milkdown-editor :deep(table:hover::after) {
  opacity: 1;
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

/* Diagram preview styles for WYSIWYG mode */
.milkdown-editor :deep(.diagram-preview) {
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

.milkdown-editor :deep(.diagram-preview .diagram-image) {
  display: block;
  max-width: 100%;
  height: auto;
  margin: 0 auto;
}

/* Diagram menu styles */
.diagram-menu-container {
  position: relative;
}

.diagram-btn {
  min-width: 100px;
}

.diagram-dropdown {
  position: absolute;
  top: 100%;
  left: 0;
  margin-top: 4px;
  background: var(--toolbar-bg);
  border: 1px solid var(--border-color);
  border-radius: 8px;
  box-shadow: 0 4px 12px rgba(0, 0, 0, 0.15);
  z-index: 1000;
  min-width: 200px;
  max-height: 400px;
  overflow-y: auto;
}

.diagram-section {
  padding: 8px;
}

.diagram-section-title {
  font-size: 11px;
  font-weight: 600;
  color: var(--text-secondary);
  padding: 4px 8px;
  text-transform: uppercase;
  letter-spacing: 0.5px;
}

.diagram-section-divider {
  height: 1px;
  background: var(--border-color);
  margin: 4px 0;
}

.diagram-option {
  display: block;
  width: 100%;
  padding: 8px 12px;
  background: transparent;
  border: none;
  text-align: left;
  cursor: pointer;
  color: var(--text-primary);
  font-size: 13px;
  border-radius: 4px;
  transition: background 0.15s;
}

.diagram-option:hover {
  background: var(--btn-hover-bg);
}

/* Reminder Card styles for WYSIWYG mode */
.milkdown-editor :deep(.reminder-card-wysiwyg) {
  margin: 16px 0;
  padding: 0;
  background: var(--reminder-bg);
  border-radius: 12px;
  border: 1px solid var(--reminder-border);
  overflow: hidden;
  box-shadow: 0 2px 8px rgba(0, 0, 0, 0.08);
  transition: transform 0.2s, box-shadow 0.2s;
}

.milkdown-editor :deep(.reminder-card-wysiwyg:hover) {
  transform: translateY(-2px);
  box-shadow: 0 4px 16px rgba(0, 0, 0, 0.12);
}

.milkdown-editor :deep(.reminder-card-header) {
  display: flex;
  align-items: center;
  gap: 8px;
  padding: 12px 16px;
  background: var(--reminder-header-bg);
  border-bottom: 1px solid var(--reminder-border);
}

.milkdown-editor :deep(.reminder-icon) {
  font-size: 1.25rem;
}

.milkdown-editor :deep(.reminder-title) {
  flex: 1;
  font-weight: 600;
  font-size: 1rem;
  color: var(--text-primary);
}

.milkdown-editor :deep(.reminder-badge) {
  padding: 4px 10px;
  border-radius: 12px;
  font-size: 0.75rem;
  font-weight: 500;
  text-transform: uppercase;
  letter-spacing: 0.5px;
}

.milkdown-editor :deep(.reminder-badge.upcoming) {
  background: var(--badge-upcoming-bg);
  color: var(--badge-upcoming-color);
}

.milkdown-editor :deep(.reminder-badge.past) {
  background: var(--badge-past-bg);
  color: var(--badge-past-color);
}

.milkdown-editor :deep(.reminder-card-body) {
  padding: 12px 16px;
  display: flex;
  flex-direction: column;
  gap: 8px;
}

.milkdown-editor :deep(.reminder-meta-item) {
  display: flex;
  align-items: center;
  gap: 8px;
  font-size: 0.9rem;
}

.milkdown-editor :deep(.reminder-meta-icon) {
  width: 20px;
  text-align: center;
}

.milkdown-editor :deep(.reminder-meta-label) {
  color: var(--text-secondary);
  min-width: 80px;
}

.milkdown-editor :deep(.reminder-meta-value) {
  font-weight: 500;
  color: var(--text-primary);
}

.milkdown-editor :deep(.reminder-meta-value.past) {
  color: var(--text-secondary);
  text-decoration: line-through;
}

.milkdown-editor :deep(.reminder-description) {
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
    --danger-color: #ff453a;
    --danger-bg: rgba(255, 69, 58, 0.2);
    --transition-fast: 0.2s;
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
    --danger-color: #dc2626;
    --danger-bg: #fee2e2;
    --transition-fast: 0.2s;
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
