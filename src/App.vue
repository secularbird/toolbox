<script setup lang="ts">
import { ref, onMounted, computed } from "vue";
import { invoke } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";

interface Reminder {
  id: number;
  title: string;
  description: string;
  time: string;
  completed: boolean;
  category: string;
  frequency: string;
  priority: number; // 0=none, 1=low, 2=medium, 3=high
  flagged: boolean; // Star/flag for important items
  tags: string[]; // hashtags
}

interface Category {
  id: string;
  name: string;
  icon: string;
  color: string;
}

interface SyncSettings {
  id: number;
  sync_enabled: boolean;
  data_source: string;
  github_token?: string | null;
  github_repo?: string | null;
  sync_method?: string | null;
  last_sync?: string | null;
  auto_sync: boolean;
  sync_interval_minutes: number;
}

// Smart Lists (like macOS Reminders)
const smartLists = ref<Category[]>([
  { id: "today", name: "Today", icon: "📅", color: "#007aff" },
  { id: "scheduled", name: "Scheduled", icon: "📆", color: "#ff9500" },
  { id: "flagged", name: "Flagged", icon: "🚩", color: "#ff3b30" },
  { id: "all", name: "All", icon: "📋", color: "#8e8e93" },
]);

// User Lists (categories)
const categories = ref<Category[]>([
  { id: "work", name: "Work", icon: "💼", color: "#ff9800" },
  { id: "personal", name: "Personal", icon: "👤", color: "#4caf50" },
  { id: "shopping", name: "Shopping", icon: "🛒", color: "#e91e63" },
  { id: "health", name: "Health", icon: "🏥", color: "#00bcd4" },
  { id: "other", name: "Other", icon: "📌", color: "#9c27b0" },
]);

const frequencyOptions = [
  { value: "once", label: "Once", icon: "🔵" },
  { value: "daily", label: "Daily", icon: "📅" },
  { value: "weekly", label: "Weekly", icon: "📆" },
  { value: "monthly", label: "Monthly", icon: "🗓️" },
  { value: "yearly", label: "Yearly", icon: "📊" },
];

const selectedCategory = ref("today");
const reminderTitle = ref("");
const reminderDescription = ref("");
const reminderTime = ref("");
const reminderCategory = ref("personal");
const reminderFrequency = ref("once");
const reminderFlagged = ref(false);
const reminders = ref<Reminder[]>([]);
const message = ref("");
const newCategoryName = ref("");
const showAddCategory = ref(false);
const debugMode = ref(true);
const showDetails = ref(false);
const selectedReminder = ref<Reminder | null>(null);
const editingReminder = ref<Partial<Reminder>>({});

// Sync settings
const showSyncSettings = ref(false);
const syncSettings = ref<SyncSettings>({
  id: 1,
  sync_enabled: false,
  data_source: "local",
  github_token: null,
  github_repo: null,
  sync_method: "gist",
  last_sync: null,
  auto_sync: false,
  sync_interval_minutes: 30,
});
const syncStatus = ref("");
const isSyncing = ref(false);

const filteredReminders = computed(() => {
  const now = new Date();
  const today = new Date(now.getFullYear(), now.getMonth(), now.getDate());
  const tomorrow = new Date(today.getTime() + 24 * 60 * 60 * 1000);
  
  switch (selectedCategory.value) {
    case "today":
      return reminders.value.filter(r => {
        if (!r.completed) {
          const reminderDate = new Date(r.time);
          // Show reminders for today (between today 00:00 and tomorrow 00:00)
          return reminderDate >= today && reminderDate < tomorrow;
        }
        return false;
      });
    case "scheduled":
      return reminders.value.filter(r => r.time);
    case "flagged":
      return reminders.value.filter(r => r.flagged);
    case "all":
      return reminders.value;
    default:
      // User lists (categories)
      return reminders.value.filter(r => r.category === selectedCategory.value);
  }
});

