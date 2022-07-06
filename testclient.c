#include <stdio.h>
#include <unistd.h>
#include <fcntl.h>
#include <string.h>

int main() {
	int fd = open("/dev/parrot", O_RDONLY);
	if (fd < 0) {
		printf("%s\n", strerror(-fd));
		return -fd;
	}

	char buf[2];
	while (1) {
		// Buffer needs to be manually null terminated.
		int r = read(fd, buf, 1);
		if (r < 0) {
			printf("%s\n", strerror(-r));
			return -r;
		} else {
			buf[r] = 0;
		}
		printf("%s", buf);
	}
}
