#![no_std]
#![no_main]

// The macro for our start-up function
use cortex_m_rt::entry;

use embedded_hal::digital::v2::OutputPin;
// The macro for marking our interrupt functions
use rp_pico::hal::pac::interrupt;

// Time handling traits
use embedded_time::rate::*;

// Ensure we halt the program on panic (if we don't mention this crate it won't
// be linked)
// use panic_halt as _;

// Pull in any important traits
use rp_pico::hal::prelude::*;

// A shorter alias for the Peripheral Access Crate, which provides low-level
// register access
use rp_pico::hal::pac;

// A shorter alias for the Hardware Abstraction Layer, which provides
// higher-level drivers.
use rp_pico::hal;
use rp_pico::hal::gpio::dynpin::DynPin;

// USB Device support
use usb_device::{class_prelude::*, prelude::*};

// USB Communications Class Device support
use usbd_serial::SerialPort;

// ============================================================================

mod application;
mod platform;

// ============================================================================

/// Application object
static mut APP_INSTANCE: Option<application::PicohaIo> = None;

/// USB bus allocator
static mut USB_BUS: Option<UsbBusAllocator<hal::usb::UsbBus>> = None;
/// USB device object
static mut USB_DEVICE: Option<UsbDevice<hal::usb::UsbBus>> = None;
/// USB serial object
static mut USB_SERIAL: Option<SerialPort<hal::usb::UsbBus>> = None;

// ============================================================================

/// Entry point to our bare-metal application.
///
/// The `#[entry]` macro ensures the Cortex-M start-up code calls this function
/// as soon as all global variables are initialised.
///
/// The function configures the RP2040 peripherals, then blinks the LED in an
/// infinite loop.
#[entry]
unsafe fn main() -> ! {
    // Grab our singleton objects
    let mut pac = pac::Peripherals::take().unwrap();
    let core = pac::CorePeripherals::take().unwrap();

    // Set up the watchdog driver - needed by the clock setup code
    let mut watchdog = hal::Watchdog::new(pac.WATCHDOG);

    // Configure the clocks
    //
    // The default is to generate a 125 MHz system clock
    let clocks = hal::clocks::init_clocks_and_plls(
        rp_pico::XOSC_CRYSTAL_FREQ,
        pac.XOSC,
        pac.CLOCKS,
        pac.PLL_SYS,
        pac.PLL_USB,
        &mut pac.RESETS,
        &mut watchdog,
    )
    .ok()
    .unwrap();

    // Set up the USB driver
    let usb_bus = UsbBusAllocator::new(hal::usb::UsbBus::new(
        pac.USBCTRL_REGS,
        pac.USBCTRL_DPRAM,
        clocks.usb_clock,
        true,
        &mut pac.RESETS,
    ));
    // Note (safety): This is safe as interrupts haven't been started yet
    USB_BUS = Some(usb_bus);

    // Grab a reference to the USB Bus allocator. We are promising to the
    // compiler not to take mutable access to this global variable whilst this
    // reference exists!
    let bus_ref = USB_BUS.as_ref().unwrap();

    USB_SERIAL = Some(platform::init_usb_serial(bus_ref));
    USB_DEVICE = Some(platform::init_usb_device(bus_ref));

    // Enable the USB interrupt
    pac::NVIC::unmask(hal::pac::Interrupt::USBCTRL_IRQ);

    unsafe {
        pac::NVIC::unmask(pac::Interrupt::IO_IRQ_BANK0);
    }

    //
    // No more USB code after this point in main! We can do anything we want in
    // here since USB is handled in the interrupt
    //

    // The single-cycle I/O block controls our GPIO pins
    let sio = hal::Sio::new(pac.SIO);

    // Set the pins up according to their function on this particular board
    let pins = rp_pico::Pins::new(
        pac.IO_BANK0,
        pac.PADS_BANK0,
        sio.gpio_bank0,
        &mut pac.RESETS,
    );

    
    let led = pins.led.into_push_pull_output();

    led.set_high().unwrap();

    // Init the application and start it
    APP_INSTANCE = Some(application::PicohaIo::new(
        cortex_m::delay::Delay::new(core.SYST, clocks.system_clock.freq().integer()), // Append delay feature to the app
        [
            pins.gpio0.into_floating_input(),
            pins.gpio1.into(),
            pins.gpio2.into(),
            pins.gpio3.into(),
            pins.gpio4.into(),
            pins.gpio5.into(),
            pins.gpio6.into(),
            pins.gpio7.into(),
            pins.gpio8.into(),
            pins.gpio9.into(),
            pins.gpio10.into(),
            pins.gpio11.into(),
            pins.gpio12.into(),
            pins.gpio13.into(),
            pins.gpio14.into(),
            pins.gpio15.into(),
            pins.gpio16.into(),
            pins.gpio17.into(),
            pins.gpio18.into(),
            pins.gpio19.into(),
            pins.gpio20.into(),
            pins.gpio21.into(),
            pins.gpio22.into(),
            pins.gpio26.into(),
            pins.gpio27.into(),
            pins.gpio28.into(),
        ],
        USB_DEVICE.as_mut().unwrap(),
        USB_SERIAL.as_mut().unwrap(),
    ));

    // Run the application
    let app = APP_INSTANCE.as_mut().unwrap();
    app.run_forever();

    loop {
        // interrupts handle everything else in this example.
        cortex_m::asm::wfi();
    }

}

