<template>
  <div class="disk-scanner">
    <div class="header">
      <h1>💾 Disk Space Scanner</h1>
      <p>Scan and analyze disk space usage of folders and files</p>
    </div>

    <div class="controls">
      <div class="path-selector">
        <input 
          v-model="scanPath" 
          type="text" 
          placeholder="Enter path to scan..."
          @keyup.enter="startScan"
          class="path-input"
        />
        <button @click="loadHomeDirectory" class="btn btn-secondary">Home</button>
        <button @click="loadRoots" class="btn btn-secondary">Roots</button>
      </div>

      <div class="scan-options">
        <label>
          <input type="checkbox" v-model="limitDepth" />
          Limit depth
        </label>
        <input 
          v-if="limitDepth" 
          v-model.number="maxDepth" 
          type="number" 
          min="1" 
          max="10"
          class="depth-input"
        />
        <button 
          @click="startScan" 
          :disabled="isScanning || !scanPath"
          class="btn btn-primary"
        >
          {{ isScanning ? 'Scanning...' : 'Scan' }}
        </button>
      </div>
    </div>

    <div v-if="roots.length > 0" class="roots-list">
      <h3>Available Roots:</h3>
      <div class="roots">
        <button 
          v-for="root in roots" 
          :key="root"
          @click="selectRoot(root)"
          class="root-btn"
        >
          {{ root }}
        </button>
      </div>
    </div>

    <div v-if="error" class="error">
      ⚠️ {{ error }}
    </div>

    <div v-if="isScanning" class="loading">
      <div class="spinner"></div>
      <p>Scanning directory...</p>
    </div>

    <div v-if="scanResult && !isScanning" class="results">
      <div class="summary">
        <h2>📊 Scan Results</h2>
        <div class="total-size">
          Total Size: <strong>{{ formatBytes(scanResult.size) }}</strong>
        </div>
      </div>

      <DiskItem :item="scanResult" :maxSize="scanResult.size" />
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref } from 'vue';
import { invoke } from '@tauri-apps/api/core';
import DiskItem from './DiskItem.vue';

interface DiskItemData {
  name: string;
  path: string;
  size: number;
  is_dir: boolean;
  children?: DiskItemData[];
  error?: string;
}

const scanPath = ref('');
const maxDepth = ref(3);
const limitDepth = ref(true);
const isScanning = ref(false);
const scanResult = ref<DiskItemData | null>(null);
const error = ref('');
const roots = ref<string[]>([]);

async function startScan() {
  if (!scanPath.value) return;
  
  isScanning.value = true;
  error.value = '';
  scanResult.value = null;

  try {
    const result = await invoke<DiskItemData>('scan_directory', {
      path: scanPath.value,
      maxDepth: limitDepth.value ? maxDepth.value : null
    });
    scanResult.value = result;
  } catch (e) {
    error.value = String(e);
  } finally {
    isScanning.value = false;
  }
}

async function loadHomeDirectory() {
  try {
    const home = await invoke<string>('get_home_directory');
    scanPath.value = home;
  } catch (e) {
    error.value = String(e);
  }
}

async function loadRoots() {
  try {
    roots.value = await invoke<string[]>('get_system_roots');
  } catch (e) {
    error.value = String(e);
  }
}

function selectRoot(root: string) {
  scanPath.value = root;
  roots.value = [];
}

function formatBytes(bytes: number): string {
  const units = ['B', 'KB', 'MB', 'GB', 'TB'];
  if (bytes === 0) return '0 B';
  
  const i = Math.floor(Math.log(bytes) / Math.log(1024));
  return `${(bytes / Math.pow(1024, i)).toFixed(2)} ${units[i]}`;
}
</script>

<style scoped>
.disk-scanner {
  padding: 20px;
  max-width: 1200px;
  margin: 0 auto;
  height: 100%;
  overflow-y: auto;
}

.header {
  text-align: center;
  margin-bottom: 30px;
}

.header h1 {
  margin: 0;
  font-size: 2em;
  color: #333;
}

.header p {
  color: #666;
  margin-top: 10px;
}

.controls {
  background: white;
  padding: 20px;
  border-radius: 8px;
  box-shadow: 0 2px 8px rgba(0,0,0,0.1);
  margin-bottom: 20px;
}

.path-selector {
  display: flex;
  gap: 10px;
  margin-bottom: 15px;
}

.path-input {
  flex: 1;
  padding: 10px;
  border: 1px solid #ddd;
  border-radius: 4px;
  font-size: 14px;
}

.scan-options {
  display: flex;
  align-items: center;
  gap: 10px;
}

.depth-input {
  width: 60px;
  padding: 5px;
  border: 1px solid #ddd;
  border-radius: 4px;
}

.btn {
  padding: 10px 20px;
  border: none;
  border-radius: 4px;
  cursor: pointer;
  font-size: 14px;
  font-weight: 500;
  transition: all 0.2s;
}

.btn:disabled {
  opacity: 0.5;
  cursor: not-allowed;
}

.btn-primary {
  background: #0066cc;
  color: white;
}

.btn-primary:hover:not(:disabled) {
  background: #0052a3;
}

.btn-secondary {
  background: #f0f0f0;
  color: #333;
}

.btn-secondary:hover {
  background: #e0e0e0;
}

.roots-list {
  background: white;
  padding: 15px;
  border-radius: 8px;
  box-shadow: 0 2px 8px rgba(0,0,0,0.1);
  margin-bottom: 20px;
}

.roots-list h3 {
  margin-top: 0;
  margin-bottom: 10px;
}

.roots {
  display: flex;
  gap: 10px;
  flex-wrap: wrap;
}

.root-btn {
  padding: 8px 16px;
  background: #f5f5f5;
  border: 1px solid #ddd;
  border-radius: 4px;
  cursor: pointer;
  transition: all 0.2s;
}

.root-btn:hover {
  background: #e8e8e8;
  border-color: #999;
}

.error {
  background: #fee;
  color: #c00;
  padding: 15px;
  border-radius: 8px;
  margin-bottom: 20px;
}

.loading {
  text-align: center;
  padding: 40px;
}

.spinner {
  width: 50px;
  height: 50px;
  border: 4px solid #f3f3f3;
  border-top: 4px solid #0066cc;
  border-radius: 50%;
  animation: spin 1s linear infinite;
  margin: 0 auto 20px;
}

@keyframes spin {
  0% { transform: rotate(0deg); }
  100% { transform: rotate(360deg); }
}

.results {
  background: white;
  padding: 20px;
  border-radius: 8px;
  box-shadow: 0 2px 8px rgba(0,0,0,0.1);
}

.summary {
  border-bottom: 2px solid #eee;
  padding-bottom: 15px;
  margin-bottom: 20px;
}

.summary h2 {
  margin: 0 0 10px 0;
  color: #333;
}

.total-size {
  font-size: 18px;
  color: #666;
}

.total-size strong {
  color: #0066cc;
  font-size: 24px;
}
</style>
