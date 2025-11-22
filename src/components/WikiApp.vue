<script setup lang="ts">
import { computed, onMounted, ref, watch } from 'vue';
import WikiSidebar from './WikiSidebar.vue';
import WikiEditor from './WikiEditor.vue';
import WikiPreview from './WikiPreview.vue';
import WikiMetadata from './WikiMetadata.vue';
import DocumentImportModal from './DocumentImportModal.vue';
import TableInsertModal from './TableInsertModal.vue';
import { useWiki } from '../composables/useWikiStore';
import type { WikiPage, WikiRevisionMeta, WikiPageList } from '../composables/useWikiStore';
import type { ImportResult } from '../composables/useDocumentImport';
import { ask } from '@tauri-apps/plugin-dialog';

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
const showImportModal = ref(false);
const showTableModal = ref(false);
const editorRef = ref<InstanceType<typeof WikiEditor> | null>(null);
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
  
  if (pages.value.length === 0) {
    await createSamplePage();
  }
  
  await applyFilters();
  if (displayedPages.value.length) {
    await selectPage(displayedPages.value[0].id);
  } else {
    clearEditor();
  }
}

async function createSamplePage() {
  const sampleContent = `# Markdown 功能演示页面

欢迎使用 Wiki！这个页面展示了所有支持的 Markdown 特性。

## 📝 基础文本格式

**粗体文本** 使用 \`**text**\`

*斜体文本* 使用 \`*text*\`

***粗斜体*** 使用 \`***text***\`

\`行内代码\` 使用 \`\`code\`\`

~~删除线~~ 使用 \`~~text~~\`

## 📋 列表

### 无序列表
- 项目 1
- 项目 2
  - 子项目 2.1
  - 子项目 2.2
- 项目 3

### 有序列表
1. 第一项
2. 第二项
3. 第三项

## 🔗 链接和图片

[GitHub](https://github.com)

自动链接: https://example.com

![示例图片](https://via.placeholder.com/300x150)

## 💬 引用

> 这是一个引用块
> 
> 可以包含多行内容

## 📊 表格

| 功能 | 支持 | 说明 |
|------|:----:|------|
| 基础语法 | ✅ | 标题、列表、链接等 |
| 代码高亮 | ✅ | 支持多种语言 |
| 图表渲染 | ✅ | Mermaid + PlantUML |

## 💻 代码块

### JavaScript
\`\`\`javascript
function greet(name) {
  console.log(\`Hello, \${name}!\`);
  return true;
}

greet("World");
\`\`\`

### Python
\`\`\`python
def factorial(n):
    if n <= 1:
        return 1
    return n * factorial(n - 1)

print(factorial(5))  # 输出: 120
\`\`\`

### TypeScript
\`\`\`typescript
interface User {
  id: number;
  name: string;
  email: string;
}

const user: User = {
  id: 1,
  name: "Alice",
  email: "alice@example.com"
};
\`\`\`

## 📈 Mermaid 图表

### 流程图
\`\`\`mermaid
graph TD
    A[开始] --> B{条件判断}
    B -->|是| C[执行操作A]
    B -->|否| D[执行操作B]
    C --> E[结束]
    D --> E
\`\`\`

### 序列图
\`\`\`mermaid
sequenceDiagram
    autonumber
    participant 用户
    participant 前端
    participant 后端
    participant 数据库
    
    用户->>前端: 发起请求
    前端->>后端: API调用
    后端->>数据库: 查询数据
    数据库-->>后端: 返回结果
    后端-->>前端: 返回数据
    前端-->>用户: 显示结果
\`\`\`

### 类图
\`\`\`mermaid
classDiagram
    class Animal {
        +String name
        +int age
        +eat()
        +sleep()
    }
    class Dog {
        +String breed
        +bark()
    }
    class Cat {
        +meow()
    }
    Animal <|-- Dog
    Animal <|-- Cat
\`\`\`

### 饼图
\`\`\`mermaid
pie title 项目时间分配
    "开发" : 45
    "测试" : 20
    "文档" : 15
    "会议" : 20
\`\`\`

## 🎨 PlantUML 图表

### 用例图
\`\`\`plantuml
@startuml
left to right direction
actor 用户
rectangle 系统 {
  usecase "登录" as UC1
  usecase "查看数据" as UC2
  usecase "编辑内容" as UC3
  usecase "保存更改" as UC4
}
用户 --> UC1
用户 --> UC2
用户 --> UC3
用户 --> UC4
@enduml
\`\`\`

### 类图
\`\`\`plantuml
@startuml
class WikiPage {
  +String id
  +String title
  +String content
  +String[] tags
  +Date created_at
  +Date updated_at
  +save()
  +delete()
}

class Section {
  +String id
  +String name
  +getPages()
}

class User {
  +String name
  +String email
  +login()
}

User "1" -- "*" WikiPage : creates
Section "1" -- "*" WikiPage : contains
@enduml
\`\`\`

### 活动图
\`\`\`plantuml
@startuml
start
:用户打开编辑器;
if (有未保存内容?) then (是)
  :显示提示;
  if (确认保存?) then (是)
    :保存内容;
  else (否)
    :丢弃更改;
  endif
endif
:加载新页面;
:显示内容;
stop
@enduml
\`\`\`

## ✨ 特殊功能

### 水平分割线

---

### 转义字符

\\*这不是斜体\\*

\\[这不是链接\\]

### 快捷键

- **粗体**: Ctrl/Cmd + B
- **斜体**: Ctrl/Cmd + I
- **链接**: Ctrl/Cmd + K
- **撤销**: Ctrl/Cmd + Z
- **重做**: Ctrl/Cmd + Shift + Z 或 Ctrl/Cmd + Y

## 📝 编辑提示

1. 使用工具栏快速插入格式
2. 支持粘贴图片（自动转换为内嵌格式）
3. 支持拖拽上传文件（最大5MB）
4. 自动保存功能
5. 支持标签和分类管理

---

*试试编辑这个页面，体验所有功能！*
`;

  try {
    await createPage(
      'Markdown 功能演示',
      sampleContent,
      ['示例', '教程', 'Markdown'],
      selectedSectionId.value || undefined
    );
  } catch (e) {
    console.error('Failed to create sample page:', e);
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
    // Generate default page title with section name and date
    let defaultTitle = '';
    if (selectedSectionId.value) {
      const section = sections.value.find(s => s.id === selectedSectionId.value);
      if (section) {
        const today = new Date().toISOString().split('T')[0]; // YYYY-MM-DD format
        defaultTitle = `${section.name} - ${today}`;
      }
    }
    
    if (!defaultTitle) {
      const today = new Date().toISOString().split('T')[0];
      defaultTitle = `Page - ${today}`;
    }
    
    const page = await createPage(defaultTitle, '# New Page\n', [], selectedSectionId.value || undefined);
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
  if (!currentPage.value) {
    console.log('[DEBUG] No current page selected');
    return;
  }
  
  console.log('[DEBUG] Deleting page:', currentPage.value.title, currentPage.value.id);
  
  const confirmed = await ask(`确定要删除页面 "${currentPage.value.title}" 吗？`, {
    title: '删除确认',
    kind: 'warning',
    okLabel: '删除',
    cancelLabel: '取消'
  });
  
  if (!confirmed) {
    console.log('[DEBUG] Delete cancelled by user');
    return;
  }
  
  saving.value = true;
  formError.value = '';
  try {
    console.log('[DEBUG] Calling deletePage API...');
    await deletePage(currentPage.value.id);
    console.log('[DEBUG] Page deleted successfully');
    
    await applyFilters();
    if (displayedPages.value.length) {
      await selectPage(displayedPages.value[0].id);
    } else {
      clearEditor();
    }
    message.value = 'Page deleted';
  } catch (e) {
    console.error('[DEBUG] Delete failed:', e);
    formError.value = String(e);
  } finally {
    saving.value = false;
  }
}

async function handleDeletePageFromSidebar(pageId: string) {
  const page = pages.value.find(p => p.id === pageId);
  if (!page) return;
  
  const confirmed = await ask(`确定要删除页面 "${page.title}" 吗？`, {
    title: '删除确认',
    kind: 'warning',
    okLabel: '删除',
    cancelLabel: '取消'
  });
  
  if (!confirmed) return;
  
  saving.value = true;
  formError.value = '';
  try {
    await deletePage(pageId);
    await applyFilters();
    
    // If the deleted page was currently selected, clear the editor
    if (currentPage.value?.id === pageId) {
      if (displayedPages.value.length) {
        await selectPage(displayedPages.value[0].id);
      } else {
        clearEditor();
      }
    }
    
    message.value = 'Page deleted';
  } catch (e) {
    formError.value = String(e);
  } finally {
    saving.value = false;
  }
}

async function handleRenamePageFromSidebar(pageId: string) {
  const page = pages.value.find(p => p.id === pageId);
  if (!page) return;
  
  // Select the page first
  await selectPage(pageId);
  
  // Focus on the title input so user can rename
  setTimeout(() => {
    const titleInput = document.querySelector('.title-input') as HTMLInputElement;
    if (titleInput) {
      titleInput.focus();
      titleInput.select();
    }
  }, 100);
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
  // Generate default name with parent name and date
  let defaultName = '';
  if (parentId) {
    const parent = sections.value.find(s => s.id === parentId);
    if (parent) {
      const today = new Date().toISOString().split('T')[0]; // YYYY-MM-DD format
      defaultName = `${parent.name} - ${today}`;
    }
  }
  
  if (!defaultName) {
    const today = new Date().toISOString().split('T')[0];
    defaultName = `Section - ${today}`;
  }
  
  const name = prompt('New section name', defaultName) || defaultName;
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
  console.log('[DEBUG] handleDeleteSection called with id:', id);
  
  const section = sections.value.find(s => s.id === id);
  const sectionName = section?.name || 'this section';
  
  console.log('[DEBUG] Section to delete:', section);
  
  const confirmed = await ask(
    `确定要删除分类 "${sectionName}" 吗？\n\n注意：该分类下的子分类和页面必须先移动到其他位置。`,
    {
      title: '删除分类',
      kind: 'warning',
      okLabel: '删除',
      cancelLabel: '取消'
    }
  );
  
  if (!confirmed) {
    console.log('[DEBUG] Delete cancelled by user');
    return;
  }
  
  try {
    console.log('[DEBUG] Calling deleteSection API...');
    await deleteSection(id);
    console.log('[DEBUG] Section deleted successfully');
    
    if (selectedSectionId.value === id) {
      selectedSectionId.value = sections.value[0]?.id || null;
      await applyFilters();
    }
    message.value = `分类 "${sectionName}" 已删除`;
  } catch (e) {
    console.error('[DEBUG] Section delete failed:', e);
    formError.value = String(e);
  }
}

function handleShowImport() {
  showImportModal.value = true;
}

function handleShowTableInsert() {
  showTableModal.value = true;
}

function handleInsertTable(markdown: string) {
  showTableModal.value = false;
  if (editorRef.value) {
    editorRef.value.insertText('\n\n' + markdown + '\n\n');
  }
}

async function handleImportDocument(result: ImportResult) {
  showImportModal.value = false;
  saving.value = true;
  formError.value = '';
  
  try {
    // Add date to imported document title if it doesn't already have one
    let finalTitle = result.title;
    const today = new Date().toISOString().split('T')[0]; // YYYY-MM-DD format
    
    // Check if title already contains a date pattern (YYYY-MM-DD)
    const hasDate = /\d{4}-\d{2}-\d{2}/.test(finalTitle);
    
    if (!hasDate) {
      // Add section name and date if in a section
      if (selectedSectionId.value) {
        const section = sections.value.find(s => s.id === selectedSectionId.value);
        if (section) {
          finalTitle = `${section.name} - ${today} - ${result.title}`;
        } else {
          finalTitle = `${result.title} - ${today}`;
        }
      } else {
        finalTitle = `${result.title} - ${today}`;
      }
    }
    
    const page = await createPage(
      finalTitle, 
      result.content, 
      ['imported'], 
      selectedSectionId.value || undefined
    );
    await applyFilters();
    await selectPage(page.id);
    message.value = 'Document imported successfully';
  } catch (e) {
    formError.value = String(e);
  } finally {
    saving.value = false;
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
          <button class="topbar-btn" @click="handleShowImport" :disabled="saving || isLoading">
            📄 Import
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
          @deletePage="handleDeletePageFromSidebar"
          @renamePage="handleRenamePageFromSidebar"
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
              <WikiEditor 
                ref="editorRef" 
                v-model="editorContent" 
                @insertTable="handleShowTableInsert"
              />
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
    
    <DocumentImportModal 
      v-if="showImportModal"
      @close="showImportModal = false"
      @import="handleImportDocument"
    />
    
    <TableInsertModal 
      v-if="showTableModal"
      @close="showTableModal = false"
      @insert="handleInsertTable"
    />
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

.view-toggle {
  display: flex;
  gap: 4px;
}

.topbar-btn.ghost {
  background: transparent;
  border-color: transparent;
}

.topbar-btn.ghost.active {
  border-color: var(--primary-color);
  color: var(--primary-color);
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
  flex-direction: column;
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