// ============================================================================

/// This function is called whenever the USB Hardware generates an Interrupt
/// Request.
///
/// We do all our USB work under interrupt, so the main thread can continue on
/// knowing nothing about USB.
#[allow(non_snake_case)]
#[interrupt]
unsafe fn USBCTRL_IRQ() {
    let app = APP_INSTANCE.as_mut().unwrap();
    app.usbctrl_irq();
}

// ============================================================================

// PANIC MANAGEMENT
use core::panic::PanicInfo;
#[panic_handler]
unsafe fn panic(_info: &PanicInfo) -> ! {
    let app = APP_INSTANCE.as_mut().unwrap();
    app.panic_handler(_info);
}

// ============================================================================

/// This pin will be our output - it will drive an LED if you run this on a Pico
type LedPin = gpio::Pin<gpio::bank0::Gpio25, gpio::PushPullOutput>;

/// This pin will be our interrupt source.
/// It will trigger an interrupt if pulled to ground (via a switch or jumper wire)
type ButtonPin = gpio::Pin<gpio::bank0::Gpio26, gpio::PullUpInput>;

/// Since we're always accessing these pins together we'll store them in a tuple.
/// Giving this tuple a type alias means we won't need to use () when putting them
/// inside an Option. That will be easier to read.
type LedAndButton = (LedPin, ButtonPin);

/// This how we transfer our Led and Button pins into the Interrupt Handler.
/// We'll have the option hold both using the LedAndButton type.
/// This will make it a bit easier to unpack them later.
static GLOBAL_PINS: Mutex<RefCell<Option<LedAndButton>>> = Mutex::new(RefCell::new(None));

#[interrupt]
fn IO_IRQ_BANK0() {
    // The `#[interrupt]` attribute covertly converts this to `&'static mut Option<LedAndButton>`
    static mut LED_AND_BUTTON: Option<LedAndButton> = None;

    // This is one-time lazy initialisation. We steal the variables given to us
    // via `GLOBAL_PINS`.
    if LED_AND_BUTTON.is_none() {
        critical_section::with(|cs| {
            *LED_AND_BUTTON = GLOBAL_PINS.borrow(cs).take();
        });
    }

    // Need to check if our Option<LedAndButtonPins> contains our pins
    if let Some(gpios) = LED_AND_BUTTON {
        // borrow led and button by *destructuring* the tuple
        // these will be of type `&mut LedPin` and `&mut ButtonPin`, so we don't have
        // to move them back into the static after we use them
        let (led, button) = gpios;
        // Check if the interrupt source is from the pushbutton going from high-to-low.
        // Note: this will always be true in this example, as that is the only enabled GPIO interrupt source
        if button.interrupt_status(EdgeLow) {
            // toggle can't fail, but the embedded-hal traits always allow for it
            // we can discard the return value by assigning it to an unnamed variable
            let _ = led.toggle();

            // Our interrupt doesn't clear itself.
            // Do that now so we don't immediately jump back to this interrupt handler.
            button.clear_interrupt(EdgeLow);
        }
    }
}


// End of file
