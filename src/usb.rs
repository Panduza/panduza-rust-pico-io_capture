// ============================================================================
#[path= "./bsp.rs"]
mod bsp;
use bsp::IoCaptureBsp;
use rp_pico::hal::pac::interrupt;
/// This function is called whenever the USB Hardware generates an Interrupt
/// Request.
///
/// We do all our USB work under interrupt, so the main thread can continue on
/// knowing nothing about USB.
#[allow(non_snake_case)]
#[interrupt]
unsafe fn USBCTRL_IRQ() {
    let mut usb_serial = IoCaptureBsp::get_serial();
    let mut usb_device = IoCaptureBsp::get_device();
    if usb_device.poll(&mut [&mut usb_serial]) {
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
                // usb_buffer.load(&serial_buffer, count);
            }
        }
    }
}