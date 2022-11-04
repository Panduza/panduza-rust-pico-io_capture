
// ============================================================================
use rp_pico::hal::pac::interrupt;
use rp_pico::hal::gpio::Interrupt::*;
// use super::PicoBsp;
use super::SERIAL;
use super::BUTTON;
/// This function is called whenever the USB Hardware generates an Interrupt
/// Request.
///
/// We do all our USB work under interrupt, so the main thread can continue on
/// knowing nothing about USB.
#[allow(non_snake_case)]
#[interrupt]
unsafe fn USBCTRL_IRQ()
{
    let serial = SERIAL.as_mut().unwrap();
    serial.run();
}

#[allow(non_snake_case)]
#[interrupt]
unsafe fn IO_IRQ_BANK0() {
    let serial = SERIAL.as_mut().unwrap();
    let button = BUTTON.as_mut().unwrap();
    serial.write("IT\r\n");
    button.clear_interrupt(EdgeLow);
    button.clear_interrupt(EdgeHigh);
}