const categoryStats = computed(() => {
  const now = new Date();
  const today = new Date(now.getFullYear(), now.getMonth(), now.getDate());
  const tomorrow = new Date(today.getTime() + 24 * 60 * 60 * 1000);
  const stats: Record<string, number> = {};
  
  // Smart lists stats
  stats["today"] = reminders.value.filter(r => {
    if (!r.completed) {
      const reminderDate = new Date(r.time);
      // Count reminders for today only
      return reminderDate >= today && reminderDate < tomorrow;
    }
    return false;
  }).length;
  stats["scheduled"] = reminders.value.filter(r => r.time).length;
  stats["flagged"] = reminders.value.filter(r => r.flagged).length;
  stats["all"] = reminders.value.length;
  
  // User lists stats
  categories.value.forEach(cat => {
    stats[cat.id] = reminders.value.filter(r => r.category === cat.id).length;
  });
  
  return stats;
});

async function addReminder() {
  if (!reminderTitle.value) {
    message.value = "Please fill in title";
    return;
  }
  
  // If in "today" list and no time set, default to today at current time
  let timeToUse = reminderTime.value;
  if (!timeToUse && selectedCategory.value === "today") {
    const now = new Date();
    // Format as datetime-local string: YYYY-MM-DDTHH:MM
    const year = now.getFullYear();
    const month = String(now.getMonth() + 1).padStart(2, '0');
    const day = String(now.getDate()).padStart(2, '0');
    const hours = String(now.getHours()).padStart(2, '0');
    const minutes = String(now.getMinutes()).padStart(2, '0');
    timeToUse = `${year}-${month}-${day}T${hours}:${minutes}`;
  }
  
  if (!timeToUse) {
    message.value = "Please fill in time";
    return;
  }
  
  try {
    await invoke("add_reminder", {
      title: reminderTitle.value,
      description: reminderDescription.value,
      time: timeToUse,
      category: reminderCategory.value,
      frequency: reminderFrequency.value,
    });
    
    reminderTitle.value = "";
    reminderDescription.value = "";
    reminderTime.value = "";
    reminderFrequency.value = "once";
    message.value = "Reminder added successfully!";
    // No need to manually reload - event listener will update automatically
  } catch (error) {
    message.value = `Error: ${error}`;
  }
}

async function loadReminders() {
  try {
    reminders.value = await invoke("get_reminders");
  } catch (error) {
    message.value = `Error loading reminders: ${error}`;
  }
}

async function toggleReminder(id: number) {
  try {
    await invoke("toggle_reminder", { id });
    // No need to manually reload - event listener will update automatically
  } catch (error) {
    message.value = `Error: ${error}`;
  }
}

async function toggleFlag(id: number) {
  const reminder = reminders.value.find(r => r.id === id);
  if (reminder) {
    reminder.flagged = !reminder.flagged;
    // TODO: Call backend to update flagged status
    // await invoke("update_reminder_flag", { id, flagged: reminder.flagged });
  }
}

function selectReminder(reminder: Reminder) {
  selectedReminder.value = reminder;
  editingReminder.value = { ...reminder };
  showDetails.value = true;
}

function closeDetails() {
  showDetails.value = false;
  selectedReminder.value = null;
  editingReminder.value = {};
}

async function saveReminderDetails() {
  if (!selectedReminder.value || !editingReminder.value.id) return;
  
  try {
    await invoke("update_reminder", {
      id: editingReminder.value.id,
      title: editingReminder.value.title,
      description: editingReminder.value.description,
      time: editingReminder.value.time,
      category: editingReminder.value.category,
      frequency: editingReminder.value.frequency,
    });
    closeDetails();
    message.value = "Reminder updated successfully!";
    setTimeout(() => { message.value = ""; }, 2000);
  } catch (error) {
    message.value = `Error: ${error}`;
  }
}

