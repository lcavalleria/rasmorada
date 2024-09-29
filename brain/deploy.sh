#!/bin/bash -e

# must contain USER and IP
source ssh_target.secret
echo "USER: $USER"
echo "IP: $IP"

echo "Ensure cross is installed"
cargo install cross

echo "Building the release..."
cross build --release
echo "Built"

REMOTE_PATH="/home/$USER/brain/"
echo "Delete old version."
ssh $USER@$IP rm -f $REMOTE_PATH/brain
echo "Copy the build to the ssh target."
scp -r ./target/aarch64-unknown-linux-gnu/release/brain $USER@$IP:$REMOTE_PATH
echo "Copied, executing..."
ssh $USER@$IP RUST_BACKTRACE=1 $REMOTE_PATH/brain
