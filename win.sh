#!/bin/sh
cargo build --target x86_64-pc-windows-gnu &&
cp target/x86_64-pc-windows-gnu/debug/asteroids-bevy.exe . &&
exec ./asteroids-bevy.exe "$@"