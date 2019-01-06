# knock-out
Crate for wrapping Rust code as a kernel module

# Attribution
The frames for the parrot were shamelessly lifted from parrot.live. Please go there if you like
the ASCII art and shower the maker with praise - they're not mine.

# Update
This crate not longer works on Ubuntu. There seems to be a change in the way LLVM handles
ELF relocations for `extern "C"` functions which triggers a failure when loading this module
in the Linux kernel.

## Specifics
[This](https://github.com/torvalds/linux/blob/master/arch/x86/kernel/module.c#L211) seems to
be the line that's getting hit.

The unrecognized relocation symbol is `R_X86_64_GOTPCREL` (or 9 as the kernel reports it). This
is defined in `/usr/include/llvm-6.0/llvm/BinaryFormat/ElfRelocs/x86_64.def`.

# Result
As there aren't really any practical applications for maintaining this, I will leave it available but
will probably not be maintaining it anymore.
