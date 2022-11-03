// #[interrupt]
// fn IO_IRQ_BANK0() {

    //     let mut LED_AND_BUTTON = critical_section::with(|cs| {
    //         GLOBAL_PINS.borrow(cs).take()
    //     });

    // // Need to check if our Option<LedAndButtonPins> contains our pins
    // if let Some(gpios) = LED_AND_BUTTON {
    //     // borrow led and button by *destructuring* the tuple
    //     // these will be of type `&mut LedPin` and `&mut ButtonPin`, so we don't have
    //     // to move them back into the static after we use them
    //     let (mut led, mut button) = gpios;
    //     // Check if the interrupt source is from the pushbutton going from high-to-low.
    //     // Note: this will always be true in this example, as that is the only enabled GPIO interrupt source
    //     if button.interrupt_status(EdgeLow) {
    //         // toggle can't fail, but the embedded-hal traits always allow for it
    //         // we can discard the return value by assigning it to an unnamed variable
    //         let _ = led.set_high();

    //         // Our interrupt doesn't clear itself.
    //         // Do that now so we don't immediately jump back to this interrupt handler.
    //         button.clear_interrupt(EdgeLow);
    //     }
    //     else {
    //         let _ = led.set_low();
    //         // button.clear_interrupt(EdgeHigh);
    //     }
    // }
// }

