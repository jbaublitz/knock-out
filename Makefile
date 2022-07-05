all: 
	make -C /lib/modules/$(shell uname -r)/build LLVM=1 M=$(PWD) modules

clean:
	make -C /lib/modules/$(shell uname -r)/build M=$(PWD) clean

rustfmtcheck:
	rustfmt --check parrot.rs frames.rs

rustfmt:
	rustfmt parrot.rs frames.rs
