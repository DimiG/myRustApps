# This is a Makefile to compile the application
# Just help to Automate the compilation process
# With `make', `make win', `make mac' and `make clean'
# Created by: Dmitri G.
# Date: 2019-03-22

### VARIABLEs ###
TARGET = target/release/rust_cli
TARGET_W = $(TARGET).exe

ifeq ($(OS),Windows_NT)
  DETECTED_OS = Windows
else
  DETECTED_OS = $(shell sh -c 'uname 2>/dev/null || echo Unknown')
endif

ifeq ($(DETECTED_OS),Windows)
  COMPRESSOR = upx.exe
else
  COMPRESSOR = upx
endif

.PHONY: all help win mac clean

all:
	$(info =>>> Compilation in progress...)
	cargo run

help:
	@echo "[HELP COMMANDS]"
	@echo " make       : Run a Debug build"
	@echo " make win   : Run a Release build for Windows platform"
	@echo " make mac   : Run a Release build for MacOS   platform"
	@echo " make clean : Clean all build outputs"

win: clean
	$(info =>>> Optimized for Windows platform)
	cargo build --release
	$(COMPRESSOR) --brute $(TARGET_W)

mac: clean
	$(info =>>> Optimized for MacOS platform)
	cargo build --release --target=x86_64-apple-darwin

clean:
	$(info =>>> Cleaning UP...)
	cargo clean

