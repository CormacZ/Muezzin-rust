# Muezzin Versioning Scheme

Muezzin follows a **four-part semantic versioning** system: `v1.X.X.X`

## Format: v1.MAJOR.MINOR.PATCH

- **Position 1 (v1)**: Major remaster/rewrite (we start at v1 for the Rust/Tauri remaster)
- **Position 2 (MAJOR)**: Major feature updates, breaking changes
- **Position 3 (MINOR)**: Security fixes, performance improvements, non-breaking features
- **Position 4 (PATCH)**: Bug fixes only

## Examples

- **v1.0.0.0** - Initial Rust/Tauri release
- **v1.0.0.1** - Bug fix (e.g., fix adhan not playing)
- **v1.0.0.2** - Another bug fix
- **v1.0.1.0** - Security fix or performance improvement
- **v1.0.2.0** - Another security/performance update
- **v1.1.0.0** - Major new feature (e.g., new Quran reader)
- **v1.2.0.0** - Another major feature (e.g., Tasbih counter)
- **v2.0.0.0** - NEVER (reserved for future complete rewrites)

## Release Process

### Automated (Recommended)

1. Update version in all three files:
   - `tauri.conf.json`
   - `src-tauri/Cargo.toml` (in the root directory, this file maps to src-tauri)
   - `package.json`

2. Commit the version bump:
   ```bash
   git add tauri.conf.json Cargo.toml package.json
   git commit -m "chore: bump version to v1.X.X.X"
   git push origin main
   ```

3. Create and push tag:
   ```bash
   git tag v1.X.X.X
   git push origin main --tags
   ```

4. GitHub Actions will automatically:
   - Build for all platforms (Windows x64, macOS Intel, macOS Apple Silicon, Linux x64)
   - Create GitHub release
   - Upload all installers (.exe, .msi, .dmg, .deb, .AppImage)

### Manual Trigger

You can also manually trigger a release from GitHub:

1. Go to your repository on GitHub
2. Click "Actions" tab
3. Select "Release Build" workflow
4. Click "Run workflow"
5. Enter version: `v1.X.X.X`
6. Click "Run workflow"

## Version History

- **v1.0.0.0** (2025-11-12): Initial Rust/Tauri remaster release
  - Complete rewrite from Electron to Rust + Tauri 2.x
  - 70-80% memory reduction
  - 90% binary size reduction
  - 2-4x faster startup
  - Enhanced security and privacy

## Development Guidelines

### When to Increment Each Part

#### PATCH (v1.0.0.X)
Increment for:
- Bug fixes
- Typo corrections
- Minor UI adjustments
- Documentation updates

#### MINOR (v1.0.X.0)
Increment for:
- Security patches
- Performance optimizations
- Small non-breaking features
- Dependency updates (security)

#### MAJOR (v1.X.0.0)
Increment for:
- New major features
- Significant UI/UX changes
- Breaking API changes
- Database schema changes
- New platform support

#### REMASTER (vX.0.0.0)
**Never increment!** This position is reserved for:
- Complete application rewrites
- Fundamental architecture changes
- We will stay at v1 for the entire lifecycle of this Rust/Tauri version

## Quick Release Script

Use the included release script for convenience:

```bash
# Make it executable (first time only)
chmod +x scripts/release.sh

# Create a new release
./scripts/release.sh 1.0.0.1

# This will update all version files and create the tag
# Then push with:
git push origin main --tags
```

## Checking Current Version

```bash
# From Git tags
git describe --tags --abbrev=0

# From Cargo.toml
grep '^version' Cargo.toml

# From package.json
grep '"version"' package.json

# From tauri.conf.json
grep '"version"' tauri.conf.json
```

## Notes

- All version numbers must match across all three config files
- Always use the format `v1.X.X.X` for git tags (with the 'v' prefix)
- Version numbers in config files should not have the 'v' prefix
- GitHub Actions will only trigger on tags matching `v1.*.*.*`
