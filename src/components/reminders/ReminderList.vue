<script setup lang="ts">
/**
 * ReminderList Component
 * Displays the list of reminders with add form
 */

import { defineProps, defineEmits, ref } from 'vue';
import type { Reminder, Category } from '../../types/reminder';

const props = defineProps<{
  reminders: Reminder[];
  selectedCategory: string;
  smartLists: Category[];
  categories: Category[];
}>();

const emit = defineEmits<{
  (e: 'add-reminder', data: { title: string; time: string; flagged: boolean }): void;
  (e: 'toggle-reminder', id: number): void;
  (e: 'toggle-flag', id: number): void;
  (e: 'select-reminder', reminder: Reminder): void;
}>();

// Form state
const reminderTitle = ref('');
const reminderTime = ref('');
const reminderFlagged = ref(false);

function handleAddReminder() {
  if (reminderTitle.value.trim()) {
    emit('add-reminder', {
      title: reminderTitle.value.trim(),
      time: reminderTime.value,
      flagged: reminderFlagged.value,
    });
    // Reset form
    reminderTitle.value = '';
    reminderTime.value = '';
    reminderFlagged.value = false;
  }
}

function handleAddReminderBlur() {
  if (reminderTitle.value.trim()) {
    handleAddReminder();
  }
}

function getCurrentCategoryInfo() {
  return props.smartLists.find(c => c.id === props.selectedCategory) 
    || props.categories.find(c => c.id === props.selectedCategory);
}
</script>

<template>
  <main class="main-content">
    <div class="content-header">
      <h1>
        {{ getCurrentCategoryInfo()?.icon }}
        {{ getCurrentCategoryInfo()?.name }}
      </h1>
      <p class="reminder-count">{{ reminders.length }}</p>
    </div>

    <div class="content-body">
      <div v-if="reminders.length === 0" class="no-reminders">
        <div class="empty-state">
          <span class="empty-icon">✨</span>
          <p>No Reminders</p>
        </div>
      </div>
      
      <div class="reminders-list">
        <!-- Add New Reminder -->
        <form @submit.prevent="handleAddReminder" class="reminder-item add-item">
          <div class="reminder-checkbox">
            <span class="add-circle">○</span>
          </div>
          <div class="reminder-content">
            <input
              v-model="reminderTitle"
              placeholder="New Reminder"
              class="reminder-input"
              @blur="handleAddReminderBlur"
              @keyup.enter="handleAddReminder"
            />
            <div class="reminder-meta" v-if="reminderTitle && selectedCategory !== 'today'">
              <input
                v-model="reminderTime"
                type="datetime-local"
                class="meta-input"
              />
              <button 
                type="button"
                class="flag-btn"
                :class="{ flagged: reminderFlagged }"
                @click="reminderFlagged = !reminderFlagged"
              >
                🚩
              </button>
            </div>
          </div>
        </form>
        
        <!-- Existing Reminders -->
        <div
          v-for="reminder in reminders"
          :key="reminder.id"
          class="reminder-item"
          :class="{ completed: reminder.completed }"
          @dblclick="$emit('select-reminder', reminder)"
        >
          <div class="reminder-checkbox">
            <button 
              @click.stop="$emit('toggle-reminder', reminder.id)" 
              class="checkbox-btn"
              :class="{ checked: reminder.completed }"
            >
              <span v-if="reminder.completed" class="check-icon">✓</span>
            </button>
          </div>
          <div class="reminder-content">
            <div class="reminder-title-row">
              <h3 class="reminder-title">{{ reminder.title }}</h3>
              <button 
                v-if="reminder.flagged"
                @click.stop="$emit('toggle-flag', reminder.id)"
                class="flag-indicator"
              >
                🚩
              </button>
            </div>
            <div class="reminder-meta" v-if="reminder.time || reminder.description">
              <span v-if="reminder.time" class="meta-time">
                📅 {{ new Date(reminder.time).toLocaleDateString() }}
              </span>
              <span v-if="reminder.description" class="meta-desc">
                {{ reminder.description }}
              </span>
            </div>
          </div>
          <div class="reminder-actions">
            <button 
              @click.stop="$emit('toggle-flag', reminder.id)" 
              class="action-btn flag-btn"
              :class="{ flagged: reminder.flagged }"
            >
              🚩
            </button>
          </div>
        </div>
      </div>
    </div>
  </main>
</template>

<style scoped>
.main-content {
  flex: 1;
  overflow-y: auto;
  background: #ffffff;
  display: flex;
  flex-direction: column;
}

.content-header {
  background: #ffffff;
  padding: 1.2rem 2rem 0.8rem 2rem;
  border-bottom: 1px solid #e5e5ea;
  display: flex;
  justify-content: space-between;
  align-items: baseline;
  flex-shrink: 0;
}

.content-header h1 {
  margin: 0;
  font-size: 1.75rem;
  color: #1d1d1f;
  font-weight: 700;
}

.reminder-count {
  color: #86868b;
  font-size: 0.85rem;
  font-weight: 400;
}

.content-body {
  flex: 1;
  overflow-y: auto;
  padding: 0;
}

.reminders-list {
  padding: 0;
}

.reminder-item {
  display: flex;
  align-items: flex-start;
  padding: 0.75rem 1.5rem;
  border-bottom: 1px solid #f2f2f7;
  cursor: pointer;
  transition: background 0.15s ease;
  gap: 0.75rem;
}

