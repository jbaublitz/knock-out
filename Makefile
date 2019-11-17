obj-m := parrot.o
parrot-objs := src/ffi.o src/rust.o

all: src/rust.o
	make -C /lib/modules/$(shell uname -r)/build M=$(PWD) modules

.PHONY:
fmt:
	xargo fmt

.PHONY:
clippy:
	xargo clippy

clean:
	cargo clean
	rm -rf rust_objs
	make -C /lib/modules/$(shell uname -r)/build M=$(PWD) clean

src/rust.o: Cargo.toml src/ffi.c src/lib.rs src/parrot.rs
	xargo rustc --release --target=x86_64-linux-kernel
	mkdir -p rust_objs
	cd rust_objs && ar x ../target/x86_64-linux-kernel/release/librust.a
	ld -r -o src/rust.o rust_objs/*.o
