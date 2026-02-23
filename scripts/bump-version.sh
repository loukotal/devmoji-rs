#!/usr/bin/env bash
set -euo pipefail

if [ $# -ne 1 ]; then
  echo "Usage: $0 <version>"
  echo "Example: $0 0.2.0"
  exit 1
fi

VERSION="$1"
ROOT="$(cd "$(dirname "$0")/.." && pwd)"

echo "Bumping version to $VERSION..."

# Update Cargo.toml
sed -i.bak "s/^version = \".*\"/version = \"$VERSION\"/" "$ROOT/Cargo.toml"
rm -f "$ROOT/Cargo.toml.bak"

# Update root npm package version
sed -i.bak "s/\"version\": \".*\"/\"version\": \"$VERSION\"/" "$ROOT/npm/devmoji-rs/package.json"
rm -f "$ROOT/npm/devmoji-rs/package.json.bak"

# Update optionalDependencies versions in root package
sed -i.bak "s/\"devmoji-rs-\([^\"]*\)\": \"[^\"]*\"/\"devmoji-rs-\1\": \"$VERSION\"/" "$ROOT/npm/devmoji-rs/package.json"
rm -f "$ROOT/npm/devmoji-rs/package.json.bak"

# Update all platform package versions
for pkg in "$ROOT"/npm/devmoji-rs-*/package.json; do
  sed -i.bak "s/\"version\": \".*\"/\"version\": \"$VERSION\"/" "$pkg"
  rm -f "$pkg.bak"
done

echo "Updated to version $VERSION:"
echo "  - Cargo.toml"
echo "  - npm/devmoji-rs/package.json"
for pkg in "$ROOT"/npm/devmoji-rs-*/package.json; do
  echo "  - ${pkg#$ROOT/}"
done

echo ""
echo "Don't forget to run 'cargo check' and commit the changes."
