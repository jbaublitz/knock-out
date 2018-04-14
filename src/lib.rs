extern crate core;

use core::ptr;

extern "C" {
    pub static owner: *const u8;
    fn printk(msg: *const u8);
    fn alloc_chrdev_region(first: *const u64, first_minor: u32, count: u32, name: *const u8) -> i32;
    fn unregister_chrdev_region(first: u64, count: u32) -> i32;
    fn copy_to_user_ffi(to: *mut u8, from: *const u8, count: u64) -> u64;
}

pub static DEV: u64 = 0;

#[repr(C)]
pub struct FileOperations {
    owner: *const u8,
    llseek: *const u8,
    read: extern fn(*mut u8, *mut u8, u32, *const u32) -> i32,
    write: *const u8,
    ioctl: *const u8,
    open: extern fn(*mut u8, *mut u8) -> i32,
    release: extern fn(*mut u8, *mut u8) -> i32,
}

#[no_mangle]
pub extern fn parrot_read(_file: *mut u8, buf: *mut u8, _count: u32, _offset: *const u32) -> i32 {
    unsafe { copy_to_user_ffi(buf, "hello\0".as_ptr(), 6) };
    0
}

#[no_mangle]
pub extern fn parrot_open(_inode: *mut u8, _file: *mut u8) -> i32 {
    0
}

#[no_mangle]
pub extern fn parrot_release(_inode: *mut u8, _file: *mut u8) -> i32 {
    0
}

#[no_mangle]
#[link_section = ".text"]
pub extern fn init_module() -> i32 {
    if unsafe { alloc_chrdev_region(&DEV as *const u64, 0, 1, "parrot".as_ptr()) } != 0 {
        unsafe { printk("Failed to allocate char device".as_ptr()) };
        return -1;
    }

    let file_operations = FileOperations {
        owner: unsafe { owner },
        llseek: ptr::null(),
        read: parrot_read,
        write: ptr::null(),
        ioctl: ptr::null(),
        open: parrot_open,
        release: parrot_release,
    };

    0
}

#[no_mangle]
#[link_section = ".text"]
pub extern fn cleanup_module() {
    unsafe { unregister_chrdev_region(DEV, 0) };
}
