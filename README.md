# Rustberry OS
This is a WIP kernel for RaspberryPi 3 written in Rust.

## Requirements
You will nedd a nightly Rust installation. The fastest way is to use [rustup](https://rustup.rs/).
You will also need some tools, you can install the whole toolchain with:
```shell
$ curl https://sh.rustup.rs -sSf | sh -s -- --default-toolchain nightly
$ rustup component add rust-src llvm-tools
$ cargo install cargo-xbuild cargo-binutils
```

## Building
* To build a debug kernel: `DEBUG=1 make`
* To build a release kernel: `make`

## Running
### QEMU
To run the kernel in QEMU you can either run (you will need QEMU):
* `make run`
* `qemu-system-aarch64 -kernel build/kernel8.img -M raspi3 -serial null -serial mon:stdio`

## Testing (WIP)
`make test`
