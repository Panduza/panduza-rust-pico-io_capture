use core::panic::PanicInfo;
use embedded_hal::digital::v2::OutputPin;

use numtoa::NumToA;

use crate::LED;
use crate::SERIAL;

#[panic_handler]
unsafe fn panic(_info: &PanicInfo) -> ! {
    
    let serial = SERIAL.as_mut().unwrap();
    let led = LED.as_mut().unwrap();

    let _ = led.set_high();
    let mut tmp_buf = [0u8; 20];

    serial.write("{\"log\":\"");
    serial.write("PANIC! => ");
    serial
        .write(_info.location().unwrap().file())
        ;
    serial.write(":");
    serial
        .write(_info.location().unwrap().line().numtoa_str(10, &mut tmp_buf))
        ;
    serial.write("\"}\r\n");

    loop{
    }
}