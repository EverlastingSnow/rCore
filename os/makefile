# Makefile for building and running a RISC-V project with Cargo, QEMU, and GDB.

# Target binary name
TARGET = target/riscv64gc-unknown-none-elf/release/os
BIN = target/riscv64gc-unknown-none-elf/release/os.bin

# Paths for QEMU and BIOS
BIOS_PATH = ../bootloader/rustsbi-qemu.bin

# Compiler and GDB for RISC-V
GDB = riscv64-unknown-elf-gdb

# Default target: build the project and run it in QEMU
.PHONY: all
all: build run

# Step 1: Build the project in release mode
.PHONY: build
build:
	cargo build --release

# Step 2: Strip the binary and convert to raw binary format
.PHONY: strip
strip: build
	rust-objcopy --strip-all $(TARGET) -O binary $(BIN)

# Step 3: Run the project in QEMU
.PHONY: run
run: strip
	qemu-system-riscv64 \
		-machine virt \
		-nographic \
		-bios $(BIOS_PATH) \
		-device loader,file=$(BIN),addr=0x80200000

# Step 4: Run the project in QEMU with debugging enabled
.PHONY: run-debug
run-debug: strip
	qemu-system-riscv64 \
		-machine virt \
		-nographic \
		-bios $(BIOS_PATH) \
		-device loader,file=$(BIN),addr=0x80200000 \
		-s -S

# Step 5: Start GDB and connect to QEMU
.PHONY: gdb
gdb:
	$(GDB) \
		-ex 'file $(TARGET)' \
		-ex 'set arch riscv:rv64' \
		-ex 'target remote localhost:1234'

# Clean build artifacts
.PHONY: clean
clean:
	cargo clean