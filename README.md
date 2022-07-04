# knock-out
Crate with an example of a kernel module in Rust

## What does this crate actually do?
This repo is a Makefile with a Rust project that, using Kbuild, the kernel's build system, will generate
a kernel module file named `parrot.ko`.

The kernel module creates a char device that on every `read()` call, will return a new frame of an
ASCII art party parrot in your terminal, which ultimately makes an animation appear when used with `cat`.

## A lot has changed since my talk!
I gave a more in depth explanation of the process for creating this kernel module
[at DevConf.cz 2020](https://www.youtube.com/watch?v=oacmnKlWZT8&t=32s). Since then, I was contacted
by the [Rust for Linux effort](https://github.com/Rust-for-Linux/linux). As a result of the great work
they've done, I've migrated this kernel module over to use the Rust for Linux kernel API.

## Instructions for party parrot
```
# Follow https://github.com/Rust-for-Linux/linux/blob/rust/Documentation/rust/quick-start.rst

# Build and load the kernel module
make
sudo insmod parrot.ko

# Find the char device ID in the kernel
cat /proc/devices | grep parrot

# Make the device node exposing the char device
sudo mknod /dev/parrot c [ID_FROM_PREVIOUS_COMMAND] 0

# Party hard!
cat /dev/parrot
```

# Attribution for ASCII art
The frames for the parrot were shamelessly lifted from [`parrot.live`](https://github.com/hugomd/parrot.live).
Please go there if you like the ASCII art and shower the maker with praise - they're not mine.
