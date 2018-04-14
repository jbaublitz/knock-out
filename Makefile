obj-m += parrot.o
parrot-objs := build/rust.o src/ffi.o

all: build/rust.o
	make -C /lib/modules/$(shell uname -r)/build M=$(PWD) modules

clean:
	make -C /lib/modules/$(shell uname -r)/build M=$(PWD) clean

build/rust.o: src/lib.rs
	mkdir -p build
	rustc -C relocation-model=static --emit obj --crate-type=lib -o build/rust.o src/lib.rs
