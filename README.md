# 📝 Reminder App

A modern, feature-rich reminder application built with Tauri and Vue 3, featuring a clean sidebar-based UI with list-style task management and recurring reminders.

## ✨ Features

### 🗂️ Category Management
- **Sidebar Navigation**: Easy access to all your reminder categories
- **Built-in Categories**: Work, Personal, Shopping, Health, and Other
- **Custom Categories**: Create your own categories with unique colors
- **Category Counters**: See how many reminders are in each category at a glance

### 📋 List-Style Task Management
- **Table View**: Organize your reminders in a clean, spreadsheet-like interface
- **Quick Actions**: Check off tasks and delete with one click
- **Inline Details**: See task title, description, category, frequency, and due date all in one row
- **Smart Filtering**: View reminders by category or see all at once

### ⏰ Recurring Reminders
- **🔵 Once**: One-time reminder
- **📅 Daily**: Repeats every day
- **📆 Weekly**: Repeats every week
- **🗓️ Monthly**: Repeats every month
- **📊 Yearly**: Repeats every year

### ⚡ Quick Add Form
- **Compact Input**: Add reminders quickly without leaving your view
- **Category Selection**: Choose the category while creating a reminder
- **Frequency Selector**: Pick how often the reminder should repeat
- **Date/Time Picker**: Set due dates and times easily
- **Instant Updates**: Reminders appear immediately after adding

### 🎨 Modern Design
- **Clean Interface**: Minimalist design focused on productivity
- **Dark Mode**: Automatic dark mode support based on system preferences
- **Color-Coded**: Each category has its own color for easy identification
- **Smooth Animations**: Polished hover effects and transitions

## 🚀 Getting Started

### Prerequisites
- Node.js (v16 or higher)
- Rust (latest stable version)
- npm or yarn

### Installation

```bash
# Clone or navigate to the project
cd tauri-vue-app

# Install dependencies
npm install

# Run in development mode
npm run tauri dev

# Build for production
npm run tauri build
```

## 📖 How to Use

1. **Add a Reminder**: 
   - Fill in the task title
   - Select a category from the dropdown
   - Choose a due date and time
   - Select the frequency (Once, Daily, Weekly, etc.)
   - Click "+ Add"

2. **Manage Reminders**:
   - Click the checkbox to mark a task as complete
   - Click the trash icon to delete a reminder
   - Completed tasks appear with strikethrough text

3. **Organize by Category**:
   - Click any category in the sidebar to filter reminders
   - Click "All Reminders" to see everything
   - Use "+ Add Category" to create custom categories

## 🛠️ Technology Stack

- **Frontend**: Vue 3 with TypeScript
- **Backend**: Rust (Tauri)
- **Build Tool**: Vite
- **Styling**: Scoped CSS with CSS Grid for list layout

## 📝 Project Structure

```
tauri-vue-app/
├── src/                    # Vue frontend
│   ├── App.vue            # Main application component
│   ├── main.ts            # Application entry point
│   └── assets/            # Static assets
├── src-tauri/             # Rust backend
│   ├── src/
│   │   └── lib.rs         # Tauri commands and state management
│   └── Cargo.toml         # Rust dependencies
└── package.json           # Node dependencies
```

## 🎯 Roadmap

Future enhancements planned:
- [ ] Persistent storage (save reminders to file/database)
- [ ] Desktop notification system for due reminders
- [ ] Automatic creation of recurring reminder instances
- [ ] Sort and filter options (by date, priority, etc.)
- [ ] Search functionality across all reminders
- [ ] Priority levels (High, Medium, Low)
- [ ] Export/import reminders (JSON, CSV)
- [ ] Reminder history and analytics
- [ ] Subtasks and checklists

## 📄 License

This project is open source and available under the MIT License.

---

Built with ❤️ using Tauri and Vue
