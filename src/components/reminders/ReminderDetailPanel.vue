<script setup lang="ts">
/**
 * ReminderDetailPanel Component
 * Displays and edits reminder details
 */

import { defineProps, defineEmits, ref, watch } from 'vue';
import type { Reminder, Category, FrequencyOption, Evidence } from '../../types/reminder';

const props = defineProps<{
  reminder: Reminder | null;
  categories: Category[];
  frequencyOptions: FrequencyOption[];
  evidenceList: Evidence[];
  uploadingFile: boolean;
  previewImage: string | null;
}>();

const emit = defineEmits<{
  (e: 'close'): void;
  (e: 'save', data: Partial<Reminder>): void;
  (e: 'delete', id: number): void;
  (e: 'toggle-reminder', id: number): void;
  (e: 'upload-file', file: File): void;
  (e: 'delete-evidence', id: number): void;
  (e: 'open-evidence', filePath: string): void;
  (e: 'preview-image', filePath: string): void;
  (e: 'close-preview'): void;
}>();

// Local editing state
const editingReminder = ref<Partial<Reminder>>({});
const fileInputRef = ref<HTMLInputElement | null>(null);

// Watch for reminder changes
watch(() => props.reminder, (newReminder) => {
  if (newReminder) {
    editingReminder.value = { ...newReminder };
  } else {
    editingReminder.value = {};
  }
}, { immediate: true });

function handleSave() {
  if (editingReminder.value.id) {
    emit('save', editingReminder.value);
  }
}

function handleDelete() {
  if (editingReminder.value.id) {
    emit('delete', editingReminder.value.id);
    emit('close');
  }
}

function triggerFilePicker() {
  fileInputRef.value?.click();
}

function handleFileUpload(event: Event) {
  const input = event.target as HTMLInputElement;
  if (input.files && input.files[0]) {
    emit('upload-file', input.files[0]);
    input.value = '';
  }
}

function formatFileSize(bytes: number): string {
  const units = ['B', 'KB', 'MB', 'GB'];
  let size = bytes;
  let unitIndex = 0;
  
  while (size >= 1024 && unitIndex < units.length - 1) {
    size /= 1024;
    unitIndex++;
  }
  
  return `${size.toFixed(2)} ${units[unitIndex]}`;
}

function getFileIcon(fileType: string, mimeType: string): string {
  if (fileType === "image") return "🖼️";
  if (fileType === "video") return "🎥";
  if (fileType === "audio") return "🎵";
  if (mimeType.includes("pdf")) return "📄";
  if (mimeType.includes("word")) return "📝";
  if (mimeType.includes("excel") || mimeType.includes("spreadsheet")) return "📊";
  if (mimeType.includes("zip") || mimeType.includes("archive")) return "📦";
  return "📎";
}
</script>

