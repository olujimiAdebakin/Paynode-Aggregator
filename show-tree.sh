#!/bin/bash
# Simple tree implementation for Git Bash

show_tree() {
    local dir="${1:-.}"
    local prefix="${2:-}"
    local depth="${3:-3}"
    local current="${4:-0}"
    
    if [ "$current" -ge "$depth" ]; then
        return
    fi
    
    for item in "$dir"/*; do
        if [ ! -e "$item" ]; then continue; fi
        
        local name=$(basename "$item")
        
        # Skip these directories
        if [[ "$name" == "target" ]] || [[ "$name" == ".git" ]] || [[ "$name" == "node_modules" ]]; then
            continue
        fi
        
        echo "${prefix}├── $name"
        
        if [ -d "$item" ]; then
            show_tree "$item" "${prefix}│   " "$depth" $((current + 1))
        fi
    done
}

echo "$(basename $(pwd))/"
show_tree "." "" 3 0