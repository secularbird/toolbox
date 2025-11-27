/**
 * useSyncSettings Composable
 * Handles sync settings and GitHub synchronization
 */

import { ref } from 'vue';
import { invoke } from '@tauri-apps/api/core';
import type { SyncSettings } from '../types/reminder';

// Default sync settings used until backend settings are loaded.
// The id field is used by the backend database; actual values
// are loaded via loadSyncSettings().
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

export function useSyncSettings() {
  // Load sync settings from backend
  async function loadSyncSettings() {
    try {
      const settings = await invoke<SyncSettings>("get_sync_settings");
      syncSettings.value = settings;
      console.log("Sync settings loaded:", settings);
    } catch (error) {
      console.error("Failed to load sync settings:", error);
    }
  }

  // Save sync settings to backend
  async function saveSyncSettings() {
    try {
      await invoke("save_sync_settings", { settings: syncSettings.value });
      syncStatus.value = "Settings saved successfully!";
      setTimeout(() => (syncStatus.value = ""), 3000);
      return true;
    } catch (error) {
      syncStatus.value = `Error: ${error}`;
      return false;
    }
  }

  // Test GitHub connection
  async function testGitHubConnection() {
    if (!syncSettings.value.github_token) {
      syncStatus.value = "Please enter a GitHub token";
      return false;
    }

    try {
      syncStatus.value = "Testing connection...";
      const result = await invoke<boolean>("test_github_connection", {
        token: syncSettings.value.github_token,
      });
      if (result) {
        syncStatus.value = "✓ Connection successful!";
        setTimeout(() => (syncStatus.value = ""), 3000);
        return true;
      }
      return false;
    } catch (error) {
      syncStatus.value = `✗ Connection failed: ${error}`;
      return false;
    }
  }

  // Sync to GitHub
  async function syncToGitHub() {
    if (!syncSettings.value.sync_enabled) {
      syncStatus.value = "Please enable sync first";
      return null;
    }

    try {
      isSyncing.value = true;
      syncStatus.value = "Syncing to GitHub...";
      const result = await invoke<string>("sync_to_github");
      syncStatus.value = `✓ Synced successfully! URL: ${result}`;
      await loadSyncSettings();
      setTimeout(() => (syncStatus.value = ""), 5000);
      return result;
    } catch (error) {
      syncStatus.value = `✗ Sync failed: ${error}`;
      return null;
    } finally {
      isSyncing.value = false;
    }
  }

  // Sync from GitHub
  async function syncFromGitHub() {
    if (!syncSettings.value.sync_enabled) {
      syncStatus.value = "Please enable sync first";
      return null;
    }

    try {
      isSyncing.value = true;
      syncStatus.value = "Fetching from GitHub...";
      const count = await invoke<number>("sync_from_github");
      syncStatus.value = `✓ Found ${count} reminders in GitHub (validation successful). Note: Import functionality coming soon.`;
      await loadSyncSettings();
      setTimeout(() => (syncStatus.value = ""), 8000);
      return count;
    } catch (error) {
      syncStatus.value = `✗ Fetch failed: ${error}`;
      return null;
    } finally {
      isSyncing.value = false;
    }
  }

  // Format last sync time
  function formatLastSync(lastSync: string | null | undefined): string {
    if (!lastSync) return "Never";
    try {
      const date = new Date(lastSync);
      return date.toLocaleString();
    } catch {
      return "Invalid date";
    }
  }

  return {
    syncSettings,
    syncStatus,
    isSyncing,
    loadSyncSettings,
    saveSyncSettings,
    testGitHubConnection,
    syncToGitHub,
    syncFromGitHub,
    formatLastSync,
  };
}
