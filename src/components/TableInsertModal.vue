<script setup lang="ts">
import { ref, computed, watch } from 'vue';
import TableComponent from './TableComponent.vue';

const props = defineProps<{
  initialMarkdown?: string;
}>();

const emit = defineEmits<{
  close: [];
  insert: [markdown: string];
}>();

// Determine if we're editing an existing table
const isEditing = computed(() => Boolean(props.initialMarkdown && props.initialMarkdown.trim()));

// Edit mode: 'quick' for quick setup, 'visual' for visual WYSIWYG editing
const editMode = ref<'quick' | 'visual'>('visual');

// Quick mode settings
const rows = ref(3);
const cols = ref(3);
const hasHeader = ref(true);

// Visual mode - table markdown content
const tableMarkdown = ref('');
const tableComponentRef = ref<InstanceType<typeof TableComponent> | null>(null);

// Generate initial table markdown
function generateInitialTable(): string {
  const markdown: string[] = [];
  
  // Generate header row
  const headerCells = Array(cols.value).fill('Header').map((h, i) => `${h} ${i + 1}`);
  markdown.push('| ' + headerCells.join(' | ') + ' |');
  
  // Generate separator
  const separator = Array(cols.value).fill('---');
  markdown.push('| ' + separator.join(' | ') + ' |');
  
  // Generate data rows
  const dataRows = hasHeader.value ? rows.value - 1 : rows.value;
  for (let i = 0; i < dataRows; i++) {
    const cells = Array(cols.value).fill('');
    markdown.push('| ' + cells.join(' | ') + ' |');
  }
  
  return markdown.join('\n');
}

// Initialize table markdown
function initializeTable() {
  if (props.initialMarkdown) {
    tableMarkdown.value = props.initialMarkdown;
  } else {
    tableMarkdown.value = generateInitialTable();
  }
}

// Watch for quick mode changes to regenerate table (only for new tables, not when editing)
watch([rows, cols, hasHeader], () => {
  if (editMode.value === 'quick' && !isEditing.value) {
    tableMarkdown.value = generateInitialTable();
  }
});

// Switch to visual mode with current settings
function switchToVisual() {
  // Only generate new table if not editing an existing one
  if (!isEditing.value) {
    tableMarkdown.value = generateInitialTable();
  }
  editMode.value = 'visual';
}

// Insert the table
function handleInsert() {
  emit('insert', tableMarkdown.value);
}

function handleCancel() {
  emit('close');
}

// Initialize on mount
initializeTable();
</script>

<template>
  <div class="modal-overlay" @click.self="handleCancel">
    <div class="modal-content" :class="{ 'wide-modal': editMode === 'visual' }">
      <div class="modal-header">
        <h2>{{ isEditing ? 'Edit Table' : (editMode === 'quick' ? 'Insert Table' : 'Edit Table') }}</h2>
        <div class="mode-toggle" v-if="!isEditing">
          <button 
            :class="['mode-btn', { active: editMode === 'quick' }]"
            @click="editMode = 'quick'"
            title="Quick Setup"
          >
            ⚡ Quick
          </button>
          <button 
            :class="['mode-btn', { active: editMode === 'visual' }]"
            @click="switchToVisual"
            title="Visual Editor - Add/Remove Rows and Columns"
          >
            ✏️ Visual
          </button>
        </div>
        <button class="close-btn" @click="handleCancel">&times;</button>
      </div>
      
      <div class="modal-body">
        <!-- Quick Mode (only for new tables) -->
        <template v-if="editMode === 'quick' && !isEditing">
          <div class="form-group">
            <label>
              Rows:
              <input type="number" v-model.number="rows" min="1" max="20" />
            </label>
          </div>
          
          <div class="form-group">
            <label>
              Columns:
              <input type="number" v-model.number="cols" min="1" max="10" />
            </label>
          </div>
          
          <div class="form-group">
            <label class="checkbox-label">
              <input type="checkbox" v-model="hasHeader" />
              Include header row
            </label>
          </div>
          
          <div class="preview">
            <h3>Preview</h3>
            <pre class="table-preview">{{ generateInitialTable() }}</pre>
          </div>
        </template>

        <!-- Visual Mode - Interactive Table Editor -->
        <template v-else>
          <div class="visual-editor-info">
            <p>💡 Click cells to edit • Use Tab to navigate • Click + buttons to add rows/columns</p>
          </div>
          <TableComponent 
            ref="tableComponentRef"
            v-model="tableMarkdown"
          />
        </template>
      </div>
      
      <div class="modal-footer">
        <button class="btn secondary" @click="handleCancel">Cancel</button>
        <button class="btn primary" @click="handleInsert">{{ isEditing ? 'Update Table' : 'Insert Table' }}</button>
      </div>
    </div>
  </div>
</template>

