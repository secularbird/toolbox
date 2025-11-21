export class EditorHistory {
  private history: string[] = [];
  private currentIndex = -1;
  private maxHistorySize = 100;
  private lastSaveTime = 0;
  private saveDebounceMs = 500;

  constructor(initialContent: string = '') {
    if (initialContent) {
      this.history = [initialContent];
      this.currentIndex = 0;
    }
  }

  save(content: string) {
    const now = Date.now();
    if (now - this.lastSaveTime < this.saveDebounceMs) {
      return;
    }
    this.lastSaveTime = now;

    if (this.currentIndex < this.history.length - 1) {
      this.history = this.history.slice(0, this.currentIndex + 1);
    }

    const lastContent = this.history[this.currentIndex];
    if (lastContent === content) {
      return;
    }

    this.history.push(content);
    this.currentIndex++;

    if (this.history.length > this.maxHistorySize) {
      this.history.shift();
      this.currentIndex--;
    }
  }

  undo(): string | null {
    if (this.currentIndex > 0) {
      this.currentIndex--;
      return this.history[this.currentIndex];
    }
    return null;
  }

  redo(): string | null {
    if (this.currentIndex < this.history.length - 1) {
      this.currentIndex++;
      return this.history[this.currentIndex];
    }
    return null;
  }

  canUndo(): boolean {
    return this.currentIndex > 0;
  }

  canRedo(): boolean {
    return this.currentIndex < this.history.length - 1;
  }

  clear() {
    this.history = [];
    this.currentIndex = -1;
  }

  reset(content: string) {
    this.history = [content];
    this.currentIndex = 0;
  }
}
