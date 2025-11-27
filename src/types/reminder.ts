/**
 * Reminder Types
 * Shared type definitions for the Reminders feature
 */

export interface Reminder {
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

export interface Category {
  id: string;
  name: string;
  icon: string;
  color: string;
}

export interface Evidence {
  id: number;
  reminder_id: number;
  file_type: string;
  file_path: string;
  file_name: string;
  file_size: number;
  mime_type: string;
  thumbnail_path: string | null;
  description: string | null;
  metadata: string | null;
  created_at: string;
}

export interface SyncSettings {
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

export interface FrequencyOption {
  value: string;
  label: string;
  icon: string;
}
