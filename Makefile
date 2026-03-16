.phony: all build build-win64 build-linux dev dev-wsl dev-win64 dev-linux setup-steamworks clean help

PLATFORM ?= linux

all: dev


#################
## Development ##
#################

dev: dev-$(PLATFORM)

dev-wsl:
	@cargo run --target x86_64-pc-windows-gnu --features "fast_compile"

dev-win64: # WARN: Not tested with msvc target # TODO: Check out https://github.com/rust-cross/cargo-xwin
	@cargo run --target x86_64-pc-windows-msvc --features "fast_compile"

dev-linux:
	@cargo run --features "fast_compile"


###########
## Build ##
###########

build: build-$(PLATFORM)

build-wsl: 
	@cargo build --release --target x86_64-pc-windows-gnu

build-win64: # WARN: Not tested with msvc target
	@cargo build --release --target x86_64-pc-windows-msvc

build-linux:
	@cargo build --release
	

###########
## Setup ##
###########

setup-steamworks:
	@ln -s ~/snap/steam/common/.steam ~/.steam

fonts:
	$(MAKE) -C assets/fonts 
	

#############
## Utility ##
#############

clean:
	@cargo clean

help:
	@echo "dev		- 	build and run development build"
	@echo "build	- 	build production build"