<script setup lang="ts">
/**
 * RemindersAppRefactored Component
 * Main Reminders application using refactored components and composables
 * 
 * This is a refactored version of RemindersApp.vue that uses:
 * - Extracted composables for business logic
 * - Smaller, focused child components
 * - Shared types and constants
 */

import { ref, onMounted } from 'vue';
import { convertFileSrc } from '@tauri-apps/api/core';

// Composables
import { useReminders } from '../../composables/useReminders';
import { useSyncSettings } from '../../composables/useSyncSettings';
import { useEvidence } from '../../composables/useEvidence';
import { useDebugMode } from '../../composables/useDebugMode';

// Components
import ReminderSidebar from './ReminderSidebar.vue';
import ReminderList from './ReminderList.vue';
import ReminderDetailPanel from './ReminderDetailPanel.vue';
import SyncSettingsModal from '../shared/SyncSettingsModal.vue';

// Types and Constants
import type { Reminder } from '../../types/reminder';
import { FREQUENCY_OPTIONS } from '../../constants/reminder';

// Use composables
const {
  reminders,
  selectedCategory,
  message,
  categories,
  smartLists,
  filteredReminders,
  categoryStats,
  addReminder,
  toggleReminder,
  toggleFlag,
  updateReminder,
  deleteReminder,
  addCategory,
  selectCategory,
  initialize: initializeReminders,
} = useReminders();

const {
  syncSettings,
  syncStatus,
  isSyncing,
  loadSyncSettings,
  saveSyncSettings,
  testGitHubConnection,
  syncToGitHub,
  syncFromGitHub,
} = useSyncSettings();

const {
  evidenceList,
  uploadingFile,
  previewImage,
  loadReminderEvidence,
  handleFileUpload,
  deleteEvidence,
  openEvidence,
  showImagePreview,
  closeImagePreview,
  clearEvidence,
} = useEvidence();

const { debugMode, toggleDebugMode, loadDebugMode } = useDebugMode();

// Local UI state
const showDetails = ref(false);
const selectedReminder = ref<Reminder | null>(null);
const showSyncSettings = ref(false);

// Event handlers
function handleSelectReminder(reminder: Reminder) {
  selectedReminder.value = reminder;
  showDetails.value = true;
  loadReminderEvidence(reminder.id);
}

function handleCloseDetails() {
  showDetails.value = false;
  selectedReminder.value = null;
  clearEvidence();
}

async function handleSaveReminder(data: Partial<Reminder>) {
  if (data.id) {
    const success = await updateReminder(
      data.id,
      data.title || '',
      data.description || '',
      data.time || '',
      data.category || 'personal',
      data.frequency || 'once'
    );
    if (success) {
      handleCloseDetails();
    }
  }
}

async function handleDeleteReminder(id: number) {
  await deleteReminder(id);
  handleCloseDetails();
}

async function handleAddReminder(data: { title: string; time: string; flagged: boolean }) {
  await addReminder(
    data.title,
    '',
    data.time,
    'personal',
    'once'
  );
}

function handleAddCategory(name: string) {
  addCategory(name);
}

async function handleToggleDebug() {
  await toggleDebugMode();
}

async function handleFileUploadEvent(file: File) {
  if (selectedReminder.value) {
    await handleFileUpload(file, selectedReminder.value.id);
  }
}

function handlePreviewImage(filePath: string) {
  showImagePreview(filePath);
}

// Get preview image URL
function getPreviewImageUrl(): string | null {
  return previewImage.value;
}

// Initialize
onMounted(async () => {
  await initializeReminders();
  await loadDebugMode();
  await loadSyncSettings();
});
</script>

<template>
  <div class="app-layout">
    <!-- Sidebar -->
    <ReminderSidebar
      :smart-lists="smartLists"
      :categories="categories"
      :selected-category="selectedCategory"
      :category-stats="categoryStats"
      :debug-mode="debugMode"
      :reminders-count="reminders.length"
      @select-category="selectCategory"
      @add-category="handleAddCategory"
      @toggle-debug="handleToggleDebug"
      @open-sync-settings="showSyncSettings = true"
    />

    <!-- Main Content -->
    <ReminderList
      :reminders="filteredReminders"
      :selected-category="selectedCategory"
      :smart-lists="smartLists"
      :categories="categories"
      @add-reminder="handleAddReminder"
      @toggle-reminder="toggleReminder"
      @toggle-flag="toggleFlag"
      @select-reminder="handleSelectReminder"
    />

    <!-- Message display -->
    <p v-if="message" class="message">{{ message }}</p>

    <!-- Detail Panel -->
    <ReminderDetailPanel
      v-if="showDetails"
      :reminder="selectedReminder"
      :categories="categories"
      :frequency-options="FREQUENCY_OPTIONS"
      :evidence-list="evidenceList"
      :uploading-file="uploadingFile"
      :preview-image="getPreviewImageUrl()"
      @close="handleCloseDetails"
      @save="handleSaveReminder"
      @delete="handleDeleteReminder"
      @toggle-reminder="toggleReminder"
      @upload-file="handleFileUploadEvent"
      @delete-evidence="deleteEvidence"
      @open-evidence="openEvidence"
      @preview-image="handlePreviewImage"
      @close-preview="closeImagePreview"
    />

    <!-- Sync Settings Modal -->
    <SyncSettingsModal
      :visible="showSyncSettings"
      :sync-settings="syncSettings"
      :sync-status="syncStatus"
      :is-syncing="isSyncing"
      @close="showSyncSettings = false"
      @update:sync-settings="(v) => Object.assign(syncSettings, v)"
      @save="saveSyncSettings"
      @test-connection="testGitHubConnection"
      @sync-to-github="syncToGitHub"
      @sync-from-github="syncFromGitHub"
    />
  </div>
</template>

<style scoped>
.app-layout {
  display: flex;
  height: 100vh;
  overflow: hidden;
  position: relative;
}

.message {
  position: fixed;
  bottom: 20px;
  left: 50%;
  transform: translateX(-50%);
  padding: 0.6rem 1.2rem;
  background: #e8f4f8;
  border-radius: 6px;
  color: #333;
  font-size: 0.9rem;
  z-index: 1000;
  box-shadow: 0 2px 10px rgba(0, 0, 0, 0.1);
}

@media (prefers-color-scheme: dark) {
  .message {
    background: #2c2c2e;
    color: #f5f5f7;
  }
}
</style>
