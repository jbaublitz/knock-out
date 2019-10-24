# knock-out
Crate with an example of a kernel module in Rust

## What does this crate actually do
This repo is a Makefile with a Rust project that, using Kbuild, the kernel's build system, will generate
a kernel module file named `parrot.ko`.

The kernel module creates a char device that on every `read()` call, will return a new frame of an
ASCII art party parrot in your terminal, which ultimately makes an animation appear when used with `cat`.

## Why on earth would you do this?!
Well... the longer story is that I was talking with a colleague, [rpless](https://github.com/rpless),
about `parrot.live`. This came up
during the same conversation as my desire to do something in kernel space. This also came up
in the same conversation as my love of Rust. He suggested jokingly I should do exactly what I did in this 
project. So I did!

The shorter answer is "for science!"

A more serious answer is that I was interested in seeing how compatible Rust actually is
on a C ABI level. I was also interested in digging into Rust internals a bit more.
Debugging some of the initial issues that came up trying to load the Rust kernel
module was pretty fun and taught me a lot about the compiler.

## Instructions for party parrot
```
# Make sure Rust is installed via rustup
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# This project requires a nightly Rust build
rustup default nightly

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
I'm a lot of things but ASCII artist is not one of them.
