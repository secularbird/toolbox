<script setup lang="ts">
import { ref, computed } from 'vue';
import { invoke } from '@tauri-apps/api/core';

const emit = defineEmits<{
  close: [];
  insert: [markdown: string, reminderId: number];
}>();

const reminderTitle = ref('');
const reminderDescription = ref('');
const reminderTime = ref('');
const reminderCategory = ref('personal');
const reminderFrequency = ref('once');
const isCreating = ref(false);
const error = ref('');

const categories = [
  { id: 'work', name: 'Work', icon: '💼' },
  { id: 'personal', name: 'Personal', icon: '👤' },
  { id: 'shopping', name: 'Shopping', icon: '🛒' },
  { id: 'health', name: 'Health', icon: '🏥' },
  { id: 'other', name: 'Other', icon: '📌' },
];

const frequencies = [
  { value: 'once', label: 'Once', icon: '🔵' },
  { value: 'daily', label: 'Daily', icon: '📅' },
  { value: 'weekly', label: 'Weekly', icon: '📆' },
  { value: 'monthly', label: 'Monthly', icon: '🗓️' },
  { value: 'yearly', label: 'Yearly', icon: '📊' },
];

const canInsert = computed(() => {
  return reminderTitle.value.trim().length > 0 && reminderTime.value.length > 0;
});

