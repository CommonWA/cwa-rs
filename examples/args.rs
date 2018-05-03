#![no_main]

#[macro_use]
extern crate cwa;

main!({
    let args = cwa::startup::args();
    println!("{:?}", args);
});
