<script setup lang="ts">
import { ref } from 'vue';
import { useWikiExport } from '../composables/useWikiExport';

interface Props {
  title: string;
  content: string;
}

const props = defineProps<Props>();
const emit = defineEmits<{
  close: [];
}>();

const { isExporting, exportError, exportPage } = useWikiExport();
const selectedFormat = ref<'pdf' | 'docx'>('pdf');

async function handleExport() {
  try {
    await exportPage({
      title: props.title,
      content: props.content,
      format: selectedFormat.value,
    });
    
    // Close modal after successful export
    if (!exportError.value) {
      emit('close');
    }
  } catch (error) {
    console.error('Export failed:', error);
  }
}

function handleClose() {
  if (!isExporting.value) {
    emit('close');
  }
}
</script>

<template>
  <div class="modal-overlay" @click="handleClose">
    <div class="modal-content" @click.stop>
      <div class="modal-header">
        <h2>📤 Export Wiki Page</h2>
        <button class="close-btn" @click="handleClose" :disabled="isExporting">✕</button>
      </div>

      <div class="modal-body">
        <div class="export-info">
          <p><strong>Page:</strong> {{ title }}</p>
          <p class="note">
            Export will include all images, PlantUML diagrams, and Mermaid diagrams rendered as images.
          </p>
        </div>

        <div class="format-selector">
          <label class="format-option">
            <input
              type="radio"
              v-model="selectedFormat"
              value="pdf"
              :disabled="isExporting"
            />
            <div class="format-card">
              <div class="format-icon">📄</div>
              <div class="format-info">
                <div class="format-name">PDF Document</div>
                <div class="format-desc">Portable Document Format, best for viewing and printing</div>
              </div>
            </div>
          </label>

          <label class="format-option">
            <input
              type="radio"
              v-model="selectedFormat"
              value="docx"
              :disabled="isExporting"
            />
            <div class="format-card">
              <div class="format-icon">📝</div>
              <div class="format-info">
                <div class="format-name">Word Document</div>
                <div class="format-desc">Microsoft Word format (.docx), best for editing</div>
              </div>
            </div>
          </label>
        </div>

        <div v-if="exportError" class="error-message">
          ⚠️ {{ exportError }}
        </div>

        <div v-if="isExporting" class="exporting-status">
          <div class="spinner"></div>
          <p>Exporting {{ selectedFormat.toUpperCase() }}... This may take a moment.</p>
        </div>
      </div>

      <div class="modal-footer">
        <button class="btn btn-secondary" @click="handleClose" :disabled="isExporting">
          Cancel
        </button>
        <button class="btn btn-primary" @click="handleExport" :disabled="isExporting">
          {{ isExporting ? 'Exporting...' : 'Export' }}
        </button>
      </div>
    </div>
  </div>
</template>

<style scoped>
.modal-overlay {
  position: fixed;
  inset: 0;
  background: rgba(0, 0, 0, 0.5);
  display: flex;
  align-items: center;
  justify-content: center;
  z-index: 2000;
  backdrop-filter: blur(4px);
}

.modal-content {
  background: var(--modal-bg);
  border-radius: 12px;
  box-shadow: 0 8px 32px rgba(0, 0, 0, 0.2);
  max-width: 600px;
  width: 90%;
  max-height: 90vh;
  overflow: hidden;
  display: flex;
  flex-direction: column;
}

.modal-header {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 20px 24px;
  border-bottom: 1px solid var(--border-color);
}

.modal-header h2 {
  margin: 0;
  font-size: 20px;
  font-weight: 600;
  color: var(--text-primary);
}

.close-btn {
  background: transparent;
  border: none;
  font-size: 24px;
  cursor: pointer;
  color: var(--text-secondary);
  padding: 4px 8px;
  border-radius: 4px;
  transition: all 0.2s;
}

.close-btn:hover:not(:disabled) {
  background: var(--hover-bg);
  color: var(--text-primary);
}

.close-btn:disabled {
  opacity: 0.5;
  cursor: not-allowed;
}

.modal-body {
  padding: 24px;
  overflow-y: auto;
  flex: 1;
}

.export-info {
  margin-bottom: 24px;
}

.export-info p {
  margin: 8px 0;
  color: var(--text-primary);
}

.export-info strong {
  color: var(--text-primary);
}

.note {
  font-size: 14px;
  color: var(--text-secondary);
  background: var(--note-bg);
  padding: 12px;
  border-radius: 8px;
  border-left: 3px solid var(--primary-color);
}

