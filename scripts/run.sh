#! /bin/bash

# Gets the output of all sample files and outputs them as files

# Get the absolute path of the script, to make sure it works regardless
# of where we're running it from
script_dir="$(dirname "$(realpath "$0")")"

# Go to the cargo workspace
cd "$script_dir/../"

# Get all non-directory files
files="$(find "sample_files" -maxdepth 1 -type f)"

for src_path in $files; do
  file_name="$(basename "$src_path")"
  write_path="sample_files/output/$file_name"

  echo "Running \"$file_name\":"

  # Redirect both stderr and stdout to the file
  cargo run --quiet "$src_path" > "$write_path" 2>&1
done
