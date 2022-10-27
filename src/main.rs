#![no_std]
#![no_main]
#![deny(unsafe_code)]

// The macro for our start-up function
use cortex_m_rt::entry;

// use embedded_hal::digital::v2::OutputPin;
// use embedded_hal::digital::v2::ToggleableOutputPin;
// The macro for marking our interrupt functions
// use rp_pico::hal::pac::interrupt;

// Time handling traits
// use embedded_time::rate::*;

// Ensure we halt the program on panic (if we don't mention this crate it won't
// be linked)
// use panic_halt as _;

// Pull in any important traits
// use rp_pico::hal::prelude::*;

// A shorter alias for the Peripheral Access Crate, which provides low-level
// register access
// use rp_pico::hal::pac;

// A shorter alias for the Hardware Abstraction Layer, which provides
// higher-level drivers.
// use rp_pico::hal;
// use rp_pico::hal::gpio;
// use rp_pico::hal::gpio::Interrupt::EdgeLow;

// use rp_pico::pac::dma::timer0;
// USB Device support
// use usb_device::{class_prelude::*, prelude::*};

// USB Communications Class Device support
// use usbd_serial::SerialPort;

// ============================================================================

// mod application;
// mod platform;
mod bsp;
use bsp::PicoBsp;
use embedded_hal::digital::v2::OutputPin;
// ============================================================================

/// Application object
// static mut APP_INSTANCE: Option<application::PicohaIo> = None;

/// USB bus allocator
// static mut USB_BUS: Option<UsbBusAllocator<hal::usb::UsbBus>> = None;
/// USB device object
// static mut USB_DEVICE: Option<UsbDevice<hal::usb::UsbBus>> = None;
/// USB serial object
// static mut USB_SERIAL: Option<SerialPort<hal::usb::UsbBus>> = None;

// ============================================================================

/// Entry point to our bare-metal application.
///
/// The `#[entry]` macro ensures the Cortex-M start-up code calls this function
/// as soon as all global variables are initialised.
///
/// The function configures the RP2040 peripherals, then blinks the LED in an
/// infinite loop.
#[entry]
fn main() -> ! {

    PicoBsp::init();
    let bsp = PicoBsp::borrow().unwrap();

    let mut led = bsp.led;
    led.set_high().unwrap();

    loop {
        // interrupts handle everything else in this example.
        cortex_m::asm::wfi();
    }

}



// ============================================================================

// use core::cell::RefCell;
// PANIC MANAGEMENT
mod panic;

// ============================================================================

/// This pin will be our output - it will drive an LED if you run this on a Pico
// type LedPin = gpio::Pin<gpio::bank0::Gpio25, gpio::PushPullOutput>;

/// This pin will be our interrupt source.
/// It will trigger an interrupt if pulled to ground (via a switch or jumper wire)
// type ButtonPin = gpio::Pin<gpio::bank0::Gpio26, gpio::PullUpInput>;

/// Since we're always accessing these pins together we'll store them in a tuple.
/// Giving this tuple a type alias means we won't need to use () when putting them
/// inside an Option. That will be easier to read.
// type LedAndButton = (LedPin, ButtonPin);
/// This how we transfer our Led and Button pins into the Interrupt Handler.
/// We'll have the option hold both using the LedAndButton type.
/// This will make it a bit easier to unpack them later.
// static GLOBAL_PINS: Mutex<RefCell<Option<LedAndButton>>> = Mutex::new(RefCell::new(None));
// use critical_section::Mutex;
// #[interrupt]
fn IO_IRQ_BANK0() {

    //     let mut LED_AND_BUTTON = critical_section::with(|cs| {
    //         GLOBAL_PINS.borrow(cs).take()
    //     });

    // // Need to check if our Option<LedAndButtonPins> contains our pins
    // if let Some(gpios) = LED_AND_BUTTON {
    //     // borrow led and button by *destructuring* the tuple
    //     // these will be of type `&mut LedPin` and `&mut ButtonPin`, so we don't have
    //     // to move them back into the static after we use them
    //     let (mut led, mut button) = gpios;
    //     // Check if the interrupt source is from the pushbutton going from high-to-low.
    //     // Note: this will always be true in this example, as that is the only enabled GPIO interrupt source
    //     if button.interrupt_status(EdgeLow) {
    //         // toggle can't fail, but the embedded-hal traits always allow for it
    //         // we can discard the return value by assigning it to an unnamed variable
    //         let _ = led.set_high();

    //         // Our interrupt doesn't clear itself.
    //         // Do that now so we don't immediately jump back to this interrupt handler.
    //         button.clear_interrupt(EdgeLow);
    //     }
    //     else {
    //         let _ = led.set_low();
    //         // button.clear_interrupt(EdgeHigh);
    //     }
    // }
}


// End of file
