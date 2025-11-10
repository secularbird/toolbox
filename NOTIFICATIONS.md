# Notification System

The application automatically shows popup notifications when reminders are due.

## Features

- ✅ Automatic notification checking (every 30 seconds)
- ✅ Popup window in top-right corner
- ✅ Beautiful gradient design
- ✅ Dismiss button
- ✅ Snooze for 5 minutes
- ✅ Auto-dismiss after 10 seconds
- ✅ Always on top
- ✅ No taskbar entry

## How It Works

### Automatic Checking

The notification service runs in the background and checks for due reminders every 30 seconds:

```rust
const CHECK_INTERVAL_SECONDS: u64 = 30;
```

### Notification Window

When a reminder is due, a popup window appears in the top-right corner:
- **Size**: 350x120 pixels
- **Position**: 20px from top-right edge
- **Style**: Gradient purple background
- **Animation**: Slides in from right

### Notification Query

Reminders are considered "due" when:
- `completed = 0` (not marked complete)
- `time <= now` (time has passed)
- `time > now - 30 seconds` (within last 30 seconds)

This ensures you don't miss notifications even if the app was closed.

## User Actions

### Dismiss
Closes the notification immediately without any changes to the reminder.

### Snooze 5m
- Updates the reminder time to +5 minutes from now
- Closes the notification
- Reminder will appear again after 5 minutes

## Customization

### Change Check Interval

Edit `src-tauri/src/notifications/mod.rs`:

```rust
const CHECK_INTERVAL_SECONDS: u64 = 60;  // Check every minute
```

### Change Snooze Duration

Edit `notification.html`:

```html
<button class="snooze-btn" onclick="snooze(10)">Snooze 10m</button>
```

### Change Auto-Dismiss Time

Edit `src-tauri/src/notifications/mod.rs`:

```rust
sleep(Duration::from_secs(15)).await;  // Auto-close after 15 seconds
```

### Change Position

Edit notification position in `src-tauri/src/notifications/mod.rs`:

```rust
// Top-left corner
let x = margin;
let y = margin;

// Bottom-right corner
let x = screen_width - width - margin;
let y = screen_height - height - margin;

// Bottom-left corner
let x = margin;
let y = screen_height - height - margin;
```

### Change Appearance

Edit `notification.html` CSS:

```css
.notification {
  background: linear-gradient(135deg, #f093fb 0%, #f5576c 100%); /* Pink gradient */
  /* or */
  background: #2c3e50; /* Solid dark gray */
}
```

## Testing

### Create a Test Reminder

1. Set time to current time + 1 minute
2. Wait for notification to appear
3. Test Dismiss and Snooze buttons

### Manual Test

```bash
# Add reminder with near-future time
sqlite3 ~/Library/Application\ Support/com.yaozhuang.tauri-vue-app/reminders.db \
  "INSERT INTO reminders (title, description, time, completed, category, frequency) 
   VALUES ('Test', 'Test notification', datetime('now', '+1 minute'), 0, 'personal', 'once');"
```

## Troubleshooting

### Notifications Not Showing

**Check the logs:**
```bash
npm run tauri dev
# Look for: "Found X due reminders"
```

**Verify reminder time:**
```sql
SELECT id, title, time, completed FROM reminders 
WHERE completed = 0 AND time <= datetime('now');
```

### Window Doesn't Appear

1. Check if window already exists (only one per reminder)
2. Check screen resolution detection
3. Verify `notification.html` is in correct location

### Notification Stays Too Long

Adjust auto-dismiss timer or click Dismiss button.

## Future Enhancements

- [ ] Add notification sound
- [ ] Multiple snooze options (5m, 10m, 30m, 1h)
- [ ] Custom snooze time input
- [ ] Mark as complete from notification
- [ ] Rich notification content (description, category)
- [ ] Notification history
- [ ] Do Not Disturb mode
- [ ] Notification preferences per category
