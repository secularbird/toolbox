<script setup lang="ts">
import { computed, onMounted, ref, watch } from 'vue';
import WikiSidebar from './WikiSidebar.vue';
import WikiEditor from './WikiEditor.vue';
import WikiPreview from './WikiPreview.vue';
import WikiMetadata from './WikiMetadata.vue';
import { useWiki } from '../composables/useWikiStore';
import type { WikiPage, WikiRevisionMeta, WikiPageList } from '../composables/useWikiStore';

const {
  pages,
  currentPage,
  isLoading,
  error,
  loadPages,
  loadPage,
  createPage,
  updatePage,
  deletePage,
  searchPages,
  clearCurrentPage,
  listRevisions,
  restoreRevision,
  sections,
  loadSections,
  createSection,
  updateSection,
  deleteSection,
} = useWiki();

const editorTitle = ref('');
const editorContent = ref('');
const editorTags = ref<string[]>([]);
const displayedPages = ref(pages.value);
const saving = ref(false);
const autoSaving = ref(false);
const message = ref('');
const formError = ref('');
const searchQuery = ref('');
const tagFilter = ref('');
const selectedSectionId = ref<string | null>(null);
const unsavedChanges = ref(false);
const revisions = ref<WikiRevisionMeta[]>([]);
const isHydrating = ref(false);
let autosaveTimer: number | null = null;

const availableTags = computed(() => {
  const set = new Set<string>();
  pages.value.forEach((p) => p.tags.forEach((t) => set.add(t)));
  return Array.from(set).sort((a, b) => a.localeCompare(b));
});

const breadcrumbs = computed(() => {
  const title = editorTitle.value.trim() || 'Untitled';
  const sectionName = currentSectionName.value || 'Section';
  return hasPageSelected.value ? `Home / Wiki / ${sectionName} / ${title}` : `Home / Wiki / ${sectionName}`;
});

const currentSectionName = computed(() => {
  if (!selectedSectionId.value) return 'Notebook';
  const sec = sections.value.find((s) => s.id === selectedSectionId.value);
  return sec ? sec.name : 'Notebook';
});

const selectedPageId = computed(() => currentPage.value?.id || '');
const hasPageSelected = computed(() => Boolean(currentPage.value));

onMounted(async () => {
  await bootstrapSections();
  await bootstrapPages();
  if (!selectedSectionId.value && sections.value.length) {
    selectedSectionId.value = sections.value[0].id;
  }
});

watch(pages, (newPages) => {
  if (!searchQuery.value.trim()) {
    displayedPages.value = newPages;
  }
});

watch(currentPage, (page) => {
  if (page) {
    hydrateFromPage(page);
    loadRevisionsForPage(page.id);
  }
});

watch([editorTitle, editorContent, editorTags], () => {
  if (!hasPageSelected.value || isHydrating.value) return;
  markDirtyAndScheduleAutosave();
});

function clearEditor() {
  editorTitle.value = '';
  editorContent.value = '';
  editorTags.value = [];
  clearCurrentPage();
  unsavedChanges.value = false;
  if (autosaveTimer) {
    clearTimeout(autosaveTimer);
    autosaveTimer = null;
  }
  revisions.value = [];
}

async function bootstrapPages() {
  await loadPages();
  await applyFilters();
  if (displayedPages.value.length) {
    await selectPage(displayedPages.value[0].id);
  } else {
    clearEditor();
  }
}

async function bootstrapSections() {
  await loadSections();
  if (!selectedSectionId.value && sections.value.length) {
    selectedSectionId.value = sections.value[0].id;
  }
}

async function selectPage(id: string) {
  message.value = '';
  formError.value = '';
  await loadPage(id);
}

function hydrateFromPage(page: WikiPage) {
  isHydrating.value = true;
  editorTitle.value = page.title;
  editorContent.value = page.content;
  editorTags.value = [...page.tags];
  unsavedChanges.value = false;
  if (autosaveTimer) {
    clearTimeout(autosaveTimer);
    autosaveTimer = null;
  }
  setTimeout(() => (isHydrating.value = false), 0);
}

