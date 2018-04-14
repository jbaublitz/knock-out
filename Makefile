obj-m += parrot.o
parrot-objs := src/rust.o src/ffi.o

all: src/rust.o
	make -C /lib/modules/$(shell uname -r)/build M=$(PWD) modules

clean:
	make -C /lib/modules/$(shell uname -r)/build M=$(PWD) clean

src/rust.o: src/lib.rs
	rustc -C relocation-model=static --emit obj --crate-type=lib -o src/rust.o src/lib.rs