<template>
  <aside v-if="reminder" class="detail-panel">
    <div class="detail-header">
      <button @click="$emit('close')" class="btn-close">✕</button>
      <h2>Details</h2>
    </div>

    <div class="detail-body" v-if="editingReminder">
      <div class="detail-section">
        <div class="detail-row">
          <button 
            @click="$emit('toggle-reminder', editingReminder.id!)" 
            class="checkbox-btn-large"
            :class="{ checked: editingReminder.completed }"
          >
            <span v-if="editingReminder.completed" class="check-icon">✓</span>
          </button>
          <input
            v-model="editingReminder.title"
            class="detail-title-input"
            placeholder="Title"
          />
        </div>
      </div>

      <div class="detail-section">
        <label class="detail-label">Notes</label>
        <textarea
          v-model="editingReminder.description"
          class="detail-textarea"
          placeholder="Add notes..."
          rows="3"
        ></textarea>
      </div>

      <div class="detail-section">
        <label class="detail-label">Date & Time</label>
        <input
          v-model="editingReminder.time"
          type="datetime-local"
          class="detail-input"
        />
      </div>

      <div class="detail-section">
        <label class="detail-label">Repeat</label>
        <select v-model="editingReminder.frequency" class="detail-select">
          <option 
            v-for="freq in frequencyOptions" 
            :key="freq.value" 
            :value="freq.value"
          >
            {{ freq.icon }} {{ freq.label }}
          </option>
        </select>
      </div>

      <div class="detail-section">
        <label class="detail-label">List</label>
        <select v-model="editingReminder.category" class="detail-select">
          <option 
            v-for="cat in categories" 
            :key="cat.id" 
            :value="cat.id"
          >
            {{ cat.icon }} {{ cat.name }}
          </option>
        </select>
      </div>

      <div class="detail-section">
        <label class="detail-label">Flag</label>
        <button 
          @click="editingReminder.flagged = !editingReminder.flagged"
          class="flag-toggle"
          :class="{ active: editingReminder.flagged }"
        >
          🚩 {{ editingReminder.flagged ? 'Flagged' : 'Add Flag' }}
        </button>
      </div>

      <!-- Attachments Section -->
      <div class="detail-section">
        <label class="detail-label">Attachments</label>
        
        <div class="attachment-upload">
          <input 
            ref="fileInputRef"
            type="file" 
            id="file-upload" 
            @change="handleFileUpload" 
            style="display: none"
            accept="image/*,video/*,audio/*,.pdf,.doc,.docx,.xls,.xlsx,.txt,.json,.zip"
          />
          <button 
            @click="triggerFilePicker"
            class="btn-upload"
            :disabled="uploadingFile"
          >
            📎 {{ uploadingFile ? 'Uploading...' : 'Add Attachment' }}
          </button>
        </div>

        <div v-if="evidenceList.length > 0" class="attachment-list">
          <div 
            v-for="evidence in evidenceList" 
            :key="evidence.id"
            class="attachment-item"
          >
            <div 
              class="attachment-info" 
              @click="evidence.file_type === 'image' ? $emit('preview-image', evidence.file_path) : $emit('open-evidence', evidence.file_path)"
            >
              <span class="attachment-icon">{{ getFileIcon(evidence.file_type, evidence.mime_type) }}</span>
              <div class="attachment-details">
                <div class="attachment-name">{{ evidence.file_name }}</div>
                <div class="attachment-meta">
                  {{ formatFileSize(evidence.file_size) }} • {{ evidence.file_type }}
                </div>
              </div>
            </div>
            <button 
              @click="$emit('delete-evidence', evidence.id)"
              class="btn-delete-attachment"
              title="Delete attachment"
            >
              🗑️
            </button>
          </div>
        </div>

        <div v-else class="attachment-empty">
          No attachments yet
        </div>
      </div>

      <div class="detail-actions">
        <button @click="handleSave" class="btn-save-detail">
          Save Changes
        </button>
        <button @click="handleDelete" class="btn-delete-detail">
          Delete Reminder
        </button>
      </div>
    </div>

    <!-- Image Preview Modal -->
    <div v-if="previewImage" class="image-preview-modal" @click="$emit('close-preview')">
      <div class="image-preview-content" @click.stop>
        <button @click="$emit('close-preview')" class="btn-close-preview">✕</button>
        <img :src="previewImage" alt="Preview" class="preview-image" />
      </div>
    </div>
  </aside>
</template>

<style scoped>
.detail-panel {
  position: absolute;
  right: 0;
  top: 0;
  bottom: 0;
  width: 320px;
  background: #ffffff;
  border-left: 1px solid #e5e5ea;
  box-shadow: -2px 0 8px rgba(0, 0, 0, 0.1);
  z-index: 100;
  display: flex;
  flex-direction: column;
  animation: slideIn 0.3s ease-out;
}

@keyframes slideIn {
  from {
    transform: translateX(100%);
  }
  to {
    transform: translateX(0);
  }
}

.detail-header {
  padding: 1rem 1.5rem;
  border-bottom: 1px solid #e5e5ea;
  display: flex;
  align-items: center;
  gap: 0.75rem;
}

.detail-header h2 {
  margin: 0;
  font-size: 1.1rem;
  font-weight: 600;
  color: #1d1d1f;
  flex: 1;
}

.btn-close {
  width: 28px;
  height: 28px;
  border: none;
  background: #f2f2f7;
  color: #1d1d1f;
  border-radius: 50%;
  cursor: pointer;
  display: flex;
  align-items: center;
  justify-content: center;
  font-size: 0.9rem;
  transition: background 0.15s ease;
}

.btn-close:hover {
  background: #e5e5ea;
}

.detail-body {
  flex: 1;
  overflow-y: auto;
  padding: 1.5rem;
}

