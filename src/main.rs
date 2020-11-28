use rusb::GlobalContext;
use rusb::{DeviceHandle, Result, UsbContext};
use modular_bitfield::prelude::*;
use std::fmt::{self, Debug, Formatter};
use std::time::Duration;
use binread::BinRead;
use binread::BinReaderExt;

pub use rusb;

pub mod constants {
    pub const ADAPTER_VID: u16 = 0x057e;
    pub const ADAPTER_PID: u16 = 0x0337;
}

mod axis;
pub use axis::{SignedAxis, InvertedSignedAxis, UnsignedAxis};

fn main() -> Result<()> {
    //let mut handle = rusb::open_device_with_vid_pid(constants::ADAPTER_VID, constants::ADAPTER_PID).unwrap();
    //let endpoint = 0;

    //let has_kernel_driver = match handle.kernel_driver_active(endpoint) {
    //    Ok(true) => {
    //        handle.detach_kernel_driver(endpoint)?;
    //        true
    //    }
    //    _ => false,
    //};
    //println!("has kernel driver? {}", has_kernel_driver);

    //handle.claim_interface(endpoint).unwrap();

    //handle.write_interrupt(2, &[0x13], Duration::from_secs(0)).unwrap();
    //handle.write_interrupt(2, &[0x11, 1, 1, 1, 1], Duration::from_secs(0)).unwrap();

    //for i in 0..=60 {
    //    let mut buf = [0u8; 37];
    //    handle.read_interrupt(0x81, &mut buf, Duration::from_secs(0)).unwrap();
    //    let mut reader = std::io::Cursor::new(&buf[..]);
    //    
    //    use binread::BinReaderExt;
    //    let packet: Packet = reader.read_ne().unwrap();
    //    if let Packet::ControllerInfo { ports } = &packet {
    //        if ports[3].status == Status::Normal {
    //            dbg!(i, &packet);
    //            break
    //        }
    //    }
    //}

    //std::thread::sleep_ms(1000);

    //handle.write_interrupt(2, &[0x11, 0, 0, 0, 0], Duration::from_secs(0)).unwrap();
    //let mut buf = [0u8; 37];
    //handle.read_interrupt(0x81, &mut buf, Duration::from_secs(0)).unwrap();

    //// cleanup after use
    //handle.release_interface(endpoint)?;
    //if has_kernel_driver {
    //    handle.attach_kernel_driver(endpoint)?;
    //}

    let mut adapter = GcAdapter::from_usb().unwrap();
    adapter.refresh_inputs();
    dbg!(adapter.read_controllers());

    Ok(())
}

pub trait AdapterHardware {
    fn write_interrupt(&mut self, data: &[u8]);
    fn read_interrupt(&mut self, data: &mut [u8]);
}

#[derive(BitfieldSpecifier, Debug)]
pub enum ControllerType {
    Disconnected = 0,
    Normal = 1,
    Wavebird = 2,
    Invalid = 3,
}

impl Default for ControllerType {
    fn default() -> Self {
        Self::Disconnected
    }
}

#[bitfield]
#[derive(BinRead, Debug, Default, Copy, Clone, PartialEq)]
#[br(map = Self::from_bytes)]
pub struct ControllerStatus {
    pub unk: bool,
    pub unk2: bool,
    pub has_rumble: bool,
    pub unk3: bool,
    pub controller_type: ControllerType,
    padding: B2
}

#[bitfield]
#[derive(BinRead, Debug, Default)]
#[br(map = Self::from_bytes)]
pub struct Buttons {
    // TODO: rename
    pub a: bool,
    pub b: bool,
    pub x: bool,
    pub y: bool,

    pub dpad_left: bool,
    pub dpad_right: bool,
    pub dpad_down: bool,
    pub dpad_up: bool,

    pub start: bool,
    pub z: bool,
    pub right_trigger: bool,
    pub left_trigger: bool,
    pub padding: B4,
}

#[derive(BinRead, Debug, Default)]
pub struct Stick {
    pub x: SignedAxis,
    pub y: InvertedSignedAxis,
}

#[derive(BinRead, Debug, Default)]
pub struct Triggers {
    pub left: UnsignedAxis,
    pub right: UnsignedAxis,
}

#[derive(BinRead, Default)]
pub struct Controller {
    pub status: ControllerStatus,
    pub buttons: Buttons,
    pub left_stick: Stick,
    pub right_stick: Stick,
    pub triggers: Triggers,
}

impl Controller {
    pub fn connected(&self) -> bool {
        match self.status.controller_type() {
            ControllerType::Normal | ControllerType::Wavebird => true,
            ControllerType::Disconnected | ControllerType::Invalid => false,
        }
    }
}

impl Debug for Controller {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        if self.connected() {
            f.debug_struct("Controller")
                .field("status", &self.status)
                .field("buttons", &self.buttons)
                .field("left_stick", &self.left_stick)
                .field("right_stick", &self.right_stick)
                .field("triggers", &self.triggers)
                .finish()
        } else {
            f.write_str("Controller(Disconnected)")
        }
    }
}

#[derive(BinRead, Debug)]
enum Packet {
    #[br(magic = 0x21u8)]
    ControllerInfo {
        ports: [Controller; 4],
    },
    Unknown(u8),
}

impl Packet {
    fn parse(buffer: [u8; 37]) -> Self {
        let mut reader = std::io::Cursor::new(&buffer[..]);
        let packet: Packet = reader.read_ne().unwrap();

        packet
    }
}

pub struct LibUsbAdapter<Context: UsbContext> {
    handle: DeviceHandle<Context>,
    has_kernel_driver: bool,
}

impl<Context: UsbContext> LibUsbAdapter<Context> {
    pub fn from_usb_context(context: Context) -> Option<Self> {
        context.open_device_with_vid_pid(constants::ADAPTER_VID, constants::ADAPTER_PID)
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
        rusb::open_device_with_vid_pid(constants::ADAPTER_VID, constants::ADAPTER_PID)
            .map(Self::from_handle)
    }
}

impl<Context: UsbContext> AdapterHardware for LibUsbAdapter<Context> {
    fn read_interrupt(&mut self, data: &mut [u8]) {
        self.handle.read_interrupt(0x81, data, Duration::from_secs(0)).unwrap();
    }

    fn write_interrupt(&mut self, data: &[u8]) {
        self.handle.write_interrupt(2, data, Duration::from_secs(0)).unwrap();
    }
}

pub struct GcAdapter<T: AdapterHardware> {
    usb: T
}

impl<T: AdapterHardware> GcAdapter<T> {
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

    pub fn refresh_inputs(&mut self) {
        for _ in 0..10 {
            let mut buf = [0u8; 37];
            self.usb.read_interrupt(&mut buf);
        }
    }

    pub fn read_controllers(&mut self) -> [Controller; 4] {
        let mut buf = [0u8; 37];
        self.usb.read_interrupt(&mut buf);

        if let Packet::ControllerInfo { ports } = Packet::parse(buf) {
            ports
        } else {
            Default::default()
        }
    }
}

impl GcAdapter<LibUsbAdapter<GlobalContext>> {
    pub fn from_usb() -> Option<Self> {
        LibUsbAdapter::from_usb().map(|usb| GcAdapter { usb })
    }

}
