<script setup lang="ts">
import { ref, computed, watch, onMounted, nextTick } from 'vue';

const props = defineProps<{
  modelValue: string;
}>();

const emit = defineEmits<{
  'update:modelValue': [value: string];
  'close': [];
}>();

// Table data as a 2D array
const tableData = ref<string[][]>([]);
const hasHeader = ref(true);
const focusedCell = ref<{ row: number; col: number } | null>(null);
const showToolbar = ref(true);

// Parse markdown table to 2D array
function parseMarkdownTable(markdown: string): string[][] {
  const lines = markdown.trim().split('\n').filter(line => line.trim());
  if (lines.length < 2) {
    // Return a default 2x2 table
    return [
      ['Header 1', 'Header 2'],
      ['', ''],
    ];
  }

  const data: string[][] = [];
  let separatorIndex = -1;

  for (let i = 0; i < lines.length; i++) {
    const line = lines[i].trim();
    // Skip separator line (contains ---)
    if (/^\|[\s\-:|]+\|$/.test(line) || /^[\s\-:|]+$/.test(line)) {
      separatorIndex = i;
      continue;
    }
    
    // Parse cells from pipe-separated line
    const cells = line
      .split('|')
      .map(cell => cell.trim())
      .filter((_, index, arr) => {
        // Remove empty first and last elements from split
        return index > 0 && index < arr.length - 1;
      });
    
    if (cells.length > 0) {
      data.push(cells);
    }
  }

  // Ensure all rows have the same number of columns
  const maxCols = Math.max(...data.map(row => row.length), 1);
  return data.map(row => {
    while (row.length < maxCols) {
      row.push('');
    }
    return row;
  });
}

// Convert 2D array back to markdown
function toMarkdown(data: string[][]): string {
  if (data.length === 0) return '';

  const colCount = data[0]?.length || 2;
  const lines: string[] = [];

  // Header row
  const header = data[0] || Array(colCount).fill('Header');
  lines.push('| ' + header.join(' | ') + ' |');

  // Separator row
  lines.push('| ' + Array(colCount).fill('---').join(' | ') + ' |');

  // Data rows
  for (let i = 1; i < data.length; i++) {
    lines.push('| ' + data[i].join(' | ') + ' |');
  }

  return lines.join('\n');
}

// Initialize table from props
function initTable() {
  if (props.modelValue && props.modelValue.trim()) {
    tableData.value = parseMarkdownTable(props.modelValue);
  } else {
    // Default empty table
    tableData.value = [
      ['Header 1', 'Header 2', 'Header 3'],
      ['', '', ''],
      ['', '', ''],
    ];
  }
}

// Computed properties
const rowCount = computed(() => tableData.value.length);
const colCount = computed(() => tableData.value[0]?.length || 0);

// Table operations
function addRow(position: 'above' | 'below' = 'below') {
  const newRow = Array(colCount.value).fill('');
  if (focusedCell.value) {
    const insertIndex = position === 'above' ? focusedCell.value.row : focusedCell.value.row + 1;
    tableData.value.splice(insertIndex, 0, newRow);
    if (position === 'above') {
      focusedCell.value.row++;
    }
  } else {
    tableData.value.push(newRow);
  }
  emitUpdate();
}

function addRowAtEnd() {
  const newRow = Array(colCount.value).fill('');
  tableData.value.push(newRow);
  emitUpdate();
}

// Helper function to get default cell value
function getDefaultCellValue(rowIndex: number): string {
  return rowIndex === 0 ? `Header ${colCount.value + 1}` : '';
}

function addColumn(position: 'left' | 'right' = 'right') {
  tableData.value.forEach((row, rowIndex) => {
    const insertIndex = focusedCell.value
      ? (position === 'left' ? focusedCell.value.col : focusedCell.value.col + 1)
      : row.length;
    row.splice(insertIndex, 0, getDefaultCellValue(rowIndex));
  });
  if (focusedCell.value && position === 'left') {
    focusedCell.value.col++;
  }
  emitUpdate();
}

function addColumnAtEnd() {
  tableData.value.forEach((row, rowIndex) => {
    row.push(getDefaultCellValue(rowIndex));
  });
  emitUpdate();
}

