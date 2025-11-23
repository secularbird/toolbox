# Wiki Reminder Integration

## Overview
The Wiki now supports creating real reminders directly from wiki pages. When you insert a reminder from the Wiki editor, it creates an actual reminder in your Reminder app database, not just markdown text.

## How It Works

### User Flow
1. Open the Wiki editor
2. Click the 🔔 button in the toolbar (or press `Ctrl+Shift+R`)
3. Fill in the reminder details:
   - **Title** (required): Reminder title
   - **Description** (optional): Additional details
   - **Time** (required): When to be reminded
   - **Category**: work, personal, shopping, health, or other
   - **Frequency**: once, daily, weekly, monthly, or yearly
4. Click "Create & Insert"
5. The reminder is:
   - ✅ Created in the Reminder app database
   - ✅ Inserted as formatted markdown in the wiki page

### Technical Implementation

#### Components Modified

**1. WikiEditor.vue**
- Added 🔔 toolbar button
- Added keyboard shortcut: `Ctrl+Shift+R`
- Emits `insertReminder` event when button clicked

**2. ReminderInsertModal.vue**
- Modal form for reminder creation
- Validates required fields (title, time)
- Calls `invoke('add_reminder')` to create real reminder
- Generates markdown with reminder metadata
- Shows loading state and error handling

**3. WikiApp.vue**
- Manages modal visibility
- Receives reminder markdown and ID from modal
- Inserts markdown into editor at cursor position

#### Data Flow

```
User clicks 🔔 button
  ↓
WikiEditor emits 'insertReminder' event
  ↓
WikiApp shows ReminderInsertModal
  ↓
User fills form & clicks "Create & Insert"
  ↓
Modal calls invoke('add_reminder', {...})
  ↓
Tauri backend creates reminder in SQLite
  ↓
Modal emits 'insert' with (markdown, reminderId)
  ↓
WikiApp inserts markdown into editor
  ↓
Wiki page now contains reminder reference
```

#### Markdown Format

The generated markdown looks like:

```markdown
> **🔔 Reminder: Buy groceries**
> 
> 🛒 Category: Shopping  
> 🔵 Frequency: Once  
> ⏰ Time: 12/21/2024, 3:00:00 PM
> 
> <!-- reminder-data:{"title":"Buy groceries","category":"shopping","frequency":"once","time":"2024-12-21T15:00"} -->
```

The hidden comment preserves reminder metadata for future features (e.g., syncing or linking).

## Features

### Current Implementation ✅
- Real reminder creation in database
- Form validation (required fields)
- Category and frequency selection
- Loading states and error handling
- Keyboard shortcut support
- Formatted markdown output with metadata

### Future Enhancements 🚀
- Click reminder markdown to open in Reminder app
- Sync reminder status (completed, snoozed) back to wiki
- Search/filter wiki pages by embedded reminders
- Reminder link syntax: `[🔔 Reminder](#reminder-{id})`
- Bulk reminder insertion from task lists
- Reminder quick-add from wiki preview

## Technical Details

### Tauri Command Used
```rust
#[tauri::command]
async fn add_reminder(
    title: String,
    description: String,
    time: String,
    category: String,
    frequency: String,
    state: tauri::State<'_, AppState>,
) -> Result<(), String>
```

### Frontend API Call
```typescript
await invoke('add_reminder', {
  title: reminderTitle.value.trim(),
  description: reminderDescription.value.trim() || 'Created from Wiki',
  time: reminderTime.value,
  category: reminderCategory.value,
  frequency: reminderFrequency.value,
});
```

## Benefits

1. **Unified Workflow**: Create reminders without leaving wiki editor
2. **Real Integration**: Reminders appear in Reminder app immediately
3. **Documentation**: Reminder metadata preserved in wiki markdown
4. **Flexibility**: Standard markdown remains human-readable
5. **Future-Proof**: Comment-based metadata enables future features

## Usage Example

### Scenario: Project Planning
You're writing a wiki page about a new project and want to set milestone reminders:

```markdown
# New Website Project

## Phase 1: Design
We'll create mockups and wireframes first.

> **🔔 Reminder: Review design mockups**
> 
> 💼 Category: Work  
> 🔵 Frequency: Once  
> ⏰ Time: 1/5/2025, 10:00:00 AM

## Phase 2: Development
Start coding after design approval.

> **🔔 Reminder: Begin frontend development**
> 
> 💼 Category: Work  
> 🔵 Frequency: Once  
> ⏰ Time: 1/12/2025, 9:00:00 AM
```

All reminders are real and will notify you at the specified times!
