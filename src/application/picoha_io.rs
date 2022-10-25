

// ============================================================================

/// Store all the usefull objects for the application

// ============================================================================

/// Number of io on the rp2040
pub const NB_IO_RP2040: usize = 27;
pub const MAX_IO_INDEX_RP2040: usize = 28;

/// Size of the answer buffer, used to convert answer message into a json string
pub const SIZE_ANS_BUFFER: usize = 400;

// ============================================================================

use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug)]
struct Command {
    /// 0 set mode / 1 write val / 2 read val / 10 test
    cod: u8,
    /// id of the pin (X => gpioX)
    pin: u8,
    /// if cmd = 0 mode  { 0 mode input_pullup, 1 mode input_pulldown, 2 mode output }
    /// if cmd = 1 write { the io value 0 or 1 }
    /// if cmd = 2 read  { none }
    arg: u8,
}

#[derive(Serialize, Debug)]
struct Answer<'a> {
    /// Status code
    sts: u8,
    /// id of the pin (X => gpioX)
    pin: u8,
    ///
    arg: u8,
    /// Text message
    msg: &'a str,
}

// ============================================================================

enum AnsStatus {
    Ok = 0,
    Error = 1,
}



// HAL
use embedded_hal::digital::v2::OutputPin;
use rp_pico::hal;
use rp_pico::hal::gpio::{Pin, PinId, FloatingInput};

// USB crates
use usb_device::prelude::UsbDevice;

// USB Communications Class Device support
use usbd_serial::SerialPort;

use super::buffer::UsbBuffer;

use core::panic::PanicInfo;

// Algos
use numtoa::NumToA;
use arrayvec::ArrayString;
pub struct PicohaIo {
    /// To manage delay
    delay: cortex_m::delay::Delay,

    /// Objects to control io of the board
    dyn_ios: [Pin<PinId::DYN, FloatingInput>; NB_IO_RP2040],

    /// The USB Device Driver (shared with the interrupt).
    usb_device: &'static mut UsbDevice<'static, hal::usb::UsbBus>,

    usb_serial: &'static mut SerialPort<'static, hal::usb::UsbBus>,

    /// Buffer to store incomming serial command
    usb_buffer: UsbBuffer<1024>,

    /// Buffer to prepare answer message
    ans_buffer: [u8; SIZE_ANS_BUFFER]
}

// ============================================================================

/// Implementation of the App
impl PicohaIo {

    // ------------------------------------------------------------------------

    /// Application intialization
    pub fn new(
        delay: cortex_m::delay::Delay,
        dyn_ios: [Pin; NB_IO_RP2040],
        usb_dev: &'static mut UsbDevice<'static, hal::usb::UsbBus>,
        usb_ser: &'static mut SerialPort<'static, hal::usb::UsbBus>,
    ) -> Self {
        Self {
            delay: delay,
            dyn_ios: dyn_ios,
            usb_device: usb_dev,
            usb_serial: usb_ser,
            usb_buffer: UsbBuffer::new(),
            ans_buffer: [0; SIZE_ANS_BUFFER]
        }
    }

    // ------------------------------------------------------------------------

    /// To send a message back to the user
    ///
    fn send_answer(&mut self, ans: &Answer) {
        // Convert the message into a json string
        let size = serde_json_core::to_slice(&ans, &mut self.ans_buffer).unwrap();

        // Send message on the serial port
        self.usb_serial.write(&self.ans_buffer[0..size]).unwrap();
        self.usb_serial.write(b"\n").unwrap();
    }

    // ------------------------------------------------------------------------

    /// To configure the  mode of the io
    ///
    fn process_set_io_mode(&mut self, cmd: &Command) {
        // Get io from cmd
        let io = &mut self.dyn_ios[cmd.pin as usize];

        // error flag
        let mut error: bool = false;

        // Process the argument
        match cmd.arg {
            0 => {
                io.into_pull_up_input();
            }
            1 => {
                io.into_pull_down_input();
            }
            2 => {
                io.into_readable_output();
            }
            default => {
                error = true;
                let mut num = [0u8; 20];
                let mut txt = ArrayString::<100>::new();
                txt.push_str("Unknown arg value for set io mode command (");
                txt.push_str(default.numtoa_str(10, &mut num));
                txt.push_str(")");
                self.send_answer(&Answer{ sts: AnsStatus::Error as u8, pin: 0, arg: 0, msg: &txt });
            }
        }

        // Send ack
        if !error
        {
            self.send_answer(&Answer{ sts: AnsStatus::Ok as u8, pin: 0, arg: 0, msg: "" });
        }
    }

