use rp_pico::pac::{Peripherals, CorePeripherals};
// USB crates
use usb_device::{class_prelude::*, prelude::*};
use usbd_serial::SerialPort;


use rp_pico::hal;

use rp_pico::hal::Clock;

use rp_pico::{
    pac,
    Pins,
    hal::{
        Watchdog,
        Sio,
        clocks,
        usb,
        Timer,
    }
};

pub struct PicoBsp
{
    pub usb_bus: UsbBusAllocator<usb::UsbBus>,
    pub pins: Pins,
    pub delay: cortex_m::delay::Delay,
    pub timer: Timer,
}

impl PicoBsp
{
    pub fn new() -> Self
    {
            let mut pac = Peripherals::take().unwrap();
            let core = CorePeripherals::take().unwrap();

            let clocks = clocks::init_clocks_and_plls(
                rp_pico::XOSC_CRYSTAL_FREQ,
                pac.XOSC,
                pac.CLOCKS,
                pac.PLL_SYS,
                pac.PLL_USB,
                &mut pac.RESETS,
                &mut Watchdog::new(pac.WATCHDOG),
            )
            .ok()
            .unwrap();
    
            let sio = Some(Sio::new(pac.SIO));
    
            let pins = rp_pico::Pins::new(
                pac.IO_BANK0,
                pac.PADS_BANK0,
                sio.unwrap().gpio_bank0,
                &mut pac.RESETS,
            );

            let usb_bus = UsbBusAllocator::new(usb::UsbBus::new(
                pac.USBCTRL_REGS,
                pac.USBCTRL_DPRAM,
                clocks.usb_clock,
                true,
                &mut pac.RESETS,
            ));

            let delay = cortex_m::delay::Delay::new(core.SYST, clocks.system_clock.freq().to_Hz());
            let timer = hal::Timer::new(pac.TIMER, &mut pac.RESETS);
            
            Self{
            usb_bus: usb_bus,
            pins: pins,
            delay: delay,
            timer: timer,
            }
    }
}

pub struct PicoSerial<'a>
{
    usb_device: UsbDevice<'a,hal::usb::UsbBus>,
    usb_serial: SerialPort<'a,hal::usb::UsbBus>,
}

impl<'a> PicoSerial<'a>
{

    pub fn new(usb_bus: &'a UsbBusAllocator<hal::usb::UsbBus>) -> Self
    {
        Self {
            usb_serial: SerialPort::new(usb_bus),
            usb_device: UsbDeviceBuilder::new(usb_bus, UsbVidPid(0x16c0, 0x05e1))
                            .manufacturer("Fake company")
                            .product("Serial port")
                            .serial_number("TEST")
                            .device_class(2) // from: https://www.usb.org/defined-class-codes
                            .build(),
        }
    }

    pub fn enable_interrupt()
    {
        unsafe
        {
            pac::NVIC::unmask(pac::Interrupt::USBCTRL_IRQ);
        };
    }

    pub fn run(&mut self)
    {
        critical_section::with(|cs|{
            let _ = self.usb_device.poll(&mut [&mut self.usb_serial]);
        });
    }

    pub fn write(&mut self, s: &str)
    {
        critical_section::with(|cs|{
            let _ = self.usb_serial.write(s.as_bytes());
        });
    }
}
