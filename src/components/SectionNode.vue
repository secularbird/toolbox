<script setup lang="ts">
import { computed, ref } from 'vue';

interface SectionNode {
  id: string;
  name: string;
  parent_id?: string | null;
  children?: SectionNode[];
}

defineOptions({ name: 'SectionNode' });

const props = defineProps<{
  node: SectionNode;
  current?: string | null;
}>();

const emit = defineEmits<{
  select: [id: string];
  add: [parentId: string];
  rename: [id: string];
  delete: [id: string];
}>();

const expanded = ref(true);

const hasChildren = computed(() => (props.node.children || []).length > 0);
const isActive = computed(() => props.current === props.node.id);

function toggle() {
  expanded.value = !expanded.value;
}
</script>

<template>
  <div class="section-node">
    <div class="section-row" :class="{ active: isActive }">
      <button v-if="hasChildren" class="toggle" @click="toggle">
        {{ expanded ? '▾' : '▸' }}
      </button>
      <span v-else class="toggle-placeholder">•</span>
      <button class="label" :class="{ active: isActive }" @click="emit('select', node.id)">
        {{ node.name }}
      </button>
      <div class="actions">
        <button class="ghost" @click="emit('add', node.id)" title="Add subsection">+</button>
        <button class="ghost" @click="emit('rename', node.id)" title="Rename">✎</button>
        <button class="ghost" @click="emit('delete', node.id)" title="Delete">🗑️</button>
      </div>
    </div>
    <div v-if="hasChildren && expanded" class="children">
      <SectionNode
        v-for="child in node.children"
        :key="child.id"
        :node="child"
        :current="current"
        @select="emit('select', $event)"
        @add="emit('add', $event)"
        @rename="emit('rename', $event)"
        @delete="emit('delete', $event)"
      />
    </div>
  </div>
</template>

<style scoped>
.section-node {
  display: flex;
  flex-direction: column;
}

.section-row {
  display: flex;
  align-items: center;
  gap: 6px;
  padding: 4px 6px;
  border-radius: 6px;
}

.section-row.active {
  background: var(--hover-bg);
}

.toggle,
.toggle-placeholder {
  width: 16px;
  border: none;
  background: transparent;
  color: var(--text-secondary);
  cursor: pointer;
  padding: 0;
}

.toggle-placeholder {
  text-align: center;
}

.label {
  flex: 1;
  text-align: left;
  background: transparent;
  border: none;
  color: var(--text-primary);
  cursor: pointer;
  font-size: 13px;
}

.label.active {
  color: var(--primary-color);
  font-weight: 600;
}

.actions {
  display: flex;
  gap: 4px;
}

.ghost {
  border: none;
  background: transparent;
  color: var(--text-secondary);
  cursor: pointer;
  font-size: 12px;
}

.ghost:hover {
  color: var(--primary-color);
}

.children {
  margin-left: 16px;
  display: flex;
  flex-direction: column;
  gap: 4px;
}
</style>
