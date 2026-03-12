.phony: all build build-win64 build-linux dev dev-win64 dev-linux setup-steamworks clean help

STEAMWORKS_SDK_PATH ?= $(HOME)/.steamworks/sdk

all: dev

#################
## Development ##
#################

dev: dev-win64

dev-win64: # WARN: Only tested on WSL
	@STEAMWORKS_SDK_PATH=${STEAMWORKS_SDK_PATH} cargo run --target x86_64-pc-windows-gnu

dev-linux:
	@cargo run

###########
## Build ##
###########

build: build-win64 build-linux

build-win64: # WARN: Only tested on WSL
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