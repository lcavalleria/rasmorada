#!/bin/bash

# Path to the UF2 file
UF2_FILE=target/thumbv6m-none-eabi/release/nerve.uf2

# Path to the mounted device.
# Assume it's the last connected device.
MOUNT_POINT=/media/last_connected_usb

# Copy UF2 file to the mounted device
if [ -n "$UF2_FILE" ]; then
  echo "Found file: $UF2_FILE. Copying to $MOUNT_POINT..."
  cp "$UF2_FILE" "$MOUNT_POINT/"
  echo "Done."
else
  echo "No UF2 file found."
fi

