# Local Storage

The application uses SQLite for local storage. All data is stored on your device.

## Features

- ✅ All reminders stored in local SQLite database
- ✅ Persistent storage across app restarts
- ✅ Fast local queries
- ✅ No internet connection required
- ✅ Complete data privacy

## Database Location

| Platform | Location |
|----------|----------|
| macOS | `~/Library/Application Support/com.yaozhuang.tauri-vue-app/reminders.db` |
| Linux | `~/.local/share/tauri-vue-app/reminders.db` |
| Windows | `C:\Users\{username}\AppData\Roaming\com.yaozhuang.tauri-vue-app\reminders.db` |

## Database Schema

### `reminders` Table
```sql
CREATE TABLE reminders (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    title TEXT NOT NULL,
    description TEXT NOT NULL,
    time TEXT NOT NULL,
    completed INTEGER NOT NULL DEFAULT 0,
    category TEXT NOT NULL,
    frequency TEXT NOT NULL,
    created_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP
)
```

## Features

### Create Reminder
- Automatically saved to local database
- Instant feedback
- No network delay

### Read Reminders
- Fast queries from local database
- Works offline
- No API calls

### Update Reminder
- Immediate update in local database
- Toggle completion status
- Edit any field

### Delete Reminder
- Permanent deletion from local database
- No server sync needed

## Backup & Restore

### Manual Backup

**macOS:**
```bash
cp ~/Library/Application\ Support/com.yaozhuang.tauri-vue-app/reminders.db ~/Desktop/reminders-backup.db
```

**Linux:**
```bash
cp ~/.local/share/tauri-vue-app/reminders.db ~/reminders-backup.db
```

**Windows:**
```powershell
Copy-Item "$env:APPDATA\com.yaozhuang.tauri-vue-app\reminders.db" "$env:USERPROFILE\Desktop\reminders-backup.db"
```

### Restore from Backup

1. Close the application
2. Copy backup file to the database location
3. Restart the application

### Export Data

You can use any SQLite browser to export data:

```bash
# Export to CSV
sqlite3 reminders.db -header -csv "SELECT * FROM reminders;" > reminders.csv

# Export to JSON
sqlite3 reminders.db -json "SELECT * FROM reminders;" > reminders.json

# View data
sqlite3 reminders.db "SELECT * FROM reminders;"
```

## Database Tools

### SQLite Browser
Download [DB Browser for SQLite](https://sqlitebrowser.org/) to view and edit the database.

### Command Line
```bash
# Open database
sqlite3 path/to/reminders.db

# List tables
.tables

# View schema
.schema reminders

# Query data
SELECT * FROM reminders;

# Count reminders
SELECT COUNT(*) FROM reminders;

# View recent reminders
SELECT title, category, created_at FROM reminders ORDER BY created_at DESC LIMIT 10;

# Exit
.quit
```

## Data Privacy

- ✅ All data stored locally on your device
- ✅ No data sent to external servers
- ✅ No analytics or tracking
- ✅ You own your data completely
- ✅ No account required

## Storage Size

- Database file is very small (typically < 1MB)
- Grows with number of reminders
- No automatic cleanup (keeps all historical data)

## Troubleshooting

### Database Locked
If you see "database is locked" error:
1. Close all instances of the application
2. Check for any hanging processes
3. Restart the application

### Corrupted Database
If database is corrupted:
1. Restore from backup
2. Or delete database file (will lose data)
3. Application will create new database on restart

### Lost Data
If you lose reminders:
1. Check if database file exists
2. Try restoring from backup
3. Use SQLite recovery tools if needed

## Migration

### From Other Apps

To import data from other reminder apps:

1. Export data from other app (CSV/JSON)
2. Use SQLite to import:

```sql
-- Import from CSV
.mode csv
.import reminders.csv reminders

-- Or write custom import script
INSERT INTO reminders (title, description, time, completed, category, frequency)
VALUES ('New Task', 'Description', '2024-11-10T10:00:00', 0, 'work', 'once');
```

## Performance

- Fast queries (< 1ms for typical operations)
- Indexed by created_at and category
- Handles thousands of reminders efficiently
- No network latency

## Future Plans

Currently the app uses local storage only. Potential future features:

- ⏳ Optional server sync (coming soon)
- ⏳ Cloud backup
- ⏳ Multi-device sync
- ⏳ Sharing reminders

But for now, enjoy the simplicity and privacy of local-only storage!
