#![cfg_attr(not(feature = "libusb"), no_std)]
//! A library for working with the Nintendo Gamecube controller adapter.
//!
//! **Supports:**
//!
//! * Official Nintendo Gamecube Controller Adapter for Wii U and Switch
//! * Mayflash Gamecube Controller Adapter (in "Wii U/Switch" mode)
//! * Other 3rd party adapters (untested)
//!
//! ## Example
//!
//! ```rust
//! use gc_adapter::GcAdapter;
//!
//! // get adapter from global context
//! let mut adapter = GcAdapter::from_usb().unwrap();
//!
//! // refresh inputs to ensure they are up to date
//! adapter.refresh_inputs();
//!
//! // read and display all controller ports
//! dbg!(adapter.read_controllers());
//!
//! // enable rumble for only ports 4
//! adapter.set_rumble([false, false, false, true]);
//! 
//! std::thread::sleep(std::time::Duration::from_millis(100));
//!
//! // on drop all rumble will be disabled and the USB connection
//! // will be cleaned up
//! let _ = adapter;
//! ```

#[cfg(feature = "libusb")]
pub use rusb;

/// Vendor and Product IDs for adapter
pub mod constants {
    pub const ADAPTER_VID: u16 = 0x057e;
    pub const ADAPTER_PID: u16 = 0x0337;
}

/// Packet parsing code
mod parsing;
pub use parsing::*;

/// Types for represent various axis types
mod axis;
pub use axis::{SignedAxis, UnsignedAxis};

/// Types/Traits for handling USB connections
mod usb;
pub use usb::*;

/// A connection to a gamecube adapter
pub struct GcAdapter<T: AdapterHardware> {
    usb: T
}

impl<T: AdapterHardware> GcAdapter<T> {
    /// Set rumble for all 4 ports at once
    pub fn set_rumble(&mut self, ports: [bool; 4]) {
        let payload = [
            0x11,
            ports[0] as u8,
            ports[1] as u8,
            ports[2] as u8,
            ports[3] as u8
        ];

        self.usb.write_interrupt(&payload[..]);
        let mut buf = [0u8; 37];
        self.usb.read_interrupt(&mut buf);
    }

    /// Refresh the set of inputs by polling the adapter 10 times, this ensures the results of
    /// [`read_controllers`](GcAdapter::read_controllers) is current.
    pub fn refresh_inputs(&mut self) {
        for _ in 0..10 {
            let mut buf = [0u8; 37];
            self.usb.read_interrupt(&mut buf);
        }
    }

    /// Read the current state of all the controllers plugged into the adapter
    pub fn read_controllers(&mut self) -> [Controller; 4] {
        let mut buf = [0u8; 37];
        self.usb.read_interrupt(&mut buf);

        if let Packet::ControllerInfo { ports } = Packet::parse(buf) {
            ports
        } else {
            Default::default()
        }
    }

    /// Creates a new `GcAdapter` from a USB connection that implements
    /// [`AdapterHardware`](AdapterHardware).
    pub fn new(usb: T) -> Self {
        Self { usb }
    }
}

#[cfg(feature = "libusb")]
impl GcAdapter<LibUsbAdapter<rusb::GlobalContext>> {
    /// Get an adapter from the libusb global context.
    /// Returns false if no adapter is plugged in.
    pub fn from_usb() -> Option<Self> {
        LibUsbAdapter::from_usb().map(Self::new)
    }
}

impl<T: AdapterHardware> Drop for GcAdapter<T> {
    fn drop(&mut self) {
        // clear rumble on drop
        self.set_rumble([false; 4]);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[cfg(feature = "libusb")]
    fn test_display_controllers() {
        // get adapter from global context
        let mut adapter = GcAdapter::from_usb().unwrap();

        // refresh inputs to ensure they are up to date
        adapter.refresh_inputs();

        // read and display all controller ports
        dbg!(adapter.read_controllers());

        dbg!(adapter.read_controllers()[3].left_stick.coords());
    }
}
