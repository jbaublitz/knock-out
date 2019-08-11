obj-m += parrot.o
parrot-objs := rust.o src/ffi.o

all: rust.o
	make -C /lib/modules/$(shell uname -r)/build M=$(PWD) modules

clean:
	rm rust.o
	cargo clean
	make -C /lib/modules/$(shell uname -r)/build M=$(PWD) clean

rust.o: src/lib.rs
	cargo rustc --release -- -C relocation-model=static -C code-model=kernel -Z plt=y --emit=obj=rust.o