async function deleteReminder(id: number) {
  try {
    await invoke("delete_reminder", { id });
    // No need to manually reload - event listener will update automatically
  } catch (error) {
    message.value = `Error: ${error}`;
  }
}

function addCategory() {
  if (!newCategoryName.value.trim()) return;
  
  const newCategory: Category = {
    id: newCategoryName.value.toLowerCase().replace(/\s+/g, "-"),
    name: newCategoryName.value,
    icon: "📁",
    color: "#" + Math.floor(Math.random()*16777215).toString(16),
  };
  
  categories.value.push(newCategory);
  newCategoryName.value = "";
  showAddCategory.value = false;
}

function selectCategory(categoryId: string) {
  selectedCategory.value = categoryId;
}

function handleAddReminderBlur() {
  if (reminderTitle.value.trim()) {
    addReminder();
  }
}

async function toggleDebugMode() {
  try {
    await invoke("set_debug_mode", { enabled: debugMode.value });
    console.log(`Debug mode ${debugMode.value ? 'enabled' : 'disabled'}`);
  } catch (error) {
    console.error("Failed to set debug mode:", error);
  }
}

async function loadSyncSettings() {
  try {
    const settings = await invoke<SyncSettings>("get_sync_settings");
    syncSettings.value = settings;
    console.log("Sync settings loaded:", settings);
  } catch (error) {
    console.error("Failed to load sync settings:", error);
  }
}

async function saveSyncSettings() {
  try {
    await invoke("save_sync_settings", { settings: syncSettings.value });
    syncStatus.value = "Settings saved successfully!";
    setTimeout(() => syncStatus.value = "", 3000);
  } catch (error) {
    syncStatus.value = `Error: ${error}`;
  }
}

async function testGitHubConnection() {
  if (!syncSettings.value.github_token) {
    syncStatus.value = "Please enter a GitHub token";
    return;
  }
  
  try {
    syncStatus.value = "Testing connection...";
    const result = await invoke<boolean>("test_github_connection", { 
      token: syncSettings.value.github_token 
    });
    if (result) {
      syncStatus.value = "✓ Connection successful!";
      setTimeout(() => syncStatus.value = "", 3000);
    }
  } catch (error) {
    syncStatus.value = `✗ Connection failed: ${error}`;
  }
}

async function syncToGitHub() {
  if (!syncSettings.value.sync_enabled) {
    syncStatus.value = "Please enable sync first";
    return;
  }
  
  try {
    isSyncing.value = true;
    syncStatus.value = "Syncing to GitHub...";
    const result = await invoke<string>("sync_to_github");
    syncStatus.value = `✓ Synced successfully! URL: ${result}`;
    await loadSyncSettings(); // Reload to get updated last_sync time
    setTimeout(() => syncStatus.value = "", 5000);
  } catch (error) {
    syncStatus.value = `✗ Sync failed: ${error}`;
  } finally {
    isSyncing.value = false;
  }
}

async function syncFromGitHub() {
  if (!syncSettings.value.sync_enabled) {
    syncStatus.value = "Please enable sync first";
    return;
  }
  
  try {
    isSyncing.value = true;
    syncStatus.value = "Fetching from GitHub...";
    const count = await invoke<number>("sync_from_github");
    syncStatus.value = `✓ Found ${count} reminders in GitHub (validation successful). Note: Import functionality coming soon.`;
    await loadSyncSettings(); // Reload to get updated last_sync time
    setTimeout(() => syncStatus.value = "", 8000);
  } catch (error) {
    syncStatus.value = `✗ Fetch failed: ${error}`;
  } finally {
    isSyncing.value = false;
  }
}

function formatLastSync(lastSync: string | null | undefined): string {
  if (!lastSync) return "Never";
  try {
    const date = new Date(lastSync);
    return date.toLocaleString();
  } catch {
    return "Invalid date";
  }
}

async function loadDebugMode() {
  try {
    debugMode.value = await invoke("get_debug_mode");
  } catch (error) {
    console.error("Failed to get debug mode:", error);
  }
}