.format-selector {
  display: flex;
  flex-direction: column;
  gap: 12px;
  margin-bottom: 20px;
}

.format-option {
  display: flex;
  cursor: pointer;
}

.format-option input[type="radio"] {
  margin-right: 12px;
  cursor: pointer;
  flex-shrink: 0;
}

.format-card {
  flex: 1;
  display: flex;
  gap: 16px;
  padding: 16px;
  border: 2px solid var(--border-color);
  border-radius: 8px;
  background: var(--card-bg);
  transition: all 0.2s;
}

.format-option:hover .format-card {
  border-color: var(--primary-color);
  background: var(--card-hover-bg);
}

.format-option input[type="radio"]:checked ~ .format-card {
  border-color: var(--primary-color);
  background: var(--primary-light);
}

.format-option input[type="radio"]:disabled ~ .format-card {
  opacity: 0.6;
  cursor: not-allowed;
}

.format-icon {
  font-size: 32px;
  line-height: 1;
}

.format-info {
  flex: 1;
}

.format-name {
  font-weight: 600;
  font-size: 16px;
  color: var(--text-primary);
  margin-bottom: 4px;
}

.format-desc {
  font-size: 14px;
  color: var(--text-secondary);
}

.error-message {
  padding: 12px;
  background: var(--error-bg);
  color: var(--error-color);
  border-radius: 8px;
  border-left: 3px solid var(--error-color);
  margin-bottom: 16px;
}

.exporting-status {
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: 12px;
  padding: 20px;
  background: var(--status-bg);
  border-radius: 8px;
  margin-bottom: 16px;
}

.spinner {
  width: 32px;
  height: 32px;
  border: 3px solid var(--border-color);
  border-top-color: var(--primary-color);
  border-radius: 50%;
  animation: spin 1s linear infinite;
}

@keyframes spin {
  to {
    transform: rotate(360deg);
  }
}

.exporting-status p {
  margin: 0;
  color: var(--text-secondary);
  font-size: 14px;
}

.modal-footer {
  display: flex;
  justify-content: flex-end;
  gap: 12px;
  padding: 16px 24px;
  border-top: 1px solid var(--border-color);
}

.btn {
  padding: 10px 20px;
  border-radius: 8px;
  border: none;
  font-weight: 600;
  cursor: pointer;
  transition: all 0.2s;
  font-size: 14px;
}

.btn:disabled {
  opacity: 0.6;
  cursor: not-allowed;
}

.btn-secondary {
  background: var(--btn-secondary-bg);
  color: var(--text-primary);
  border: 1px solid var(--border-color);
}

.btn-secondary:hover:not(:disabled) {
  background: var(--btn-secondary-hover);
}

.btn-primary {
  background: var(--primary-color);
  color: white;
}

.btn-primary:hover:not(:disabled) {
  transform: translateY(-1px);
  box-shadow: 0 4px 12px rgba(37, 99, 235, 0.3);
}

/* Light mode */
@media (prefers-color-scheme: light) {
  .modal-content {
    --modal-bg: #ffffff;
    --border-color: #e5e5ea;
    --text-primary: #1d1d1f;
    --text-secondary: #86868b;
    --primary-color: #2563eb;
    --primary-light: rgba(37, 99, 235, 0.1);
    --hover-bg: #f5f5f7;
    --note-bg: #eef2ff;
    --card-bg: #fafafa;
    --card-hover-bg: #f5f5f7;
    --error-bg: rgba(255, 59, 48, 0.1);
    --error-color: #ff3b30;
    --status-bg: #f5f5f7;
    --btn-secondary-bg: #ffffff;
    --btn-secondary-hover: #f5f5f7;
  }
}

/* Dark mode */
@media (prefers-color-scheme: dark) {
  .modal-content {
    --modal-bg: #1c1c1e;
    --border-color: #38383a;
    --text-primary: #f5f5f7;
    --text-secondary: #98989d;
    --primary-color: #38bdf8;
    --primary-light: rgba(56, 189, 248, 0.15);
    --hover-bg: #2c2c2e;
    --note-bg: rgba(56, 189, 248, 0.1);
    --card-bg: #2c2c2e;
    --card-hover-bg: #3a3a3c;
    --error-bg: rgba(255, 69, 58, 0.1);
    --error-color: #ff453a;
    --status-bg: #2c2c2e;
    --btn-secondary-bg: #2c2c2e;
    --btn-secondary-hover: #3a3a3c;
  }
}
</style>
