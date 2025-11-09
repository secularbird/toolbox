# Reminder App Features

## Overview
A modern reminder application built with Tauri and Vue 3, featuring a clean sidebar-based UI with list-style task management and recurring reminders.

## Features

### ✅ Sidebar Navigation
- **Category-based organization**: View reminders by category
- **All Reminders view**: See all reminders in one place
- **Built-in categories**:
  - 📋 All Reminders
  - 💼 Work
  - 👤 Personal
  - 🛒 Shopping
  - 🏥 Health
  - 📌 Other
- **Add custom categories**: Create your own categories with random colors
- **Category counters**: See the number of reminders in each category

### ✅ Reminder Management
- **Create reminders**: Add title, description, date/time, category, and frequency
- **View reminders**: Browse reminders in a clean table layout
- **Toggle completion**: Mark reminders as complete/incomplete with checkboxes
- **Delete reminders**: Remove reminders you no longer need
- **Color-coded badges**: Each reminder shows its category with a colored badge

### ⏰ Frequency Options
- **🔵 Once**: One-time reminder
- **📅 Daily**: Repeats every day
- **📆 Weekly**: Repeats every week
- **🗓️ Monthly**: Repeats every month
- **📊 Yearly**: Repeats every year

### 📋 List View Features
- **Table layout**: Organized columns for easy scanning
  - Checkbox column for completion status
  - Task column with title and optional description
  - Category badge with color coding
  - Frequency indicator
  - Due date/time display
  - Delete action button
- **Visual feedback**: Row hover effects and completed task styling
- **Efficient layout**: See all information at a glance

### 🎨 Modern UI
- **Clean, responsive design**: Works on all screen sizes
- **Dark mode support**: Automatically adapts to system theme
- **Smooth animations**: Hover effects and transitions
- **Color-coded categories**: Each category has its own color
- **Professional table design**: Spreadsheet-like organization

## How to Run

```bash
# Install dependencies
npm install

# Run in development mode
npm run tauri dev

# Build for production
npm run tauri build
```

## Technology Stack
- **Frontend**: Vue 3 with TypeScript
- **Backend**: Rust (Tauri)
- **Build Tool**: Vite
- **Styling**: Scoped CSS with CSS Grid for table layout
- **State Management**: Rust Mutex for thread-safe state

## Usage Tips

### Adding a Reminder
1. Enter your task in "What do you need to do?"
2. Select the category (Work, Personal, etc.)
3. Pick the due date and time
4. Choose the frequency (Once, Daily, Weekly, etc.)
5. Click "+ Add" button

### Managing Reminders
- **Complete**: Click the checkbox in the first column
- **Delete**: Click the trash icon in the last column
- **Filter**: Use the sidebar to view specific categories
- **Track**: Watch the counter update as you add/remove reminders

### Custom Categories
1. Scroll to the bottom of the sidebar
2. Click "+ Add Category"
3. Enter your category name
4. It will be assigned a random color automatically

## Future Enhancements
- [ ] Persistent storage (save to file/database)
- [ ] Desktop notifications for due reminders
- [ ] Automatic recurring reminder creation
- [ ] Sort and filter options
- [ ] Search functionality
- [ ] Priority levels (High, Medium, Low)
- [ ] Tags and labels
- [ ] Export/import functionality
