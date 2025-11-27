/**
 * Reminder Constants
 * Shared constants for the Reminders feature
 */

import type { Category, FrequencyOption } from '../types/reminder';

// Smart Lists (like macOS Reminders)
export const SMART_LISTS: Category[] = [
  { id: "today", name: "Today", icon: "📅", color: "#007aff" },
  { id: "scheduled", name: "Scheduled", icon: "📆", color: "#ff9500" },
  { id: "flagged", name: "Flagged", icon: "🚩", color: "#ff3b30" },
  { id: "all", name: "All", icon: "📋", color: "#8e8e93" },
];

// Default User Lists (categories)
export const DEFAULT_CATEGORIES: Category[] = [
  { id: "work", name: "Work", icon: "💼", color: "#ff9800" },
  { id: "personal", name: "Personal", icon: "👤", color: "#4caf50" },
  { id: "shopping", name: "Shopping", icon: "🛒", color: "#e91e63" },
  { id: "health", name: "Health", icon: "🏥", color: "#00bcd4" },
  { id: "other", name: "Other", icon: "📌", color: "#9c27b0" },
];

// Frequency options for recurring reminders
export const FREQUENCY_OPTIONS: FrequencyOption[] = [
  { value: "once", label: "Once", icon: "🔵" },
  { value: "daily", label: "Daily", icon: "📅" },
  { value: "weekly", label: "Weekly", icon: "📆" },
  { value: "monthly", label: "Monthly", icon: "🗓️" },
  { value: "yearly", label: "Yearly", icon: "📊" },
];
