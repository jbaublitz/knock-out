use std::{error::Error, fs::File, io::Read, str::from_utf8};

fn main() -> Result<(), Box<dyn Error>> {
    let mut f = File::open("/dev/parrot")?;
    let buffer = &mut [0; 4096];
    loop {
        let read = f.read(buffer)?;
        print!("{}", from_utf8(&buffer[..read])?);
    }
}