.detail-section {
  margin-bottom: 1.5rem;
}

.detail-row {
  display: flex;
  align-items: center;
  gap: 0.75rem;
}

.checkbox-btn-large {
  width: 28px;
  height: 28px;
  border: 2px solid #c7c7cc;
  border-radius: 50%;
  background: white;
  cursor: pointer;
  display: flex;
  align-items: center;
  justify-content: center;
  transition: all 0.2s ease;
  padding: 0;
  flex-shrink: 0;
}

.checkbox-btn-large:hover {
  border-color: #007aff;
}

.checkbox-btn-large.checked {
  background: #007aff;
  border-color: #007aff;
}

.checkbox-btn-large .check-icon {
  color: white;
  font-size: 0.9rem;
  font-weight: bold;
}

.detail-label {
  display: block;
  font-size: 0.8rem;
  font-weight: 600;
  color: #86868b;
  margin-bottom: 0.5rem;
  text-transform: uppercase;
  letter-spacing: 0.3px;
}

.detail-title-input {
  flex: 1;
  padding: 0.5rem;
  border: 1px solid #e5e5ea;
  border-radius: 8px;
  font-size: 1rem;
  color: #1d1d1f;
  font-weight: 500;
  background: #f9f9f9;
}

.detail-title-input:focus {
  outline: none;
  border-color: #007aff;
  background: white;
}

.detail-input,
.detail-select {
  width: 100%;
  padding: 0.65rem 0.75rem;
  border: 1px solid #e5e5ea;
  border-radius: 8px;
  font-size: 0.9rem;
  color: #1d1d1f;
  background: #f9f9f9;
  transition: all 0.15s ease;
}

.detail-input:focus,
.detail-select:focus {
  outline: none;
  border-color: #007aff;
  background: white;
}

.detail-textarea {
  width: 100%;
  padding: 0.65rem 0.75rem;
  border: 1px solid #e5e5ea;
  border-radius: 8px;
  font-size: 0.9rem;
  color: #1d1d1f;
  background: #f9f9f9;
  resize: vertical;
  font-family: inherit;
  line-height: 1.5;
}

.detail-textarea:focus {
  outline: none;
  border-color: #007aff;
  background: white;
}

.flag-toggle {
  width: 100%;
  padding: 0.65rem 0.75rem;
  border: 1px solid #e5e5ea;
  border-radius: 8px;
  font-size: 0.9rem;
  color: #1d1d1f;
  background: #f9f9f9;
  cursor: pointer;
  transition: all 0.15s ease;
  text-align: left;
}

.flag-toggle:hover {
  background: #f2f2f7;
}

.flag-toggle.active {
  background: #fff3e0;
  border-color: #ff9500;
  color: #ff9500;
}

.detail-actions {
  display: flex;
  flex-direction: column;
  gap: 0.75rem;
  margin-top: 2rem;
  padding-top: 1.5rem;
  border-top: 1px solid #e5e5ea;
}

.btn-save-detail {
  width: 100%;
  padding: 0.75rem;
  background: #007aff;
  color: white;
  border: none;
  border-radius: 8px;
  font-size: 0.9rem;
  font-weight: 600;
  cursor: pointer;
  transition: background 0.15s ease;
}

.btn-save-detail:hover {
  background: #0051d5;
}

.btn-delete-detail {
  width: 100%;
  padding: 0.75rem;
  background: transparent;
  color: #ff3b30;
  border: 1px solid #ff3b30;
  border-radius: 8px;
  font-size: 0.9rem;
  font-weight: 600;
  cursor: pointer;
  transition: all 0.15s ease;
}

.btn-delete-detail:hover {
  background: #ff3b30;
  color: white;
}

/* Attachment styles */
.attachment-upload {
  margin-bottom: 12px;
}

.btn-upload {
  width: 100%;
  padding: 10px 16px;
  background: #f2f2f7;
  border: 1px dashed #c6c6c8;
  border-radius: 8px;
  color: #007aff;
  font-size: 14px;
  cursor: pointer;
  transition: all 0.2s;
}

.btn-upload:hover:not(:disabled) {
  background: #e5e5ea;
  border-color: #007aff;
}

.btn-upload:disabled {
  opacity: 0.5;
  cursor: not-allowed;
}

.attachment-list {
  display: flex;
  flex-direction: column;
  gap: 8px;
}

