<script setup lang="ts">
import { ref, onMounted, onUnmounted } from 'vue';

export interface ContextMenuItem {
  label: string;
  icon?: string;
  action: () => void;
  danger?: boolean;
  separator?: boolean;
}

interface Props {
  items: ContextMenuItem[];
  x: number;
  y: number;
}

defineProps<Props>();

const emit = defineEmits<{
  close: [];
}>();

const menuRef = ref<HTMLDivElement | null>(null);

onMounted(() => {
  document.addEventListener('click', handleClickOutside);
  document.addEventListener('contextmenu', handleClickOutside);
  adjustPosition();
});

onUnmounted(() => {
  document.removeEventListener('click', handleClickOutside);
  document.removeEventListener('contextmenu', handleClickOutside);
});

function handleClickOutside() {
  emit('close');
}

function handleItemClick(item: ContextMenuItem) {
  if (!item.separator) {
    item.action();
    emit('close');
  }
}

function adjustPosition() {
  if (!menuRef.value) return;
  
  const menu = menuRef.value;
  const rect = menu.getBoundingClientRect();
  const viewportWidth = window.innerWidth;
  const viewportHeight = window.innerHeight;
  
  // Adjust horizontal position if menu goes off-screen
  if (rect.right > viewportWidth) {
    menu.style.left = `${viewportWidth - rect.width - 10}px`;
  }
  
  // Adjust vertical position if menu goes off-screen
  if (rect.bottom > viewportHeight) {
    menu.style.top = `${viewportHeight - rect.height - 10}px`;
  }
}
</script>

<template>
  <div 
    ref="menuRef"
    class="context-menu" 
    :style="{ left: `${x}px`, top: `${y}px` }"
    @click.stop
    @contextmenu.prevent
  >
    <div
      v-for="(item, index) in items"
      :key="index"
      :class="['menu-item', { danger: item.danger, separator: item.separator }]"
      @click="handleItemClick(item)"
    >
      <template v-if="!item.separator">
        <span v-if="item.icon" class="item-icon">{{ item.icon }}</span>
        <span class="item-label">{{ item.label }}</span>
      </template>
    </div>
  </div>
</template>

<style scoped>
.context-menu {
  position: fixed;
  background: var(--panel-bg, #fff);
  border: 1px solid var(--border-color, #ddd);
  border-radius: 6px;
  box-shadow: 0 4px 12px rgba(0, 0, 0, 0.15);
  padding: 4px 0;
  min-width: 180px;
  z-index: 10000;
  user-select: none;
}

.menu-item {
  padding: 8px 12px;
  cursor: pointer;
  display: flex;
  align-items: center;
  gap: 8px;
  color: var(--text-color, #333);
  font-size: 14px;
  transition: background 0.15s;
}

.menu-item:not(.separator):hover {
  background: var(--hover-bg, #f0f0f0);
}

.menu-item.separator {
  height: 1px;
  background: var(--border-color, #ddd);
  margin: 4px 0;
  padding: 0;
  cursor: default;
}

.menu-item.danger {
  color: var(--error-color, #dc2626);
}

.menu-item.danger:hover {
  background: rgba(220, 38, 38, 0.1);
}

.item-icon {
  font-size: 16px;
  display: flex;
  align-items: center;
  justify-content: center;
  width: 20px;
}

.item-label {
  flex: 1;
}

@media (prefers-color-scheme: dark) {
  .context-menu {
    background: var(--panel-bg, #1a1a1a);
    border-color: var(--border-color, #333);
    box-shadow: 0 4px 12px rgba(0, 0, 0, 0.5);
  }
  
  .menu-item {
    color: var(--text-color, #e5e5e5);
  }
  
  .menu-item:not(.separator):hover {
    background: var(--hover-bg, #2a2a2a);
  }
  
  .menu-item.separator {
    background: var(--border-color, #333);
  }
}
</style>
