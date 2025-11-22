<script setup lang="ts">
import { ref } from 'vue';
import { useDocumentImport, type ImportResult } from '../composables/useDocumentImport';

const emit = defineEmits<{
  close: [];
  import: [result: ImportResult];
}>();

const { importDocument } = useDocumentImport();
const importing = ref(false);
const error = ref('');
const preview = ref<ImportResult | null>(null);

async function handleFileSelect(event: Event) {
  const target = event.target as HTMLInputElement;
  const file = target.files?.[0];
  
  if (!file) return;
  
  importing.value = true;
  error.value = '';
  preview.value = null;
  
  try {
    const result = await importDocument(file);
    
    if (result.error) {
      error.value = result.error;
    } else {
      preview.value = result;
    }
  } catch (e) {
    error.value = String(e);
  } finally {
    importing.value = false;
  }
}

function handleImport() {
  if (preview.value) {
    emit('import', preview.value);
  }
}

function handleCancel() {
  emit('close');
}
</script>

<template>
  <div class="modal-overlay" @click.self="handleCancel">
    <div class="modal-content">
      <div class="modal-header">
        <h2>Import Document</h2>
        <button class="close-btn" @click="handleCancel">&times;</button>
      </div>
      
      <div class="modal-body">
        <div v-if="!preview" class="upload-section">
          <p class="help-text">Select a Word document to import (.docx, .doc)</p>
          
          <label class="file-input-label">
            <input 
              type="file" 
              accept=".doc,.docx"
              @change="handleFileSelect"
              :disabled="importing"
              class="file-input"
            />
            <span class="file-btn">
              {{ importing ? 'Processing...' : 'Choose File' }}
            </span>
          </label>
          
          <p v-if="error" class="error-text">{{ error }}</p>
          
          <div v-if="importing" class="loading">
            <div class="spinner"></div>
            <p>Importing document...</p>
          </div>
        </div>
        
        <div v-else class="preview-section">
          <h3>Preview</h3>
          <div class="preview-info">
            <label>
              <strong>Title:</strong>
              <input 
                v-model="preview.title" 
                class="title-input"
                placeholder="Page title"
              />
            </label>
          </div>
          
          <div class="preview-content">
            <label><strong>Content:</strong></label>
            <textarea 
              v-model="preview.content" 
              class="content-textarea"
              placeholder="Document content will appear here"
              rows="10"
            ></textarea>
          </div>
          
          <p class="preview-help">
            You can edit the title and content before importing
          </p>
        </div>
      </div>
      
      <div class="modal-footer">
        <button class="btn secondary" @click="handleCancel">Cancel</button>
        <button 
          v-if="preview" 
          class="btn primary" 
          @click="handleImport"
          :disabled="!preview.title.trim() || !preview.content.trim()"
        >
          Import as New Page
        </button>
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
  max-width: 700px;
  max-height: 90vh;
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
  overflow-y: auto;
  flex: 1;
}

.upload-section {
  text-align: center;
}

.help-text {
  color: var(--text-secondary);
  margin-bottom: 20px;
}

.file-input {
  display: none;
}

.file-input-label {
  display: inline-block;
  cursor: pointer;
}

.file-btn {
  display: inline-block;
  padding: 10px 24px;
  background: var(--primary-color);
  color: white;
  border-radius: 4px;
  cursor: pointer;
  transition: background 0.2s;
}

.file-btn:hover {
  background: var(--primary-hover);
}

.file-input:disabled + .file-btn {
  opacity: 0.5;
  cursor: not-allowed;
}

.error-text {
  color: var(--error-color);
  margin-top: 12px;
}

.loading {
  margin-top: 24px;
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: 12px;
}

.spinner {
  width: 40px;
  height: 40px;
  border: 4px solid var(--border-color);
  border-top-color: var(--primary-color);
  border-radius: 50%;
  animation: spin 1s linear infinite;
}

@keyframes spin {
  to { transform: rotate(360deg); }
}

.preview-section {
  display: flex;
  flex-direction: column;
  gap: 16px;
}

.preview-section h3 {
  margin: 0 0 8px 0;
  color: var(--text-color);
}

.preview-info label {
  display: flex;
  flex-direction: column;
  gap: 4px;
}

.title-input {
  padding: 8px 12px;
  border: 1px solid var(--border-color);
  border-radius: 4px;
  background: var(--input-bg);
  color: var(--text-color);
  font-size: 14px;
}

.content-textarea {
  width: 100%;
  padding: 12px;
  border: 1px solid var(--border-color);
  border-radius: 4px;
  background: var(--input-bg);
  color: var(--text-color);
  font-size: 14px;
  font-family: 'Monaco', 'Menlo', monospace;
  resize: vertical;
  min-height: 200px;
}

.preview-help {
  color: var(--text-secondary);
  font-size: 13px;
  margin: 0;
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

.btn.primary:hover:not(:disabled) {
  background: var(--primary-hover);
}

.btn.primary:disabled {
  opacity: 0.5;
  cursor: not-allowed;
}

.btn.secondary {
  background: var(--border-color);
  color: var(--text-color);
}

.btn.secondary:hover {
  background: var(--hover-bg);
}
</style>
