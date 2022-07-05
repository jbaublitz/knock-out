use core::sync::atomic::{AtomicUsize, Ordering};
use kernel::{
    bindings::msleep,
    chrdev::Registration,
    declare_file_operations,
    file::{File, Operations},
    io_buffer::IoBufferWriter,
    prelude::*,
};

mod frames;
use frames::FRAMES;

module! {
    type: ParrotSafe,
    name: b"party_parrot",
    author: b"John Baublitz <john.m.baublitz@gmail.com>",
    description: b"Get the party started",
    license: b"GPL",
}

struct ParrotSafe(Pin<Box<Registration<1>>>);

impl kernel::Module for ParrotSafe {
    fn init(name: &'static CStr, module: &'static ThisModule) -> Result<Self> {
        let mut registration = Registration::new_pinned(name, 0, module)?;
        registration.as_mut().register::<ParrotOps>()?;
        Ok(ParrotSafe(registration))
    }
}

struct ParrotOps;

impl Operations for ParrotOps {
    declare_file_operations!(read);

    type Data = Box<AtomicUsize>;

    fn open(_: &Self::OpenData, _: &File) -> Result<Self::Data> {
        Ok(Box::try_new(AtomicUsize::new(0))?)
    }

    fn read(data: &AtomicUsize, _: &File, buf: &mut impl IoBufferWriter, _: u64) -> Result<usize> {
        let frame = FRAMES
            .get(data.fetch_add(1, Ordering::Relaxed) % FRAMES.len())
            .unwrap_or(&"");
        buf.write_slice(frame.as_bytes())?;
        // Yes, this is terrible
        unsafe { msleep(50) };
        Ok(frame.len())
    }
}
