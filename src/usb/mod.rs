/// A libusb implementation of the adapter hardware interface
mod libusb;
pub use libusb::*;

/// A trait representing a struct which provides access to a limited set of
/// USB operations
pub trait AdapterHardware {
    fn write_interrupt(&mut self, data: &[u8]);
    fn read_interrupt(&mut self, data: &mut [u8]);
}
