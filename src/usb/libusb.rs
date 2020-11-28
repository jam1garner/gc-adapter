use rusb::{DeviceHandle, UsbContext, GlobalContext};
use std::time::Duration;

use super::AdapterHardware;
use crate::constants::*;

#[cfg(doc)]
use crate::GcAdapter;

/// Adapter interface for libusb to provide USB access for the gamecube adapter.
///
/// The suggested interface for using this is [`GcAdapter::from_usb`](GcAdapter::from_usb). This
/// should only be used if you want to provide your own [`rusb::UsbContext`](UsbContext)
pub struct LibUsbAdapter<Context: UsbContext> {
    handle: DeviceHandle<Context>,
    has_kernel_driver: bool,
}

impl<Context: UsbContext> LibUsbAdapter<Context> {
    pub fn from_usb_context(context: Context) -> Option<Self> {
        context.open_device_with_vid_pid(ADAPTER_VID, ADAPTER_PID)
            .map(Self::from_handle)
    }

    pub fn from_handle(mut handle: DeviceHandle<Context>) -> Self {
        let endpoint = 0;
        let has_kernel_driver = handle.kernel_driver_active(endpoint).unwrap_or(false);

        if has_kernel_driver {
            handle.detach_kernel_driver(endpoint).unwrap();
        }

        handle.claim_interface(endpoint).unwrap();
        let mut adapter = Self { handle, has_kernel_driver };

        adapter.write_interrupt(&[0x13]);

        adapter
    }
}

impl<Context: UsbContext> Drop for LibUsbAdapter<Context> {
    fn drop(&mut self) {
        let endpoint = 0;
        let _ = self.handle.release_interface(endpoint);
        if self.has_kernel_driver {
            let _ = self.handle.attach_kernel_driver(endpoint);
        }
    }
}

impl LibUsbAdapter<GlobalContext> {
    pub fn from_usb() -> Option<Self> {
        rusb::open_device_with_vid_pid(ADAPTER_VID, ADAPTER_PID)
            .map(Self::from_handle)
    }
}

impl<Context: UsbContext> super::AdapterHardware for LibUsbAdapter<Context> {
    fn read_interrupt(&mut self, data: &mut [u8]) {
        self.handle.read_interrupt(0x81, data, Duration::from_secs(0)).unwrap();
    }

    fn write_interrupt(&mut self, data: &[u8]) {
        self.handle.write_interrupt(2, data, Duration::from_secs(0)).unwrap();
    }
}
