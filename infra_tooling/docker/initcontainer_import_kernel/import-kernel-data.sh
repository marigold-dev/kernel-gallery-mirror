#!/usr/bin/env bash
set -x

VOLUME_PATH="/mounted_volume/rollup_workdir"
TARGET_DIR="$VOLUME_PATH/rollup"
SOURCE_ROLLUP_DIR="/app/rollup"
CHECKSUM_FILE="/app/installer.md5.metadata"
FORCE_RE_IMPORT="${FORCE_RE_IMPORT:-NO}"

# Function to copy directories and files
copy_contents() {
    cp -r "$SOURCE_ROLLUP_DIR" "$1"
    cp /app/*.metadata "$1"
}

echo "Current content of $VOLUME_PATH:"
ls -last "$VOLUME_PATH"
echo ; echo "Content of the /app dir in Docker img:"
ls -last /app

echo "Checking if the target directory exists..."
if [[ -d "$TARGET_DIR" ]]; then
    echo "Directory exists, checking the checksum..."
    CURRENT_CHECKSUM=$(cat $VOLUME_PATH/installer.md5.metadata | awk '{ print $1 }')
    ORIGINAL_CHECKSUM=$(cat $CHECKSUM_FILE | awk '{ print $1 }')

    echo "Comparing the checksums..."
    if [[ "$CURRENT_CHECKSUM" != "$ORIGINAL_CHECKSUM" ]] || [[ "$FORCE_RE_IMPORT" == *"YES"* ]]; then
        echo "Checksums are different, or re-import is forced. Updating the directory..."
        rm -rf "$TARGET_DIR"
        rm -f "$VOLUME_PATH"/*.metadata
        copy_contents "$VOLUME_PATH"
        chmod -R 777 "$VOLUME_PATH"
        ls -last "$VOLUME_PATH"
    else
        echo "This rollup kernel is already present on the machine. Nothing to do, exiting."
    fi
else
    echo "Directory does not exist, importing..."
    copy_contents "$VOLUME_PATH"
    chmod -R 777 "$VOLUME_PATH"
    ls -last "$VOLUME_PATH"
fi
