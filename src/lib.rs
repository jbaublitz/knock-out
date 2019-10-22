#![feature(lang_items)]
#![no_std]

use core::ffi::c_void;
use core::panic::PanicInfo;
use core::result::Result;

#[macro_use]
mod macros;
mod parrot;

use parrot::*;

#[lang = "eh_personality"]
#[no_mangle]
pub extern "C" fn eh_personality() {}
#[lang = "eh_unwind_resume"]
#[no_mangle]
pub extern "C" fn eh_unwind_resume() {}
#[panic_handler]
fn panic_handler(_info: &PanicInfo) -> ! {
    loop {}
}

extern "C" {
    fn set_fops_c(
        open: extern "C" fn(*mut c_void, *mut c_void) -> i32,
        read: extern "C" fn(*mut c_void, *mut c_void, u32, *mut c_void) -> i32,
        release: extern "C" fn(*mut c_void, *mut c_void) -> i32,
    );

    fn printk(msg: *const i8, ...);
    fn alloc_chrdev_region(first: *const u32, first_minor: u32, count: u32, name: *const u8)
        -> i32;
    fn unregister_chrdev_region(first: u32, count: u32);
    #[inline]
    fn copy_to_user_c(to: *mut c_void, from: *const u8, count: u64) -> u64;
    fn cdev_init_c() -> i32;
    fn cdev_add_c(dev: u32, count: u32) -> i32;
    fn cdev_del_c();
    fn msleep(msecs: u64);
}

const FRAMES: [&str; 10] = [
    FRAME0, FRAME1, FRAME2, FRAME3, FRAME4, FRAME5, FRAME6, FRAME7, FRAME8, FRAME9,
];
static mut FRAME_COUNTER: u8 = 0;

extern "C" fn parrot_read(
    _file: *mut c_void,
    buf: *mut c_void,
    _count: u32,
    _offset: *mut c_void,
) -> i32 {
    let frame = FRAMES.get(unsafe { FRAME_COUNTER } as usize).unwrap_or(&"");
    ParrotSafe::copy_to_user(buf, frame.as_bytes());
    unsafe {
        FRAME_COUNTER = FRAME_COUNTER.wrapping_add(1) % 10;
        // Yes, this is terrible
        msleep(50);
    }
    frame.len() as i32
}

extern "C" fn parrot_open(_inode: *mut c_void, _file: *mut c_void) -> i32 {
    0
}

extern "C" fn parrot_release(_inode: *mut c_void, _file: *mut c_void) -> i32 {
    0
}

struct ParrotSafe {
    dev: u32,
    count: u32,
}

impl ParrotSafe {
    #[inline]
    fn set_fops(
        &mut self,
        open: extern "C" fn(*mut c_void, *mut c_void) -> i32,
        read: extern "C" fn(*mut c_void, *mut c_void, u32, *mut c_void) -> i32,
        release: extern "C" fn(*mut c_void, *mut c_void) -> i32,
    ) {
        unsafe { set_fops_c(open, read, release) };
    }

    #[inline]
    fn alloc_chrdev_region_safe(
        &mut self,
        first_minor: u32,
        count: u32,
        name: &'static str,
    ) -> i32 {
        self.count = count;
        unsafe {
            alloc_chrdev_region(
                &mut self.dev as *mut u32,
                first_minor,
                self.count,
                name.as_ptr(),
            )
        }
    }

    #[inline]
    fn unregister_chrdev_region_safe(&mut self) {
        unsafe { unregister_chrdev_region(self.dev, self.count) }
    }

    #[inline]
    fn copy_to_user(to: *mut c_void, from: &[u8]) -> u64 {
        unsafe { copy_to_user_c(to, from.as_ptr(), from.len() as u64) }
    }

    #[inline]
    fn cdev_init(&mut self) {
        unsafe { cdev_init_c() };
    }

    #[inline]
    fn cdev_add(&mut self) -> Result<(), &'static str> {
        let rc = unsafe { cdev_add_c(self.dev, self.count) };
        if rc == 0 {
            Ok(())
        } else {
            Err(c_string!("Failed to add char dev"))
        }
    }

    #[inline]
    fn cdev_del(&mut self) {
        unsafe { cdev_del_c() }
    }

    fn new() -> Result<Self, &'static str> {
        let mut psafe = ParrotSafe { dev: 0, count: 0 };
        psafe.set_fops(parrot_open, parrot_read, parrot_release);
        if psafe.alloc_chrdev_region_safe(0, 1, "parrot\0") != 0 {
            return Err(c_string!("Failed to allocate char device region"));
        }
        psafe.cdev_init();
        psafe.cdev_add()?;
        Ok(psafe)
    }

    fn cleanup(&mut self) -> Result<(), &'static str> {
        self.unregister_chrdev_region_safe();
        self.cdev_del();
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
            unsafe { printk!(KERN_ERR "%s", e) };
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
            Some(ref mut ps) => match ps.cleanup() {
                Ok(_) => (),
                Err(e) => {
                    printk!(KERN_ERR "%s", to_ptr!(e));
                }
            },
            None => (),
        }
    }
}
