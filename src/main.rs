#![no_std]
#![no_main]
// #![deny(unsafe_code)]

use rp_pico::{entry, hal::gpio::{Pin, bank0::{Gpio15, Gpio25}, Input, Output, PullUp, PushPull}};
use usb_device::class_prelude::*;

static mut SERIAL: Option<PicoSerial> = None;
static mut BUTTON: Option<Pin<Gpio15, Input<PullUp>>> = None;
static mut USB_BUS: Option<UsbBusAllocator<rp_pico::hal::usb::UsbBus>> = None;
static mut LED : Option<Pin<Gpio25, Output<PushPull>>> = None;

use embedded_hal::digital::v2::{ToggleableOutputPin, InputPin};
mod pico_bsp;
use pico_bsp::{PicoBsp, PicoSerial};

mod callbacks;

use rp_pico::hal::gpio::Interrupt::*;

#[entry]
fn main() -> ! {
    let bsp = PicoBsp::new();
    let (led, 
        button,
        mut delay, 
        usb,
        timer) = 
            (bsp.pins.led.into_push_pull_output(), 
            bsp.pins.gpio15.into_pull_up_input(),
            bsp.delay, 
            bsp.usb_bus,
            bsp.timer);

    let (led, serial, button) = unsafe {
        LED = Some(led);
        USB_BUS = Some(usb);
        SERIAL = Some(PicoSerial::new(&USB_BUS.as_ref().unwrap()));
        BUTTON = Some(button);
        ( LED.as_mut().unwrap(), SERIAL.as_mut().unwrap(), BUTTON.as_ref().unwrap() )
    };

    unsafe {
        rp_pico::pac::NVIC::unmask(rp_pico::pac::Interrupt::IO_IRQ_BANK0);
        rp_pico::pac::NVIC::unmask(rp_pico::pac::Interrupt::USBCTRL_IRQ);
    }

    // button.set_interrupt_enabled(EdgeLow, true);

    let mut said_hello = false;
    loop
    {
        // serial.run();
        if !said_hello && timer.get_counter() >= 2_000_000
        {
            said_hello = true;
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
// mod panic;