async function handleCreatePage() {
  saving.value = true;
  formError.value = '';
  try {
    const page = await createPage('Untitled Page', '# New Page\n', [], selectedSectionId.value || undefined);
    await applyFilters();
    await selectPage(page.id);
    message.value = 'Page created';
  } catch (e) {
    formError.value = String(e);
  } finally {
    saving.value = false;
  }
}

async function handleSave() {
  if (!currentPage.value) return;
  saving.value = true;
  formError.value = '';
  try {
    const title = editorTitle.value.trim() || 'Untitled Page';
    await updatePage(currentPage.value.id, title, editorContent.value, editorTags.value, selectedSectionId.value || currentPage.value.section_id || undefined);
    await applyFilters();
    message.value = 'Saved';
    unsavedChanges.value = false;
    if (autosaveTimer) {
      clearTimeout(autosaveTimer);
      autosaveTimer = null;
    }
    await loadRevisionsForPage(currentPage.value.id);
  } catch (e) {
    formError.value = String(e);
  } finally {
    saving.value = false;
    setTimeout(() => (message.value = ''), 1200);
  }
}

async function handleDelete() {
  if (!currentPage.value) return;
  saving.value = true;
  formError.value = '';
  try {
    await deletePage(currentPage.value.id);
    await applyFilters();
    if (displayedPages.value.length) {
      await selectPage(displayedPages.value[0].id);
    } else {
      clearEditor();
    }
    message.value = 'Page deleted';
  } catch (e) {
    formError.value = String(e);
  } finally {
    saving.value = false;
  }
}

async function handleSearch(query: string) {
  searchQuery.value = query;
  await applyFilters();
}

async function handleTagFilterChange(tag: string) {
  tagFilter.value = tag;
  await applyFilters();
}

function updateTags(tags: string[]) {
  editorTags.value = tags;
}

function markDirtyAndScheduleAutosave() {
  if (autosaveTimer) {
    clearTimeout(autosaveTimer);
  }
  unsavedChanges.value = true;
  autosaveTimer = window.setTimeout(() => {
    void handleAutosave();
  }, 1500);
}

async function handleAutosave() {
  if (!currentPage.value || saving.value || autoSaving.value) return;
  autoSaving.value = true;
  formError.value = '';
  try {
    const title = editorTitle.value.trim() || 'Untitled Page';
    await updatePage(currentPage.value.id, title, editorContent.value, editorTags.value, selectedSectionId.value || currentPage.value.section_id || undefined);
    unsavedChanges.value = false;
    message.value = 'Autosaved';
    await applyFilters();
    await loadRevisionsForPage(currentPage.value.id);
  } catch (e) {
    formError.value = String(e);
  } finally {
    autoSaving.value = false;
    if (autosaveTimer) {
      clearTimeout(autosaveTimer);
      autosaveTimer = null;
    }
    setTimeout(() => (message.value = ''), 1200);
  }
}

async function loadRevisionsForPage(pageId: string) {
  revisions.value = await listRevisions(pageId);
}

async function applyFilters() {
  let base: WikiPageList[];
  if (searchQuery.value.trim()) {
    base = await searchPages(searchQuery.value.trim());
  } else {
    base = pages.value;
  }

  if (tagFilter.value) {
    base = base.filter((p) => p.tags.includes(tagFilter.value));
  }

  if (selectedSectionId.value) {
    const target = selectedSectionId.value;
    base = base.filter((p) => {
      if (p.section_id) return p.section_id === target;
      return target === 'root';
    });
  }

  displayedPages.value = base;
}

async function handleRestoreRevision(revisionId: string) {
  if (!currentPage.value) return;
  saving.value = true;
  formError.value = '';
  try {
    const restored = await restoreRevision(currentPage.value.id, revisionId);
    hydrateFromPage(restored);
    selectedSectionId.value = restored.section_id || selectedSectionId.value;
    await loadRevisionsForPage(restored.id);
    message.value = 'Revision restored';
  } catch (e) {
    formError.value = String(e);
  } finally {
    saving.value = false;
  }
}

