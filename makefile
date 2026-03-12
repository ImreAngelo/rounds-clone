.phony: all build build-win64 build-linux dev dev-wsl dev-win64 dev-linux setup-steamworks clean help

STEAMWORKS_SDK_PATH ?= $(HOME)/.steamworks/sdk

all: dev

#################
## Development ##
#################

dev: dev-wsl

dev-wsl:
	@STEAMWORKS_SDK_PATH=${STEAMWORKS_SDK_PATH} cargo run --target x86_64-pc-windows-gnu

dev-win64: # --features "fast_compile"
	@STEAMWORKS_SDK_PATH=${STEAMWORKS_SDK_PATH} cargo run --target x86_64-pc-windows-msvc

dev-linux:
	@cargo run --features "fast_compile"

###########
## Build ##
###########

build: build-win64 build-linux

build-win64: # WARN: Only tested on WSL, using msvc target might be better on native windows
	@STEAMWORKS_SDK_PATH=${STEAMWORKS_SDK_PATH} cargo build --target x86_64-pc-windows-gnu --release

build-linux:
	@cargo build --release

###########
## Setup ##
###########

setup-steamworks:
	@ln -s ~/snap/steam/common/.steam ~/.steam
	
#############
## Utility ##
#############

clean:
	@cargo clean

help:
	@echo "dev		- 	build and run development build"
	@echo "build	- 	build production build"