onMounted(async () => {
  loadReminders();
  loadDebugMode();
  loadSyncSettings();
  
  // Listen for real-time reminder updates from any window
  await listen<Reminder[]>('reminders-updated', (event) => {
    console.log('[APP] Received reminders-updated event:', event.payload);
    reminders.value = event.payload;
    message.value = "Reminders updated in real-time ✨";
    setTimeout(() => { message.value = ""; }, 2000);
  });
  
  console.log('[APP] Event listener setup complete');
});
</script>

<template>
  <div class="app-layout">
    <!-- Sidebar -->
    <aside class="sidebar">
      <div class="sidebar-header">
        <h2>📝 Reminders</h2>
        <button @click="showSyncSettings = true" class="btn-header-settings" title="Settings">
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
          @click="selectCategory(list.id)"
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
          @click="selectCategory(category.id)"
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
            @keyup.enter="addCategory"
          />
          <div class="category-form-actions">
            <button @click="addCategory" class="btn-save">✓</button>
            <button @click="showAddCategory = false; newCategoryName = ''" class="btn-cancel">✕</button>
          </div>
        </div>
      </div>

      <!-- Debug Mode Toggle -->
      <div class="debug-section">
        <label class="debug-toggle">
          <input 
            type="checkbox" 
            v-model="debugMode" 
            @change="toggleDebugMode"
          />
          <span class="debug-label">🐛 Debug Logs</span>
        </label>
        <div class="debug-info" v-if="debugMode">
          <div class="debug-item">
            <small>Press <kbd>F12</kbd> for DevTools</small>
          </div>
          <div class="debug-item">
            <small>{{ reminders.length }} reminders loaded</small>
          </div>
          <div class="debug-item">
            <small>Real-time sync: Active ✓</small>
          </div>
        </div>
      </div>

      <!-- Sync Settings Button -->
      <div class="sync-button-section">
        <button @click="showSyncSettings = true" class="btn-sync-settings">
          <span>⚙️ Sync Settings</span>
        </button>
      </div>
    </aside>

    <!-- Main Content -->
    <main class="main-content">
      <div class="content-header">
        <h1>
          {{ smartLists.find(c => c.id === selectedCategory)?.icon || categories.find(c => c.id === selectedCategory)?.icon }}
          {{ smartLists.find(c => c.id === selectedCategory)?.name || categories.find(c => c.id === selectedCategory)?.name }}
        </h1>
        <p class="reminder-count">{{ filteredReminders.length }}</p>
      </div>

      <p v-if="message" class="message">{{ message }}</p>

      <div class="content-body">
        <div v-if="filteredReminders.length === 0" class="no-reminders">
          <div class="empty-state">
            <span class="empty-icon">✨</span>
            <p>No Reminders</p>
          </div>
        </div>
        
        <div class="reminders-list">
          
          <!-- Add New Reminder -->
          <form @submit.prevent="addReminder" class="reminder-item add-item">
            <div class="reminder-checkbox">
              <span class="add-circle">○</span>
            </div>
            <div class="reminder-content">
              <input
                v-model="reminderTitle"
                placeholder="New Reminder"
                class="reminder-input"
                @blur="handleAddReminderBlur"
                @keyup.enter="addReminder"
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
            v-for="reminder in filteredReminders"
            :key="reminder.id"
            class="reminder-item"
            :class="{ completed: reminder.completed }"
            @dblclick="selectReminder(reminder)"
          >
            <div class="reminder-checkbox">
              <button 
                @click.stop="toggleReminder(reminder.id)" 
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
                  @click.stop="toggleFlag(reminder.id)"
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
                @click.stop="toggleFlag(reminder.id)" 
                class="action-btn flag-btn"
                :class="{ flagged: reminder.flagged }"
              >
                �
              </button>
            </div>
          </div>
        </div>
      </div>
    </main>

    <!-- Detail Panel -->
    <aside v-if="showDetails" class="detail-panel">
      <div class="detail-header">
        <button @click="closeDetails" class="btn-close">✕</button>
        <h2>Details</h2>
      </div>

      <div class="detail-body" v-if="editingReminder">
        <div class="detail-section">
          <div class="detail-row">
            <button 
              @click="toggleReminder(editingReminder.id!)" 
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

        <div class="detail-actions">
          <button @click="saveReminderDetails" class="btn-save-detail">
            Save Changes
          </button>
          <button @click="deleteReminder(editingReminder.id!); closeDetails();" class="btn-delete-detail">
            Delete Reminder
          </button>
        </div>
      </div>
    </aside>

    <!-- Sync Settings Modal -->
    <div v-if="showSyncSettings" class="modal-overlay" @click="showSyncSettings = false">
      <div class="modal-content sync-settings-modal" @click.stop>
        <div class="modal-header">
          <h2>⚙️ Sync Settings</h2>
          <button @click="showSyncSettings = false" class="btn-close">✕</button>
        </div>

        <div class="modal-body">
          <div class="settings-section">
            <label class="settings-toggle">
              <input 
                type="checkbox" 
                v-model="syncSettings.sync_enabled"
                @change="saveSyncSettings"
              />
              <span class="settings-label">Enable Sync</span>
            </label>
            <p class="settings-help">Sync your reminders with GitHub</p>
          </div>

          <div class="settings-section" v-if="syncSettings.sync_enabled">
            <label class="settings-label-text">Data Source</label>
            <select v-model="syncSettings.data_source" class="settings-select">
              <option value="local">Local Only</option>
              <option value="github">GitHub</option>
            </select>
          </div>

          <div v-if="syncSettings.sync_enabled && syncSettings.data_source === 'github'">
            <div class="settings-section">
              <label class="settings-label-text">Sync Method</label>
              <select v-model="syncSettings.sync_method" class="settings-select">
                <option value="gist">GitHub Gist (Private)</option>
                <option value="repo_json">Repository JSON File</option>
              </select>
              <p class="settings-help">
                <span v-if="syncSettings.sync_method === 'gist'">Creates a private gist for your reminders</span>
                <span v-else>Stores reminders as JSON in a repository</span>
              </p>
            </div>

            <div class="settings-section">
              <label class="settings-label-text">GitHub Token</label>
              <input 
                type="password"
                v-model="syncSettings.github_token"
                class="settings-input"
                placeholder="ghp_xxxxxxxxxxxxxxxxxxxxx"
              />
              <p class="settings-help">
                Generate at: <a href="https://github.com/settings/tokens/new?scopes=repo,gist" target="_blank">GitHub Settings</a>
                <br/>Required scopes: <code>repo</code>, <code>gist</code>
              </p>
              <button @click="testGitHubConnection" class="btn-test-connection">
                Test Connection
              </button>
            </div>

            <div class="settings-section" v-if="syncSettings.sync_method === 'repo_json'">
              <label class="settings-label-text">Repository</label>
              <input 
                type="text"
                v-model="syncSettings.github_repo"
                class="settings-input"
                placeholder="username/repository"
              />
              <p class="settings-help">Format: owner/repo (e.g., "john/my-reminders")</p>
            </div>

            <div class="settings-section">
              <label class="settings-toggle">
                <input 
                  type="checkbox" 
                  v-model="syncSettings.auto_sync"
                />
                <span class="settings-label">Auto Sync</span>
              </label>
              <p class="settings-help">Automatically sync every {{ syncSettings.sync_interval_minutes }} minutes</p>
            </div>

            <div class="settings-section" v-if="syncSettings.last_sync">
              <label class="settings-label-text">Last Sync</label>
              <p class="last-sync-time">{{ formatLastSync(syncSettings.last_sync) }}</p>
            </div>
          </div>

          <div v-if="syncStatus" class="sync-status" :class="{ error: syncStatus.includes('✗') }">
            {{ syncStatus }}
          </div>
        </div>

        <div class="modal-footer">
          <div class="sync-actions" v-if="syncSettings.sync_enabled && syncSettings.data_source === 'github'">
            <button @click="syncToGitHub" class="btn-sync" :disabled="isSyncing">
              {{ isSyncing ? '⏳ Syncing...' : '⬆️ Sync to GitHub' }}
            </button>
            <button @click="syncFromGitHub" class="btn-sync" :disabled="isSyncing || syncSettings.sync_method !== 'repo_json'">
              {{ isSyncing ? '⏳ Validating...' : '⬇️ Validate GitHub Data' }}
            </button>
          </div>
          <button @click="saveSyncSettings" class="btn-save-settings">
            Save Settings
          </button>
          <button @click="showSyncSettings = false" class="btn-cancel-settings">
            Close
          </button>
        </div>
      </div>
    </div>
  </div>
