#include <linux/module.h>
#include <linux/kernel.h>
#include <linux/uaccess.h>

struct module *owner = THIS_MODULE;

extern unsigned long copy_to_user_ffi(void *to, const void *from, unsigned long count) {
    return copy_to_user(to, from, count);
};
