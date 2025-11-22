<script setup lang="ts">
import { ref } from 'vue';

const emit = defineEmits<{
  close: [];
  insert: [markdown: string];
}>();

const rows = ref(3);
const cols = ref(3);
const hasHeader = ref(true);

function generateTable() {
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

function handleInsert() {
  const table = generateTable();
  emit('insert', table);
}

function handleCancel() {
  emit('close');
}
</script>

<template>
  <div class="modal-overlay" @click.self="handleCancel">
    <div class="modal-content">
      <div class="modal-header">
        <h2>Insert Table</h2>
        <button class="close-btn" @click="handleCancel">&times;</button>
      </div>
      
      <div class="modal-body">
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
          <pre class="table-preview">{{ generateTable() }}</pre>
        </div>
      </div>
      
      <div class="modal-footer">
        <button class="btn secondary" @click="handleCancel">Cancel</button>
        <button class="btn primary" @click="handleInsert">Insert Table</button>
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
  display: flex;
  flex-direction: column;
  box-shadow: 0 4px 20px rgba(0, 0, 0, 0.3);
}

.modal-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 16px 20px;
  border-bottom: 1px solid var(--border-color);
}

.modal-header h2 {
  margin: 0;
  font-size: 20px;
  color: var(--text-color);
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

.modal-footer {
  padding: 16px 20px;
  border-top: 1px solid var(--border-color);
  display: flex;
  justify-content: flex-end;
  gap: 12px;
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
</style>
