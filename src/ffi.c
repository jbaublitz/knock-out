#include <linux/fs.h>
#include <linux/cdev.h>
#include <linux/kern_levels.h>
#include <linux/kernel.h>
#include <linux/module.h>
#include <linux/mutex.h>
#include <linux/uaccess.h>

DEFINE_MUTEX(parrot_mutex);

struct file_operations fops;

void set_fops_c(
	int (*open)(struct inode *, struct file *),
	ssize_t (*read)(struct file *, char *, size_t, loff_t *),
	int (*release)(struct inode *, struct file *)
) {
	fops.owner = THIS_MODULE;
	fops.open = open;
	fops.read = read;
	fops.release = release;
}

struct cdev cdev;

void cdev_init_c(void) {
	cdev_init(&cdev, &fops);
}

int cdev_add_c(int dev, int count) {
	return cdev_add(&cdev, dev, count);
}

void cdev_del_c(void) {
	cdev_del(&cdev);
}

extern unsigned long copy_to_user_c(void *to, const void *from, unsigned long count) {
    return copy_to_user(to, from, count);
};

MODULE_LICENSE("GPL v2");
