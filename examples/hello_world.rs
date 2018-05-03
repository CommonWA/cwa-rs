#![no_main]

#[macro_use]
extern crate cwa;

main!({
    for i in 0..10000 {
        println!("Hello world");
        println!("Seq id = {}", i);
    }
});
