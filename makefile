.phony: all build build-win64 build-linux dev dev-wsl dev-win64 dev-linux setup-steamworks clean help

all: dev

#################
## Development ##
#################

dev: dev-wsl-win64

dev-wsl-win64: # TODO: --features "fast_compile"
	@cargo run --target x86_64-pc-windows-gnu

dev-win64: # WARN: Not tested with msvc target
	@cargo run --target x86_64-pc-windows-msvc

dev-linux:
	@cargo run --features "fast_compile"

###########
## Build ##
###########

build: build-wsl-win64 build-linux

build-wsl-win64: 
	@cargo build --target x86_64-pc-windows-gnu --release

build-win64: # WARN: Not tested with msvc target
	@cargo build --target x86_64-pc-windows-gnu --release

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