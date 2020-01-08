#![feature(lang_items)]
#![no_std]

#[macro_use]
extern crate lazy_static;

use core::ffi::c_void;
use core::panic::PanicInfo;
use core::result::Result;

#[macro_use]
mod macros;
mod mutex;
mod parrot;

use mutex::Mutex;
use parrot::*;

#[lang = "eh_personality"]
#[no_mangle]
pub extern "C" fn eh_personality() {}
#[lang = "eh_unwind_resume"]
#[no_mangle]
pub extern "C" fn eh_unwind_resume() {}
#[panic_handler]
fn panic_handler(_info: &PanicInfo) -> ! {
    unsafe { panic(to_ptr!(c_string!("Rust panic was triggered"))) }
}

extern "C" {
    fn set_fops_c(
        open: extern "C" fn(*mut c_void, *mut c_void) -> i32,
        read: extern "C" fn(*mut c_void, *mut c_void, u32, *mut c_void) -> i32,
        release: extern "C" fn(*mut c_void, *mut c_void) -> i32,
    );

    fn panic(msg: *const i8, ...) -> !;
    fn printk(msg: *const i8, ...);
    fn alloc_chrdev_region_c(first_minor: u32, count: u32, name: *const u8) -> i32;
    fn get_chrdev_major_c() -> u32;
    fn get_chrdev_minor_c() -> u32;
    fn unregister_chrdev_region_c(count: u32);
    fn copy_to_user_c(to: *mut c_void, from: *const u8, count: u64) -> u64;
    fn cdev_init_c() -> i32;
    fn cdev_add_c(count: u32) -> i32;
    fn cdev_del_c();
    fn init_counter_c(p: *mut c_void);
    fn get_counter_c(p: *mut c_void) -> usize;
    fn increment_counter_c(p: *mut c_void);
    fn msleep(msecs: u64);
}

type KernelResult<T> = Result<T, &'static str>;

const FRAMES: [&str; 10] = [
    FRAME0, FRAME1, FRAME2, FRAME3, FRAME4, FRAME5, FRAME6, FRAME7, FRAME8, FRAME9,
];

extern "C" fn parrot_open(_inode: *mut c_void, file: *mut c_void) -> i32 {
    unsafe { init_counter_c(file) };
    0
}

extern "C" fn parrot_read(
    file: *mut c_void,
    buf: *mut c_void,
    _count: u32,
    _offset: *mut c_void,
) -> i32 {
    let frame = FRAMES.get(unsafe { get_counter_c(file) }).unwrap_or(&"");
    ParrotSafe::copy_to_user(buf, frame.as_bytes());
    unsafe {
        increment_counter_c(file);
        // Yes, this is terrible
        msleep(50);
    }
    frame.len() as i32
}

extern "C" fn parrot_release(_inode: *mut c_void, _file: *mut c_void) -> i32 {
    0
}

pub struct ParrotSafe(u32);

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
    fn alloc_chrdev_region(
        &mut self,
        first_minor: u32,
        count: u32,
        name: &'static str,
    ) -> KernelResult<()> {
        self.0 = count;
        let (major, minor) = unsafe {
            if alloc_chrdev_region_c(first_minor, self.0, name.as_ptr()) != 0 {
                return Err("Failed to allocate character device region");
            }
            (get_chrdev_major_c(), get_chrdev_minor_c())
        };
        unsafe {
            printk!("Major: %d", major);
            printk!("Minor: %d", minor);
        };
        Ok(())
    }

    #[inline]
    fn unregister_chrdev_region(&mut self) {
        unsafe { unregister_chrdev_region_c(self.0) }
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
    fn cdev_add(&mut self) -> KernelResult<()> {
        let rc = unsafe { cdev_add_c(self.0) };
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

    fn init(&mut self) -> KernelResult<()> {
        self.set_fops(parrot_open, parrot_read, parrot_release);
        self.alloc_chrdev_region(0, 1, c_string!("parrot"))?;
        self.cdev_init();
        self.cdev_add()?;
        Ok(())
    }

    fn cleanup(&mut self) -> KernelResult<()> {
        self.unregister_chrdev_region();
        self.cdev_del();
        Ok(())
    }
}

lazy_static! {
    static ref MUTEX: Mutex<ParrotSafe> = Mutex::init(ParrotSafe(0));
}

#[no_mangle]
pub extern "C" fn init_module() -> i32 {
    let mut mutex_guard = MUTEX.acquire();
    let parrot_ref = match mutex_guard.get_mut() {
        Some(p) => p,
        None => {
            unsafe {
                printk!("%s", to_ptr!(c_string!("Failed to get reference to global state")))
            };
            return -1;
        }
    };
    match parrot_ref.init() {
        Ok(_) => 0,
        Err(e) => {
            unsafe { printk!("%s", to_ptr!(e)) };
            -1
        }
    }
}

#[no_mangle]
pub extern "C" fn cleanup_module() {
    let mut mutex_guard = MUTEX.acquire();
    let parrot_ref = match mutex_guard.get_mut() {
        Some(p) => p,
        None => {
            unsafe {
                printk!("%s", to_ptr!(c_string!("Failed to get reference to global state")))
            };
            return;
        }
    };
    match parrot_ref.cleanup() {
        Ok(_) => (),
        Err(e) => {
            unsafe { printk!("%s", to_ptr!(e)) };
        }
    }
}
