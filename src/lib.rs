#![feature(lang_items,untagged_unions,extern_types)]
#![no_std]

use core::ptr;
use core::slice;
use core::result::Result;

#[lang = "eh_personality"]
#[no_mangle]
pub extern fn eh_personality() {}
#[lang = "eh_unwind_resume"]
#[no_mangle]
pub extern fn eh_unwind_resume() {}
#[lang = "panic_fmt"]
fn panic_fmt() -> ! {
    loop { }
}

extern "C" {
    static owner: *const u8;
    static cdev_buffer: *mut u8;
    static cdev_len: u32;
    fn printk(msg: *const u8);
    fn alloc_chrdev_region(first: *const u32, first_minor: u32, count: u32, name: *const u8) -> i32;
    fn unregister_chrdev_region(first: u32, count: u32) -> i32;
	#[inline]
    fn copy_to_user_ffi(to: *mut u8, from: *const u8, count: u64) -> u64;
    fn cdev_init(cdev: *mut u8, fops: *const FileOperations);
    fn cdev_add(cdev: *mut u8, dev: u32, count: u32) -> i32;
    fn cdev_del(cdev: *mut u8);
}

#[no_mangle]
pub extern "C" fn parrot_read(_file: *mut u8, buf: *mut u8, _count: u32, _offset: *const u32) -> i32 {
    ParrotSafe::copy_to_user_ffi_safe(buf, "hello\0".as_bytes());
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

struct ParrotSafe<'a> {
    dev: u32,
    count: u32,
    fops: FileOperations,
    cdev: &'a mut [u8],
}

impl<'a> ParrotSafe<'a> {
    fn cdev() -> &'a mut [u8] {
        unsafe { slice::from_raw_parts_mut(cdev_buffer, cdev_len as usize) }
    }

    #[inline]
    fn owner() -> *const u8 {
        unsafe { owner }
    }

    #[inline]
    fn printk_safe(msg: &'static str) {
        unsafe { printk(msg.as_ptr()) }
    }

    #[inline]
    fn alloc_chrdev_region_safe(&mut self, first_minor: u32, count: u32, name: &'static str) -> i32 {
        self.count = count;
        unsafe { alloc_chrdev_region(&mut self.dev as *mut u32, first_minor, self.count, name.as_ptr()) }
    }

    #[inline]
    fn unregister_chrdev_region_safe(&mut self) -> i32 {
        unsafe { unregister_chrdev_region(self.dev, self.count) }
    }

    #[inline]
    fn copy_to_user_ffi_safe(to: *mut u8, from: &[u8]) -> u64 {
        unsafe { copy_to_user_ffi(to, from.as_ptr(), from.len() as u64) }
    }

    #[inline]
    fn cdev_init_safe(&mut self) {
        unsafe { cdev_init(self.cdev.as_mut_ptr(), &self.fops as *const FileOperations) }
    }

    #[inline]
    fn cdev_add_safe(&mut self) -> Result<(), &'static str> {
        let rc = unsafe { cdev_add(self.cdev.as_mut_ptr(), self.dev, self.count) };
        if rc == 0 {
            Ok(())
        } else {
            Err("Failed to add char dev")
        }
    }

    #[inline]
    fn cdev_del_safe(&mut self) {
        unsafe { cdev_del(self.cdev.as_mut_ptr()) }
    }

    fn new() -> Result<Self, &'static str> {
        let fops = FileOperations::new(parrot_read, parrot_open, parrot_release);
        let mut psafe = ParrotSafe { dev: 0, count: 0, fops, cdev: Self::cdev() };
        if psafe.alloc_chrdev_region_safe(0, 1, "parrot\0") != 0 {
            return Err("Failed to allocate char device region");
        }
        psafe.cdev_init_safe();
        psafe.cdev_add_safe()?;
        Ok(psafe)
    }

    fn cleanup(&mut self) -> Result<(), &'static str> {
        self.unregister_chrdev_region_safe();
        self.cdev_del_safe();
        Ok(())
    }
}

#[repr(C)]
struct FileOperations {
    owner: *const u8,
    llseek: *const u8,
    read: extern "C" fn(*mut u8, *mut u8, u32, *const u32) -> i32,
    write: *const u8,
    read_iter: *const u8,
    write_iter: *const u8,
    iterate: *const u8,
    iterate_shared: *const u8,
    poll: *const u8,
    unlocked_ioctl: *const u8,
    compat_ioctl: *const u8,
    mmap: *const u8,
    open: extern "C" fn(*mut u8, *mut u8) -> i32,
    flush: *const u8,
    release: extern "C" fn(*mut u8, *mut u8) -> i32,
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

impl FileOperations {
    fn new(read: extern "C" fn(*mut u8, *mut u8, u32, *const u32) -> i32,
           open: extern fn(*mut u8, *mut u8) -> i32,
           release: extern fn(*mut u8, *mut u8) -> i32) -> FileOperations {
        FileOperations {
            owner: ParrotSafe::owner(),
            llseek: ptr::null(),
            read,
            write: ptr::null(),
            read_iter: ptr::null(),
            write_iter: ptr::null(),
            iterate: ptr::null(),
            iterate_shared: ptr::null(),
            poll: ptr::null(),
            unlocked_ioctl: ptr::null(),
            compat_ioctl: ptr::null(),
            mmap: ptr::null(),
            open,
            flush: ptr::null(),
            release,
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
        }
    }
}

static mut GLOBAL_STATE: Option<ParrotSafe> = None;

#[no_mangle]
#[link_section = ".text"]
pub extern "C" fn init_module() -> i32 {
    let parrot_safe = match ParrotSafe::new() {
        Ok(ps) => ps,
        Err(e) => {
            ParrotSafe::printk_safe(e);
            return -1;
        }
    };
    unsafe { GLOBAL_STATE = Some(parrot_safe) };
    0
}

#[no_mangle]
#[link_section = ".text"]
pub extern "C" fn cleanup_module() {
    unsafe {
        match GLOBAL_STATE {
            Some(ref mut ps) => {
                match ps.cleanup() {
                    Ok(_) => (),
                    Err(e) => {
                        ParrotSafe::printk_safe(e);
                    }
                }
            }
            None => (),
        }
    }
}
