#!/bin/bash

# Muezzin Release Script
# Usage: ./scripts/release.sh 1.0.0.1

set -e

if [ -z "$1" ]; then
    echo "Usage: ./scripts/release.sh <version>"
    echo "Example: ./scripts/release.sh 1.0.0.1"
    exit 1
fi

VERSION="$1"
FULL_VERSION="v${VERSION}"

echo "🕌 Preparing Muezzin release ${FULL_VERSION}"

# Update version in all files
echo "📝 Updating version in config files..."

# Update tauri.conf.json
if [[ "$OSTYPE" == "darwin"* ]]; then
    # macOS
    sed -i '' "s/\"version\": \".*\"/\"version\": \"${VERSION}\"/" tauri.conf.json
    sed -i '' "s/^version = \".*\"/version = \"${VERSION}\"/" Cargo.toml
    sed -i '' "s/\"version\": \".*\"/\"version\": \"${VERSION}\"/" package.json
else
    # Linux
    sed -i "s/\"version\": \".*\"/\"version\": \"${VERSION}\"/" tauri.conf.json
    sed -i "s/^version = \".*\"/version = \"${VERSION}\"/" Cargo.toml
    sed -i "s/\"version\": \".*\"/\"version\": \"${VERSION}\"/" package.json
fi

echo "✅ Version updated to ${VERSION}"

# Commit changes
git add tauri.conf.json Cargo.toml package.json
git commit -m "chore: bump version to ${FULL_VERSION}"

# Create tag
git tag ${FULL_VERSION}

echo "✅ Created tag ${FULL_VERSION}"
echo ""
echo "To push and trigger release build, run:"
echo "  git push origin main --tags"
echo ""
echo "Or to cancel:"
echo "  git reset --hard HEAD~1"
echo "  git tag -d ${FULL_VERSION}"
