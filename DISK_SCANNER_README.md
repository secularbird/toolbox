# Disk Space Scanner - Tauri Application

A desktop application built with Tauri 2 and Vue 3 that scans and analyzes disk space usage of folders and files.

## Features

- 🔍 **Recursive Directory Scanning** - Scan any folder and get detailed size information
- 📊 **Visual Size Representation** - See file/folder sizes with visual bars and percentages
- 🎯 **Depth Limiting** - Control how deep the scan goes to avoid long scans
- 🚀 **Fast Native Performance** - Built with Rust for optimal performance
- 💾 **Cross-Platform** - Works on macOS, Windows, and Linux
- 📁 **Easy Navigation** - Click to expand/collapse folders
- 🏠 **Quick Access** - Shortcut buttons for home directory and system roots
- ⚠️ **Error Handling** - Shows permission errors and inaccessible folders

## Screenshots

The app displays:
- A path input field with Home and Roots quick-access buttons
- Depth limit control for scanning
- Real-time scanning progress
- Hierarchical view of folders and files sorted by size
- Visual size bars showing relative space usage
- Total size summary

## Technology Stack

### Backend (Rust)
- Tauri 2.x
- Serde for serialization
- Tokio for async operations
- dirs for system path detection

### Frontend (Vue 3)
- Vue 3 Composition API
- TypeScript
- Vite build tool

## Installation

### Prerequisites
- Node.js (v16 or higher)
- npm or yarn
- Rust (latest stable)
- Tauri CLI

### Setup

1. Clone the repository
```bash
cd /Users/yaozhuang/projects/toolbox/tauri-vue-app
```

2. Install dependencies
```bash
npm install
```

3. Run in development mode
```bash
npm run tauri dev
```

4. Build for production
```bash
npm run tauri build
```

## Usage

### Scanning a Directory

1. **Enter a Path**: Type or paste a directory path in the input field
2. **Or Use Quick Access**: 
   - Click "Home" to scan your home directory
   - Click "Roots" to see available system roots (C:\ on Windows, / on Unix)
3. **Set Depth Limit** (Optional): Check "Limit depth" and set a value (1-10) to control how deep the scan goes
4. **Click Scan**: The app will analyze the directory structure
5. **Explore Results**: Click on folders to expand/collapse and see their contents

### Understanding the Results

- **📂 Folder Icon**: Directory (click to expand/collapse)
- **📄 File Icon**: File
- **Size Bar**: Visual representation of size relative to parent
- **Percentage**: Exact percentage of space used
- **⚠️ Warning Icon**: Permission error or inaccessible item

## API Commands

The application exposes the following Tauri commands:

### `scan_directory(path: string, maxDepth?: number)`
Scans a directory and returns its contents with size information.

**Parameters:**
- `path`: The directory path to scan
- `maxDepth` (optional): Maximum depth to scan (null for unlimited)

**Returns:** `DiskItem` object with hierarchical structure

### `get_home_directory()`
Returns the user's home directory path.

**Returns:** `string`

### `get_system_roots()`
Returns available system roots (drives on Windows, "/" on Unix).

**Returns:** `string[]`

### `format_bytes(bytes: number)`
Formats byte size into human-readable format (B, KB, MB, GB, TB).

**Returns:** `string`

## Data Structures

### DiskItem
```typescript
interface DiskItem {
  name: string;           // File/folder name
  path: string;           // Full path
  size: number;           // Size in bytes
  is_dir: boolean;        // Is directory?
  children?: DiskItem[];  // Child items (for directories)
  error?: string;         // Error message if any
}
```

## Performance Notes

- **Large Directories**: Scanning very large directories (100,000+ files) may take time
- **Depth Limiting**: Use depth limits for faster scans of large directory trees
- **Permission Errors**: The app handles permission errors gracefully and continues scanning
- **Sorting**: Results are automatically sorted by size (largest first) for easy identification of space hogs

## Troubleshooting

### Scan is taking too long
- Use the depth limit feature to scan only top-level folders
- Start with depth=2 or 3 for large directories

### Permission Denied errors
- Some system directories require elevated privileges
- These will show with a ⚠️ warning icon but won't stop the scan

### App won't start
- Ensure all dependencies are installed: `npm install`
- Check Rust is installed: `rustc --version`
- Try cleaning and rebuilding: `npm run tauri build --clean`

## Development

### Project Structure
```
tauri-vue-app/
├── src/                      # Vue frontend
│   ├── components/
│   │   ├── DiskScanner.vue  # Main scanner component
│   │   └── DiskItem.vue     # Individual item component
│   ├── App.vue
│   ├── AppSimple.vue        # Simple app with navigation
│   └── main.ts
├── src-tauri/               # Rust backend
│   ├── src/
│   │   ├── disk_scanner.rs  # Disk scanning logic
│   │   ├── lib.rs
│   │   └── main.rs
│   └── Cargo.toml
└── package.json
```

### Adding New Features

To extend the disk scanner:

1. **Add Rust command** in `src-tauri/src/disk_scanner.rs`
2. **Register command** in `src-tauri/src/lib.rs`
3. **Call from Vue** using `invoke()` from `@tauri-apps/api/core`

## License

MIT

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## Changelog

### Version 0.1.0 (Initial Release)
- Basic directory scanning functionality
- Visual size representation
- Depth limiting
- Error handling for permissions
- Cross-platform support (macOS, Windows, Linux)
