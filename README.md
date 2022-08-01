# Swordfish


## Purpose

An attempt tu create a lightweight and reactive microkernel in Rust.

## Target

Here we are using a stm32wb55 which is using a cortex-m4 as main processor and a cortex-m0 as co-processor.

## Build

The cortex-m7 includes the armv7-m instruction set without a hardware floating point unit. So we need to add the corresponding toolchain to our project :

```shell
rustup target add thumbv7em-none-eabi
```

And we can build the project:
```shell
cargo build --target thumbv7em-none-eabi
```

We can also add this configuration in a **./cargo/config.toml** file:
```toml
target = "thumbv7em-none-eabi" # Cortex-M4F and Cortex-M7F (with FPU)
```

we also need to set the linker configuration for this target:
```toml
[target.thumbv7em-none-eabi]
rustflags = [
  "-C", 
  "link-arg=-Tlink.x",
]
```

## Debug

To launch a debug session, first start OpenOCD server:

```shell
openocd -s /usr/share/openocd/scripts -f /usr/share/openocd/scripts/interface/stlink-v2-1.cfg -f /usr/share/openocd/scripts/target/stm32wbx.cfg
```

Then connect to the server with a gdb client:

```shell
gdb-multiarch -q target/thumbv7em-none-eabi/debug/swordfish
```

In gdb, type:

```shell
(gdb) target remote :3333
```

> Remote debugging using :3333
> 0x0800124e in ?? ()

```shell
(gdb) load
```

> Loading section .vector_table, size 0x400 lma 0x8000000
> Loading section .text, size 0x324 lma 0x8000400
> Loading section .rodata, size 0x228 lma 0x8000724
> Start address 0x08000400, load size 2380
> Transfer rate: 8 KB/sec, 793 bytes/write.