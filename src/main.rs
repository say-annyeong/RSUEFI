#![no_main]
#![no_std]

use core::panic::PanicInfo;
use uefi::prelude::*;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    panic!("패닉이다!!!")
}

#[entry]
fn main() -> Status {
    uefi::helpers::init().unwrap();

    loop {}
    Status::SUCCESS
}
