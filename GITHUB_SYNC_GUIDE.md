# GitHub Sync Feature

This document describes how to use the GitHub sync feature to backup and synchronize your reminders.

## Overview

The Toolbox app now supports syncing your reminders with GitHub, providing:
- **Cloud Backup**: Store your reminders on GitHub for safekeeping
- **Cross-Device Sync**: Access your reminders from multiple devices
- **Version History**: GitHub automatically tracks changes to your data
- **Privacy**: All data is stored in your own GitHub account

## Setup Instructions

### 1. Generate a GitHub Personal Access Token

1. Go to [GitHub Settings → Tokens](https://github.com/settings/tokens/new)
2. Click "Generate new token (classic)"
3. Give your token a name (e.g., "Toolbox Reminders Sync")
4. Select the following scopes:
   - ✅ `repo` - Full control of private repositories
   - ✅ `gist` - Create gists (if using Gist method)
5. Click "Generate token" at the bottom
6. **Important**: Copy the token immediately - you won't be able to see it again!

### 2. Configure Sync Settings in Toolbox

1. Open the Toolbox app
2. Click the "⚙️ Sync Settings" button in the sidebar (bottom)
3. Enable sync by checking "Enable Sync"
4. Select "GitHub" as the data source
5. Choose your sync method:
   - **GitHub Gist**: Creates a private gist for your reminders (simpler, automatic)
   - **Repository JSON File**: Stores reminders as a JSON file in a repository (more control)

### 3. Enter Your GitHub Token

1. Paste your GitHub token in the "GitHub Token" field
2. If using "Repository JSON File" method, enter your repository in the format `username/repository`
3. Click "Test Connection" to verify your credentials
4. Click "Save Settings"

## Sync Methods

### Method 1: GitHub Gist (Recommended for Beginners)

**Pros:**
- Simple setup - no need to create a repository
- Automatically creates a private gist
- Each sync creates a new gist (version history)

**Cons:**
- Creates a new gist each time (not updating existing)
- No built-in way to fetch back from gist (one-way backup)

**Use Case:** Simple backup solution, one-way sync to cloud

### Method 2: Repository JSON File (Recommended for Advanced Users)

**Pros:**
- Full bidirectional sync (push and pull)
- Updates the same file (cleaner version history)
- Works with existing repositories
- Easy to view and edit reminders in GitHub

**Cons:**
- Requires creating a repository first
- Slightly more complex setup

**Use Case:** Cross-device sync, manual editing, integration with other tools

## Usage

### Manual Sync

**Sync to GitHub (Upload):**
1. Click "⬆️ Sync to GitHub" button
2. Wait for confirmation message
3. Your reminders are now backed up on GitHub!

**Sync from GitHub (Validate):**
1. Click "⬇️ Validate GitHub Data" button (only for Repository JSON method)
2. This checks your GitHub repository and validates the backup data
3. **Note**: Currently this only validates that data exists and is readable
4. **Coming Soon**: Full import functionality with merge/conflict resolution

### Auto Sync

1. Enable "Auto Sync" in settings
2. Set your preferred interval (default: 30 minutes)
3. The app will automatically sync TO GitHub at the specified interval
4. Check "Last Sync" timestamp to verify

**Important**: Auto-sync currently only uploads to GitHub (one-way backup)

## Security Considerations

### Token Security

- Your GitHub token is stored locally on your device
- The token is **never** sent to any server except GitHub
- Keep your token secret - it grants access to your repositories
- If you suspect your token is compromised, revoke it immediately at [GitHub Settings](https://github.com/settings/tokens)

### Best Practices

1. **Use a dedicated token**: Create a token specifically for Toolbox
2. **Minimum permissions**: Only grant necessary scopes (`repo` and `gist`)
3. **Regular token rotation**: Regenerate your token periodically
4. **Repository privacy**: Use private repositories for sensitive data

## Troubleshooting

### "Connection failed" Error

**Possible causes:**
- Invalid GitHub token
- Token doesn't have required scopes
- Network connectivity issues
- GitHub API rate limiting

**Solutions:**
1. Verify your token has `repo` and `gist` scopes
2. Generate a new token and try again
3. Check your internet connection
4. Wait a few minutes if rate-limited

### "Sync failed" Error

**Possible causes:**
- Repository doesn't exist or you don't have access
- Invalid repository format (should be `owner/repo`)
- GitHub API rate limiting

**Solutions:**
1. Verify repository name format: `username/repository`
2. Ensure the repository exists and you have write access
3. Try syncing again after a few minutes

### "Validation failed" Error

**Possible causes:**
- No reminders.json file exists in the repository
- File format is invalid
- Repository is private but token doesn't have access

**Solutions:**
1. Sync to GitHub first to create the file
2. Verify the repository is accessible with your token
3. Check that reminders.json contains valid JSON

## Data Format

When using the Repository JSON method, your reminders are stored as a JSON array:

```json
[
  {
    "id": 1,
    "title": "Buy groceries",
    "description": "Milk, bread, eggs",
    "time": "2024-01-15T10:00:00Z",
    "completed": false,
    "category": "shopping",
    "frequency": "once"
  }
]
```

You can manually edit this file on GitHub if needed, but be careful to maintain valid JSON format.

## FAQ

**Q: Can I sync to multiple devices?**
A: For backup: Yes! Use GitHub sync to backup from multiple devices. For true multi-device sync: Coming soon with import/merge functionality.

**Q: Will validation/import delete my local reminders?**
A: No. Currently, the "Validate GitHub Data" feature only checks that your backup exists and is readable. It does NOT modify local data. Full import with merge functionality is a planned enhancement.

**Q: How often should I sync?**
A: It depends on your usage. Auto-sync every 30 minutes is a good default for backups. Sync manually after important changes for peace of mind.

**Q: Can I use this with GitHub Enterprise?**
A: Not currently. The feature is designed for github.com only.

**Q: What happens if I change the token?**
A: Simply enter the new token in settings and save. Previous syncs remain unaffected.

**Q: Can I export my reminders without syncing?**
A: Currently, GitHub sync is the primary export mechanism. The data is stored in standard JSON format which is portable.

## Future Enhancements

Planned improvements for the sync feature:
- [ ] Smart merge for bidirectional sync
- [ ] Conflict resolution UI
- [ ] Sync status indicators in sidebar
- [ ] Multiple sync profiles
- [ ] Webhook support for real-time sync
- [ ] Support for syncing wiki pages
- [ ] Export to other formats (CSV, Markdown)

## Support

If you encounter issues with GitHub sync:
1. Enable Debug Logs in the sidebar
2. Check the console for error messages (F12)
3. Verify your GitHub token is valid and has correct permissions
4. Open an issue on the GitHub repository with details

---

**Version**: 1.0  
**Last Updated**: 2024-01-15
