<script setup lang="ts">
/**
 * ReminderSidebar Component
 * Displays the sidebar with smart lists and user categories
 */

import { defineProps, defineEmits } from 'vue';
import type { Category } from '../../types/reminder';

const props = defineProps<{
  smartLists: Category[];
  categories: Category[];
  selectedCategory: string;
  categoryStats: Record<string, number>;
  debugMode: boolean;
  remindersCount: number;
}>();

const emit = defineEmits<{
  (e: 'select-category', categoryId: string): void;
  (e: 'add-category', name: string): void;
  (e: 'toggle-debug'): void;
  (e: 'open-sync-settings'): void;
}>();

// Local state
import { ref } from 'vue';
const showAddCategory = ref(false);
const newCategoryName = ref('');

function handleAddCategory() {
  if (newCategoryName.value.trim()) {
    emit('add-category', newCategoryName.value.trim());
    newCategoryName.value = '';
    showAddCategory.value = false;
  }
}

function cancelAddCategory() {
  showAddCategory.value = false;
  newCategoryName.value = '';
}
</script>

<template>
  <aside class="sidebar">
    <div class="sidebar-header">
      <h2>📝 Reminders</h2>
      <button @click="$emit('open-sync-settings')" class="btn-header-settings" title="Settings">
        ⚙️
      </button>
    </div>
    
    <!-- Smart Lists -->
    <div class="categories smart-lists">
      <div
        v-for="list in smartLists"
        :key="list.id"
        class="category-item"
        :class="{ active: selectedCategory === list.id }"
        @click="$emit('select-category', list.id)"
      >
        <span class="category-icon" :style="{ color: list.color }">{{ list.icon }}</span>
        <span class="category-name">{{ list.name }}</span>
        <span class="category-count" v-if="categoryStats[list.id]">{{ categoryStats[list.id] }}</span>
      </div>
    </div>

    <div class="list-separator"></div>

    <!-- User Lists -->
    <div class="categories user-lists">
      <h3>My Lists</h3>
      <div
        v-for="category in categories"
        :key="category.id"
        class="category-item"
        :class="{ active: selectedCategory === category.id }"
        @click="$emit('select-category', category.id)"
      >
        <span class="category-icon">{{ category.icon }}</span>
        <span class="category-name">{{ category.name }}</span>
        <span class="category-count" v-if="categoryStats[category.id]">{{ categoryStats[category.id] }}</span>
      </div>
    </div>

    <div class="add-category-section">
      <button 
        v-if="!showAddCategory"
        @click="showAddCategory = true" 
        class="btn-add-category"
      >
        + Add Category
      </button>
      <div v-else class="new-category-form">
        <input
          v-model="newCategoryName"
          placeholder="Category name"
          class="category-input"
          @keyup.enter="handleAddCategory"
        />
        <div class="category-form-actions">
          <button @click="handleAddCategory" class="btn-save">✓</button>
          <button @click="cancelAddCategory" class="btn-cancel">✕</button>
        </div>
      </div>
    </div>

    <!-- Debug Mode Toggle -->
    <div class="debug-section">
      <label class="debug-toggle">
        <input 
          type="checkbox" 
          :checked="debugMode"
          @change="$emit('toggle-debug')"
        />
        <span class="debug-label">🐛 Debug Logs</span>
      </label>
      <div class="debug-info" v-if="debugMode">
        <div class="debug-item">
          <small>Press <kbd>F12</kbd> for DevTools</small>
        </div>
        <div class="debug-item">
          <small>{{ remindersCount }} reminders loaded</small>
        </div>
        <div class="debug-item">
          <small>Real-time sync: Active ✓</small>
        </div>
      </div>
    </div>

    <!-- Sync Settings Button -->
    <div class="sync-button-section">
      <button @click="$emit('open-sync-settings')" class="btn-sync-settings">
        <span>⚙️ Sync Settings</span>
      </button>
    </div>
  </aside>
</template>

<style scoped>
.sidebar {
  width: 240px;
  background: #f7f7f7;
  border-right: 1px solid #d1d1d6;
  display: flex;
  flex-direction: column;
  overflow-y: auto;
}

.sidebar-header {
  padding: 1.2rem 1rem 0.8rem 1rem;
  display: flex;
  justify-content: space-between;
  align-items: center;
}

.sidebar-header h2 {
  margin: 0;
  font-size: 1.4rem;
  color: #1d1d1f;
  font-weight: 700;
}

.btn-header-settings {
  width: 32px;
  height: 32px;
  border: none;
  background: transparent;
  color: #86868b;
  border-radius: 6px;
  cursor: pointer;
  font-size: 1.2rem;
  display: flex;
  align-items: center;
  justify-content: center;
  transition: all 0.15s ease;
}

.btn-header-settings:hover {
  background: rgba(0, 0, 0, 0.05);
  color: #007aff;
}

.categories {
  padding: 0.5rem 0.5rem;
}

.smart-lists {
  padding-top: 0;
}

.user-lists {
  padding-top: 0.5rem;
}

.categories h3 {
  font-size: 0.68rem;
  text-transform: uppercase;
  color: #86868b;
  margin: 0.8rem 0 0.4rem 0.75rem;
  font-weight: 600;
  letter-spacing: 0.5px;
}

