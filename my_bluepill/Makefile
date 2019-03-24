# This is a Makefile to compile the application
# Just help to Automate the compilation process
# With `make',`make run',`make release'
# Also `make check', `make doc' and `make clean'
# Created by: Dmitri G.
# Date: 2019-03-22

### VARIABLEs ###
PROG_NAME = my_bluepill
PROG_PATH = target/thumbv7m-none-eabi/release
BINS_PATH = target

.PHONY: all help debug release run run_release check doc telnet clean

all: debug

help:
	@echo "[HELP COMMANDS]"
	@echo " make debug  : Run a Debug build"
	@echo " make release: Run a Release build"
	@echo " make clean  : Clean all build outputs"

debug: build/Debug
	@echo "[DEBUG]"

release: build/Release
	@echo "[RELEASE]"

run:
	$(info =>>> Debug RUNNING in progress...)
	cargo run
	@echo "[RUN]"

run_release:
	$(info =>>> Release RUNNING in progress...)
	cargo run --release
	@echo "[RUN_RELEASE]"

check:
	$(info =>>> Checking the binary SIZE, HEADER and CODE...)
	cargo size --bin $(PROG_NAME) --release -- -A
	cargo readobj --bin $(PROG_NAME) --release -- -file-headers
	cargo objdump --bin $(PROG_NAME) --release -- -disassemble -no-show-raw-insn -print-imm-hex
	@echo "[CHECK]"

build/Debug:
	$(info =>>> Debug compilation in progress...)
	$(info =>>> Checking the ELF binary file SIZE)
	cargo build
	cargo size --bin $(PROG_NAME) -- -A

build/Release:
	$(info =>>> Release making in progress...)
	$(info =>>> Making BIN and HEX files...)
	$(info =>>> Checking the ELF BINARY file SIZE)
	cargo build --release
	cargo objcopy --bin $(PROG_NAME) --release -- -O binary  $(BINS_PATH)/$(PROG_NAME).bin
	arm-none-eabi-objcopy -O ihex $(PROG_PATH)/$(PROG_NAME) $(BINS_PATH)/$(PROG_NAME).hex
	cargo size --bin $(PROG_NAME) --release -- -A

doc:
	$(info =>>> Generating documentation...)
	cargo doc -p stm32f1 --open
	@echo "[DOCS]"

telnet:
	$(info =>>> Telneting to OpenOCD...)
	$(info =>>> Load Firmware by: > reset halt)
	$(info =>>> > flash write_image erase target/thumbv7m-none-eabi/release/my_bluepill)
	$(info =>>> > reset)
	telnet 127.0.0.1 4444
	@echo "[TELNET]"

clean:
	$(info =>>> Cleaning UP...)
	cargo clean -v
	@echo "[CLEAN]"
