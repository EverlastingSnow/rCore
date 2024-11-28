# rCore

如果要输出调试信息 export DEBUG即可，查看logging.rs了解详细信息

cd os/
make run即可运行，详见makefile
cargo 设置见cargo.toml

build项目
cargo build --release

$ rust-objcopy --strip-all target/riscv64gc-unknown-none-elf/release/os -O binary target/riscv64gc-unknown-none-elf/release/os.bin
去掉多余元数据，不然会寄

$ qemu-system-riscv64 \
    -machine virt \
    -nographic \
    -bios ../bootloader/rustsbi-qemu.bin \
    -device loader,file=target/riscv64gc-unknown-none-elf/release/os.bin,addr=0x80200000 \
    -s -S
去掉-s -S以关闭gdb调试

riscv64-unknown-elf-gdb \
    -ex 'file target/riscv64gc-unknown-none-elf/release/os' \
    -ex 'set arch riscv:rv64' \
    -ex 'target remote localhost:1234'

riscv64-unknown-elf-gdb\
    -ex 'file 00hello_world'\
    -ex 'set arch riscv:rv64'\
    -ex 'target remote localhost:1234'