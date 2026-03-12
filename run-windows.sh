#!/bin/bash
# Runner script for cargo to execute Windows .exe files from WSL.
# Strips DWARF debug sections — GNU ld inflates SizeOfImage to ~2GB when debug symbols
# are embedded, which causes Windows to reject loading the exe.
set -e

EXE_PATH="$1"
shift
# Strip DWARF debug sections in-place so Windows can load the exe
x86_64-w64-mingw32-strip --strip-debug "$EXE_PATH"

WIN_EXE=$(wslpath -w "$EXE_PATH")
WIN_DIR=$(wslpath -w "$(dirname "$EXE_PATH")")
exec powershell.exe -NoProfile -Command "Set-Location '$WIN_DIR'; & '$WIN_EXE'"
