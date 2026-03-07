# Makefile for building/running a Rust project and syncing assets into target dirs

SHELL := /usr/bin/env bash

CARGO      ?= cargo
ASSETS_DIR ?= assets

# Default targets (override from CLI if desired, e.g. `make build-linux TARGET_LINUX=aarch64-unknown-linux-gnu`)
TARGET_WIN   ?= x86_64-pc-windows-gnu
TARGET_LINUX ?= x86_64-unknown-linux-gnu

# macOS target: default based on host arch (override if you want the other)
UNAME_M := $(shell uname -m 2>/dev/null || echo unknown)
ifeq ($(UNAME_M),arm64)
  TARGET_MAC ?= aarch64-apple-darwin
else ifeq ($(UNAME_M),aarch64)
  TARGET_MAC ?= aarch64-apple-darwin
else
  TARGET_MAC ?= x86_64-apple-darwin
endif

# Profile control: PROFILE=debug|release
PROFILE     ?= debug
PROFILE_DIR := $(if $(filter release,$(PROFILE)),release,debug)
CARGO_PROFILE_FLAG := $(if $(filter release,$(PROFILE)),--release,)

# Derived paths
TARGET_DIR := target/$(TARGET)/$(PROFILE_DIR)
ASSETS_OUT := $(TARGET_DIR)/assets

.PHONY: help \
        sync-assets clean-assets \
        run-windows \
        build-windows build-linux build-mac \
        release-windows release-linux release-mac release \
        debug

help:
	@echo "Targets:"
	@echo "  run-windows        Sync assets + cargo run --target $(TARGET_WIN) (debug)"
	@echo "  build-windows      Sync assets + cargo build --target $(TARGET_WIN) (debug)"
	@echo "  build-linux        Sync assets + cargo build --target \$$$${TARGET_LINUX} (debug)"
	@echo "  build-mac          Sync assets + cargo build --target \$$$${TARGET_MAC} (debug)"
	@echo "  release-windows    Sync assets + cargo build --release --target $(TARGET_WIN)"
	@echo "  release-linux      Sync assets + cargo build --release --target \$$$${TARGET_LINUX}"
	@echo "  release-mac        Sync assets + cargo build --release --target \$$$${TARGET_MAC}"
	@echo "  release            Build all release targets (windows+linux+mac)"
	@echo ""
	@echo "Variables:"
	@echo "  PROFILE=debug|release (default: debug)"
	@echo "  ASSETS_DIR=assets     (default: assets)"
	@echo "  TARGET=<triple>       (used by sync-assets/clean-assets)"
	@echo "  TARGET_LINUX=<triple> (default: $(TARGET_LINUX))"
	@echo "  TARGET_MAC=<triple>   (default: $(TARGET_MAC))"
	@echo ""
	@echo "Examples:"
	@echo "  make run-windows"
	@echo "  make build-linux"
	@echo "  make release-mac"
	@echo "  make release TARGET_LINUX=aarch64-unknown-linux-gnu"

# --- Asset syncing ---
# Usage: make sync-assets TARGET=<triple> PROFILE=debug|release
sync-assets:
	@mkdir -p "$(TARGET_DIR)"
	@rm -rf "$(ASSETS_OUT)"
	@cp -R "$(ASSETS_DIR)" "$(ASSETS_OUT)"
	@echo "Synced assets -> $(ASSETS_OUT)"

clean-assets:
	@rm -rf "$(ASSETS_OUT)"
	@echo "Removed assets -> $(ASSETS_OUT)"

# --- Targets mirroring your script (Windows in WSL) ---
run-windows: TARGET := $(TARGET_WIN)
run-windows: PROFILE := debug
run-windows: sync-assets
	$(CARGO) run --target "$(TARGET_WIN)"

# --- Debug builds ---
build-windows: TARGET := $(TARGET_WIN)
build-windows: PROFILE := debug
build-windows: sync-assets
	$(CARGO) build --target "$(TARGET_WIN)"

build-linux: TARGET := $(TARGET_LINUX)
build-linux: PROFILE := debug
build-linux: sync-assets
	$(CARGO) build --target "$(TARGET_LINUX)"

build-mac: TARGET := $(TARGET_MAC)
build-mac: PROFILE := debug
build-mac: sync-assets
	$(CARGO) build --target "$(TARGET_MAC)"

# --- Release builds ---
release-windows: TARGET := $(TARGET_WIN)
release-windows: PROFILE := release
release-windows: sync-assets
	$(CARGO) build --release --target "$(TARGET_WIN)"

release-linux: TARGET := $(TARGET_LINUX)
release-linux: PROFILE := release
release-linux: sync-assets
	$(CARGO) build --release --target "$(TARGET_LINUX)"

release-mac: TARGET := $(TARGET_MAC)
release-mac: PROFILE := release
release-mac: sync-assets
	$(CARGO) build --release --target "$(TARGET_MAC)"

release: release-windows release-linux release-mac

# Convenience
debug: build-windows build-linux build-mac