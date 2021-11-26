#![deny(unsafe_code)]
#![no_main]
#![no_std]

use cortex_m::asm;
use cortex_m_rt::entry;
use panic_halt as _;

#[entry]
fn main() -> ! {
    let _y;
    let x = 42;
    _y = x;
    asm::nop();

    // infinite loop; just so we don't leave this stack frame
    loop {}
}
