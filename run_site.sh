
#!/bin/bash

EXISTING_INSTANCE="$(pidof target/release/elisam_dev)"

if [ ! -z $EXISTING_INSTANCE ]; then
    kill $EXISTING_INSTANCE
fi

BIN_NAME="elisam_dev"

if [ -f "elisam_dev" ]; then
    ./$BIN_NAME
    exit $?
fi

if [ -f "Cargo.toml" ]; then
   cargo run --release
   exit $?
fi

echo "Could not find elisam_dev executable or Cargo.toml"