function deleteRow(index?: number) {
  const rowIndex = index ?? focusedCell.value?.row;
  if (rowIndex === undefined || tableData.value.length <= 1) return;
  
  tableData.value.splice(rowIndex, 1);
  if (focusedCell.value && focusedCell.value.row >= tableData.value.length) {
    focusedCell.value.row = tableData.value.length - 1;
  }
  emitUpdate();
}

function deleteColumn(index?: number) {
  const colIndex = index ?? focusedCell.value?.col;
  if (colIndex === undefined || colCount.value <= 1) return;
  
  tableData.value.forEach(row => {
    row.splice(colIndex, 1);
  });
  // Adjust focused cell after column is removed
  const newColCount = tableData.value[0]?.length || 0;
  if (focusedCell.value && focusedCell.value.col >= newColCount) {
    focusedCell.value.col = Math.max(0, newColCount - 1);
  }
  emitUpdate();
}

function updateCell(row: number, col: number, value: string) {
  if (tableData.value[row]) {
    tableData.value[row][col] = value;
    emitUpdate();
  }
}

function handleCellFocus(row: number, col: number) {
  focusedCell.value = { row, col };
}

function handleCellBlur() {
  // Keep focus info for toolbar actions
}

function emitUpdate() {
  emit('update:modelValue', toMarkdown(tableData.value));
}

function handleKeydown(event: KeyboardEvent, row: number, col: number) {
  if (event.key === 'Tab') {
    event.preventDefault();
    if (event.shiftKey) {
      // Move to previous cell
      if (col > 0) {
        focusCell(row, col - 1);
      } else if (row > 0) {
        focusCell(row - 1, colCount.value - 1);
      }
    } else {
      // Move to next cell
      if (col < colCount.value - 1) {
        focusCell(row, col + 1);
      } else if (row < rowCount.value - 1) {
        focusCell(row + 1, 0);
      } else {
        // At last cell, add a new row
        addRowAtEnd();
        nextTick(() => focusCell(rowCount.value - 1, 0));
      }
    }
  } else if (event.key === 'Enter' && !event.shiftKey) {
    event.preventDefault();
    if (row < rowCount.value - 1) {
      focusCell(row + 1, col);
    } else {
      // At last row, add a new row
      addRowAtEnd();
      nextTick(() => focusCell(rowCount.value - 1, col));
    }
  } else if (event.key === 'ArrowUp' && row > 0) {
    event.preventDefault();
    focusCell(row - 1, col);
  } else if (event.key === 'ArrowDown' && row < rowCount.value - 1) {
    event.preventDefault();
    focusCell(row + 1, col);
  }
}

function focusCell(row: number, col: number) {
  const input = document.querySelector(
    `.table-cell[data-row="${row}"][data-col="${col}"] input`
  ) as HTMLInputElement;
  if (input) {
    input.focus();
  }
}

// Watch for external changes
watch(() => props.modelValue, (newVal) => {
  // Only re-parse if the external value is substantially different
  const currentMarkdown = toMarkdown(tableData.value);
  if (newVal !== currentMarkdown) {
    initTable();
  }
});

onMounted(() => {
  initTable();
});

// Expose methods for parent components
defineExpose({
  addRow,
  addColumn,
  deleteRow,
  deleteColumn,
  getMarkdown: () => toMarkdown(tableData.value),
});
</script>

