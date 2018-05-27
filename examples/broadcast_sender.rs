#![no_main]

#[macro_use]
extern crate cwa;

use std::io::Write;
use cwa::resource::Resource;

static BYTES: &'static [u8] = &[0x20; 4096];

main!({
    let args = cwa::startup::args();

    let channel_name = &args[1];
    let mut handle = Resource::open(
        format!("ipc-broadcast://{}?new=1", channel_name).as_str()
    ).unwrap_or_else(|| {
        println!("Unable to open channel");
        panic!();
    });

    loop {
        handle.write(BYTES).unwrap();
    }
});
