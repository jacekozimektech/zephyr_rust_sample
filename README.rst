cbindgen --config cbindgen.toml --crate zephyr-rust-sample --output generated/zephyr_rust.h --lang c

cargo build --package zephyr-rust-sample --target thumbv7m-none-eabi --release --no-default-features

west build --board=qemu_cortex_m3 -t run

