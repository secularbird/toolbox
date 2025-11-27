/**
 * useReminders Composable
 * Handles reminder CRUD operations and state management
 */

import { ref, computed } from 'vue';
import { invoke } from '@tauri-apps/api/core';
import { listen } from '@tauri-apps/api/event';
import type { Reminder, Category } from '../types/reminder';
import { SMART_LISTS, DEFAULT_CATEGORIES } from '../constants/reminder';

const reminders = ref<Reminder[]>([]);
const selectedCategory = ref("today");
const message = ref("");
// Note: Categories are initialized with defaults. User-added categories
// are stored in local state only. Backend persistence for custom categories
// is a future enhancement.
const categories = ref<Category[]>([...DEFAULT_CATEGORIES]);
const isInitialized = ref(false);

export function useReminders() {
  // Computed: Filtered reminders based on selected category
  const filteredReminders = computed(() => {
    const now = new Date();
    const today = new Date(now.getFullYear(), now.getMonth(), now.getDate());
    const tomorrow = new Date(today.getTime() + 24 * 60 * 60 * 1000);
    
    switch (selectedCategory.value) {
      case "today":
        return reminders.value.filter(r => {
          if (!r.completed) {
            const reminderDate = new Date(r.time);
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
        return reminders.value.filter(r => r.category === selectedCategory.value);
    }
  });

  // Computed: Category stats (count of reminders per category)
  const categoryStats = computed(() => {
    const now = new Date();
    const today = new Date(now.getFullYear(), now.getMonth(), now.getDate());
    const tomorrow = new Date(today.getTime() + 24 * 60 * 60 * 1000);
    const stats: Record<string, number> = {};
    
    stats["today"] = reminders.value.filter(r => {
      if (!r.completed) {
        const reminderDate = new Date(r.time);
        return reminderDate >= today && reminderDate < tomorrow;
      }
      return false;
    }).length;
    stats["scheduled"] = reminders.value.filter(r => r.time).length;
    stats["flagged"] = reminders.value.filter(r => r.flagged).length;
    stats["all"] = reminders.value.length;
    
    categories.value.forEach(cat => {
      stats[cat.id] = reminders.value.filter(r => r.category === cat.id).length;
    });
    
    return stats;
  });

  // Load all reminders from backend
  async function loadReminders() {
    try {
      reminders.value = await invoke("get_reminders");
    } catch (error) {
      message.value = `Error loading reminders: ${error}`;
    }
  }

  // Add a new reminder
  async function addReminder(
    title: string,
    description: string,
    time: string,
    category: string,
    frequency: string
  ) {
    if (!title) {
      message.value = "Please fill in title";
      return false;
    }
    
    let timeToUse = time;
    if (!timeToUse && selectedCategory.value === "today") {
      const now = new Date();
      const year = now.getFullYear();
      const month = String(now.getMonth() + 1).padStart(2, '0');
      const day = String(now.getDate()).padStart(2, '0');
      const hours = String(now.getHours()).padStart(2, '0');
      const minutes = String(now.getMinutes()).padStart(2, '0');
      timeToUse = `${year}-${month}-${day}T${hours}:${minutes}`;
    }
    
    if (!timeToUse) {
      message.value = "Please fill in time";
      return false;
    }
    
    try {
      await invoke("add_reminder", {
        title,
        description,
        time: timeToUse,
        category,
        frequency,
      });
      message.value = "Reminder added successfully!";
      return true;
    } catch (error) {
      message.value = `Error: ${error}`;
      return false;
    }
  }

  // Toggle reminder completion status
  async function toggleReminder(id: number) {
    try {
      await invoke("toggle_reminder", { id });
    } catch (error) {
      message.value = `Error: ${error}`;
    }
  }

  // Toggle flagged status
  // Note: This currently only updates local state. Backend persistence
  // for flagged status is TODO - see original component's comment.
  function toggleFlag(id: number) {
    const reminder = reminders.value.find(r => r.id === id);
    if (reminder) {
      reminder.flagged = !reminder.flagged;
      // TODO: Call backend to update flagged status when API is available
      // await invoke("update_reminder_flag", { id, flagged: reminder.flagged });
    }
  }

  // Update a reminder
  async function updateReminder(
    id: number,
    title: string,
    description: string,
    time: string,
    category: string,
    frequency: string
  ) {
    try {
      await invoke("update_reminder", {
        id,
        title,
        description,
        time,
        category,
        frequency,
      });
      message.value = "Reminder updated successfully!";
      setTimeout(() => { message.value = ""; }, 2000);
      return true;
    } catch (error) {
      message.value = `Error: ${error}`;
      return false;
    }
  }

  // Delete a reminder
  async function deleteReminder(id: number) {
    try {
      await invoke("delete_reminder", { id });
    } catch (error) {
      message.value = `Error: ${error}`;
    }
  }

  // Add a new category
  function addCategory(name: string): Category | null {
    if (!name.trim()) return null;
    
    const newCategory: Category = {
      id: name.toLowerCase().replace(/\s+/g, "-"),
      name: name,
      icon: "📁",
      color: "#" + Math.floor(Math.random() * 16777215).toString(16),
    };
    
    categories.value.push(newCategory);
    return newCategory;
  }

  // Select a category
  function selectCategory(categoryId: string) {
    selectedCategory.value = categoryId;
  }

  // Initialize: Load reminders and set up event listener
  async function initialize() {
    if (isInitialized.value) return;
    
    await loadReminders();
    
    await listen<Reminder[]>('reminders-updated', (event) => {
      console.log('[REMINDERS] Received reminders-updated event:', event.payload);
      reminders.value = event.payload;
      message.value = "Reminders updated in real-time ✨";
      setTimeout(() => { message.value = ""; }, 2000);
    });
    
    isInitialized.value = true;
    console.log('[REMINDERS] Event listener setup complete');
  }

  // Clear message after timeout
  function clearMessage(timeout = 2000) {
    setTimeout(() => { message.value = ""; }, timeout);
  }

  return {
    // State
    reminders,
    selectedCategory,
    message,
    categories,
    smartLists: SMART_LISTS,
    
    // Computed
    filteredReminders,
    categoryStats,
    
    // Actions
    loadReminders,
    addReminder,
    toggleReminder,
    toggleFlag,
    updateReminder,
    deleteReminder,
    addCategory,
    selectCategory,
    initialize,
    clearMessage,
  };
}
