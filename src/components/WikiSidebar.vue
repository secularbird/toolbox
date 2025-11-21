<script setup lang="ts">
import { ref, computed, watch } from 'vue';
import type { WikiPageList } from '../composables/useWikiStore';
import SectionNode from './SectionNode.vue';

const props = defineProps<{
  pages: WikiPageList[];
  currentPageId?: string;
  loading?: boolean;
  externalSearch?: boolean;
  availableTags?: string[];
  tagFilter?: string;
  sections?: Array<{ id: string; name: string; parent_id?: string | null }>;
  selectedSectionId?: string | null;
}>();

const emit = defineEmits<{
  selectPage: [id: string];
  createPage: [];
  search: [query: string];
  'update:tagFilter': [tag: string];
  selectSection: [id: string | null];
  addSection: [parentId: string | null];
  renameSection: [id: string];
  deleteSection: [id: string];
}>();

const searchQuery = ref('');

const filteredPages = computed(() => {
  if (props.externalSearch) {
    return props.pages;
  }
  if (!searchQuery.value.trim()) {
    return props.pages;
  }
  const query = searchQuery.value.toLowerCase();
  return props.pages.filter((page) =>
    page.title.toLowerCase().includes(query) ||
    page.tags.some((tag) => tag.toLowerCase().includes(query))
  );
});

watch(searchQuery, (query) => {
  emit('search', query);
});

function formatDate(timestamp: number): string {
  const date = new Date(timestamp * 1000);
  const now = new Date();
  const diffMs = now.getTime() - date.getTime();
  const diffMins = Math.floor(diffMs / 60000);
  const diffHours = Math.floor(diffMs / 3600000);
  const diffDays = Math.floor(diffMs / 86400000);

  if (diffMins < 1) return 'Just now';
  if (diffMins < 60) return `${diffMins}m ago`;
  if (diffHours < 24) return `${diffHours}h ago`;
  if (diffDays < 7) return `${diffDays}d ago`;
  
  return date.toLocaleDateString('en-US', { month: 'short', day: 'numeric' });
}

function buildTree() {
  if (!props.sections) return [] as Array<{ id: string; name: string; parent_id?: string | null; children: any[] }>;
  const map = new Map<string, any>();
  props.sections.forEach((s) => {
    map.set(s.id, { ...s, children: [] });
  });
  const roots: any[] = [];
  map.forEach((node) => {
    if (node.parent_id && map.has(node.parent_id)) {
      map.get(node.parent_id).children.push(node);
    } else {
      roots.push(node);
    }
  });
  return roots;
}

const sectionTree = computed(buildTree);
</script>

<template>
  <div class="wiki-sidebar">
    <div class="sidebar-header">
      <h2>📚 Wiki Pages</h2>
      <button class="create-btn" @click="emit('createPage')" title="New Page">
        ➕
      </button>
    </div>

    <div class="search-box">
      <input
        v-model="searchQuery"
        type="text"
        placeholder="Search pages & tags..."
        class="search-input"
      />
      <select
        v-if="props.availableTags && props.availableTags.length"
        class="tag-filter"
        :value="props.tagFilter || ''"
        @change="emit('update:tagFilter', ($event.target as HTMLSelectElement).value)"
      >
        <option value="">All tags</option>
        <option v-for="tag in props.availableTags" :key="tag" :value="tag">#{{ tag }}</option>
      </select>
    </div>

    <div v-if="sectionTree.length" class="sections-tree">
      <div class="sections-header">
        <span>Sections</span>
        <button class="create-btn ghost" @click="emit('addSection', null)" title="Add section">+</button>
      </div>
      <div class="section-list">
        <SectionNode
          v-for="node in sectionTree"
          :key="node.id"
          :node="node"
          :current="props.selectedSectionId"
          @select="emit('selectSection', $event)"
          @add="emit('addSection', $event)"
          @rename="emit('renameSection', $event)"
          @delete="emit('deleteSection', $event)"
        />
      </div>
    </div>

    <div class="pages-list">
      <div v-if="loading" class="loading">Loading pages...</div>
      
      <div v-else-if="filteredPages.length === 0" class="empty-state">
        <p v-if="searchQuery">No matching pages</p>
        <p v-else>No wiki pages yet. Create your first!</p>
      </div>

      <div
        v-for="page in filteredPages"
        :key="page.id"
        class="page-item"
        :class="{ active: page.id === currentPageId }"
        @click="emit('selectPage', page.id)"
      >
        <div class="page-title">{{ page.title }}</div>
        <div class="page-meta">
          <span class="page-date">{{ formatDate(page.updated_at) }}</span>
          <div v-if="page.tags.length > 0" class="page-tags">
            <span v-for="tag in page.tags.slice(0, 2)" :key="tag" class="tag">
              #{{ tag }}
            </span>
            <span v-if="page.tags.length > 2" class="tag-more">
              +{{ page.tags.length - 2 }}
            </span>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<style scoped>
