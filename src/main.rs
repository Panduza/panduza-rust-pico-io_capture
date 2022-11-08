#![no_std]
#![no_main]
// #![deny(unsafe_code)]

use heapless::pool::Box;
use rp_pico::
    {
        entry, 
        hal::pac::interrupt
    };

use irq::{scoped_interrupts, handler, scope};

use embedded_hal::digital::v2::{ToggleableOutputPin, InputPin};
mod pico_bsp;
use pico_bsp::PicoBsp;
use usb_device::prelude::*;
// mod callbacks;

use rp_pico::hal::gpio::Interrupt::*;

struct Capture
{
    io: u8,
    pub t0: u64,
    pub t1: u64
}

scoped_interrupts! {
    enum Interrupt{
        USBCTRL_IRQ,
        IO_IRQ_BANK0
    }

    use #[rp_pico::hal::pac::interrupt];
}

#[entry]
fn main() -> ! {
    let bsp = PicoBsp::new();
    let (mut led, 
        mut button,
        mut delay, 
        usb,
        timer,
        mut nvic ) = 
            (bsp.pins.led.into_push_pull_output(), 
            bsp.pins.gpio15.into_pull_up_input(),
            bsp.delay, 
            bsp.usb_bus,
            bsp.timer,
        bsp.nvic);



    let mut serial = SerialPort::new(&usb);

    let mut usb_device = UsbDeviceBuilder::new(&usb, UsbVidPid(0x16c0, 0x05e1))
    .manufacturer("Fake company")
    .product("Serial port")
    .serial_number("TEST")
    .device_class(2) // from: https://www.usb.org/defined-class-codes
    .build();

    let _ = led.toggle();

    let mut button_capture = Capture {io:15,t0:0,t1:0};

    handler!(usb_cb = || { 
        if usb_device.poll(&mut [&mut serial]) {
            let mut buf = [0u8; 64];
            match serial.read(&mut buf) {
                Err(_e) => {
                    // Do nothing
                }
                Ok(0) => {
                    // Do nothing
                }
                Ok(count) => {

                    // Send back to the host
                    let mut wr_ptr = &buf[..count];
                    while !wr_ptr.is_empty() {
                        let _ = serial.write(wr_ptr).map(|len| {
                            wr_ptr = &wr_ptr[len..];
                        });
                    }
                }
            }
        }
    });

    button.set_interrupt_enabled(EdgeLow, true);
    button.set_interrupt_enabled(EdgeHigh, true);

    handler!(io_cb = move | | { 
        let _ = led.toggle();
        if button.interrupt_status(EdgeLow)
        {
            button_capture.t0 = timer.get_counter();
            button.clear_interrupt(EdgeLow);
        }
        else if button.interrupt_status(EdgeHigh)
        {
            button_capture.t1 = timer.get_counter();
            button.clear_interrupt(EdgeHigh);
        }
    });

    scope(|scope| -> ! {
        scope.register(Interrupt::USBCTRL_IRQ, usb_cb);
        scope.register(Interrupt::IO_IRQ_BANK0, io_cb);

        // The interrupts stay registered for the duration of this closure.
        // This is a good place for the application's idle loop.
    

        unsafe {
            nvic.set_priority(rp_pico::pac::Interrupt::IO_IRQ_BANK0, 0);
            rp_pico::pac::NVIC::unmask(rp_pico::pac::Interrupt::IO_IRQ_BANK0);
            rp_pico::pac::NVIC::unmask(rp_pico::pac::Interrupt::USBCTRL_IRQ);
        }

        loop
        {
                // delay.delay_ms(500);
        }
    })
}

use panic_halt as _;
use usbd_serial::SerialPort;
// mod panic;