<template>
  <div class="table-component">
    <!-- Toolbar -->
    <div v-if="showToolbar" class="table-toolbar">
      <div class="toolbar-group">
        <button 
          class="toolbar-btn" 
          @click="addRow('above')" 
          :disabled="!focusedCell"
          title="Insert row above"
        >
          ⬆️ Row Above
        </button>
        <button 
          class="toolbar-btn" 
          @click="addRow('below')" 
          title="Insert row below"
        >
          ⬇️ Row Below
        </button>
        <button 
          class="toolbar-btn" 
          @click="addColumn('left')" 
          :disabled="!focusedCell"
          title="Insert column left"
        >
          ⬅️ Col Left
        </button>
        <button 
          class="toolbar-btn" 
          @click="addColumn('right')" 
          title="Insert column right"
        >
          ➡️ Col Right
        </button>
      </div>
      <div class="toolbar-divider"></div>
      <div class="toolbar-group">
        <button 
          class="toolbar-btn danger" 
          @click="deleteRow()" 
          :disabled="rowCount <= 1 || !focusedCell"
          title="Delete current row"
        >
          🗑️ Row
        </button>
        <button 
          class="toolbar-btn danger" 
          @click="deleteColumn()" 
          :disabled="colCount <= 1 || !focusedCell"
          title="Delete current column"
        >
          🗑️ Col
        </button>
      </div>
      <div class="toolbar-spacer"></div>
      <div class="toolbar-info">
        {{ rowCount }} × {{ colCount }}
        <span v-if="focusedCell">&nbsp;| Cell ({{ focusedCell.row + 1 }}, {{ focusedCell.col + 1 }})</span>
      </div>
    </div>

    <!-- Table -->
    <div class="table-container">
      <table class="editable-table">
        <thead>
          <tr>
            <th class="row-header">#</th>
            <th 
              v-for="(_, colIndex) in tableData[0]" 
              :key="colIndex"
              class="column-header"
            >
              {{ colIndex + 1 }}
              <button 
                class="header-action" 
                @click="deleteColumn(colIndex)"
                :disabled="colCount <= 1"
                title="Delete column"
              >×</button>
            </th>
            <th class="add-column-header">
              <button class="add-btn" @click="addColumnAtEnd" title="Add column">+</button>
            </th>
          </tr>
        </thead>
        <tbody>
          <tr v-for="(row, rowIndex) in tableData" :key="rowIndex">
            <td class="row-header">
              {{ rowIndex + 1 }}
              <button 
                class="row-action" 
                @click="deleteRow(rowIndex)"
                :disabled="rowCount <= 1"
                title="Delete row"
              >×</button>
            </td>
            <td 
              v-for="(cell, colIndex) in row" 
              :key="colIndex"
              class="table-cell"
              :class="{ 
                'is-header': rowIndex === 0,
                'is-focused': focusedCell?.row === rowIndex && focusedCell?.col === colIndex 
              }"
              :data-row="rowIndex"
              :data-col="colIndex"
            >
              <input
                type="text"
                :value="cell"
                @input="(e) => updateCell(rowIndex, colIndex, (e.target as HTMLInputElement).value)"
                @focus="handleCellFocus(rowIndex, colIndex)"
                @blur="handleCellBlur"
                @keydown="(e) => handleKeydown(e, rowIndex, colIndex)"
                :placeholder="rowIndex === 0 ? 'Header' : ''"
              />
            </td>
            <td class="add-column-cell"></td>
          </tr>
          <tr class="add-row-row">
            <td :colspan="colCount + 2">
              <button class="add-row-btn" @click="addRowAtEnd" title="Add row">
                + Add Row
              </button>
            </td>
          </tr>
        </tbody>
      </table>
    </div>
  </div>
</template>

