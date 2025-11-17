<template>
  <div class="disk-item">
    <div class="item-header" @click="toggleExpanded">
      <div class="item-info">
        <span class="icon">{{ item.is_dir ? (expanded ? '📂' : '📁') : '📄' }}</span>
        <span class="name" :title="item.path">{{ item.name }}</span>
        <span v-if="item.error" class="error-badge" :title="item.error">⚠️</span>
      </div>
      <div class="item-size">
        <div class="size-bar" :style="{ width: sizePercentage + '%' }"></div>
        <span class="size-text">{{ formatBytes(item.size) }}</span>
        <span class="percentage">({{ sizePercentage.toFixed(1) }}%)</span>
      </div>
    </div>

    <div v-if="expanded && item.children && item.children.length > 0" class="children">
      <DiskItem 
        v-for="(child, index) in item.children" 
        :key="child.path + index"
        :item="child"
        :maxSize="item.size"
      />
    </div>

    <div v-if="expanded && item.children && item.children.length === 0" class="empty">
      <em>Empty directory</em>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed } from 'vue';

interface DiskItemData {
  name: string;
  path: string;
  size: number;
  is_dir: boolean;
  children?: DiskItemData[];
  error?: string;
}

const props = defineProps<{
  item: DiskItemData;
  maxSize: number;
}>();

const expanded = ref(false);

const sizePercentage = computed(() => {
  if (props.maxSize === 0) return 0;
  return (props.item.size / props.maxSize) * 100;
});

function toggleExpanded() {
  if (props.item.is_dir) {
    expanded.value = !expanded.value;
  }
}

function formatBytes(bytes: number): string {
  const units = ['B', 'KB', 'MB', 'GB', 'TB'];
  if (bytes === 0) return '0 B';
  
  const i = Math.floor(Math.log(bytes) / Math.log(1024));
  return `${(bytes / Math.pow(1024, i)).toFixed(2)} ${units[i]}`;
}
</script>

<style scoped>
.disk-item {
  margin: 2px 0;
}

.item-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 8px 10px;
  background: #f9f9f9;
  border-radius: 4px;
  cursor: pointer;
  transition: background 0.2s;
}

.item-header:hover {
  background: #f0f0f0;
}

.item-info {
  display: flex;
  align-items: center;
  gap: 8px;
  flex: 1;
  min-width: 0;
}

.icon {
  font-size: 18px;
  flex-shrink: 0;
}

.name {
  font-weight: 500;
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
}

.error-badge {
  font-size: 14px;
  cursor: help;
}

.item-size {
  display: flex;
  align-items: center;
  gap: 10px;
  position: relative;
  min-width: 200px;
}

.size-bar {
  position: absolute;
  left: 0;
  top: 0;
  height: 100%;
  background: linear-gradient(90deg, #e3f2fd, #2196f3);
  border-radius: 4px;
  opacity: 0.3;
  transition: width 0.3s;
}

.size-text {
  font-weight: 600;
  color: #333;
  z-index: 1;
  position: relative;
}

.percentage {
  font-size: 12px;
  color: #666;
  z-index: 1;
  position: relative;
}

.children {
  margin-left: 20px;
  border-left: 2px solid #e0e0e0;
  padding-left: 10px;
  margin-top: 4px;
}

.empty {
  margin-left: 30px;
  padding: 10px;
  color: #999;
  font-size: 14px;
}
</style>
