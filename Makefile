obj-m += parrot.o
parrot-objs := src/rust.o src/ffi.o

all: src/rust.o
	make -C /lib/modules/$(shell uname -r)/build M=$(PWD) modules

clean:
	cargo clean
	make -C /lib/modules/$(shell uname -r)/build M=$(PWD) clean

src/rust.o: src/lib.rs
	cargo rustc --release -- --crate-type=staticlib -C relocation-model=static -C code-model=kernel -Z plt=y --emit obj=src/rust.o
