use core::sync::atomic::{AtomicUsize, Ordering};
use kernel::{
    bindings::msleep,
    chrdev::Registration,
    file::{File, Operations, ToUse},
    io_buffer::IoBufferWriter,
    prelude::*,
};

mod frames;
use frames::*;

module! {
    type: ParrotSafe,
    name: b"party_parrot",
    author: b"John Baublitz (john.m.baublitz@gmail.com)",
    description: b"Get the party started",
    license: b"GPL",
}

const FRAMES: [&str; 10] = [
    FRAME0, FRAME1, FRAME2, FRAME3, FRAME4, FRAME5, FRAME6, FRAME7, FRAME8, FRAME9,
];

struct ParrotSafe(Pin<Box<Registration<1>>>);

impl kernel::Module for ParrotSafe {
    fn init(name: &'static CStr, module: &'static ThisModule) -> Result<Self> {
        let mut registration = Registration::new_pinned(name, 0, module)?;
        registration.as_mut().register::<ParrotOps>()?;
        Ok(ParrotSafe(registration))
    }
}

struct ParrotOps(usize);

impl<'a> Operations for ParrotOps {
    const TO_USE: ToUse = ToUse {
        read: true,
        read_iter: false,
        write: false,
        write_iter: false,
        compat_ioctl: false,
        fsync: false,
        ioctl: false,
        mmap: false,
        poll: false,
        seek: false,
    };

    type Data = Box<AtomicUsize>;

    fn open(_: &Self::OpenData, _: &File) -> Result<Self::Data> {
        Ok(Box::try_new(AtomicUsize::new(0))?)
    }

    fn read(data: &AtomicUsize, _: &File, buf: &mut impl IoBufferWriter, _: u64) -> Result<usize> {
        let frame = FRAMES
            .get(
                match data.fetch_update(Ordering::Relaxed, Ordering::Relaxed, |i| {
                    Some((i + 1) % FRAMES.len())
                }) {
                    Ok(i) => i,
                    Err(i) => i,
                },
            )
            .unwrap_or(&"");
        buf.write_slice(&frame.as_bytes())?;
        // Yes, this is terrible
        unsafe { msleep(50) };
        Ok(frame.len())
    }
}
