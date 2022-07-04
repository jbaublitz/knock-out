all: 
	make -C /lib/modules/$(shell uname -r)/build M=$(PWD) modules

clean:
	make -C /lib/modules/$(shell uname -r)/build M=$(PWD) clean

fmt-check:
	rustfmt --check parrot.rs frames.rs

fmt:
	rustfmt parrot.rs frames.rs
