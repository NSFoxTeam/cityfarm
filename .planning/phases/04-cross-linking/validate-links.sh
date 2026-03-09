#!/usr/bin/env bash
# validate-links.sh — Cross-folder link validation for CityFarm vault
# Usage: bash validate-links.sh [vault_path]
# Default vault_path: CityFarm/
# Compatible with bash 3.x (macOS default)

set -euo pipefail

VAULT="${1:-CityFarm/}"
VAULT="${VAULT%/}"

if [[ ! -d "$VAULT" ]]; then
  echo "ERROR: Directory '$VAULT' not found"
  exit 1
fi

# Build filename index as temp files
INDEX_FILE=$(mktemp)
FOLDER_FILE=$(mktemp)
trap 'rm -f "$INDEX_FILE" "$FOLDER_FILE"' EXIT

# Index: filename<TAB>path<TAB>folder
find "$VAULT" -name "*.md" -not -path "*/_templates/*" -type f | while IFS= read -r filepath; do
  filename=$(basename "$filepath" .md)
  rel="${filepath#$VAULT/}"
  folder="${rel%%/*}"
  echo "${filename}	${filepath}	${folder}"
done > "$INDEX_FILE"

# Exclusions for section check
EXCLUDE_PATTERN="MOC\|Home\|Glossary"

# Counters
TOTAL_LINKS=0
INTRA_FOLDER=0
CROSS_FOLDER=0
BROKEN_COUNT=0
MISSING_SECTION=0

BROKEN_REPORT=""
MISSING_REPORT=""

# Process each .md file
while IFS= read -r filepath; do
  filename=$(basename "$filepath" .md)
  rel="${filepath#$VAULT/}"
  source_folder="${rel%%/*}"

  # Skip templates
  case "$rel" in _templates/*) continue ;; esac

  # Check for ## Связанные заметки (only content notes)
  if ! echo "$filename" | grep -q "$EXCLUDE_PATTERN"; then
    if ! grep -q "## Связанные заметки" "$filepath" 2>/dev/null; then
      MISSING_SECTION=$((MISSING_SECTION + 1))
      MISSING_REPORT="${MISSING_REPORT}  MISSING: ${rel}\n"
    fi
  fi

  # Extract all wiki-links
  while IFS= read -r link_match; do
    [[ -z "$link_match" ]] && continue
    # Strip [[ and ]]
    inner="${link_match#\[\[}"
    inner="${inner%\]\]}"
    # Strip display text
    target="${inner%%|*}"
    # Strip heading anchors
    target="${target%%#*}"
    # Trim whitespace
    target=$(echo "$target" | sed 's/^[[:space:]]*//;s/[[:space:]]*$//')

    [[ -z "$target" ]] && continue

    TOTAL_LINKS=$((TOTAL_LINKS + 1))

    # Look up target in index
    target_line=$(grep "^${target}	" "$INDEX_FILE" 2>/dev/null | head -1 || true)

    if [[ -z "$target_line" ]]; then
      BROKEN_COUNT=$((BROKEN_COUNT + 1))
      BROKEN_REPORT="${BROKEN_REPORT}  BROKEN: ${rel} -> [[${target}]]\n"
      continue
    fi

    # Get target folder
    target_folder=$(echo "$target_line" | cut -f3)

    if [[ "$source_folder" == "$target_folder" ]]; then
      INTRA_FOLDER=$((INTRA_FOLDER + 1))
    else
      CROSS_FOLDER=$((CROSS_FOLDER + 1))
    fi
  done < <(grep -oE '\[\[[^]]+\]\]' "$filepath" 2>/dev/null || true)

done < <(find "$VAULT" -name "*.md" -not -path "*/_templates/*" -type f | sort)

# Calculate ratio
if [[ $TOTAL_LINKS -gt 0 ]]; then
  RATIO=$(awk "BEGIN {printf \"%.1f\", ($CROSS_FOLDER / $TOTAL_LINKS) * 100}")
else
  RATIO="0.0"
fi

# Output
echo "=== CityFarm Wiki-Link Validation ==="
echo ""
echo "--- Link Counts ---"
echo "Total wiki-links:    $TOTAL_LINKS"
echo "  Intra-folder:      $INTRA_FOLDER"
echo "  Cross-folder:      $CROSS_FOLDER"
echo "  Cross-folder ratio: ${RATIO}%"
echo ""
echo "--- Broken Links ---"
echo "Broken links: $BROKEN_COUNT"
if [[ $BROKEN_COUNT -gt 0 ]]; then
  printf "$BROKEN_REPORT"
fi
echo ""
echo "--- Missing ## Связанные заметки ---"
echo "Notes missing section: $MISSING_SECTION"
if [[ $MISSING_SECTION -gt 0 ]]; then
  printf "$MISSING_REPORT"
fi
echo ""
echo "=== Summary ==="
echo "Total: $TOTAL_LINKS links | Intra: $INTRA_FOLDER | Cross: $CROSS_FOLDER (${RATIO}%) | Broken: $BROKEN_COUNT | Missing sections: $MISSING_SECTION"
