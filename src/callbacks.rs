
// ============================================================================
use rp_pico::hal::pac::interrupt;
// use super::PicoBsp;
use super::SERIAL;
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