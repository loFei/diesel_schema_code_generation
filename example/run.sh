#!/bin/bash

cargo build --release

if [ $? -eq 0 ]; then
  cp ./target/release/example ./
  echo ""
  ./example
else
  echo "Build failed"
fi
