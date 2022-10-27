use rp_pico::hal::gpio;
use rp_pico::hal::pac;
use rp_pico::hal::Watchdog;
use rp_pico::hal::clocks;
use rp_pico::hal::Sio;
use rp_pico::Pins;
use critical_section::Mutex;
use core::cell::RefCell;
use rp_pico::hal::gpio::Interrupt::{EdgeLow, EdgeHigh};

static BSP: Mutex<RefCell<Option<PicoBsp>>> = Mutex::new(RefCell::new(None));

pub struct PicoBsp
{
    pub led: gpio::Pin<gpio::bank0::Gpio25, gpio::PushPullOutput>,
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
        let clocks = clocks::init_clocks_and_plls(
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

        #[allow(unsafe_code)]
        unsafe {
            pac::NVIC::unmask(pac::Interrupt::IO_IRQ_BANK0);
        }

        let sio = Sio::new(pac.SIO);

        // Set the pins up according to their function on this particular board
        let pins = rp_pico::Pins::new(
            pac.IO_BANK0,
            pac.PADS_BANK0,
            sio.gpio_bank0,
            &mut pac.RESETS,
        );

        // let timer = rp_pico::hal::Timer::new(pac.TIMER, &mut pac.RESETS);

        // let time = timer.get_counter();

        PicoBsp::replace(Self {
            led: pins.led.into_push_pull_output(),
        });
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