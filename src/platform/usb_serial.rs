// ============================================================================

// USB crates
use rp_pico::hal::usb::UsbBus;
use usb_device::class_prelude::UsbBusAllocator;
use usb_device::prelude::UsbDevice;
use usb_device::prelude::UsbDeviceBuilder;
use usb_device::prelude::UsbVidPid;

// USB Communications Class Device support
use usbd_serial::SerialPort;

// ============================================================================

/// Create a USB device with a fake VID and PID
pub fn init_usb_device(usb_bus: &'static UsbBusAllocator<UsbBus>) -> UsbDevice<UsbBus> {
    UsbDeviceBuilder::new(
        &usb_bus,
        UsbVidPid(super::config::USB_MANUFACTURER_ID, super::config::USB_PRODUCT_ID),
    )
    .manufacturer(super::config::USB_MANUFACTURER_NAME)
    .product(super::config::USB_PRODUCT_NAME)
    .serial_number(super::config::USB_SERIAL_NUMBER)
    .device_class(2) // from: https://www.usb.org/defined-class-codes
    .build()
}

// ============================================================================

/// Intialize the usb device object
pub fn init_usb_serial(usb_bus: &'static UsbBusAllocator<UsbBus>) -> SerialPort<UsbBus> {
    return SerialPort::new(&usb_bus);
}

// ============================================================================
