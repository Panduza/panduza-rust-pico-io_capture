#![no_std]
#![no_main]

// The macro for our start-up function
use cortex_m_rt::entry;
#[path = "./panic.rs"]
mod panic;

#[path = "./bsp.rs"]
mod bsp;

#[path = "./usb.rs"]
mod usb;
// ============================================================================

/// Entry point to our bare-metal application.
///
/// The `#[entry]` macro ensures the Cortex-M start-up code calls this function
/// as soon as all global variables are initialised.
///
/// The function configures the RP2040 peripherals, then blinks the LED in an
/// infinite loop.
#[entry]
fn main() -> !
{

    loop {}
}