</template>

<style scoped>
.app-layout {
  display: flex;
  height: 100vh;
  overflow: hidden;
  position: relative;
}

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

/* Detail Panel */
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

.message {
  margin-top: 0.5rem;
  font-size: 0.85rem;
  color: #4caf50;
}

.message {
  margin-top: 0.75rem;
  padding: 0.6rem;
  background: #e8f4f8;
  border-radius: 6px;
  color: #333;
  font-size: 0.9rem;
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


</style>

<style>
* {
  margin: 0;
  padding: 0;
  box-sizing: border-box;
}

:root {
  font-family: Inter, Avenir, Helvetica, Arial, sans-serif;
  font-size: 16px;
  line-height: 24px;
  font-weight: 400;

  color: #0f0f0f;
  background-color: #f6f6f6;

  font-synthesis: none;
  text-rendering: optimizeLegibility;
  -webkit-font-smoothing: antialiased;
  -moz-osx-font-smoothing: grayscale;
  -webkit-text-size-adjust: 100%;
}

body {
  margin: 0;
  padding: 0;
  overflow: hidden;
}

#app {
  height: 100vh;
  overflow: hidden;
}

select {
  font-family: inherit;
}

@media (prefers-color-scheme: dark) {
  :root {
    color: #f5f5f7;
    background-color: #1c1c1e;
  }

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

/* Sync Settings Button */
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

/* Modal Overlay */
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
  background: white;
  border-radius: 12px;
  box-shadow: 0 10px 40px rgba(0, 0, 0, 0.3);
  max-width: 600px;
  width: 90%;
  max-height: 90vh;
  display: flex;
  flex-direction: column;
}

