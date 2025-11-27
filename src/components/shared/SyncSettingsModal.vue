<script setup lang="ts">
/**
 * SyncSettingsModal Component
 * Modal for configuring sync settings
 */

import { defineProps, defineEmits } from 'vue';
import type { SyncSettings } from '../../types/reminder';

const props = defineProps<{
  visible: boolean;
  syncSettings: SyncSettings;
  syncStatus: string;
  isSyncing: boolean;
}>();

const emit = defineEmits<{
  (e: 'close'): void;
  (e: 'update:syncSettings', value: SyncSettings): void;
  (e: 'save'): void;
  (e: 'test-connection'): void;
  (e: 'sync-to-github'): void;
  (e: 'sync-from-github'): void;
}>();

function formatLastSync(lastSync: string | null | undefined): string {
  if (!lastSync) return "Never";
  try {
    const date = new Date(lastSync);
    return date.toLocaleString();
  } catch {
    return "Invalid date";
  }
}

function updateSetting<K extends keyof SyncSettings>(key: K, value: SyncSettings[K]) {
  emit('update:syncSettings', { ...props.syncSettings, [key]: value });
}
</script>

<template>
  <div v-if="visible" class="modal-overlay" @click="$emit('close')">
    <div class="modal-content sync-settings-modal" @click.stop>
      <div class="modal-header">
        <h2>⚙️ Sync Settings</h2>
        <button @click="$emit('close')" class="btn-close">✕</button>
      </div>

      <div class="modal-body">
        <div class="settings-section">
          <label class="settings-toggle">
            <input 
              type="checkbox" 
              :checked="syncSettings.sync_enabled"
              @change="updateSetting('sync_enabled', ($event.target as HTMLInputElement).checked); $emit('save')"
            />
            <span class="settings-label">Enable Sync</span>
          </label>
          <p class="settings-help">Sync your reminders with GitHub</p>
        </div>

        <div class="settings-section" v-if="syncSettings.sync_enabled">
          <label class="settings-label-text">Data Source</label>
          <select 
            :value="syncSettings.data_source"
            @change="updateSetting('data_source', ($event.target as HTMLSelectElement).value)"
            class="settings-select"
          >
            <option value="local">Local Only</option>
            <option value="github">GitHub</option>
          </select>
        </div>

        <div v-if="syncSettings.sync_enabled && syncSettings.data_source === 'github'">
          <div class="settings-section">
            <label class="settings-label-text">Sync Method</label>
            <select 
              :value="syncSettings.sync_method"
              @change="updateSetting('sync_method', ($event.target as HTMLSelectElement).value)"
              class="settings-select"
            >
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
              :value="syncSettings.github_token || ''"
              @input="updateSetting('github_token', ($event.target as HTMLInputElement).value)"
              class="settings-input"
              placeholder="ghp_xxxxxxxxxxxxxxxxxxxxx"
            />
            <p class="settings-help">
              Generate at: <a href="https://github.com/settings/tokens/new?scopes=repo,gist" target="_blank">GitHub Settings</a>
              <br/>Required scopes: <code>repo</code>, <code>gist</code>
            </p>
            <button @click="$emit('test-connection')" class="btn-test-connection">
              Test Connection
            </button>
          </div>

          <div class="settings-section" v-if="syncSettings.sync_method === 'repo_json'">
            <label class="settings-label-text">Repository</label>
            <input 
              type="text"
              :value="syncSettings.github_repo || ''"
              @input="updateSetting('github_repo', ($event.target as HTMLInputElement).value)"
              class="settings-input"
              placeholder="username/repository"
            />
            <p class="settings-help">Format: owner/repo (e.g., "john/my-reminders")</p>
          </div>

          <div class="settings-section">
            <label class="settings-toggle">
              <input 
                type="checkbox" 
                :checked="syncSettings.auto_sync"
                @change="updateSetting('auto_sync', ($event.target as HTMLInputElement).checked)"
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
          <button @click="$emit('sync-to-github')" class="btn-sync" :disabled="isSyncing">
            {{ isSyncing ? '⏳ Syncing...' : '⬆️ Sync to GitHub' }}
          </button>
          <button 
            @click="$emit('sync-from-github')" 
            class="btn-sync" 
            :disabled="isSyncing || syncSettings.sync_method !== 'repo_json'"
          >
            {{ isSyncing ? '⏳ Validating...' : '⬇️ Validate GitHub Data' }}
          </button>
        </div>
        <button @click="$emit('save')" class="btn-save-settings">
          Save Settings
        </button>
        <button @click="$emit('close')" class="btn-cancel-settings">
          Close
        </button>
      </div>
    </div>
  </div>
</template>

<style scoped>
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

/* Dark mode */
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

  .btn-close {
    background: #2c2c2e;
    color: #f5f5f7;
  }

  .btn-close:hover {
    background: #3a3a3c;
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
}
</style>