function handleSelectSection(id: string | null) {
  selectedSectionId.value = id;
  void applyFilters();
}

async function handleAddSection(parentId: string | null) {
  const name = prompt('New section name', 'New Section') || 'New Section';
  try {
    const sec = await createSection(name, parentId || undefined);
    selectedSectionId.value = sec.id;
    await applyFilters();
  } catch (e) {
    formError.value = String(e);
  }
}

async function handleRenameSection(id: string) {
  const current = sections.value.find((s) => s.id === id)?.name || '';
  const name = prompt('Rename section', current || 'Section');
  if (!name) return;
  try {
    await updateSection(id, name);
  } catch (e) {
    formError.value = String(e);
  }
}

async function handleDeleteSection(id: string) {
  if (!confirm('Delete this section? Child sections/pages must be moved first.')) return;
  try {
    await deleteSection(id);
    if (selectedSectionId.value === id) {
      selectedSectionId.value = sections.value[0]?.id || null;
      await applyFilters();
    }
  } catch (e) {
    formError.value = String(e);
  }
}
</script>

<template>
  <div class="wiki-app">
    <div class="wiki-shell">
      <header class="wiki-topbar">
        <div class="wiki-topbar-left">
          <div class="title-chip">📚 Wiki</div>
          <button class="topbar-btn" @click="handleCreatePage" :disabled="saving || isLoading">
            + New Page
          </button>
          <button class="topbar-btn primary" @click="handleSave" :disabled="!hasPageSelected || saving">
            {{ saving ? 'Saving…' : 'Save' }}
          </button>
        </div>
        <div class="status-text">
          <span v-if="error">{{ error }}</span>
          <span v-else-if="formError">{{ formError }}</span>
          <span v-else-if="autoSaving">Autosaving…</span>
          <span v-else-if="unsavedChanges">Unsaved changes</span>
          <span v-else-if="message">{{ message }}</span>
        </div>
      </header>

      <div class="wiki-layout">
        <WikiSidebar
          :pages="displayedPages"
          :current-page-id="selectedPageId"
          :loading="isLoading || saving"
          external-search
          :available-tags="availableTags"
          :tag-filter="tagFilter"
          :sections="sections"
          :selected-section-id="selectedSectionId"
          @selectPage="selectPage"
          @createPage="handleCreatePage"
          @search="handleSearch"
          @update:tagFilter="handleTagFilterChange"
          @selectSection="handleSelectSection"
          @addSection="handleAddSection"
          @renameSection="handleRenameSection"
          @deleteSection="handleDeleteSection"
        />

        <div class="wiki-editor-panel">
          <div class="title-row" v-if="hasPageSelected">
            <input
              v-model="editorTitle"
              class="title-input"
              placeholder="Page title"
              :disabled="saving"
            />
            <div class="title-meta">
              <span v-if="currentPage">Updated {{ new Date(currentPage.updated_at * 1000).toLocaleString() }}</span>
            </div>
            <div class="breadcrumbs">{{ breadcrumbs }}</div>
          </div>

          <div v-if="!hasPageSelected" class="empty-editor">
            <h3>No page selected</h3>
            <p>Create a new page or pick one from the sidebar.</p>
            <button class="topbar-btn primary" @click="handleCreatePage">Create first page</button>
          </div>

          <div v-else class="editor-split">
            <div class="editor-pane">
              <WikiEditor v-model="editorContent" />
            </div>
            <div class="preview-pane">
              <WikiPreview :content="editorContent" />
            </div>
          </div>
        </div>

        <WikiMetadata
          v-if="hasPageSelected"
          :tags="editorTags"
          :created-at="currentPage!.created_at"
          :updated-at="currentPage!.updated_at"
          :revisions="revisions"
          @update:tags="updateTags"
          @delete="handleDelete"
          @restore="handleRestoreRevision"
        />
      </div>
    </div>
  </div>
