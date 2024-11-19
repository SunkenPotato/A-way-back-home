#!/bin/sh
cargo build --target x86_64-pc-windows-gnu --features "bevy/dynamic_linking" &&
cp target/x86_64-pc-windows-gnu/debug/mygame.exe . &&
exec ./mygame.exe "$@"