async function handleInsert() {
  if (!canInsert.value || isCreating.value) return;

  isCreating.value = true;
  error.value = '';

  try {
    // Create actual reminder in the reminder database
    const reminderId = await invoke<number>('add_reminder', {
      title: reminderTitle.value.trim(),
      description: reminderDescription.value.trim() || 'Created from Wiki',
      time: reminderTime.value,
      category: reminderCategory.value,
      frequency: reminderFrequency.value,
    });

    console.log('[REMINDER INSERT] Created reminder with ID:', reminderId);

    const category = categories.find(c => c.id === reminderCategory.value);
    const frequency = frequencies.find(f => f.value === reminderFrequency.value);
    
    // Generate markdown with reminder link
    // Using inline code with base64-encoded JSON data to preserve metadata through Milkdown parsing
    const time = new Date(reminderTime.value).toLocaleString();
    const reminderData = {
      id: reminderId,
      title: reminderTitle.value,
      description: reminderDescription.value.trim() || 'Created from Wiki',
      category: reminderCategory.value,
      frequency: reminderFrequency.value,
      time: reminderTime.value
    };
    const encodedData = btoa(JSON.stringify(reminderData));
    const markdown = `
> **🔔 Reminder: ${reminderTitle.value}**
> 
> ${category?.icon} Category: ${category?.name}  
> ${frequency?.icon} Frequency: ${frequency?.label}  
> ⏰ Time: ${time}
> 
> \`[reminder:${encodedData}]\`
`.trim();

    emit('insert', markdown, reminderId);
  } catch (e) {
    error.value = `Failed to create reminder: ${e}`;
    console.error('[REMINDER INSERT]', e);
  } finally {
    isCreating.value = false;
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
        <h2>🔔 Insert Reminder</h2>
        <button class="close-btn" @click="handleCancel">✕</button>
      </div>

      <div class="modal-body">
        <div v-if="error" class="error-message">{{ error }}</div>

        <div class="form-group">
          <label for="reminder-title">Title *</label>
          <input
            id="reminder-title"
            v-model="reminderTitle"
            type="text"
            placeholder="Enter reminder title..."
            class="form-input"
            autofocus
          />
        </div>

        <div class="form-group">
          <label for="reminder-description">Description</label>
          <textarea
            id="reminder-description"
            v-model="reminderDescription"
            placeholder="Optional description..."
            class="form-textarea"
            rows="2"
          ></textarea>
        </div>

        <div class="form-group">
          <label for="reminder-time">Time *</label>
          <input
            id="reminder-time"
            v-model="reminderTime"
            type="datetime-local"
            class="form-input"
          />
        </div>

        <div class="form-row">
          <div class="form-group">
            <label for="reminder-category">Category</label>
            <select id="reminder-category" v-model="reminderCategory" class="form-select">
              <option v-for="cat in categories" :key="cat.id" :value="cat.id">
                {{ cat.icon }} {{ cat.name }}
              </option>
            </select>
          </div>

          <div class="form-group">
            <label for="reminder-frequency">Frequency</label>
            <select id="reminder-frequency" v-model="reminderFrequency" class="form-select">
              <option v-for="freq in frequencies" :key="freq.value" :value="freq.value">
                {{ freq.icon }} {{ freq.label }}
              </option>
            </select>
          </div>
        </div>

        <div class="help-text">
          This will create a real reminder in your Reminder app and insert a reference in your wiki page.
        </div>
      </div>

      <div class="modal-footer">
        <button class="btn btn-secondary" @click="handleCancel" :disabled="isCreating">
          Cancel
        </button>
        <button class="btn btn-primary" :disabled="!canInsert || isCreating" @click="handleInsert">
          {{ isCreating ? 'Creating...' : 'Create & Insert' }}
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
  animation: fadeIn 0.2s ease;
}

@keyframes fadeIn {
  from {
    opacity: 0;
  }
  to {
    opacity: 1;
  }
}

.modal-content {
  background: var(--modal-bg);
  border-radius: 12px;
  width: 90%;
  max-width: 500px;
  box-shadow: 0 20px 60px rgba(0, 0, 0, 0.3);
  animation: slideUp 0.3s ease;
}

@keyframes slideUp {
  from {
    transform: translateY(20px);
    opacity: 0;
  }
  to {
    transform: translateY(0);
    opacity: 1;
  }
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
  font-size: 1.25rem;
  color: var(--text-primary);
  font-weight: 600;
}

.close-btn {
  width: 32px;
  height: 32px;
  border: none;
  background: transparent;
  color: var(--text-secondary);
  font-size: 1.5rem;
  cursor: pointer;
  border-radius: 6px;
  display: flex;
  align-items: center;
  justify-content: center;
  transition: all 0.15s;
}

.close-btn:hover {
  background: var(--hover-bg);
  color: var(--text-primary);
}

.modal-body {
  padding: 24px;
}

.form-group {
  margin-bottom: 16px;
}

.form-row {
  display: grid;
  grid-template-columns: 1fr 1fr;
  gap: 16px;
}

.form-group label {
  display: block;
  margin-bottom: 8px;
  font-size: 0.875rem;
  font-weight: 500;
  color: var(--text-secondary);
}

.form-input,
.form-select,
.form-textarea {
  width: 100%;
  padding: 10px 12px;
  border: 1px solid var(--border-color);
  border-radius: 8px;
  background: var(--input-bg);
  color: var(--text-primary);
  font-size: 0.9375rem;
  font-family: inherit;
  transition: all 0.15s;
}

.form-textarea {
  resize: vertical;
  min-height: 60px;
  line-height: 1.5;
}

.form-input:focus,
.form-select:focus,
.form-textarea:focus {
  outline: none;
  border-color: var(--primary-color);
  box-shadow: 0 0 0 3px var(--primary-color-alpha);
}

.form-select {
  cursor: pointer;
}

.error-message {
  margin-bottom: 16px;
  padding: 12px;
  background: var(--error-bg);
  border: 1px solid var(--error-border);
  border-radius: 6px;
  color: var(--error-text);
  font-size: 0.875rem;
  line-height: 1.5;
}

.help-text {
  margin-top: 16px;
  padding: 12px;
  background: var(--info-bg);
  border-radius: 6px;
  font-size: 0.8125rem;
  color: var(--text-secondary);
  line-height: 1.5;
}

.modal-footer {
  display: flex;
  gap: 12px;
  justify-content: flex-end;
  padding: 16px 24px;
  border-top: 1px solid var(--border-color);
}

.btn {
  padding: 10px 20px;
  border: none;
  border-radius: 8px;
  font-size: 0.9375rem;
  font-weight: 500;
  cursor: pointer;
  transition: all 0.15s;
  font-family: inherit;
}

.btn-secondary {
  background: var(--btn-secondary-bg);
  color: var(--text-primary);
}

.btn-secondary:hover {
  background: var(--btn-secondary-hover);
}

.btn-primary {
  background: var(--primary-color);
  color: white;
}

.btn-primary:hover {
  background: var(--primary-color-dark);
  transform: translateY(-1px);
  box-shadow: 0 4px 12px var(--primary-color-alpha);
}

.btn-primary:disabled {
  opacity: 0.5;
  cursor: not-allowed;
  transform: none;
  box-shadow: none;
}

/* Light mode */
@media (prefers-color-scheme: light) {
  .modal-content {
    --modal-bg: #ffffff;
    --border-color: #e5e5ea;
    --text-primary: #1d1d1f;
    --text-secondary: #86868b;
    --input-bg: #f5f5f7;
    --hover-bg: #f5f5f7;
    --primary-color: #007aff;
    --primary-color-dark: #0051d5;
    --primary-color-alpha: rgba(0, 122, 255, 0.1);
    --btn-secondary-bg: #f5f5f7;
    --btn-secondary-hover: #e8e8ed;
    --info-bg: #f0f7ff;
    --error-bg: #fff0f0;
    --error-border: #ffcccc;
    --error-text: #d32f2f;
  }
}

/* Dark mode */
@media (prefers-color-scheme: dark) {
  .modal-content {
    --modal-bg: #1c1c1e;
    --border-color: #38383a;
    --text-primary: #f5f5f7;
    --text-secondary: #98989d;
    --input-bg: #2c2c2e;
    --hover-bg: #2c2c2e;
    --primary-color: #0a84ff;
    --primary-color-dark: #0066cc;
    --primary-color-alpha: rgba(10, 132, 255, 0.15);
    --btn-secondary-bg: #2c2c2e;
    --btn-secondary-hover: #3a3a3c;
    --info-bg: #1a2433;
    --error-bg: #3a1f1f;
    --error-border: #5a2f2f;
    --error-text: #ff6b6b;
  }
}
</style>
