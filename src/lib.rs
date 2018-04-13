extern {
    fn printk(msg: *const u8);
    fn alloc_chrdev_region(first: *const u64, first_minor: u32, count: u32, name: *const u8) -> i32;
    fn unregister_chrdev_region(first: u64, count: u32) -> i32;
}

pub static DEV: u64 = 0;

#[no_mangle]
#[link_section = ".text"]
pub extern fn init_module() -> i32 {
    if unsafe { alloc_chrdev_region(&DEV as *const u64, 0, 1, "parrot".as_ptr()) } != 0 {
        unsafe { printk("Failed to allocate char device".as_ptr()) };
        return -1;
    }

    0
}

#[no_mangle]
#[link_section = ".text"]
pub extern fn cleanup_module() {
    unsafe { unregister_chrdev_region(DEV, 0) };
}
