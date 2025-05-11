#!/bin/bash
cargo build --target=x86_64-pc-windows-gnu

./target/x86_64-pc-windows-gnu/debug/tanker_client.exe
