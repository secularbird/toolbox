<script setup lang="ts">
import { ref } from 'vue';
import DiskScanner from './components/DiskScanner.vue';
import RemindersApp from './components/RemindersApp.vue';
import WikiApp from './components/WikiApp.vue';

const currentView = ref<'reminders' | 'diskscanner' | 'wiki'>('reminders');
</script>

<template>
  <div class="app-container">
    <nav class="app-nav">
      <button 
        @click="currentView = 'reminders'" 
        :class="{ active: currentView === 'reminders' }"
        class="nav-btn"
      >
        📝 Reminders
      </button>
      <button 
        @click="currentView = 'wiki'" 
        :class="{ active: currentView === 'wiki' }"
        class="nav-btn"
      >
        📚 Wiki
      </button>
      <button 
        @click="currentView = 'diskscanner'" 
        :class="{ active: currentView === 'diskscanner' }"
        class="nav-btn"
      >
        💾 Disk Scanner
      </button>
    </nav>

    <div class="view-container">
      <RemindersApp v-if="currentView === 'reminders'" />
      <WikiApp v-else-if="currentView === 'wiki'" />
      <DiskScanner v-else-if="currentView === 'diskscanner'" />
    </div>
  </div>
</template>

<style scoped>
.app-container {
  height: 100vh;
  display: flex;
  flex-direction: column;
  background: #f5f5f5;
}

.app-nav {
  display: flex;
  background: white;
  border-bottom: 1px solid #ddd;
  padding: 0;
}

.nav-btn {
  padding: 15px 30px;
  background: transparent;
  border: none;
  border-bottom: 3px solid transparent;
  cursor: pointer;
  font-size: 16px;
  font-weight: 500;
  transition: all 0.2s;
  color: #666;
}

.nav-btn:hover {
  background: #f9f9f9;
  color: #333;
}

.nav-btn.active {
  border-bottom-color: #0066cc;
  color: #0066cc;
}

.view-container {
  flex: 1;
  overflow: hidden;
  background: #f5f5f5;
}

.reminders-placeholder {
  display: flex;
  align-items: center;
  justify-content: center;
  height: 100%;
  color: #999;
  font-size: 18px;
}
</style>
