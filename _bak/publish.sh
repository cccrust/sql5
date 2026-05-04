#!/bin/bash
set -euo pipefail

function usage() {
    echo "Usage: ./publish.sh <version>"
    echo "  e.g., ./publish.sh 1.22"
    echo ""
    echo "Publishes sql5 to PyPI and creates GitHub Release."
    echo "Version must be higher than current: $(cat _version_current.txt 2>/dev/null || echo 'unknown')"
    exit 1
}

# Check arguments
if [ $# -ne 1 ]; then
    usage
fi

NEW_VERSION="$1"

# Validate version format (must be like 1.22, 1.22.3, etc.)
if ! [[ "$NEW_VERSION" =~ ^[0-9]+\.[0-9]+(\.[0-9]+)?$ ]]; then
    echo "ERROR: Invalid version format: $NEW_VERSION"
    echo "Must be like: 1.22 or 1.22.3"
    exit 1
fi

# Get current version
if [ -f _version_current.txt ]; then
    CURRENT_VERSION=$(cat _version_current.txt)
else
    CURRENT_VERSION="0.0.0"
fi

# Compare versions (semantic: 1.22 > 1.21 > 1.20)
function version_compare() {
    local v1=$1
    local v2=$2

    local v1_major=$(echo $v1 | cut -d. -f1)
    local v1_minor=$(echo $v1 | cut -d. -f2)
    local v1_patch=$(echo $v1 | cut -d. -f3)
    v1_patch=${v1_patch:-0}

    local v2_major=$(echo $v2 | cut -d. -f1)
    local v2_minor=$(echo $v2 | cut -d. -f2)
    local v2_patch=$(echo $v2 | cut -d. -f3)
    v2_patch=${v2_patch:-0}

    if [ "$v1_major" -gt "$v2_major" ]; then return 0; fi
    if [ "$v1_major" -lt "$v2_major" ]; then return 1; fi
    if [ "$v1_minor" -gt "$v2_minor" ]; then return 0; fi
    if [ "$v1_minor" -lt "$v2_minor" ]; then return 1; fi
    if [ "$v1_patch" -gt "$v2_patch" ]; then return 0; fi
    return 1
}

# Check if new version > current version
if ! version_compare "$NEW_VERSION" "$CURRENT_VERSION"; then
    echo "ERROR: Version must be higher than current ($CURRENT_VERSION)"
    echo "       Got: $NEW_VERSION"
    echo ""
    echo "To downgrade, manually edit:"
    echo "  - Cargo.toml"
    echo "  - sql5_pypi/sql5/__init__.py"
    echo "  - _version_current.txt"
    exit 1
fi

echo "Publishing sql5 v$NEW_VERSION (from v$CURRENT_VERSION)..."

# Check for uncommitted changes
if ! git diff --quiet || ! git diff --cached --quiet || [ -n "$(git status --porcelain)" ]; then
    echo "WARNING: Working tree has uncommitted changes"
    git status --short
    echo ""
    read -p "Continue? (y/N) " -n 1 -r
    echo
    if [[ ! $REPLY =~ ^[Yy]$ ]]; then
        exit 1
    fi
fi

# Backup current version
echo "Backing up current version to _version_backup.txt..."
echo "$CURRENT_VERSION" > _version_backup.txt

# Update Cargo.toml
echo "Updating Cargo.toml..."
sed -i.bak "s/^version = \".*\"/version = \"$NEW_VERSION\"/" Cargo.toml
rm -f Cargo.toml.bak

# Update Python package __init__.py
echo "Updating sql5_pypi/sql5/__init__.py..."
sed -i.bak "s/__version__ = \".*\"/__version__ = \"$NEW_VERSION\"/" sql5_pypi/sql5/__init__.py
rm -f sql5_pypi/sql5/__init__.py.bak

# Update pyproject.toml
echo "Updating pyproject.toml..."
sed -i.bak "s/^version = \".*\"/version = \"$NEW_VERSION\"/" sql5_pypi/pyproject.toml
rm -f sql5_pypi/pyproject.toml.bak

# Update setup.cfg
echo "Updating setup.cfg..."
sed -i.bak "s/^version = attr:.*/version = attr: sql5.__version__/" sql5_pypi/setup.cfg
rm -f sql5_pypi/setup.cfg.bak

# Update _version_current.txt
echo "$NEW_VERSION" > _version_current.txt

# Stage changes
echo "Staging files..."
git add Cargo.toml sql5_pypi/sql5/__init__.py sql5_pypi/pyproject.toml sql5_pypi/setup.cfg _version_current.txt

# Commit
echo "Committing..."
git commit -m "Bump version to $NEW_VERSION"

# Create and push tag
TAG="v$NEW_VERSION"
echo "Creating tag $TAG..."
git tag "$TAG"

echo ""
echo "Pushing to GitHub..."
git push origin main "$TAG"

echo ""
echo "Done! GitHub Actions will:"
echo "  1. Build binary for 4 platforms (macOS arm64, macOS x86_64, Linux, Windows)"
echo "  2. Upload to GitHub Release"
echo "  3. Publish to PyPI"
echo ""
echo "Watch: https://github.com/cccrust/sql5/actions"
echo ""
echo "After ~2 min, install with: pip install sql5==$NEW_VERSION"