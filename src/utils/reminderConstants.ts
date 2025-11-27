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

// Reminder data format constants for encoding/decoding
// Pattern to match base64-encoded reminder data in inline code format
export const REMINDER_DATA_PATTERN = /\[reminder:([A-Za-z0-9+/]+=*)\]/;

// Prefix used in the inline code format
export const REMINDER_DATA_PREFIX = '[reminder:';
export const REMINDER_DATA_SUFFIX = ']';

// Legacy HTML comment pattern for backward compatibility
export const REMINDER_LEGACY_PATTERN = /<!--\s*reminder-data:([\s\S]*?)-->/;

/**
 * Encode reminder data to base64 string
 */
export function encodeReminderData(data: ReminderData): string {
  return btoa(JSON.stringify(data));
}

/**
 * Decode reminder data from base64 string
 * Returns null if decoding fails
 */
export function decodeReminderData(encoded: string): ReminderData | null {
  try {
    const decoded = atob(encoded);
    return JSON.parse(decoded) as ReminderData;
  } catch {
    return null;
  }
}

/**
 * Format reminder data as inline code marker
 */
export function formatReminderMarker(data: ReminderData): string {
  return `${REMINDER_DATA_PREFIX}${encodeReminderData(data)}${REMINDER_DATA_SUFFIX}`;
}