    // ------------------------------------------------------------------------

    /// To write a value on the io
    ///
    fn process_write_io(&mut self, cmd: &Command) {
        // Get io from cmd
        let io = &mut self.dyn_ios[cmd.pin];

        // error flag
        let mut error: bool = false;

        // Process the argument
        match cmd.arg {
            0 => {
                io.set_low().unwrap();
            }
            1 => {
                io.set_high().unwrap();
            }
            default => {
                error = true;
                let mut num = [0u8; 20];
                let mut txt = ArrayString::<100>::new();
                txt.push_str("Unknown arg value for write command (");
                txt.push_str(default.numtoa_str(10, &mut num));
                txt.push_str(")");
                self.send_answer(&Answer{ sts: AnsStatus::Error as u8, pin: 0, arg: 0, msg: &txt });
            }
        }

        // Send ack
        if !error
        {
            self.send_answer(&Answer{ sts: AnsStatus::Ok as u8, pin: 0, arg: 0, msg: "" });
        }
    }

    // ------------------------------------------------------------------------

    /// To read an io
    ///
    fn process_read_io(&mut self, cmd: &Command) {
        // Get io from cmd

        // if(io.is_high().unwrap()) {

        // } else {

        // }
    }

    // ------------------------------------------------------------------------

    /// Main loop of the main task of the application
    ///
    pub fn run_forever(&mut self) -> ! {
        let mut cmd_buffer = [0u8; 1024];

        loop {

            match self.usb_buffer.get_command(&mut cmd_buffer) {
                None => {}
                Some(cmd_end_index) => {
                    let cmd_slice_ref = &cmd_buffer[0..cmd_end_index];

                    match serde_json_core::de::from_slice::<Command>(cmd_slice_ref) {

                        // Process parsing error
                        Err(_e) => {
                            self.send_answer(&Answer {
                                sts: AnsStatus::Error as u8,
                                pin: 0,
                                arg: 0,
                                msg: "error parsing command",
                            });
                        }

                        Ok(cmd) => {
                            let data = &cmd.0;
                            match data.cod {
                                0 => {
                                    self.process_set_io_mode(data);
                                }

                                1 => {
                                    self.process_write_io(data);
                                }

                                2 => {
                                    self.process_read_io(data);
                                }

                                10 => {
                                    // version maj.min
                                    // pin = v-maj
                                    // arg = v-min
                                    self.send_answer(&Answer{ sts: AnsStatus::Ok as u8, pin: 0, arg: 1, msg: "" });
                                }

                                default => {
                                    let mut num = [0u8; 20];
                                    let mut txt = ArrayString::<100>::new();
                                    txt.push_str("Unknown arg value for command (");
                                    txt.push_str(default.numtoa_str(10, &mut num));
                                    txt.push_str(")");
                                    self.send_answer(&Answer{ sts: AnsStatus::Error as u8, pin: 0, arg: 0, msg: &txt });
                                }
                            }
                        }
                    }
                }
            }
        }
    }

     /// Panic handler implementation for the application
     pub fn panic_handler(&mut self, _info: &PanicInfo) -> ! {
        let mut tmp_buf = [0u8; 20];

        self.usb_serial.write(b"{\"log\":\"").ok();
        self.usb_serial.write(b"PANIC! => ").ok();
        self.usb_serial
            .write(_info.location().unwrap().file().as_bytes())
            .ok();
        self.usb_serial.write(b":").ok();
        self.usb_serial
            .write(_info.location().unwrap().line().numtoa(10, &mut tmp_buf))
            .ok();
        self.usb_serial.write(b"\"}\r\n").ok();
        loop {
            // self.led_pin.set_high().ok();
            // self.delay.delay_ms(100);
            // self.led_pin.set_low().ok();
            // self.delay.delay_ms(100);
        }
    }

    pub fn usbctrl_irq(&mut self) {
        // Poll the USB driver with all of our supported USB Classes
        if self.usb_device.poll(&mut [self.usb_serial]) {
            // Buffer to read the serial port
            let mut serial_buffer = [0u8; 512];
            match self.usb_serial.read(&mut serial_buffer) {
                Err(_e) => {
                    // Do nothing
                }
                Ok(0) => {
                    // Do nothing
                }
                Ok(count) => {
                    self.usb_buffer.load(&serial_buffer, count);
                }
            }
        }

    }

}