# knock-out
Crate with an example of a kernel module in Rust

## What does this crate actually do?
This repo is a Makefile with a Rust project that, using Kbuild, the kernel's build system, will generate
a kernel module file named `parrot.ko`.

The kernel module creates a char device that on every `read()` call, will return a new frame of an
ASCII art party parrot in your terminal, which ultimately makes an animation appear when used with `cat`.

## Why would you do this?
The impetus was originally that I was talking with a colleague, [rpless](https://github.com/rpless),
about `parrot.live`. This came up during the same conversation as my desire to do something in kernel
space and my love of Rust. He suggested jokingly that I should do this. I decided to try it out.

On a more technical level, I was interested in seeing how compatible Rust actually is
on a C ABI level. I was also interested in digging into Rust internals a bit more.
Debugging some of the initial issues that came up trying to load the Rust kernel
module was pretty fun and taught me a lot about the compiler and the Linux kernel.

## What are the practical applications of this?
There are a few interesting applications that can be derived from this project.

1. How to integrate with Kbuild, the kernel build system
2. How to link the core Rust library into a kernel module (see the Makefile)
3. Design patterns to reduce the amount of `unsafe` code in a kernel module
  * One example is providing a wrapper for the kernel's mutex interface
    for accessing global state safely
  * Another example is wrapping certain FFI calls in type safe Rust functions
    to remove the need for passing in a calculated length parameter in Rust
4. Above all, I learned that using C where C is needed should not be avoided
  * There are circumstances where it makes sense to provide an interface
    that passes `void` pointers to C and uses C to carry out the operations
    natively
  * There are other circumstances where the developer can lift some of those
    operations into Rust
  * Typically this depends heavily on whether macros or C structs are involved
    (as those often are littered with `#ifdef`s in the kernel)
    * If there are C macros, you're most likely better off doing it in C

## Watch my talk!
I gave a more in depth explanation of the process of creating this kernel module
[at DevConf.cz 2020](https://www.youtube.com/watch?v=oacmnKlWZT8&t=32s).

## Instructions for party parrot
```
# Make sure Rust is installed via rustup
# See https://rustup.rs/ for instructions

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