<style scoped>
.modal-overlay {
  position: fixed;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
  background: rgba(0, 0, 0, 0.5);
  display: flex;
  align-items: center;
  justify-content: center;
  z-index: 1000;
}

.modal-content {
  background: var(--page-bg);
  border-radius: 8px;
  width: 90%;
  max-width: 600px;
  max-height: 90vh;
  display: flex;
  flex-direction: column;
  box-shadow: 0 4px 20px rgba(0, 0, 0, 0.3);
  overflow: hidden;
}

.modal-content.wide-modal {
  max-width: 900px;
}

.modal-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 16px 20px;
  border-bottom: 1px solid var(--border-color);
  gap: 12px;
}

.modal-header h2 {
  margin: 0;
  font-size: 20px;
  color: var(--text-color);
  flex-shrink: 0;
}

.mode-toggle {
  display: flex;
  gap: 4px;
  flex: 1;
  justify-content: center;
}

.mode-btn {
  padding: 6px 12px;
  font-size: 13px;
  background: var(--btn-bg, #ffffff);
  border: 1px solid var(--border-color);
  border-radius: 6px;
  cursor: pointer;
  transition: all 0.15s;
  color: var(--text-color);
}

.mode-btn:hover {
  background: var(--hover-bg);
}

.mode-btn.active {
  background: var(--primary-color);
  border-color: var(--primary-color);
  color: white;
}

.close-btn {
  background: none;
  border: none;
  font-size: 28px;
  cursor: pointer;
  color: var(--text-secondary);
  padding: 0;
  width: 32px;
  height: 32px;
  display: flex;
  align-items: center;
  justify-content: center;
  border-radius: 4px;
  flex-shrink: 0;
}

.close-btn:hover {
  background: var(--hover-bg);
  color: var(--text-color);
}

.modal-body {
  padding: 20px;
  display: flex;
  flex-direction: column;
  gap: 16px;
  overflow-y: auto;
  flex: 1;
}

.form-group {
  display: flex;
  flex-direction: column;
  gap: 4px;
}

.form-group label {
  color: var(--text-color);
  font-size: 14px;
  display: flex;
  align-items: center;
  gap: 8px;
}

.form-group input[type="number"] {
  padding: 8px 12px;
  border: 1px solid var(--border-color);
  border-radius: 4px;
  background: var(--input-bg);
  color: var(--text-color);
  font-size: 14px;
  max-width: 100px;
}

.checkbox-label {
  cursor: pointer;
}

.checkbox-label input[type="checkbox"] {
  cursor: pointer;
}

.preview {
  margin-top: 8px;
}

.preview h3 {
  margin: 0 0 8px 0;
  font-size: 14px;
  color: var(--text-secondary);
}

.table-preview {
  padding: 12px;
  background: var(--input-bg);
  border: 1px solid var(--border-color);
  border-radius: 4px;
  color: var(--text-color);
  font-size: 12px;
  font-family: 'Monaco', 'Menlo', monospace;
  overflow-x: auto;
  white-space: pre;
}

.visual-editor-info {
  padding: 8px 12px;
  background: var(--info-bg, rgba(0, 122, 255, 0.1));
  border-radius: 6px;
  font-size: 13px;
  color: var(--text-secondary);
}

.visual-editor-info p {
  margin: 0;
}

.modal-footer {
  padding: 16px 20px;
  border-top: 1px solid var(--border-color);
  display: flex;
  justify-content: flex-end;
  gap: 12px;
  flex-shrink: 0;
}

.btn {
  padding: 8px 16px;
  border: none;
  border-radius: 4px;
  cursor: pointer;
  font-size: 14px;
  transition: all 0.2s;
}

.btn.primary {
  background: var(--primary-color);
  color: white;
}

.btn.primary:hover {
  background: var(--primary-hover);
}

.btn.secondary {
  background: var(--border-color);
  color: var(--text-color);
}

.btn.secondary:hover {
  background: var(--hover-bg);
}

/* Dark mode variables */
@media (prefers-color-scheme: dark) {
  .modal-content {
    --page-bg: #1c1c1e;
    --border-color: #38383a;
    --text-color: #f5f5f7;
    --text-secondary: #98989d;
    --input-bg: #2c2c2e;
    --hover-bg: #3a3a3c;
    --btn-bg: #3a3a3c;
    --primary-color: #0a84ff;
    --primary-hover: #0066cc;
    --info-bg: rgba(10, 132, 255, 0.15);
  }
}

/* Light mode variables */
@media (prefers-color-scheme: light) {
  .modal-content {
    --page-bg: #ffffff;
    --border-color: #e5e5ea;
    --text-color: #1d1d1f;
    --text-secondary: #86868b;
    --input-bg: #f5f5f7;
    --hover-bg: #e8e8ed;
    --btn-bg: #ffffff;
    --primary-color: #007aff;
    --primary-hover: #0066cc;
    --info-bg: rgba(0, 122, 255, 0.1);
  }
}
</style>
