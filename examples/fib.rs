#![no_main]

#[macro_use]
extern crate cwa;

fn fib(n: i32) -> i32 {
    if n == 1 || n == 2 {
        1
    } else {
        fib(n - 1) + fib(n - 2)
    }
}

main!({
    let args = cwa::startup::args();
    let n: i32 = args[1].parse().expect("Invalid input");
    println!("{}", fib(n));
});
