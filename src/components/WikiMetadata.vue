<script setup lang="ts">
import { ref, watch } from 'vue';

const props = defineProps<{
  tags: string[];
  createdAt: number;
  updatedAt: number;
  revisions?: Array<{
    id: string;
    page_id: string;
    title: string;
    created_at: number;
  }>;
}>();

const emit = defineEmits<{
  'update:tags': [tags: string[]];
  delete: [];
  restore: [revisionId: string];
}>();

const localTags = ref<string[]>([...props.tags]);
const newTagInput = ref('');

watch(() => props.tags, (newTags) => {
  localTags.value = [...newTags];
});

function addTag() {
  const tag = newTagInput.value.trim().replace(/^#/, '');
  if (tag && !localTags.value.includes(tag)) {
    localTags.value.push(tag);
    emit('update:tags', localTags.value);
    newTagInput.value = '';
  }
}

function removeTag(tag: string) {
  localTags.value = localTags.value.filter((t) => t !== tag);
  emit('update:tags', localTags.value);
}

function handleKeydown(e: KeyboardEvent) {
  if (e.key === 'Enter') {
    e.preventDefault();
    addTag();
  }
}

function formatDate(timestamp: number): string {
  return new Date(timestamp * 1000).toLocaleString('en-US', {
    year: 'numeric',
    month: 'short',
    day: 'numeric',
    hour: '2-digit',
    minute: '2-digit',
  });
}
</script>

<template>
  <div class="wiki-metadata">
    <div class="metadata-section">
      <h3>Tags</h3>
      <div class="tags-container">
        <div
          v-for="tag in localTags"
          :key="tag"
          class="tag-chip"
        >
          <span class="tag-text">#{{ tag }}</span>
          <button @click="removeTag(tag)" class="tag-remove">×</button>
        </div>
      </div>
      <div class="tag-input-container">
        <input
          v-model="newTagInput"
          type="text"
          placeholder="Add tag..."
          class="tag-input"
          @keydown="handleKeydown"
        />
        <button @click="addTag" class="tag-add-btn">+</button>
      </div>
    </div>

    <div class="metadata-section">
      <h3>Info</h3>
      <div class="info-row">
        <span class="info-label">Created:</span>
        <span class="info-value">{{ formatDate(createdAt) }}</span>
      </div>
      <div class="info-row">
        <span class="info-label">Updated:</span>
        <span class="info-value">{{ formatDate(updatedAt) }}</span>
      </div>
    </div>

    <div class="metadata-section">
      <h3>Shortcuts</h3>
      <div class="shortcuts">
        <div class="shortcut-row"><span>Cmd/Ctrl + B</span><span>Bold</span></div>
        <div class="shortcut-row"><span>Cmd/Ctrl + I</span><span>Italic</span></div>
        <div class="shortcut-row"><span>Cmd/Ctrl + K</span><span>Link</span></div>
        <div class="shortcut-row"><span>Tab</span><span>Indent (spaces)</span></div>
        <div class="shortcut-row"><span>Autosave</span><span>Every few seconds</span></div>
      </div>
    </div>

    <div v-if="props.revisions && props.revisions.length" class="metadata-section">
      <h3>History</h3>
      <div class="history-list">
        <div
          v-for="rev in props.revisions"
          :key="rev.id"
          class="history-item"
        >
          <div class="history-title">{{ rev.title || 'Untitled' }}</div>
          <div class="history-meta">{{ formatDate(rev.created_at) }}</div>
          <button class="history-restore" @click="emit('restore', rev.id)">
            Restore
          </button>
        </div>
      </div>
    </div>

    <div class="metadata-section">
      <button @click="emit('delete')" class="delete-btn">
        🗑️ Delete Page
      </button>
    </div>
  </div>
</template>

<style scoped>
.wiki-metadata {
  width: 260px;
  background: var(--sidebar-bg);
  border-left: 1px solid var(--border-color);
  padding: 16px;
  overflow-y: auto;
}

.metadata-section {
  margin-bottom: 24px;
}

.metadata-section h3 {
  font-size: 14px;
  font-weight: 600;
  color: var(--text-primary);
  margin-bottom: 12px;
  text-transform: uppercase;
  letter-spacing: 0.5px;
}

.tags-container {
  display: flex;
  flex-wrap: wrap;
  gap: 6px;
  margin-bottom: 8px;
}

.tag-chip {
  display: flex;
  align-items: center;
  gap: 4px;
  background: var(--tag-bg);
  padding: 4px 8px;
  border-radius: 6px;
  font-size: 12px;
  color: var(--text-primary);
}

.tag-text {
  color: var(--tag-color);
}

.tag-remove {
  background: none;
  border: none;
  color: var(--text-secondary);
  font-size: 16px;
  cursor: pointer;
  padding: 0;
  width: 16px;
  height: 16px;
  display: flex;
  align-items: center;
  justify-content: center;
  border-radius: 3px;
  transition: all 0.15s;
}

.tag-remove:hover {
  background: var(--hover-bg);
  color: var(--text-primary);
}

.tag-input-container {
  display: flex;
  gap: 4px;
}

.tag-input {
  flex: 1;
  padding: 6px 8px;
  border: 1px solid var(--border-color);
  border-radius: 6px;
  background: var(--input-bg);
  color: var(--text-primary);
  font-size: 12px;
  outline: none;
}

.tag-input:focus {
  border-color: var(--primary-color);
}

.tag-add-btn {
  width: 28px;
  height: 28px;
  background: var(--primary-color);
  color: white;
  border: none;
  border-radius: 6px;
  font-size: 16px;
  cursor: pointer;
  transition: opacity 0.15s;
}

.tag-add-btn:hover {
  opacity: 0.8;
}

.info-row {
  display: flex;
  justify-content: space-between;
  align-items: flex-start;
  margin-bottom: 8px;
  font-size: 12px;
}

.info-label {
  color: var(--text-secondary);
  font-weight: 500;
}

.info-value {
  color: var(--text-primary);
  text-align: right;
  max-width: 60%;
}

.delete-btn {
  width: 100%;
  padding: 10px;
  background: var(--danger-bg);
  color: var(--danger-color);
  border: 1px solid var(--danger-border);
  border-radius: 8px;
  font-size: 13px;
  cursor: pointer;
  transition: all 0.15s;
}

.delete-btn:hover {
  background: var(--danger-hover-bg);
  border-color: var(--danger-color);
}

.history-list {
  display: flex;
  flex-direction: column;
  gap: 8px;
}

.history-item {
  padding: 8px;
  border: 1px solid var(--border-color);
  border-radius: 8px;
  background: var(--input-bg);
  display: flex;
  flex-direction: column;
  gap: 4px;
}

.history-title {
  font-weight: 600;
  color: var(--text-primary);
  font-size: 13px;
}

.history-meta {
  font-size: 12px;
  color: var(--text-secondary);
}

.history-restore {
  align-self: flex-start;
  margin-top: 4px;
  padding: 6px 10px;
  border-radius: 6px;
  border: 1px solid var(--border-color);
  background: var(--btn-bg);
  color: var(--text-primary);
  cursor: pointer;
  transition: all 0.15s;
}

.history-restore:hover {
  border-color: var(--primary-color);
  color: var(--primary-color);
}

.shortcuts {
  display: flex;
  flex-direction: column;
  gap: 6px;
}

.shortcut-row {
  display: flex;
  justify-content: space-between;
  font-size: 12px;
  color: var(--text-primary);
}

/* Dark mode */
@media (prefers-color-scheme: dark) {
  .wiki-metadata {
    --sidebar-bg: #1c1c1e;
    --border-color: #38383a;
    --text-primary: #f5f5f7;
    --text-secondary: #98989d;
    --primary-color: #0a84ff;
    --tag-bg: rgba(10, 132, 255, 0.15);
    --tag-color: #0a84ff;
    --hover-bg: #2c2c2e;
    --input-bg: #2c2c2e;
    --danger-bg: rgba(255, 59, 48, 0.1);
    --danger-color: #ff453a;
    --danger-border: rgba(255, 59, 48, 0.3);
    --danger-hover-bg: rgba(255, 59, 48, 0.15);
  }
}

/* Light mode */
@media (prefers-color-scheme: light) {
  .wiki-metadata {
    --sidebar-bg: #f5f5f7;
    --border-color: #e5e5ea;
    --text-primary: #1d1d1f;
    --text-secondary: #86868b;
    --primary-color: #007aff;
    --tag-bg: rgba(0, 122, 255, 0.1);
    --tag-color: #007aff;
    --hover-bg: #e8e8ed;
    --input-bg: white;
    --danger-bg: rgba(255, 59, 48, 0.05);
    --danger-color: #ff3b30;
    --danger-border: rgba(255, 59, 48, 0.2);
    --danger-hover-bg: rgba(255, 59, 48, 0.1);
  }
}
</style>
