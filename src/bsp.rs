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

// USB crates
use rp_pico::hal::usb::UsbBus;
use usb_device::class_prelude::UsbBusAllocator;
use usb_device::prelude::UsbDevice;
use usb_device::prelude::UsbDeviceBuilder;
use usb_device::prelude::UsbVidPid;

// USB Communications Class Device support
use usbd_serial::SerialPort;

/// Usb Manufacturer Name
pub const USB_MANUFACTURER_NAME: &str = "github.com/XdoctorwhoZ";

/// Usb Product Name
pub const USB_PRODUCT_NAME: &str = "Aardvark_Pico_Clone";

/// Usb Manufacturer Id
pub const USB_MANUFACTURER_ID: u16 = 0x16c0;

/// Usb Product Id
pub const USB_PRODUCT_ID: u16 = 0x05e1;

/// Usb Serial Number
/// TEST_123456789 is the serial used for tests
pub const USB_SERIAL_NUMBER: &str = "TEST_123456789";

static mut USBBUS: Option<UsbBusAllocator<hal::usb::UsbBus>> = None;

pub struct IoCaptureBsp<'a>
{
    usb_device: Option<UsbDevice<'a, hal::usb::UsbBus>>,
    usb_serial: Option<SerialPort<'a, hal::usb::UsbBus>>
}

static mut BSP: bool = false;

impl IoCaptureBsp<'_>
{
    #[inline]
    pub fn take() -> Option<Self> {
        cortex_m::interrupt::free(|_| {
            if unsafe { BSP } {
                None
            } else {
                Some(unsafe { IoCaptureBsp::steal() })
            }
        })
    }
    #[inline]
    pub unsafe fn steal() -> Self
    {
        BSP = true;
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
        USBBUS = Some(usb_bus);

        // Grab a reference to the USB Bus allocator. We are promising to the
        // compiler not to take mutable access to this global variable whilst this
        // reference exists!
        let bus_ref = USBBUS.as_ref().unwrap();

        let usb_device = Some(UsbDeviceBuilder::new(
            bus_ref,
            UsbVidPid(USB_MANUFACTURER_ID, USB_PRODUCT_ID),
        )
        .manufacturer(USB_MANUFACTURER_NAME)
        .product(USB_PRODUCT_NAME)
        .serial_number(USB_SERIAL_NUMBER)
        .device_class(2) // from: https://www.usb.org/defined-class-codes
        .build());

        let usb_serial = Some(SerialPort::new(bus_ref));

        // Enable the USB interrupt
        pac::NVIC::unmask(hal::pac::Interrupt::USBCTRL_IRQ);

        //
        // No more USB code after this point in main! We can do anything we want in
        // here since USB is handled in the interrupt
        //

        // The single-cycle I/O block controls our GPIO pins
        let sio = hal::Sio::new(pac.SIO);

        Self
        {
            usb_device: usb_device,
            usb_serial: usb_serial
        }
    }

    pub fn get_serial() -> SerialPort<'static, hal::usb::UsbBus>
    {
        let bsp = IoCaptureBsp::take().unwrap();
        bsp.usb_serial.unwrap()
    }

    pub fn get_device() -> UsbDevice<'static, hal::usb::UsbBus>
    {
        let bsp = IoCaptureBsp::take().unwrap();
        bsp.usb_device.unwrap()
    }
}