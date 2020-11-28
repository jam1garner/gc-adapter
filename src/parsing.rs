use std::fmt::{self, Debug, Formatter};
use binread::{BinRead, BinReaderExt};
use modular_bitfield::prelude::*;

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
pub enum Packet {
    #[br(magic = 0x21u8)]
    ControllerInfo {
        ports: [Controller; 4],
    },
    Unknown(u8),
}

impl Packet {
    pub fn parse(buffer: [u8; 37]) -> Self {
        let mut reader = std::io::Cursor::new(&buffer[..]);
        let packet: Packet = reader.read_ne().unwrap();

        packet
    }
}
