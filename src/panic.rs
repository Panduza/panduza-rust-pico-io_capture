use core::panic::PanicInfo;
use embedded_hal::digital::v2::OutputPin;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    
    // let (mut led, mut delay) = {
    //     let bsp = PicoBsp::borrow().unwrap();
    //     (bsp.led,bsp.delay)
    // };

    loop{
        // delay.delay_ms(100);
        // let _ = led.set_low();
        // delay.delay_ms(100);
        // let _ = led.set_high();
    }
}