.attachment-item {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 10px;
  background: #f2f2f7;
  border-radius: 8px;
  transition: background 0.2s;
}

.attachment-item:hover {
  background: #e5e5ea;
}

.attachment-info {
  display: flex;
  align-items: center;
  gap: 10px;
  flex: 1;
  cursor: pointer;
}

.attachment-icon {
  font-size: 24px;
  flex-shrink: 0;
}

.attachment-details {
  flex: 1;
  min-width: 0;
}

.attachment-name {
  font-size: 14px;
  font-weight: 500;
  color: #1c1c1e;
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
}

.attachment-meta {
  font-size: 12px;
  color: #8e8e93;
  margin-top: 2px;
}

.btn-delete-attachment {
  background: none;
  border: none;
  font-size: 18px;
  cursor: pointer;
  padding: 4px 8px;
  opacity: 0.6;
  transition: opacity 0.2s;
}

.btn-delete-attachment:hover {
  opacity: 1;
}

.attachment-empty {
  padding: 20px;
  text-align: center;
  color: #8e8e93;
  font-size: 14px;
}

/* Image Preview Modal */
.image-preview-modal {
  position: fixed;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
  background: rgba(0, 0, 0, 0.85);
  display: flex;
  align-items: center;
  justify-content: center;
  z-index: 9999;
  animation: fadeIn 0.2s;
}

@keyframes fadeIn {
  from { opacity: 0; }
  to { opacity: 1; }
}

.image-preview-content {
  position: relative;
  max-width: 90vw;
  max-height: 90vh;
  background: white;
  border-radius: 12px;
  padding: 20px;
  box-shadow: 0 20px 60px rgba(0, 0, 0, 0.3);
}

.btn-close-preview {
  position: absolute;
  top: 10px;
  right: 10px;
  background: rgba(0, 0, 0, 0.5);
  color: white;
  border: none;
  border-radius: 50%;
  width: 32px;
  height: 32px;
  font-size: 20px;
  cursor: pointer;
  display: flex;
  align-items: center;
  justify-content: center;
  transition: background 0.2s;
  z-index: 1;
}

.btn-close-preview:hover {
  background: rgba(0, 0, 0, 0.7);
}

.preview-image {
  max-width: 100%;
  max-height: 80vh;
  object-fit: contain;
  border-radius: 8px;
}

/* Dark mode */
@media (prefers-color-scheme: dark) {
  .detail-panel {
    background: #1c1c1e;
    border-left-color: #38383a;
    box-shadow: -2px 0 8px rgba(0, 0, 0, 0.3);
  }

  .detail-header {
    border-bottom-color: #38383a;
  }

  .detail-header h2 {
    color: #f5f5f7;
  }

  .btn-close {
    background: #2c2c2e;
    color: #f5f5f7;
  }

  .btn-close:hover {
    background: #3a3a3c;
  }

  .detail-label {
    color: #98989d;
  }

  .detail-title-input,
  .detail-input,
  .detail-select,
  .detail-textarea {
    background: #2c2c2e;
    border-color: #48484a;
    color: #f5f5f7;
  }

  .detail-title-input:focus,
  .detail-input:focus,
  .detail-select:focus,
  .detail-textarea:focus {
    background: #1c1c1e;
    border-color: #0a84ff;
  }

  .checkbox-btn-large {
    background: #2c2c2e;
    border-color: #636366;
  }

  .checkbox-btn-large:hover {
    border-color: #0a84ff;
  }

  .checkbox-btn-large.checked {
    background: #0a84ff;
    border-color: #0a84ff;
  }

  .flag-toggle {
    background: #2c2c2e;
    border-color: #48484a;
    color: #f5f5f7;
  }

  .flag-toggle:hover {
    background: #3a3a3c;
  }

  .flag-toggle.active {
    background: #3d2f00;
    border-color: #ff9500;
    color: #ff9500;
  }

  .detail-actions {
    border-top-color: #38383a;
  }

  .btn-save-detail {
    background: #0a84ff;
  }

  .btn-save-detail:hover {
    background: #0066cc;
  }

  .btn-delete-detail {
    border-color: #ff453a;
    color: #ff453a;
  }

  .btn-delete-detail:hover {
    background: #ff453a;
    color: white;
  }
}
</style>
