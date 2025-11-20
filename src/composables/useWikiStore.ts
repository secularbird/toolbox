import { ref } from 'vue';
import { invoke } from '@tauri-apps/api/core';

export interface WikiPage {
  id: string;
  title: string;
  content: string;
  tags: string[];
  created_at: number;
  updated_at: number;
}

export interface WikiPageList {
  id: string;
  title: string;
  tags: string[];
  updated_at: number;
}

export interface WikiRevisionMeta {
  id: string;
  page_id: string;
  title: string;
  created_at: number;
}

const pages = ref<WikiPageList[]>([]);
const currentPage = ref<WikiPage | null>(null);
const isLoading = ref(false);
const error = ref<string | null>(null);

export function useWiki() {
  async function loadPages() {
    isLoading.value = true;
    error.value = null;
    try {
      pages.value = await invoke<WikiPageList[]>('list_wiki_pages');
    } catch (e) {
      error.value = e as string;
      console.error('[WIKI] Failed to load pages:', e);
    } finally {
      isLoading.value = false;
    }
  }

  async function loadPage(id: string) {
    isLoading.value = true;
    error.value = null;
    try {
      currentPage.value = await invoke<WikiPage>('get_wiki_page', { id });
    } catch (e) {
      error.value = e as string;
      console.error('[WIKI] Failed to load page:', e);
    } finally {
      isLoading.value = false;
    }
  }

  async function createPage(title: string, content: string, tags: string[]) {
    isLoading.value = true;
    error.value = null;
    try {
      const page = await invoke<WikiPage>('create_wiki_page', { title, content, tags });
      await loadPages();
      currentPage.value = page;
      return page;
    } catch (e) {
      error.value = e as string;
      console.error('[WIKI] Failed to create page:', e);
      throw e;
    } finally {
      isLoading.value = false;
    }
  }

  async function updatePage(id: string, title: string, content: string, tags: string[]) {
    isLoading.value = true;
    error.value = null;
    try {
      const page = await invoke<WikiPage>('update_wiki_page', { id, title, content, tags });
      await loadPages();
      currentPage.value = page;
      return page;
    } catch (e) {
      error.value = e as string;
      console.error('[WIKI] Failed to update page:', e);
      throw e;
    } finally {
      isLoading.value = false;
    }
  }

  async function deletePage(id: string) {
    isLoading.value = true;
    error.value = null;
    try {
      await invoke('delete_wiki_page', { id });
      await loadPages();
      if (currentPage.value?.id === id) {
        currentPage.value = null;
      }
    } catch (e) {
      error.value = e as string;
      console.error('[WIKI] Failed to delete page:', e);
      throw e;
    } finally {
      isLoading.value = false;
    }
  }

  async function searchPages(query: string) {
    isLoading.value = true;
    error.value = null;
    try {
      const results = await invoke<WikiPageList[]>('search_wiki_pages', { query });
      return results;
    } catch (e) {
      error.value = e as string;
      console.error('[WIKI] Failed to search pages:', e);
      return [];
    } finally {
      isLoading.value = false;
    }
  }

  async function listRevisions(pageId: string) {
    isLoading.value = true;
    error.value = null;
    try {
      const revisions = await invoke<WikiRevisionMeta[]>('list_wiki_revisions', { pageId });
      return revisions;
    } catch (e) {
      error.value = e as string;
      console.error('[WIKI] Failed to list revisions:', e);
      return [];
    } finally {
      isLoading.value = false;
    }
  }

  async function restoreRevision(pageId: string, revisionId: string) {
    isLoading.value = true;
    error.value = null;
    try {
      const page = await invoke<WikiPage>('restore_wiki_revision', { pageId, revisionId });
      await loadPages();
      currentPage.value = page;
      return page;
    } catch (e) {
      error.value = e as string;
      console.error('[WIKI] Failed to restore revision:', e);
      throw e;
    } finally {
      isLoading.value = false;
    }
  }

  function clearCurrentPage() {
    currentPage.value = null;
  }

  return {
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
  };
}
