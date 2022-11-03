use embedded_hal::digital::v2::OutputPin;
use rp_pico::hal::gpio;
use rp_pico::hal::pac;
use rp_pico::hal::Watchdog;
use rp_pico::hal::clocks;
use rp_pico::hal::Sio;
use rp_pico::Pins;
use critical_section::Mutex;
use rp_pico::pac::Peripherals;

use core::cell::RefCell;
use rp_pico::hal::gpio::Interrupt::{EdgeLow, EdgeHigh};

// USB crates
use usb_device::class_prelude::UsbBusAllocator;
use usb_device::prelude::UsbDevice;
use usbd_serial::SerialPort;
use usb_device::prelude::UsbDeviceBuilder;
use usb_device::prelude::UsbVidPid;

use rp_pico::hal::prelude::*;
use rp_pico::hal;
// use embedded_time::rate::*;

static BSP: Mutex<RefCell<Option<PicoBsp>>> = Mutex::new(RefCell::new(None));
static mut USB_BUS: Option<UsbBusAllocator<hal::usb::UsbBus>> = None;
/// USB device object
static mut USB_DEVICE: Option<UsbDevice<hal::usb::UsbBus>> = None;
/// USB serial object
static mut USB_SERIAL: Option<SerialPort<hal::usb::UsbBus>> = None;

pub struct PicoBsp
{
    pub delay: cortex_m::delay::Delay,
    pub led: gpio::Pin<gpio::bank0::Gpio25, gpio::PushPullOutput>,
    // pub usb_device: UsbDevice<'static, rp_pico::hal::usb::UsbBus>,
    // pub usb_serial: SerialPort<'static, rp_pico::hal::usb::UsbBus>,
}


impl PicoBsp
{
    pub fn init()
    {
        let mut pac = pac::Peripherals::take().unwrap();
        let core = pac::CorePeripherals::take().unwrap();

        // Set up the watchdog driver - needed by the clock setup code
        let mut watchdog = Watchdog::new(pac.WATCHDOG);

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

        let delay = cortex_m::delay::Delay::new(core.SYST, clocks.system_clock.freq().to_Hz());

        // #[allow(unsafe_code)]
        // unsafe {
        //     pac::NVIC::unmask(pac::Interrupt::IO_IRQ_BANK0);
        // }

        let sio = Sio::new(pac.SIO);

        // Set the pins up according to their function on this particular board
        let pins = rp_pico::Pins::new(
            pac.IO_BANK0,
            pac.PADS_BANK0,
            sio.gpio_bank0,
            &mut pac.RESETS,
        );

        PicoBsp::replace(Self {
            delay: delay,
            led: pins.led.into_push_pull_output(),
            // usb_device: usb_device,
            // usb_serial: SerialPort::new(&bus_ref),
        });

        // #[allow(unsafe_code)]
        // unsafe {
        //     pac::NVIC::unmask(rp_pico::hal::pac::Interrupt::USBCTRL_IRQ);
        // };

        #[allow(unsafe_code)]
        unsafe {
            // USB_BUS = Some(UsbBusAllocator::new(rp_pico::hal::usb::UsbBus::new(
            //     pac.USBCTRL_REGS,
            //     pac.USBCTRL_DPRAM,
            //     clocks.usb_clock,
            //     true,
            //     &mut pac.RESETS,
            // )));
            // let bus_ref = USB_BUS.as_ref().unwrap();

            // USB_DEVICE = Some(UsbDeviceBuilder::new(
            //     bus_ref,
            //     UsbVidPid(0x16c0, 0x05e1),
            // )
            // .manufacturer("http://github.com/BCadet")
            // .product("Aardvark_Pico_Clone")
            // .serial_number("TEST_123456789")
            // .device_class(2) // from: https://www.usb.org/defined-class-codes
            // .build());

            // USB_SERIAL = Some(SerialPort::new(bus_ref));

            // Set up the USB driver
            let usb_bus = UsbBusAllocator::new(hal::usb::UsbBus::new(
                pac.USBCTRL_REGS,
                pac.USBCTRL_DPRAM,
                clocks.usb_clock,
                true,
                &mut pac.RESETS,
            ));
            // Set up the USB Communications Class Device driver
            let mut serial = SerialPort::new(&usb_bus);

            // Create a USB device with a fake VID and PID
            let mut usb_dev = UsbDeviceBuilder::new(&usb_bus, UsbVidPid(0x16c0, 0x27dd))
                .manufacturer("Fake company")
                .product("Serial port")
                .serial_number("TEST")
                .device_class(2) // from: https://www.usb.org/defined-class-codes
                .build();
        };
        
    }

    pub fn borrow() -> Option<PicoBsp>
    {
        critical_section::with(|cs| {
            BSP.borrow(cs).take()
        })
    }

    pub fn replace(new_bsp: PicoBsp)
    {
        critical_section::with(|cs| {
            BSP.borrow(cs).replace(Some(new_bsp));
        });
    }
}


// ============================================================================
use rp_pico::hal::pac::interrupt;
/// This function is called whenever the USB Hardware generates an Interrupt
/// Request.
///
/// We do all our USB work under interrupt, so the main thread can continue on
/// knowing nothing about USB.
#[allow(non_snake_case)]
#[interrupt]
fn USBCTRL_IRQ()
{
    
    // let (mut led, mut usb_device, mut usb_serial) = {
    //     let bsp = PicoBsp::borrow().unwrap();
    //     (bsp.led, bsp.usb_device, bsp.usb_serial)
    // };

    // led.toggle();

    // // Poll the USB driver with all of our supported USB Classes
    #[allow(unsafe_code)]
    unsafe {
    let usb_device = USB_DEVICE.as_mut().unwrap();
    let usb_serial = USB_SERIAL.as_mut().unwrap();
    if usb_device.poll(&mut [ usb_serial]) {
        // Buffer to read the serial port
        let mut serial_buffer = [0u8; 512];
        match usb_serial.read(&mut serial_buffer) {
            Err(_e) => {
                // Do nothing
            }
            Ok(0) => {
                // Do nothing
            }
            Ok(count) => {
                // bsp.usb_buffer.load(&serial_buffer, count);
            }
        }
    }
};
}