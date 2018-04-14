#![feature(lang_items)]
#![no_std]

use core::ptr;
use core::mem::size_of;

#[lang = "eh_personality"] extern fn eh_personality() {}
#[lang = "panic_fmt"] fn panic_fmt() -> ! { loop {} }

extern "C" {
    static owner: *const u8;
    static cdev_len: usize;
    static kobject_len: usize;
    fn printk(msg: *const u8);
    fn alloc_chrdev_region(first: *const u32, first_minor: u32, count: u32, name: *const u8) -> i32;
    fn unregister_chrdev_region(first: u32, count: u32) -> i32;
    fn copy_to_user_ffi(to: *mut u8, from: *const u8, count: u64) -> u64;
}

pub static mut DEV: u32 = 0;

#[repr(C)]
pub struct FileOperations {
    owner: *const u8,
    llseek: *const u8,
    read: extern fn(*mut u8, *mut u8, u32, *const u32) -> i32,
    write: *const u8,
    read_iter: *const u8,
    write_iter: *const u8,
    iterate: *const u8,
    iterate_shared: *const u8,
    poll: *const u8,
    unlocked_ioctl: *const u8,
    compat_ioctl: *const u8,
    mmap: *const u8,
    open: extern fn(*mut u8, *mut u8) -> i32,
    flush: *const u8,
    release: extern fn(*mut u8, *mut u8) -> i32,
    fsync: *const u8,
    fasync: *const u8,
    lock: *const u8,
    sendpage: *const u8,
    get_unmapped_area: *const u8,
    check_flags: *const u8,
    flock: *const u8,
    splice_write: *const u8,
    splice_read: *const u8,
    setlease: *const u8,
    fallocate: *const u8,
    show_fdinfo: *const u8,
    mmap_capabilities: *const u8,
    copy_file_range: *const u8,
    clone_file_range: *const u8,
    dedupe_file_range: *const u8,
}

#[repr(C)]
pub struct CDev<'a> {
    buf: &'a mut [u8]
}

impl<'a> CDev<'a> {
    pub fn new(buf: &'a mut [u8]) -> Option<Self> {
		if buf.len() < unsafe { cdev_len } {
			None
		} else {
			Some(CDev { buf })
		}
    }

    pub fn set_fops(&mut self, fops_ptr: *const FileOperations) {
        let fops = unsafe { self.buf.as_mut_ptr().add(kobject_len + size_of::<*const u8>())
            as *mut *const FileOperations };
		unsafe {
			*fops = fops_ptr;
		}
    }
}

#[no_mangle]
pub extern "C" fn parrot_read(_file: *mut u8, buf: *mut u8, _count: u32, _offset: *const u32) -> i32 {
    unsafe { copy_to_user_ffi(buf, "hello\0".as_ptr(), 6) };
    0
}

#[no_mangle]
pub extern "C" fn parrot_open(_inode: *mut u8, _file: *mut u8) -> i32 {
    0
}

#[no_mangle]
pub extern "C" fn parrot_release(_inode: *mut u8, _file: *mut u8) -> i32 {
    0
}

#[no_mangle]
#[link_section = ".text"]
pub extern "C" fn init_module() -> i32 {
    if unsafe { alloc_chrdev_region(&mut DEV as *mut u32, 0, 1, "parrot\0".as_ptr()) } != 0 {
        unsafe { printk("Failed to allocate char device".as_ptr()) };
        return -1;
    }

    let file_operations = FileOperations {
        owner: unsafe { owner },
        llseek: ptr::null(),
        read: parrot_read,
        write: ptr::null(),
        read_iter: ptr::null(),
        write_iter: ptr::null(),
        iterate: ptr::null(),
        iterate_shared: ptr::null(),
        poll: ptr::null(),
        unlocked_ioctl: ptr::null(),
        compat_ioctl: ptr::null(),
        mmap: ptr::null(),
        open: parrot_open,
        flush: ptr::null(),
        release: parrot_release,
        fsync: ptr::null(),
        fasync: ptr::null(),
        lock: ptr::null(),
        sendpage: ptr::null(),
        get_unmapped_area: ptr::null(),
        check_flags: ptr::null(),
        flock: ptr::null(),
        splice_write: ptr::null(),
        splice_read: ptr::null(),
        setlease: ptr::null(),
        fallocate: ptr::null(),
        show_fdinfo: ptr::null(),
        mmap_capabilities: ptr::null(),
        copy_file_range: ptr::null(),
        clone_file_range: ptr::null(),
        dedupe_file_range: ptr::null(),
    };

    0
}

#[no_mangle]
#[link_section = ".text"]
pub extern "C" fn cleanup_module() {
    unsafe { unregister_chrdev_region(DEV, 0) };
}
