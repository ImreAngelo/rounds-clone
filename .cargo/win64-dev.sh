#!/bin/bash
# Runner script for cargo to execute Windows .exe files from WSL.
# Strips DWARF debug sections — GNU ld inflates SizeOfImage to ~2GB when debug symbols
# are embedded, which causes Windows to reject loading the exe.
set -e

EXE_PATH="$1"
shift

# Without fast_compile (static bevy), GNU ld embeds DWARF debug info that inflates
# SizeOfImage to ~2GB, causing Windows to reject the exe. Strip it.
# With fast_compile (bevy_dylib), the exe is small and loads fine without stripping.
EXE_SIZE=$(stat -c%s "$EXE_PATH")
if [ "$EXE_SIZE" -gt 524288000 ]; then  # 500 MB threshold
    x86_64-w64-mingw32-strip --strip-debug "$EXE_PATH"
fi

WIN_EXE=$(wslpath -w "$EXE_PATH")
WIN_DIR=$(wslpath -w "$(dirname "$EXE_PATH")")
exec powershell.exe -NoProfile -Command "Set-Location '$WIN_DIR'; & '$WIN_EXE'"
