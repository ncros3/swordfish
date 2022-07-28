# Swordfish


## Purpose

An attempt tu create a lightweight and reactive microkernel in Rust.

## build

Add the target with rustup:
```shell
rustup target add thumbv7em-none-eabi
```

Build the project:
```shell
cargo build --target thumbv7em-none-eabi
```

## Debug

To launch a debug session, first start OpenOCD server:

```shell
openocd -s /usr/share/openocd/scripts/ -f /usr/share/openocd/scripts/interface/stlink-v2-1.cfg -f /usr/share/openocd/scripts/target/stm32h7x.cfg
```
Here we use a stlink debugger and a stm32f0 microcontroller.

Then connect to the server with a gdb client:

```shell
arm-none-eabi-gdb -q -ex "target remote :3333" .\target\thumbv7em-none-eabi\debug\swordfish
```

Then connect to 

```shell
arm-none-eabi-gdb -q -ex "target remote :3333" .\target\thumbv7em-none-eabi\debug\swordfish
```