.wiki-sidebar {
  width: 260px;
  background: var(--sidebar-bg);
  border-right: 1px solid var(--border-color);
  display: flex;
  flex-direction: column;
  height: 100%;
}

.sidebar-header {
  padding: 16px;
  display: flex;
  align-items: center;
  justify-content: space-between;
  border-bottom: 1px solid var(--border-color);
}

.sidebar-header h2 {
  font-size: 18px;
  font-weight: 600;
  margin: 0;
  color: var(--text-primary);
}

.create-btn {
  background: var(--primary-color);
  color: white;
  border: none;
  width: 32px;
  height: 32px;
  border-radius: 8px;
  cursor: pointer;
  font-size: 16px;
  display: flex;
  align-items: center;
  justify-content: center;
  transition: all 0.2s;
}

.create-btn:hover {
  transform: scale(1.05);
  opacity: 0.9;
}

.search-box {
  padding: 12px 16px;
  border-bottom: 1px solid var(--border-color);
}

.search-input {
  width: 100%;
  padding: 8px 12px;
  border: 1px solid var(--border-color);
  border-radius: 8px;
  background: var(--input-bg);
  color: var(--text-primary);
  font-size: 14px;
  outline: none;
  transition: border-color 0.2s;
}

.search-input:focus {
  border-color: var(--primary-color);
}

.tag-filter {
  width: 100%;
  margin-top: 8px;
  padding: 8px 12px;
  border: 1px solid var(--border-color);
  border-radius: 8px;
  background: var(--input-bg);
  color: var(--text-primary);
  font-size: 13px;
  outline: none;
}

.sections-tree {
  padding: 8px 12px 4px;
  border-bottom: 1px solid var(--border-color);
}

.sections-header {
  display: flex;
  align-items: center;
  justify-content: space-between;
  font-size: 13px;
  color: var(--text-secondary);
  margin-bottom: 6px;
}

.section-list {
  display: flex;
  flex-direction: column;
  gap: 4px;
}

.pages-list {
  flex: 1;
  overflow-y: auto;
  padding: 8px;
}

.loading,
.empty-state {
  padding: 32px 16px;
  text-align: center;
  color: var(--text-secondary);
  font-size: 14px;
}

.page-item {
  padding: 12px;
  margin-bottom: 4px;
  border-radius: 8px;
  cursor: pointer;
  transition: background 0.15s;
}

.page-item:hover {
  background: var(--hover-bg);
}

.page-item.active {
  background: var(--primary-color);
  color: white;
}

.page-item.active .page-title {
  color: white;
}

.page-item.active .page-meta {
  color: rgba(255, 255, 255, 0.8);
}

.page-title {
  font-size: 15px;
  font-weight: 500;
  color: var(--text-primary);
  margin-bottom: 4px;
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
}

.page-meta {
  display: flex;
  align-items: center;
  gap: 8px;
  font-size: 12px;
  color: var(--text-secondary);
}

.page-date {
  flex-shrink: 0;
}

.page-tags {
  display: flex;
  gap: 4px;
  flex-wrap: wrap;
  overflow: hidden;
}

.tag {
  background: var(--tag-bg);
  padding: 2px 6px;
  border-radius: 4px;
  font-size: 11px;
  color: var(--text-secondary);
}

.page-item.active .tag {
  background: rgba(255, 255, 255, 0.2);
  color: white;
}

.tag-more {
  font-size: 11px;
  color: var(--text-secondary);
}

/* Dark mode */
@media (prefers-color-scheme: dark) {
  .wiki-sidebar {
    --sidebar-bg: #1c1c1e;
    --border-color: #38383a;
    --text-primary: #f5f5f7;
    --text-secondary: #98989d;
    --primary-color: #0a84ff;
    --hover-bg: #2c2c2e;
    --input-bg: #2c2c2e;
    --tag-bg: rgba(255, 255, 255, 0.1);
  }
}

/* Light mode */
@media (prefers-color-scheme: light) {
  .wiki-sidebar {
    --sidebar-bg: #f5f5f7;
    --border-color: #e5e5ea;
    --text-primary: #1d1d1f;
    --text-secondary: #86868b;
    --primary-color: #007aff;
    --hover-bg: #e8e8ed;
    --input-bg: white;
    --tag-bg: rgba(0, 0, 0, 0.06);
  }
}
</style>
