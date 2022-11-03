
// ============================================================================
use rp_pico::hal::pac::interrupt;
use super::PicoBsp;
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
    let mut usb_device = USB_DEVICE;
    let mut usb_serial = USB_SERIAL;
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
                // bsp.usb_buffer.load(&serial_buffer, count);
            }
        }
    }
}