.modal-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 1.5rem;
  border-bottom: 1px solid #e5e5ea;
}

.modal-header h2 {
  margin: 0;
  font-size: 1.3rem;
  font-weight: 600;
  color: #1d1d1f;
}

.modal-body {
  padding: 1.5rem;
  overflow-y: auto;
  flex: 1;
}

.modal-footer {
  padding: 1rem 1.5rem;
  border-top: 1px solid #e5e5ea;
  display: flex;
  gap: 0.5rem;
  flex-wrap: wrap;
}

.settings-section {
  margin-bottom: 1.5rem;
}

.settings-toggle {
  display: flex;
  align-items: center;
  cursor: pointer;
}

.settings-toggle input[type="checkbox"] {
  margin-right: 0.5rem;
  width: 18px;
  height: 18px;
  cursor: pointer;
}

.settings-label {
  font-weight: 500;
  font-size: 0.95rem;
  color: #1d1d1f;
}

.settings-label-text {
  display: block;
  font-weight: 500;
  font-size: 0.9rem;
  color: #1d1d1f;
  margin-bottom: 0.5rem;
}

.settings-help {
  font-size: 0.8rem;
  color: #86868b;
  margin: 0.5rem 0 0 0;
}

.settings-help a {
  color: #007aff;
  text-decoration: none;
}

