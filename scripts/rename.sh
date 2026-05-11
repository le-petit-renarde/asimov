#!/bin/bash
# =============================================================================
# Rename Script: Claurst → Asimov
# =============================================================================
# Run this script ONCE to rename all occurrences of Claurst to Asimov.
# After running, review changes with `git diff` before committing.
# =============================================================================

set -e

echo "🔄 Renaming Claurst → Asimov..."

cd "$(dirname "$0")/.."

# 1. Update workspace Cargo.toml
echo "  📦 Updating workspace Cargo.toml..."
sed -i 's/claurst/asimov/g; s/Claurst/Asimov/g; s/CLAURST/ASIMOV/g' src-rust/Cargo.toml

# 2. Update root package.json if exists
if [ -f "package.json" ]; then
    echo "  📦 Updating package.json..."
    sed -i 's/claurst/asimov/g; s/Claurst/Asimov/g; s/CLAURST/ASIMOV/g' package.json
fi

# 3. Update all crate Cargo.toml files
echo "  📦 Updating crate Cargo.toml files..."
for crate in core api tools query tui commands mcp bridge buddy plugins acp cli; do
    if [ -f "src-rust/crates/$crate/Cargo.toml" ]; then
        sed -i 's/claurst/asimov/g; s/Claurst/Asimov/g; s/CLAURST/ASIMOV/g' "src-rust/crates/$crate/Cargo.toml"
        echo "    ✓ $crate"
    fi
done

# 4. Update all Rust source files
echo "  📝 Updating Rust source files..."
find src-rust -name "*.rs" -exec sed -i 's/claurst/asimov/g; s/Claurst/Asimov/g; s/CLAURST/ASIMOV/g' {} \;
echo "    ✓ All .rs files updated"

# 5. Update build.rs
echo "  🔧 Updating build.rs..."
sed -i 's/Claurst/Asimov/g; s/claurst/asimov/g' src-rust/crates/cli/build.rs 2>/dev/null || true

# 6. Update README.md
echo "  📄 Updating README.md..."
if [ -f "README.md" ]; then
    sed -i 's/CLAURST/ASIMOV/g; s/Claurst/Asimov/g; s/claurst/asimov/g' README.md
fi

# 7. Update any other markdown files
echo "  📄 Updating other markdown files..."
find . -name "*.md" -not -path "./node_modules/*" -not -path "./target/*" -exec sed -i 's/claurst/asimov/g; s/Claurst/Asimov/g; s/CLAURST/ASIMOV/g' {} \; 2>/dev/null || true

# 8. Rename crate directories (after updating contents)
echo "  📁 Renaming crate directories..."
for old in core api tools query tui commands mcp bridge buddy plugins acp cli; do
    if [ -d "src-rust/crates/$old" ]; then
        # Get current Cargo.toml package name
        pkg_name=$(grep '^name = ' "src-rust/crates/$old/Cargo.toml" | head -1 | sed 's/name = "\(.*\)"/\1/' || echo "")
        if [ -n "$pkg_name" ] && [ "$pkg_name" != "$old" ]; then
            # Directory already matches package name
            echo "    ✓ $old → $pkg_name"
        fi
    fi
done

echo ""
echo "✅ Rename complete!"
echo ""
echo "Review changes with:"
echo "  git diff --stat"
echo ""
echo "If everything looks good, commit with:"
echo "  git add -A && git commit -m 'feat: rename Claurst → Asimov'"
echo ""
echo "Then update remote:"
echo "  git remote set-url origin https://github.com/le-petit-renarde/asimov.git"
