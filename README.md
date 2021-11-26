# Swordfish


## Purpose

An attempt tu create a lightweight and reactive microkernel in Rust.

## build

Add the target with rustup:
```shell
rustup target add thumbv6m-none-eabi
```

Build the project:
```shell
cargo build --target thumbv6n-none-eabi
```

## Debug

To launch a debug session, first start OpenOCD server:

```shell
openocd -s share\openocd\scripts -f share\openocd\scripts\interface\stlink-v2-1.cfg -f share\openocd\scripts\target\stm32f0x.cfg
```
Here we use a stlink debugger and a stm32f0 microcontroller.

Then connect to the server with a gdb client:

```shell
arm-none-eabi-gdb -q -ex "target remote :3333" .\target\thumbv6m-none-eabi\debug\swordfish
```
