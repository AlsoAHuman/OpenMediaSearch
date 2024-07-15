#!/bin/bash

# Function to search and open media
function movie() {
    # Check if fzf is installed
    if ! command -v fzf &>/dev/null; then
        echo "fzf is not installed. Please install it to use this script."
        return
    fi

    # Check if a search term is provided
    if [[ $# -eq 0 ]]; then
        echo "Please provide a search term."
        return
    fi

    # Get the search term and handle spaces within quotes
    search_term="${*// /+}"

    # Use fzf to search for local video files matching the search term
    local_file=$(find . -type f -iregex '.*\.\(mp4\|mkv\|avi\|mov\|flv\|webm\|m4v\|wmv\|mpg\|mpeg\|3gp\|mts\|m2ts\)' | fzf --query="$search_term" --preview="file {}" --preview-window=right:70% --exact --exit-0)

    # Check if a local file was found
    if [[ -n "$local_file" ]]; then
        # Open the local file
        echo "Opening local file: $local_file"
        if ! open "$local_file"; then
            echo "Failed to open the local file."
        fi
    else
        # Open the media search engine with the search term
        echo "Opening search engine for: $search_term"
        if ! open "https://watch.qtchaos.de/browse/$search_term"; then
            echo "Failed to open the URL in the default web browser."
        fi
    fi
}

# Check if the script is being sourced or executed
if [[ "${BASH_SOURCE[0]}" == "${0}" ]]; then
    # Execute the movie function with the provided search term
    movie "$@"
fi