<style scoped>
.table-component {
  display: flex;
  flex-direction: column;
  background: var(--table-bg, #ffffff);
  border: 1px solid var(--border-color, #e5e5ea);
  border-radius: 8px;
  overflow: hidden;
}

.table-toolbar {
  display: flex;
  align-items: center;
  gap: 8px;
  padding: 8px 12px;
  background: var(--toolbar-bg, #f5f5f7);
  border-bottom: 1px solid var(--border-color, #e5e5ea);
  flex-wrap: wrap;
}

.toolbar-group {
  display: flex;
  gap: 4px;
}

.toolbar-btn {
  padding: 4px 8px;
  font-size: 12px;
  background: var(--btn-bg, #ffffff);
  border: 1px solid var(--border-color, #e5e5ea);
  border-radius: 4px;
  cursor: pointer;
  transition: all 0.15s;
  white-space: nowrap;
}

.toolbar-btn:hover:not(:disabled) {
  background: var(--btn-hover-bg, #e8e8ed);
  border-color: var(--primary-color, #007aff);
}

.toolbar-btn:disabled {
  opacity: 0.5;
  cursor: not-allowed;
}

.toolbar-btn.danger:hover:not(:disabled) {
  background: #fee2e2;
  border-color: #ef4444;
  color: #dc2626;
}

.toolbar-divider {
  width: 1px;
  height: 20px;
  background: var(--border-color, #e5e5ea);
}

.toolbar-spacer {
  flex: 1;
}

.toolbar-info {
  font-size: 11px;
  color: var(--text-secondary, #86868b);
}

.table-container {
  overflow: auto;
  max-height: 400px;
}

.editable-table {
  width: 100%;
  border-collapse: collapse;
  font-size: 14px;
}

.editable-table th,
.editable-table td {
  border: 1px solid var(--border-color, #e5e5ea);
  padding: 0;
}

.row-header,
.column-header {
  background: var(--header-bg, #f5f5f7);
  font-weight: 500;
  font-size: 11px;
  color: var(--text-secondary, #86868b);
  text-align: center;
  padding: 4px 8px;
  position: relative;
  min-width: 40px;
}

.row-header {
  width: 40px;
}

.header-action,
.row-action {
  position: absolute;
  right: 2px;
  top: 50%;
  transform: translateY(-50%);
  width: 16px;
  height: 16px;
  padding: 0;
  font-size: 12px;
  line-height: 1;
  background: transparent;
  border: none;
  color: var(--text-tertiary, #98989d);
  cursor: pointer;
  opacity: 0;
  transition: opacity 0.15s;
  border-radius: 2px;
}

.column-header:hover .header-action,
.row-header:hover .row-action {
  opacity: 1;
}

.header-action:hover,
.row-action:hover {
  background: #fee2e2;
  color: #dc2626;
}

.header-action:disabled,
.row-action:disabled {
  display: none;
}

.add-column-header,
.add-column-cell {
  width: 32px;
  background: transparent;
  border: none;
}

.add-btn {
  width: 24px;
  height: 24px;
  padding: 0;
  font-size: 16px;
  background: transparent;
  border: 1px dashed var(--border-color, #e5e5ea);
  border-radius: 4px;
  cursor: pointer;
  color: var(--text-tertiary, #98989d);
  transition: all 0.15s;
}

.add-btn:hover {
  background: var(--primary-light, rgba(0, 122, 255, 0.1));
  border-color: var(--primary-color, #007aff);
  color: var(--primary-color, #007aff);
}

.table-cell {
  min-width: 100px;
  position: relative;
}

.table-cell.is-header {
  background: var(--header-bg, #f5f5f7);
}

.table-cell.is-focused {
  outline: 2px solid var(--primary-color, #007aff);
  outline-offset: -2px;
}

.table-cell input {
  width: 100%;
  padding: 8px 10px;
  border: none;
  background: transparent;
  font-size: 14px;
  font-family: inherit;
  color: var(--text-primary, #1d1d1f);
}

.table-cell input:focus {
  outline: none;
}

.table-cell.is-header input {
  font-weight: 600;
}

.table-cell input::placeholder {
  color: var(--text-tertiary, #98989d);
}

.add-row-row td {
  background: transparent;
  border: none;
  padding: 4px;
}

.add-row-btn {
  width: 100%;
  padding: 6px;
  font-size: 12px;
  background: transparent;
  border: 1px dashed var(--border-color, #e5e5ea);
  border-radius: 4px;
  cursor: pointer;
  color: var(--text-tertiary, #98989d);
  transition: all 0.15s;
}

.add-row-btn:hover {
  background: var(--primary-light, rgba(0, 122, 255, 0.1));
  border-color: var(--primary-color, #007aff);
  color: var(--primary-color, #007aff);
}

/* Dark mode */
@media (prefers-color-scheme: dark) {
  .table-component {
    --table-bg: #1c1c1e;
    --toolbar-bg: #2c2c2e;
    --border-color: #38383a;
    --text-primary: #f5f5f7;
    --text-secondary: #98989d;
    --text-tertiary: #636366;
    --primary-color: #0a84ff;
    --primary-light: rgba(10, 132, 255, 0.15);
    --btn-bg: #3a3a3c;
    --btn-hover-bg: #48484a;
    --header-bg: #2c2c2e;
  }

  .toolbar-btn.danger:hover:not(:disabled) {
    background: rgba(255, 69, 58, 0.2);
    border-color: #ff453a;
    color: #ff453a;
  }

  .header-action:hover,
  .row-action:hover {
    background: rgba(255, 69, 58, 0.2);
    color: #ff453a;
  }
}

/* Light mode */
@media (prefers-color-scheme: light) {
  .table-component {
    --table-bg: #ffffff;
    --toolbar-bg: #f5f5f7;
    --border-color: #e5e5ea;
    --text-primary: #1d1d1f;
    --text-secondary: #86868b;
    --text-tertiary: #98989d;
    --primary-color: #007aff;
    --primary-light: rgba(0, 122, 255, 0.1);
    --btn-bg: #ffffff;
    --btn-hover-bg: #e8e8ed;
    --header-bg: #f5f5f7;
  }
}
</style>