.settings-help a:hover {
  text-decoration: underline;
}

.settings-help code {
  background: #f5f5f7;
  padding: 0.1rem 0.4rem;
  border-radius: 3px;
  font-size: 0.75rem;
}

.settings-select,
.settings-input {
  width: 100%;
  padding: 0.6rem;
  border: 1px solid #d1d1d6;
  border-radius: 6px;
  font-size: 0.9rem;
  font-family: inherit;
}

.settings-select:focus,
.settings-input:focus {
  outline: none;
  border-color: #007aff;
}

.btn-test-connection {
  margin-top: 0.5rem;
  padding: 0.5rem 1rem;
  background: #f5f5f7;
  border: 1px solid #d1d1d6;
  border-radius: 6px;
  font-size: 0.85rem;
  cursor: pointer;
  transition: all 0.2s ease;
}

.btn-test-connection:hover {
  background: #e5e5ea;
}

.last-sync-time {
  font-size: 0.9rem;
  color: #1d1d1f;
  margin: 0;
}

.sync-status {
  padding: 0.75rem;
  background: #d1f4dd;
  color: #1e4d2b;
  border-radius: 6px;
  font-size: 0.85rem;
  margin-top: 1rem;
}

.sync-status.error {
  background: #fdd;
  color: #c00;
}

.sync-actions {
  display: flex;
  gap: 0.5rem;
  flex: 1;
}

.btn-sync {
  flex: 1;
  padding: 0.6rem 1rem;
  background: #007aff;
  color: white;
  border: none;
  border-radius: 6px;
  font-size: 0.85rem;
  font-weight: 500;
  cursor: pointer;
  transition: background 0.2s ease;
}

.btn-sync:hover:not(:disabled) {
  background: #0066cc;
}

.btn-sync:disabled {
  opacity: 0.5;
  cursor: not-allowed;
}

.btn-save-settings {
  padding: 0.6rem 1.2rem;
  background: #34c759;
  color: white;
  border: none;
  border-radius: 6px;
  font-size: 0.85rem;
  font-weight: 500;
  cursor: pointer;
  transition: background 0.2s ease;
}

.btn-save-settings:hover {
  background: #2fb54d;
}

.btn-cancel-settings {
  padding: 0.6rem 1.2rem;
  background: #f5f5f7;
  color: #1d1d1f;
  border: 1px solid #d1d1d6;
  border-radius: 6px;
  font-size: 0.85rem;
  font-weight: 500;
  cursor: pointer;
  transition: all 0.2s ease;
}

.btn-cancel-settings:hover {
  background: #e5e5ea;
}

/* Dark mode for sync settings */
@media (prefers-color-scheme: dark) {
  .modal-content {
    background: #1c1c1e;
  }

  .modal-header {
    border-bottom-color: #38383a;
  }

  .modal-header h2 {
    color: #f5f5f7;
  }

  .modal-footer {
    border-top-color: #38383a;
  }

  .settings-label,
  .settings-label-text,
  .last-sync-time {
    color: #f5f5f7;
  }

  .settings-help {
    color: #98989d;
  }

  .settings-help code {
    background: #2c2c2e;
  }

  .settings-select,
  .settings-input {
    background: #2c2c2e;
    border-color: #48484a;
    color: #f5f5f7;
  }

  .btn-test-connection {
    background: #2c2c2e;
    border-color: #48484a;
    color: #f5f5f7;
  }

  .btn-test-connection:hover {
    background: #3a3a3c;
  }

  .sync-status {
    background: #1e3a28;
    color: #4cd964;
  }

  .sync-status.error {
    background: #3a1f1f;
    color: #ff453a;
  }

  .btn-cancel-settings {
    background: #2c2c2e;
    border-color: #48484a;
    color: #f5f5f7;
  }

  .btn-cancel-settings:hover {
    background: #3a3a3c;
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
