.phony: all build dev dev-wsl dev-linux run

STEAMWORKS_SDK_PATH ?= $(HOME)/.steamworks/sdk

all: dev

#################
## Development ##
#################

dev: dev-wsl

dev-wsl:
	@cargo run --target x86_64-pc-windows-gnu --no-default-features

dev-linux:
	@cargo run

###########
## Build ##
###########

build:
	@cargo build --target x86_64-pc-windows-gnu --no-default-features

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