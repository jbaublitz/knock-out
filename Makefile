all: 
	make -C /lib/modules/$(shell uname -r)/build LLVM=1 M=$(PWD) modules

clean:
	make -C /lib/modules/$(shell uname -r)/build M=$(PWD) clean
	rm -f testclient

rustfmtcheck:
	rustfmt --check parrot.rs frames.rs

rustfmt:
	rustfmt parrot.rs frames.rs

testclient-rs: testclient.rs
	rustc testclient.rs

testclient-c: testclient.o
	gcc -o testclient testclient.o