.reminder-item:hover {
  background: #f9f9f9;
}

.reminder-item.completed {
  opacity: 0.5;
}

.reminder-item.completed .reminder-title {
  text-decoration: line-through;
  color: #86868b;
}

.add-item {
  background: transparent;
  border: none;
  border-bottom: 1px solid #f2f2f7;
}

.add-item:hover {
  background: #fafafa;
}

.add-circle {
  font-size: 1.3rem;
  color: #c7c7cc;
}

.reminder-checkbox {
  display: flex;
  align-items: center;
  justify-content: center;
  flex-shrink: 0;
  padding-top: 0.15rem;
}

.checkbox-btn {
  width: 20px;
  height: 20px;
  border: 1.5px solid #c7c7cc;
  border-radius: 50%;
  background: white;
  cursor: pointer;
  display: flex;
  align-items: center;
  justify-content: center;
  transition: all 0.2s ease;
  padding: 0;
}

.checkbox-btn:hover {
  border-color: #007aff;
}

.checkbox-btn.checked {
  background: #007aff;
  border-color: #007aff;
}

.checkbox-btn .check-icon {
  color: white;
  font-size: 0.7rem;
  font-weight: bold;
}

.reminder-content {
  flex: 1;
  min-width: 0;
}

.reminder-title-row {
  display: flex;
  align-items: center;
  gap: 0.5rem;
}

.reminder-title {
  margin: 0;
  font-size: 0.95rem;
  color: #1d1d1f;
  font-weight: 400;
  line-height: 1.4;
  flex: 1;
}

.reminder-input {
  width: 100%;
  padding: 0.25rem 0;
  border: none;
  background: transparent;
  font-size: 0.95rem;
  color: #1d1d1f;
  font-weight: 400;
  outline: none;
}

.reminder-input::placeholder {
  color: #c7c7cc;
}

.reminder-meta {
  display: flex;
  align-items: center;
  gap: 0.75rem;
  margin-top: 0.25rem;
  flex-wrap: wrap;
}

.meta-time {
  font-size: 0.75rem;
  color: #86868b;
  display: flex;
  align-items: center;
  gap: 0.25rem;
}

.meta-desc {
  font-size: 0.75rem;
  color: #86868b;
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
}

.meta-input {
  padding: 0.2rem 0.4rem;
  border: 1px solid #e5e5ea;
  border-radius: 4px;
  font-size: 0.7rem;
  color: #1d1d1f;
  background: white;
}

.reminder-actions {
  display: flex;
  align-items: center;
  gap: 0.5rem;
  opacity: 0;
  transition: opacity 0.2s ease;
}

.reminder-item:hover .reminder-actions {
  opacity: 1;
}

.action-btn {
  width: 24px;
  height: 24px;
  border: none;
  background: transparent;
  cursor: pointer;
  font-size: 0.9rem;
  border-radius: 4px;
  transition: background 0.15s ease;
  display: flex;
  align-items: center;
  justify-content: center;
  padding: 0;
}

.action-btn:hover {
  background: rgba(0, 0, 0, 0.05);
}

.flag-btn {
  background: transparent;
  border: none;
  cursor: pointer;
  font-size: 0.9rem;
  opacity: 0.3;
  transition: opacity 0.2s ease;
  padding: 0;
}

.flag-btn.flagged {
  opacity: 1;
}

.flag-indicator {
  background: transparent;
  border: none;
  cursor: pointer;
  font-size: 0.85rem;
  padding: 0;
  margin-left: 0.25rem;
}

.no-reminders {
  padding: 0;
  margin: 0;
}

.empty-state {
  text-align: center;
  padding: 6rem 2rem;
  color: #86868b;
}

.empty-icon {
  font-size: 3rem;
  display: block;
  margin-bottom: 1rem;
  opacity: 0.4;
}

.empty-state p {
  margin: 0;
  font-size: 1.1rem;
  color: #86868b;
  font-weight: 500;
}

/* Dark mode */
@media (prefers-color-scheme: dark) {
  .main-content {
    background: #000000;
  }

  .content-header {
    background: #000000;
    border-bottom-color: #38383a;
  }

  .content-header h1 {
    color: #f5f5f7;
  }

  .reminder-count {
    color: #98989d;
  }

  .reminder-item {
    border-bottom-color: #38383a;
  }

  .reminder-item:hover {
    background: #1c1c1e;
  }

  .reminder-title {
    color: #f5f5f7;
  }

  .reminder-input {
    color: #f5f5f7;
  }

  .reminder-input::placeholder {
    color: #636366;
  }

  .checkbox-btn {
    background: #2c2c2e;
    border-color: #636366;
  }

  .checkbox-btn:hover {
    border-color: #0a84ff;
  }

  .checkbox-btn.checked {
    background: #0a84ff;
    border-color: #0a84ff;
  }

  .meta-time,
  .meta-desc {
    color: #98989d;
  }

  .meta-input {
    background: #2c2c2e;
    border-color: #48484a;
    color: #f5f5f7;
  }

  .action-btn:hover {
    background: rgba(255, 255, 255, 0.05);
  }

  .empty-state {
    color: #98989d;
  }

  .add-circle {
    color: #636366;
  }
}
</style>
