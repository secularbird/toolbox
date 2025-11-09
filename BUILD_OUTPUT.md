# Build Output Configuration

## Build Artifacts Location

All Rust build artifacts are stored in the project directory:

```
tauri-vue-app/
├── src-tauri/
│   └── target/           # Rust build output
│       ├── debug/        # Debug builds
│       │   └── bundle/   # Debug installers
│       └── release/      # Release builds
│           └── bundle/   # Production installers
│               ├── dmg/        # macOS disk images
│               ├── macos/      # macOS .app bundles
│               ├── deb/        # Linux Debian packages
│               ├── appimage/   # Linux AppImages
│               ├── msi/        # Windows installers
│               └── nsis/       # Windows NSIS installers
└── dist/                 # Vue frontend build output
```

## Configuration

The build output location is configured in `.cargo/config.toml`:

```toml
[build]
target-dir = "src-tauri/target"
```

## Build Commands

### Development Build
```bash
npm run tauri dev
```
- Output: `src-tauri/target/debug/`
- Not optimized, includes debug symbols

### Production Build
```bash
npm run tauri build
```
- Output: `src-tauri/target/release/bundle/`
- Optimized with:
  - Size optimization (`opt-level = "z"`)
  - Link-time optimization (LTO)
  - Symbol stripping
  - Single codegen unit

## Installers

After building for production, find installers in:

**macOS:**
- `src-tauri/target/release/bundle/dmg/*.dmg` - Disk image
- `src-tauri/target/release/bundle/macos/*.app` - Application bundle

**Linux:**
- `src-tauri/target/release/bundle/deb/*.deb` - Debian package
- `src-tauri/target/release/bundle/appimage/*.AppImage` - AppImage

**Windows:**
- `src-tauri/target/release/bundle/msi/*.msi` - MSI installer
- `src-tauri/target/release/bundle/nsis/*.exe` - NSIS installer

## Cleaning Build Artifacts

To clean build outputs:

```bash
# Clean Rust artifacts
cd src-tauri && cargo clean

# Clean frontend artifacts
rm -rf dist

# Clean all
npm run clean  # (if script exists)
```

## Git Ignore

The following are ignored by git (see `.gitignore`):
- `src-tauri/target/` - All Rust build artifacts
- `dist/` - Frontend build output
- `node_modules/` - Node dependencies

## Size Optimization

Release builds are optimized for smaller binary size:
- **opt-level = "z"**: Aggressive size optimization
- **lto = true**: Link-time optimization across all crates
- **codegen-units = 1**: Better optimization at cost of compile time
- **strip = true**: Remove debug symbols from final binary

This typically reduces binary size by 50-70% compared to default settings.
