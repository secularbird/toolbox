/**
 * useDebugMode Composable
 * Handles debug mode state and operations
 */

import { ref } from 'vue';
import { invoke } from '@tauri-apps/api/core';

const debugMode = ref(true);

export function useDebugMode() {
  // Toggle debug mode
  async function toggleDebugMode() {
    try {
      await invoke("set_debug_mode", { enabled: debugMode.value });
      console.log(`Debug mode ${debugMode.value ? 'enabled' : 'disabled'}`);
      return true;
    } catch (error) {
      console.error("Failed to set debug mode:", error);
      return false;
    }
  }

  // Load debug mode from backend
  async function loadDebugMode() {
    try {
      debugMode.value = await invoke("get_debug_mode");
    } catch (error) {
      console.error("Failed to get debug mode:", error);
    }
  }

  // Set debug mode directly
  function setDebugMode(enabled: boolean) {
    debugMode.value = enabled;
  }

  return {
    debugMode,
    toggleDebugMode,
    loadDebugMode,
    setDebugMode,
  };
}
