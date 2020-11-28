use rusb::{Result, GlobalContext};
pub use rusb;

pub mod constants {
    pub const ADAPTER_VID: u16 = 0x057e;
    pub const ADAPTER_PID: u16 = 0x0337;
}

mod parsing;
pub use parsing::*;

mod axis;
pub use axis::{SignedAxis, InvertedSignedAxis, UnsignedAxis};

mod usb;
pub use usb::*;

fn main() -> Result<()> {
    let mut adapter = GcAdapter::from_usb().unwrap();
    adapter.refresh_inputs();
    dbg!(adapter.read_controllers());

    Ok(())
}

pub struct GcAdapter<T: AdapterHardware> {
    usb: T
}

impl<T: AdapterHardware> GcAdapter<T> {
    /// Set rumble.
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

impl<T: AdapterHardware> Drop for GcAdapter<T> {
    fn drop(&mut self) {
        self.set_rumble([false; 4]);
    }
}

impl GcAdapter<LibUsbAdapter<GlobalContext>> {
    pub fn from_usb() -> Option<Self> {
        LibUsbAdapter::from_usb().map(|usb| GcAdapter { usb })
    }
}
