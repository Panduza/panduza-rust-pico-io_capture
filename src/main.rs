#![no_std]
#![no_main]
// #![deny(unsafe_code)]

// mod pico_bsp;

// mod callbacks;
use rtic;


#[rtic::app(device = rp_pico::hal::pac)]
mod app {
    use heapless::Vec;
    use rp_pico::{
        Pins,
        hal::{
            Watchdog,
            Sio,
            clocks,
            usb,
            Timer,
            Clock,
        }
    };
    use rp_pico::hal::{gpio::Interrupt::*, usb::UsbBus};
    use usb_device::{prelude::*, class_prelude::UsbBusAllocator};
    use usbd_serial::SerialPort;
    use rp_pico::{
        hal::gpio::*,
        hal::gpio::bank0::*,
    };
    use embedded_hal::digital::v2::{ToggleableOutputPin, OutputPin, InputPin};
    use serde::{Serialize, Deserialize};

    #[derive(Serialize, Deserialize, Debug)]
    struct IoValue
    {
        pub val: [u8; 10],
    }


    // use cortex_m_semihosting::{debug, hprintln};

    #[shared]
    struct Shared {
        led: Pin<Gpio25, Output<PushPull>>, 
        button: Pin<Gpio15, Input<PullUp>>,
        serial: SerialPort<'static, UsbBus>,
        device: UsbDevice<'static, UsbBus>,
        buffer: Vec<u8, 255>,
    }

    #[local]
    struct Local {
    }

    // `#[init]` cannot access locals from the `#[local]` struct as they are initialized here.
    #[init(local = [usb_bus: Option<UsbBusAllocator<UsbBus>> = None])]
    fn init(cx: init::Context) -> (Shared, Local, init::Monotonics)
    {
        let mut reset = cx.device.RESETS;

        let clocks = clocks::init_clocks_and_plls(
            rp_pico::XOSC_CRYSTAL_FREQ,
            cx.device.XOSC,
            cx.device.CLOCKS,
            cx.device.PLL_SYS,
            cx.device.PLL_USB,
            &mut reset,
            &mut Watchdog::new(cx.device.WATCHDOG),
        )
        .ok()
        .unwrap();

        let usb: &'static _ = cx.local.usb_bus.insert(UsbBusAllocator::new(usb::UsbBus::new(
            cx.device.USBCTRL_REGS,
            cx.device.USBCTRL_DPRAM,
            clocks.usb_clock,
            true,
            &mut reset,
        )));

        let sio = Sio::new(cx.device.SIO);

        let pins = rp_pico::Pins::new(
            cx.device.IO_BANK0,
            cx.device.PADS_BANK0,
            sio.gpio_bank0,
            &mut reset,
        );

        // let delay = cortex_m::delay::Delay::new(core.SYST, clocks.system_clock.freq().to_Hz());
        // let timer = Timer::new(pac.TIMER, &mut pac.RESETS);

        // let mut nvic = core.NVIC;

        let button = pins.gpio15.into_pull_up_input();
        let mut led = pins.led.into_push_pull_output();
    
        // let usb: &'static _ = cx.local.usb_bus.insert(usb_bus);
        // // // let serial = PicoSerial::new(usb);
        let serial = SerialPort::new(&usb);

        let device = UsbDeviceBuilder::new(&usb, UsbVidPid(0x16c0, 0x27dd))
        .manufacturer("Fake company")
        .product("Serial port")
        .serial_number("TEST")
        .device_class(2) // from: https://www.usb.org/defined-class-codes
        .build();
    
        button.set_interrupt_enabled(EdgeLow, true);
        button.set_interrupt_enabled(EdgeHigh, true);

        let _ = led.set_high();

        let buffer = Vec::<u8, 255>::new();

        (
            Shared { led, button, serial, device, buffer },
            // Shared { led, button },
            Local {},
            init::Monotonics(),
        )
    }

    #[idle()]
    fn idle(_cx: idle::Context) -> ! {
        
        loop {
            cortex_m::asm::nop();
        }
    }

    #[task(
        binds = IO_IRQ_BANK0,
        shared = [button, led, serial]
    )]
    fn io_cb(cx: io_cb::Context) {
        (cx.shared.led, cx.shared.button).lock(
            |led, button| {
                let _ = led.toggle();
                let mut outBuff = [0;64];
                let mut outStruct = IoValue {val: [0;10]};
                outStruct.val[0] = button.is_high().unwrap() as u8;
                match serde_json_core::to_slice(&outStruct, &mut outBuff) {
                    Ok(size) => {
                        outBuff[size] = '\n' as u8;
                        let mut serial = cx.shared.serial;
                        serial.lock(|usb_serial| {usb_serial.write(&outBuff[0..(size+1)]).unwrap()});
                    }
                    Err(_) => {}
                }
                if button.interrupt_status(EdgeLow)
                {
                    button.clear_interrupt(EdgeLow);
                }
                else if button.interrupt_status(EdgeHigh)
                {
                    button.clear_interrupt(EdgeHigh);
                }
            }
        );
    }

    #[task(
        binds = USBCTRL_IRQ,
        shared = [serial, device, buffer]
    )]
    fn usb_cb(cx: usb_cb::Context) {
        let (
            serial,
            usb_device,
            mut buffer
            ) = (
            cx.shared.serial,
            cx.shared.device,
            cx.shared.buffer,
        );

        (serial, usb_device).lock(
            |serial_a, usb_dev_a| {
                if usb_dev_a.poll(&mut [serial_a]) {
                    let mut buf = [0u8; 64];
                    match serial_a.read(&mut buf) {
                        Err(_e) => {
                            // Do nothing
                        }
                        Ok(0) => {
                            // Do nothing
                        }
                        Ok(count) => {
                            buffer.lock(|buff| {buff.extend_from_slice(&buf[0..count])}).unwrap();
                        }
                    }
                }

                // buffer.lock(|buff| {
                //     match serde_json_core::de::from_slice::<IoValue>(buff.as_slice()) {
                // });
            });
        }
    }

use panic_halt as _;
// mod panic;
