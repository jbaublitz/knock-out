#![feature(lang_items,untagged_unions,extern_types)]
#![no_std]

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
    static cdev_len: u32;
    static cdev_ptr: *mut u8;
    static fops_len: u32;
    static fops_ptr: *mut u8;
    static parrot_owner_ptr: *mut *const u8;
    static parrot_read_ptr: *mut extern fn(*mut u8, *mut u8, u32, *const u32) -> i32;
    static parrot_open_ptr: *mut extern fn(*mut u8, *mut u8) -> i32;
    static parrot_release_ptr: *mut extern fn(*mut u8, *mut u8) -> i32;
    fn printk(msg: *const u8);
    fn alloc_chrdev_region(first: *const u32, first_minor: u32, count: u32, name: *const u8) -> i32;
    fn unregister_chrdev_region(first: u32, count: u32) -> i32;
	#[inline]
    fn copy_to_user_ffi(to: *mut u8, from: *const u8, count: u64) -> u64;
    fn cdev_init(cdev: *mut u8, fops: *const u8);
    fn cdev_add(cdev: *mut u8, dev: u32, count: u32) -> i32;
    fn cdev_del(cdev: *mut u8);
}

#[no_mangle]
pub extern "C" fn parrot_read(_file: *mut u8, buf: *mut u8, _count: u32, _offset: *const u32) -> i32 {
    ParrotSafe::copy_to_user_ffi_safe(buf, "hello\0".as_bytes());
    6
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
    cdev: &'a mut [u8],
    fops: &'a mut [u8],
}

impl<'a> ParrotSafe<'a> {
    #[inline]
    fn owner() -> *const u8 {
        unsafe { owner }
    }

    fn cdev() -> &'a mut [u8] {
        unsafe { slice::from_raw_parts_mut(cdev_ptr, cdev_len as usize) }
    }

    #[inline]
    fn fops() -> &'a mut [u8] {
        unsafe { slice::from_raw_parts_mut(fops_ptr, fops_len as usize) }
    }

    #[inline]
    fn printk_safe(msg: &str) {
        unsafe { printk(msg.as_ptr()) }
    }

    #[inline]
    fn set_fops_safe(read: extern "C" fn(*mut u8, *mut u8, u32, *const u32) -> i32,
                open: extern "C" fn(*mut u8, *mut u8) -> i32,
                release: extern "C" fn(*mut u8, *mut u8) -> i32) {
        unsafe {
            *parrot_owner_ptr = Self::owner();
            *parrot_read_ptr = read;
            *parrot_open_ptr = open;
            *parrot_release_ptr = release;
        }
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
        unsafe { cdev_init(self.cdev.as_mut_ptr(), self.fops.as_ptr()) }
    }

    #[inline]
    fn cdev_add_safe(&mut self) -> Result<(), &'static str> {
        let rc = unsafe { cdev_add(self.cdev.as_mut_ptr(), self.dev, self.count) };
        if rc == 0 {
            Ok(())
        } else {
            Err("Failed to add char dev\0")
        }
    }

    #[inline]
    fn cdev_del_safe(&mut self) {
        unsafe { cdev_del(self.cdev.as_mut_ptr()) }
    }

    fn new() -> Result<Self, &'static str> {
        let mut psafe = ParrotSafe { dev: 0, count: 0, fops: Self::fops(), cdev: Self::cdev() };
        Self::set_fops_safe(parrot_read, parrot_open, parrot_release);
        if psafe.alloc_chrdev_region_safe(0, 1, "parrot\0") != 0 {
            return Err("Failed to allocate char device region\0");
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
