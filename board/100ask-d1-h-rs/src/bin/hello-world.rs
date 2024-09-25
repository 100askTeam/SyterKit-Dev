#![no_std]
#![no_main]

use panic_halt as _;
use syterkit::{entry, println, Clocks, Peripherals};

#[entry]
fn main(p: Peripherals, c: Clocks) {
    println!("Hello World!");
}
