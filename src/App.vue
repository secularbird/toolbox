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
}

interface Category {
  id: string;
  name: string;
  icon: string;
  color: string;
}

const categories = ref<Category[]>([
  { id: "all", name: "All Reminders", icon: "📋", color: "#396cd8" },
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

const selectedCategory = ref("all");
const reminderTitle = ref("");
const reminderDescription = ref("");
const reminderTime = ref("");
const reminderCategory = ref("personal");
const reminderFrequency = ref("once");
const reminders = ref<Reminder[]>([]);
const message = ref("");
const newCategoryName = ref("");
const showAddCategory = ref(false);
const debugMode = ref(true);

const filteredReminders = computed(() => {
  if (selectedCategory.value === "all") {
    return reminders.value;
  }
  return reminders.value.filter(r => r.category === selectedCategory.value);
});

const categoryStats = computed(() => {
  const stats: Record<string, number> = {};
  categories.value.forEach(cat => {
    if (cat.id === "all") {
      stats[cat.id] = reminders.value.length;
    } else {
      stats[cat.id] = reminders.value.filter(r => r.category === cat.id).length;
    }
  });
  return stats;
});

async function addReminder() {
  if (!reminderTitle.value || !reminderTime.value) {
    message.value = "Please fill in title and time";
    return;
  }
  
  try {
    await invoke("add_reminder", {
      title: reminderTitle.value,
      description: reminderDescription.value,
      time: reminderTime.value,
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

function getFrequencyLabel(frequency: string): string {
  const option = frequencyOptions.find(opt => opt.value === frequency);
  return option ? `${option.icon} ${option.label}` : frequency;
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

async function toggleDebugMode() {
  try {
    await invoke("set_debug_mode", { enabled: debugMode.value });
    console.log(`Debug mode ${debugMode.value ? 'enabled' : 'disabled'}`);
  } catch (error) {
    console.error("Failed to set debug mode:", error);
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
        <h2>📝 Reminder App</h2>
      </div>
      
      <div class="categories">
        <h3>Categories</h3>
        <div
          v-for="category in categories"
          :key="category.id"
          class="category-item"
          :class="{ active: selectedCategory === category.id }"
          @click="selectCategory(category.id)"
        >
          <span class="category-icon">{{ category.icon }}</span>
          <span class="category-name">{{ category.name }}</span>
          <span class="category-count">{{ categoryStats[category.id] || 0 }}</span>
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
    </aside>

    <!-- Main Content -->
    <main class="main-content">
      <div class="content-header">
        <h1>
          {{ categories.find(c => c.id === selectedCategory)?.icon }}
          {{ categories.find(c => c.id === selectedCategory)?.name }}
        </h1>
        <p class="reminder-count">{{ filteredReminders.length }} reminder{{ filteredReminders.length !== 1 ? 's' : '' }}</p>
      </div>

      <p v-if="message" class="message">{{ message }}</p>

      <div class="content-body">
        <div v-if="filteredReminders.length === 0" class="no-reminders">
          <div class="empty-state">
            <span class="empty-icon">📝</span>
            <p>No reminders in this category yet</p>
            <small>Click below to add your first reminder</small>
          </div>
        </div>
        
        <div class="reminders-table">
          <div class="table-header">
            <div class="col-checkbox"></div>
            <div class="col-title">Task</div>
            <div class="col-category">Category</div>
            <div class="col-frequency">Frequency</div>
            <div class="col-time">Due Date</div>
            <div class="col-actions">Actions</div>
          </div>
          
          <!-- Add New Reminder Row -->
          <form @submit.prevent="addReminder" class="table-row add-row">
            <div class="col-checkbox">
              <span class="add-icon">+</span>
            </div>
            <div class="col-title">
              <input
                v-model="reminderTitle"
                placeholder="Add a new task..."
                class="inline-input"
              />
            </div>
            <div class="col-category">
              <select v-model="reminderCategory" class="inline-select">
                <option 
                  v-for="cat in categories.filter(c => c.id !== 'all')" 
                  :key="cat.id" 
                  :value="cat.id"
                >
                  {{ cat.icon }} {{ cat.name }}
                </option>
              </select>
            </div>
            <div class="col-frequency">
              <select v-model="reminderFrequency" class="inline-select">
                <option 
                  v-for="freq in frequencyOptions" 
                  :key="freq.value" 
                  :value="freq.value"
                >
                  {{ freq.icon }} {{ freq.label }}
                </option>
              </select>
            </div>
            <div class="col-time">
              <input
                v-model="reminderTime"
                type="datetime-local"
                class="inline-input"
              />
            </div>
            <div class="col-actions">
              <button type="submit" class="btn-add-inline">✓</button>
            </div>
          </form>
          
          <!-- Existing Reminders -->
          <div
            v-for="reminder in filteredReminders"
            :key="reminder.id"
            class="table-row"
            :class="{ completed: reminder.completed }"
          >
            <div class="col-checkbox">
              <button @click="toggleReminder(reminder.id)" class="checkbox-btn">
                <span v-if="reminder.completed" class="check-icon">✓</span>
              </button>
            </div>
            <div class="col-title">
              <h3>{{ reminder.title }}</h3>
              <p v-if="reminder.description" class="description">{{ reminder.description }}</p>
            </div>
            <div class="col-category">
              <span class="category-badge" :style="{ backgroundColor: categories.find(c => c.id === reminder.category)?.color }">
                {{ categories.find(c => c.id === reminder.category)?.icon }}
                {{ categories.find(c => c.id === reminder.category)?.name }}
              </span>
            </div>
            <div class="col-frequency">
              <span class="frequency-badge">
                {{ getFrequencyLabel(reminder.frequency) }}
              </span>
            </div>
            <div class="col-time">
              <span class="time-display">{{ new Date(reminder.time).toLocaleString() }}</span>
            </div>
            <div class="col-actions">
              <button @click="deleteReminder(reminder.id)" class="btn-delete-icon" title="Delete">
                🗑️
              </button>
            </div>
          </div>
        </div>
      </div>
    </main>
  </div>
</template>

<style scoped>
.app-layout {
  display: flex;
  height: 100vh;
  overflow: hidden;
}

.sidebar {
  width: 280px;
  background: #fff;
  border-right: 1px solid #e0e0e0;
  display: flex;
  flex-direction: column;
  overflow-y: auto;
}

.sidebar-header {
  padding: 1.5rem;
  border-bottom: 1px solid #e0e0e0;
}

.sidebar-header h2 {
  margin: 0;
  font-size: 1rem;
  color: #1a1a1a;
  font-weight: 600;
}

.categories {
  padding: 1rem;
  flex: 1;
}

.categories h3 {
  font-size: 0.7rem;
  text-transform: uppercase;
  color: #666;
  margin: 0 0 0.5rem 0.5rem;
  font-weight: 700;
  letter-spacing: 0.5px;
}

.category-item {
  display: flex;
  align-items: center;
  padding: 0.75rem 1rem;
  margin-bottom: 0.25rem;
  border-radius: 8px;
  cursor: pointer;
  transition: all 0.2s;
}

.category-item:hover {
  background: #f5f5f5;
}

.category-item.active {
  background: #396cd8;
  color: white;
}

.category-icon {
  font-size: 1rem;
  margin-right: 0.6rem;
}

.category-name {
  flex: 1;
  font-weight: 600;
  font-size: 0.9rem;
  color: #2a2a2a;
}

.category-count {
  background: rgba(0, 0, 0, 0.1);
  padding: 0.15rem 0.5rem;
  border-radius: 10px;
  font-size: 0.75rem;
  font-weight: 600;
}

.category-item.active .category-count {
  background: rgba(255, 255, 255, 0.2);
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
  background: #f6f6f6;
  display: flex;
  flex-direction: column;
}

.content-header {
  background: #fff;
  padding: 1.5rem 2rem;
  border-bottom: 1px solid #e0e0e0;
  display: flex;
  justify-content: space-between;
  align-items: center;
  flex-shrink: 0;
}

.content-header h1 {
  margin: 0;
  font-size: 1.3rem;
  color: #1a1a1a;
  font-weight: 700;
}

.reminder-count {
  color: #666;
  font-size: 0.75rem;
  font-weight: 500;
}

.content-body {
  flex: 1;
  overflow-y: auto;
  padding: 1.5rem 2rem;
}

.add-row {
  background: #f8fbff;
  border: 2px dashed #396cd8;
  border-radius: 8px;
  margin-bottom: 0.5rem;
}

.add-row:hover {
  background: #f0f7ff;
  border-color: #2d5ab8;
}

.add-icon {
  font-size: 1.2rem;
  color: #396cd8;
  font-weight: bold;
}

.inline-input {
  width: 100%;
  padding: 0.5rem;
  border: 1px solid #ddd;
  border-radius: 6px;
  font-size: 0.85rem;
  color: #1a1a1a;
  font-weight: 500;
  background: white;
}

.inline-input:focus {
  outline: none;
  border-color: #396cd8;
  box-shadow: 0 0 0 2px rgba(57, 108, 216, 0.1);
}

.inline-input::placeholder {
  color: #999;
  font-weight: 400;
}

.inline-select {
  width: 100%;
  padding: 0.5rem;
  border: 1px solid #ddd;
  border-radius: 6px;
  font-size: 0.75rem;
  color: #1a1a1a;
  font-weight: 500;
  background: white;
  cursor: pointer;
}

.inline-select:focus {
  outline: none;
  border-color: #396cd8;
}

.btn-add-inline {
  width: 100%;
  padding: 0.5rem;
  background: #4caf50;
  color: white;
  border: none;
  border-radius: 6px;
  font-size: 1rem;
  font-weight: bold;
  cursor: pointer;
  transition: background 0.2s;
}

.btn-add-inline:hover {
  background: #45a049;
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

.reminders-section {
  background: #fff;
  border-radius: 12px;
  box-shadow: 0 1px 3px rgba(0, 0, 0, 0.1);
  overflow: hidden;
}

.no-reminders {
  padding: 0;
  margin: 0;
}

.empty-state {
  text-align: center;
  padding: 4rem 2rem;
  color: #666;
}

.empty-icon {
  font-size: 2.5rem;
  display: block;
  margin-bottom: 1rem;
  opacity: 0.6;
}

.empty-state p {
  margin: 0.5rem 0;
  font-size: 0.95rem;
  color: #444;
  font-weight: 500;
}

.empty-state small {
  color: #888;
  font-size: 0.8rem;
}

.reminders-table {
  width: 100%;
}

.table-header {
  display: grid;
  grid-template-columns: 50px 1fr 180px 140px 200px 80px;
  gap: 1rem;
  padding: 0.8rem 1.2rem;
  background: #f8f9fa;
  border-bottom: 2px solid #e0e0e0;
  font-weight: 700;
  font-size: 0.75rem;
  text-transform: uppercase;
  color: #444;
  letter-spacing: 0.5px;
}

.table-row {
  display: grid;
  grid-template-columns: 50px 1fr 180px 140px 200px 80px;
  gap: 1rem;
  padding: 1rem 1.2rem;
  border-bottom: 1px solid #f0f0f0;
  align-items: center;
  transition: background 0.2s;
}

.table-row:hover {
  background: #f8f9fa;
}

.table-row.completed {
  opacity: 0.6;
}

.table-row.completed .col-title h3 {
  text-decoration: line-through;
}

.col-checkbox {
  display: flex;
  align-items: center;
  justify-content: center;
}

.checkbox-btn {
  width: 24px;
  height: 24px;
  border: 2px solid #ddd;
  border-radius: 6px;
  background: white;
  cursor: pointer;
  display: flex;
  align-items: center;
  justify-content: center;
  transition: all 0.2s;
  padding: 0;
}

.checkbox-btn:hover {
  border-color: #4caf50;
  background: #f0f9f0;
}

.checkbox-btn .check-icon {
  color: #4caf50;
  font-size: 0.9rem;
  font-weight: bold;
}

.col-title h3 {
  margin: 0 0 0.25rem 0;
  font-size: 0.9rem;
  color: #1a1a1a;
  font-weight: 600;
}

.col-title .description {
  margin: 0;
  font-size: 0.75rem;
  color: #555;
  line-height: 1.4;
}

.category-badge {
  display: inline-flex;
  align-items: center;
  gap: 0.3rem;
  padding: 0.3rem 0.6rem;
  border-radius: 12px;
  font-size: 0.75rem;
  font-weight: 600;
  color: white;
}

.frequency-badge {
  display: inline-flex;
  align-items: center;
  gap: 0.25rem;
  padding: 0.25rem 0.5rem;
  background: #d4e9f7;
  border-radius: 10px;
  font-size: 0.7rem;
  font-weight: 600;
  color: #1565c0;
  border: 1px solid #b3d9f2;
}

.time-display {
  font-size: 0.8rem;
  color: #444;
  font-weight: 500;
}

.col-actions {
  display: flex;
  gap: 0.5rem;
  justify-content: center;
}

.btn-delete-icon {
  width: 28px;
  height: 28px;
  border: none;
  background: transparent;
  cursor: pointer;
  font-size: 0.95rem;
  border-radius: 6px;
  transition: all 0.2s;
  display: flex;
  align-items: center;
  justify-content: center;
  padding: 0;
}

.btn-delete-icon:hover {
  background: #fee;
  transform: scale(1.1);
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
    color: #f6f6f6;
    background-color: #2f2f2f;
  }

  .sidebar {
    background-color: #1a1a1a;
    border-right-color: #333;
  }

  .sidebar-header {
    border-bottom-color: #333;
  }

  .sidebar-header h2 {
    color: #f6f6f6;
  }

  .category-item:hover {
    background: #2a2a2a;
  }

  .add-category-section {
    border-top-color: #333;
  }

  .main-content {
    background: #2f2f2f;
  }

  .content-header {
    background: #1a1a1a;
    border-bottom-color: #333;
  }

  .content-header h1 {
    color: #f6f6f6;
  }

  .reminder-count {
    color: #999;
  }

  .reminder-form,
  .reminders-section {
    background-color: #1a1a1a;
    color: #f6f6f6;
  }

  .reminder-form h2 {
    color: #f6f6f6;
  }

  .input-field,
  .category-input {
    background-color: #2a2a2a;
    color: #f6f6f6;
    border-color: #444;
  }

  .categories h3 {
    color: #999;
  }

  .table-header {
    background: #222;
    border-bottom-color: #333;
    color: #aaa;
  }

  .table-row {
    border-bottom-color: #2a2a2a;
  }

  .table-row:hover {
    background: #222;
  }

  .col-title h3 {
    color: #f6f6f6;
  }

  .col-title .description {
    color: #999;
  }

  .time-display {
    color: #aaa;
  }

  .frequency-badge {
    background: #2a3f4a;
    color: #6cb8e6;
    border-color: #3a5060;
  }

  .checkbox-btn {
    background: #2a2a2a;
    border-color: #555;
  }

  .checkbox-btn:hover {
    background: #2f3f2f;
    border-color: #4caf50;
  }

  .btn-delete-icon:hover {
    background: #3a1a1a;
  }

  .empty-state {
    color: #888;
  }

  .empty-state small {
    color: #666;
  }
}

</style>