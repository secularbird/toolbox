import { createApp } from "vue";
import App from "./App.vue";

// Enable debug mode in development
const app = createApp(App);

// Add global error handler
app.config.errorHandler = (err, instance, info) => {
  console.error('[APP] Vue Error:', err);
  console.error('[APP] Component:', instance);
  console.error('[APP] Info:', info);
};

// Log app startup
console.log('[APP] Application starting...');
console.log('[APP] Environment:', import.meta.env.MODE);
console.log('[APP] Tauri available:', typeof window !== 'undefined' && '__TAURI__' in window);

app.mount("#app");

console.log('[APP] Application mounted successfully');

// Add keyboard shortcuts for debugging
document.addEventListener('keydown', (e) => {
  // F12 or Cmd+Option+I (Mac) / Ctrl+Shift+I (Windows/Linux) - Toggle DevTools
  if (e.key === 'F12' || 
      (e.metaKey && e.altKey && e.key === 'i') ||
      (e.ctrlKey && e.shiftKey && e.key === 'I')) {
    e.preventDefault();
    console.log('[APP] DevTools shortcut triggered');
  }
});

// Log unhandled errors
window.addEventListener('error', (e) => {
  console.error('[APP] Global error:', e.error);
});

window.addEventListener('unhandledrejection', (e) => {
  console.error('[APP] Unhandled promise rejection:', e.reason);
});