</template>

<style scoped>
.wiki-app {
  height: 100vh;
  display: flex;
  flex-direction: column;
  background: var(--page-bg);
}

.wiki-shell {
  display: flex;
  flex-direction: column;
  height: 100%;
}

.wiki-topbar {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 12px 16px;
  background: var(--topbar-bg);
  border-bottom: 1px solid var(--border-color);
}

.wiki-topbar-left {
  display: flex;
  align-items: center;
  gap: 8px;
}

.title-chip {
  padding: 6px 10px;
  border-radius: 8px;
  background: var(--chip-bg);
  color: var(--text-primary);
  font-size: 13px;
}

.topbar-btn {
  padding: 8px 14px;
  border-radius: 8px;
  border: 1px solid var(--border-color);
  background: var(--btn-bg);
  color: var(--text-primary);
  cursor: pointer;
  transition: all 0.15s;
  font-weight: 600;
}

.topbar-btn.primary {
  background: var(--primary-color);
  border-color: var(--primary-color);
  color: white;
}

.topbar-btn:disabled {
  opacity: 0.6;
  cursor: not-allowed;
}

.topbar-btn:not(:disabled):hover {
  transform: translateY(-1px);
}

.status-text {
  color: var(--text-secondary);
  font-size: 13px;
}

.wiki-layout {
  display: grid;
  grid-template-columns: 260px 1fr 260px;
  flex: 1;
  min-height: 0;
}

.wiki-editor-panel {
  display: flex;
  flex-direction: column;
  border-right: 1px solid var(--border-color);
  background: var(--panel-bg);
}

.title-row {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 12px 16px;
  border-bottom: 1px solid var(--border-color);
  gap: 12px;
}

.title-input {
  flex: 1;
  padding: 10px 12px;
  font-size: 18px;
  font-weight: 600;
  border: 1px solid var(--border-color);
  border-radius: 8px;
  background: var(--input-bg);
  color: var(--text-primary);
}

.title-input:focus {
  outline: 2px solid var(--primary-light);
  border-color: transparent;
}

.title-meta {
  font-size: 12px;
  color: var(--text-secondary);
}

.breadcrumbs {
  font-size: 12px;
  color: var(--text-secondary);
}

.editor-split {
  flex: 1;
  display: grid;
  grid-template-columns: 1fr 1fr;
  min-height: 0;
}

.editor-pane,
.preview-pane {
  min-height: 0;
  display: flex;
}

.editor-pane {
  border-right: 1px solid var(--border-color);
}

.preview-pane {
  background: var(--preview-bg);
}

.empty-editor {
  flex: 1;
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  gap: 8px;
  color: var(--text-secondary);
}

@media (prefers-color-scheme: light) {
  .wiki-app {
    --page-bg: #f5f5f5;
    --panel-bg: #ffffff;
    --preview-bg: #fafafa;
    --topbar-bg: #ffffff;
    --border-color: #e5e5ea;
    --text-primary: #1d1d1f;
    --text-secondary: #71717a;
    --btn-bg: #ffffff;
    --primary-color: #2563eb;
    --primary-light: rgba(37, 99, 235, 0.2);
    --chip-bg: #eef2ff;
    --input-bg: #ffffff;
  }
}

@media (prefers-color-scheme: dark) {
  .wiki-app {
    --page-bg: #0f1115;
    --panel-bg: #111827;
    --preview-bg: #0b1222;
    --topbar-bg: #0b1222;
    --border-color: #1f2937;
    --text-primary: #f8fafc;
    --text-secondary: #94a3b8;
    --btn-bg: #111827;
    --primary-color: #38bdf8;
    --primary-light: rgba(56, 189, 248, 0.35);
    --chip-bg: rgba(56, 189, 248, 0.16);
    --input-bg: #0f172a;
  }
}
</style>