.list-separator {
  height: 1px;
  background: #d1d1d6;
  margin: 0.5rem 0;
}

.category-item {
  display: flex;
  align-items: center;
  padding: 0.5rem 0.75rem;
  margin: 1px 0;
  border-radius: 6px;
  cursor: pointer;
  transition: all 0.15s ease;
  position: relative;
}

.category-item:hover {
  background: rgba(0, 0, 0, 0.05);
}

.category-item.active {
  background: #007aff;
  color: white;
}

.category-item.active .category-name {
  color: white;
}

.category-icon {
  font-size: 1.1rem;
  margin-right: 0.5rem;
  display: flex;
  align-items: center;
  justify-content: center;
  width: 24px;
}

.category-name {
  flex: 1;
  font-weight: 500;
  font-size: 0.85rem;
  color: #1d1d1f;
}

.category-count {
  background: transparent;
  color: #86868b;
  padding: 0;
  font-size: 0.8rem;
  font-weight: 600;
  min-width: 20px;
  text-align: right;
}

.category-item.active .category-count {
  color: rgba(255, 255, 255, 0.9);
}

.add-category-section {
  padding: 1rem;
  border-top: 1px solid #e0e0e0;
}

.btn-add-category {
  width: 100%;
  padding: 0.75rem;
  background: transparent;
  color: #396cd8;
  border: 2px dashed #396cd8;
  border-radius: 8px;
  cursor: pointer;
  font-weight: 600;
  transition: all 0.2s;
}

.btn-add-category:hover {
  background: #f0f5ff;
}

.new-category-form {
  display: flex;
  flex-direction: column;
  gap: 0.5rem;
}

.category-input {
  padding: 0.6rem;
  border: 1px solid #ddd;
  border-radius: 8px;
  font-size: 0.85rem;
}

.category-form-actions {
  display: flex;
  gap: 0.5rem;
}

.btn-save,
.btn-cancel {
  flex: 1;
  padding: 0.45rem;
  border: none;
  border-radius: 6px;
  cursor: pointer;
  font-size: 0.85rem;
}

.btn-save {
  background: #4caf50;
  color: white;
}

.btn-cancel {
  background: #f44336;
  color: white;
}

.debug-section {
  margin-top: auto;
  padding: 1rem;
  border-top: 1px solid #e0e0e0;
}

.debug-toggle {
  display: flex;
  align-items: center;
  gap: 0.5rem;
  cursor: pointer;
  user-select: none;
}

.debug-toggle input[type="checkbox"] {
  width: 18px;
  height: 18px;
  cursor: pointer;
}

.debug-label {
  font-size: 0.9rem;
  color: #666;
}

.debug-info {
  margin-top: 0.75rem;
  padding: 0.5rem;
  background: #f0f7ff;
  border-radius: 4px;
  border-left: 3px solid #396cd8;
}

.debug-item {
  margin: 0.25rem 0;
  font-size: 0.75rem;
  color: #555;
  display: flex;
  align-items: center;
}

.debug-item kbd {
  padding: 2px 6px;
  background: #fff;
  border: 1px solid #ccc;
  border-radius: 3px;
  font-family: monospace;
  font-size: 0.7rem;
  box-shadow: 0 1px 2px rgba(0,0,0,0.1);
  margin: 0 2px;
}

.sync-button-section {
  padding: 0.5rem;
  margin-top: auto;
  border-top: 1px solid #d1d1d6;
}

.btn-sync-settings {
  width: 100%;
  padding: 0.6rem;
  background: #007aff;
  color: white;
  border: none;
  border-radius: 6px;
  font-size: 0.85rem;
  font-weight: 500;
  cursor: pointer;
  transition: background 0.2s ease;
}

.btn-sync-settings:hover {
  background: #0066cc;
}

/* Dark mode */
@media (prefers-color-scheme: dark) {
  .sidebar {
    background-color: #1c1c1e;
    border-right-color: #38383a;
  }

  .sidebar-header h2 {
    color: #f5f5f7;
  }

  .btn-header-settings {
    color: #98989d;
  }

  .btn-header-settings:hover {
    background: rgba(255, 255, 255, 0.05);
    color: #0a84ff;
  }

  .category-item {
    color: #f5f5f7;
  }

  .category-item:hover {
    background: rgba(255, 255, 255, 0.05);
  }

  .category-item.active {
    background: #0a84ff;
  }

  .category-name {
    color: #f5f5f7;
  }

  .category-count {
    color: #98989d;
  }

  .category-item.active .category-count {
    color: rgba(255, 255, 255, 0.9);
  }

  .list-separator {
    background: #38383a;
  }

  .categories h3 {
    color: #98989d;
  }

  .add-category-section {
    border-top-color: #38383a;
  }

  .category-input {
    background-color: #2c2c2e;
    color: #f5f5f7;
    border-color: #48484a;
  }

  .sync-button-section {
    border-top-color: #38383a;
  }

  .btn-sync-settings {
    background: #0a84ff;
  }

  .btn-sync-settings:hover {
    background: #0066cc;
  }
}
</style>
