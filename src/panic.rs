use core::panic::PanicInfo;
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    // let app = APP_INSTANCE.as_mut().unwrap();
    // app.panic_handler(_info);
    loop{}
}