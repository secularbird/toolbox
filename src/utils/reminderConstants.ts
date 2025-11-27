// Shared reminder constants for consistent display across components

export interface ReminderCategory {
  icon: string;
  name: string;
  color: string;
}

export interface ReminderFrequency {
  icon: string;
  label: string;
}

// Category icons mapping
export const categoryIcons: Record<string, ReminderCategory> = {
  work: { icon: '💼', name: 'Work', color: '#ff9800' },
  personal: { icon: '👤', name: 'Personal', color: '#4caf50' },
  shopping: { icon: '🛒', name: 'Shopping', color: '#e91e63' },
  health: { icon: '🏥', name: 'Health', color: '#00bcd4' },
  other: { icon: '📌', name: 'Other', color: '#9c27b0' },
};

// Frequency labels mapping
export const frequencyLabels: Record<string, ReminderFrequency> = {
  once: { icon: '🔵', label: 'Once' },
  daily: { icon: '📅', label: 'Daily' },
  weekly: { icon: '📆', label: 'Weekly' },
  monthly: { icon: '🗓️', label: 'Monthly' },
  yearly: { icon: '📊', label: 'Yearly' },
};

// Reminder data interface for type safety
export interface ReminderData {
  id: number;
  title: string;
  description: string;
  category: string;
  frequency: string;
  time: string;
}
