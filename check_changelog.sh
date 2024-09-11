#!/bin/bash
set -e

# Check if both files are provided as arguments
if [ "$#" -ne 2 ]; then
    echo "Usage: $0 <expected_file> <test_file>"
    exit 1
fi

file_A="$1"
file_B="$2"

# Check if both files exist
if [ ! -f "$file_A" ]; then
    echo "File A ($file_A) does not exist."
    exit 1
fi

if [ ! -f "$file_B" ]; then
    echo "File B ($file_B) does not exist."
    exit 1
fi

# Get the length of content B
length_B=$(wc -c < "$file_B")

# Save the head of file A to a temporary file
temp_file_A=$(mktemp)
head -c "$length_B" "$file_A" > "$temp_file_A"

# Compare the content
if diff "$temp_file_A" "$file_B" > /dev/null; then
    echo "File $file_A starts with the content of file $file_B."
    rm "$temp_file_A"
    exit 0
else
    echo "File $file_A does not start with the content of file $file_B."
    echo
    echo "This is the expected content at the start of $file_A:"
    echo "-----------------------------------------------------------------------"
    grep -v '^<!-- End section Unreleased -->$' < "$file_B"
    rm "$temp_file_A"
    exit 1
fi
