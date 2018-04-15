#include <linux/cdev.h>
#include <linux/kernel.h>
#include <linux/kobject.h>
#include <linux/module.h>
#include <linux/uaccess.h>

struct module *owner = THIS_MODULE;
unsigned int cdev_len = sizeof(struct cdev);
struct cdev cdev_buffer;

extern unsigned long copy_to_user_ffi(void *to, const void *from, unsigned long count) {
    return copy_to_user(to, from, count);
};
