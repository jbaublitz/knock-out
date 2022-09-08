//! Example kernel module that creates a device, /dev/parrot, that when read
//! from will generate an animation.

use core::{cmp::min, time::Duration};
use kernel::{
    delay::coarse_sleep,
    file::{File, Operations},
    io_buffer::IoBufferWriter,
    miscdev::Registration,
    prelude::*,
};

mod frames;
use frames::{calc_frame_and_offset, FRAMES};

module! {
    type: ParrotSafe,
    name: "party_parrot",
    author: "John Baublitz <john.m.baublitz@gmail.com>",
    description: "Get the party started",
    license: "GPL",
}

struct ParrotSafe(Pin<Box<Registration<ParrotOps>>>);

impl kernel::Module for ParrotSafe {
    fn init(_: &'static CStr, _: &'static ThisModule) -> Result<Self> {
        Ok(ParrotSafe(Registration::new_pinned(fmt!("parrot"), ())?))
    }
}

struct ParrotOps;

#[vtable]
impl Operations for ParrotOps {
    fn open(_: &Self::OpenData, _: &File) -> Result<Self::Data> {
        Ok(())
    }

    fn read(_: (), _: &File, buf: &mut impl IoBufferWriter, offset: u64) -> Result<usize> {
        if buf.len() < 1 {
            pr_info!("parrot device driver requires a buffer of at least 1 byte");
            return Err(EINVAL);
        }
        let (frame, frame_offset) = calc_frame_and_offset(offset);
        let frame = FRAMES.get(frame).ok_or(EIO)?;
        let offset_usize: usize = frame_offset.try_into()?;
        let s = &frame.as_bytes()[offset_usize..][..min(frame.len() - offset_usize, buf.len())];
        buf.write_slice(s)?;
        if offset_usize + s.len() == frame.len() {
            coarse_sleep(Duration::from_millis(50));
        }
        Ok(s.len())
    }
}
