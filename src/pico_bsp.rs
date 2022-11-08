use embedded_time::clock;
use rp_pico::pac::{Peripherals, CorePeripherals};
// USB crates
use usb_device::{class_prelude::*, prelude::*};
use usbd_serial::SerialPort;


use rp_pico::hal;

use rp_pico::hal::Clock;

use rp_pico::{
    pac,
    pac::NVIC,
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
    pub nvic: NVIC,
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

            let nvic = core.NVIC;
            
            Self{
            usb_bus: usb_bus,
            pins: pins,
            delay: delay,
            timer: timer,
            nvic: nvic,
            }
    }
}
