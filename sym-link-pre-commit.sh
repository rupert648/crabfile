#!/bin/bash
# Save as .git/hooks/pre-commit and make executable

# Find all symlinks that are tracked by git
git ls-files -s | grep ^120000 | cut -f2 | while read -r file; do
    if [ -L "$file" ]; then
        target=$(readlink -f "$file")  # -f follows the link recursively
        if [ -d "$target" ]; then
            # Handle directory
            rm "$file"
            cp -rL "$target" "$file"  # -r for recursive, -L to follow symlinks
        else
            # Handle regular file
            rm "$file"
            cp -L "$target" "$file"
        fi
        git add "$file"
    fi
done
