#![no_std]
#![no_main]
// #![deny(unsafe_code)]

use rp_pico::entry;
use usb_device::class_prelude::*;

static mut SERIAL: Option<PicoSerial> = None;
static mut USB_BUS: Option<UsbBusAllocator<rp_pico::hal::usb::UsbBus>> = None;

use embedded_hal::digital::v2::ToggleableOutputPin;
mod pico_bsp;
use pico_bsp::{PicoBsp, PicoSerial};

mod callbacks;


#[entry]
fn main() -> ! {
    let bsp = PicoBsp::new();
    let (mut led, 
        mut delay, 
        usb,
        timer) = 
            (bsp.pins.led.into_push_pull_output(), 
            bsp.delay, 
            bsp.usb_bus,
            bsp.timer);
    
    unsafe {
        USB_BUS = Some(usb);
        SERIAL = Some(PicoSerial::new(&USB_BUS.as_ref().unwrap()));
    };

    // let timer = hal::Timer::new(pac.TIMER, &mut pac.RESETS);
    let mut said_hello = false;
    let serial = unsafe{SERIAL.as_mut().unwrap()};

    loop{
        // serial.run();
        if !said_hello && timer.get_counter() >= 2_000_000 {
            said_hello = true;
            // serial.write("Hello PICO\r\n");
        }
        else if said_hello
        {
            delay.delay_ms(500);
            let _ = led.toggle();
            serial.write("PICO\r\n");
        }
        
    }
}

use panic_halt as _;
