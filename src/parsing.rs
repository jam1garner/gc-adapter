use std::fmt::{self, Debug, Formatter};
use binread::{BinRead, BinReaderExt};
use modular_bitfield::prelude::*;

use crate::axis::*;

/// Type of controller connected (Disconnected, Normal, or Wavebird)
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

/// Status of controller for the given port
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

/// A collection of which buttons are pressed
///
/// **Note:** `right_trigger` and `left_trigger` refer to the buttons (i.e. the click when you hold
/// them all the way down). For the analog part of the triggers, see
/// [`Controller::triggers`](Controller::triggers).
#[bitfield]
#[derive(BinRead, Debug, Default)]
#[br(map = Self::from_bytes)]
pub struct Buttons {
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

/// An analog control stick. Can represent either the left or right stick.
#[derive(BinRead, Debug, Default)]
pub struct Stick {
    pub x: SignedAxis,
    pub y: SignedAxis,
}

impl Stick {
    /// Gets the raw stick values, where ~127.5 is center.
    pub fn raw(&self) -> (u8, u8) {
        (self.x.raw(), self.y.raw())
    }

    /// Gets the stick position as a normalized 2d vector. For higher accuracy, use
    /// [`coords_centered`](Stick::coords_centered) as it allows you to specifiy the 
    pub fn coords(&self) -> (f32, f32) {
        (self.x.float(), self.y.float())
    }

    /// Gets the stick position as a normalized 2d vector. The provided center should be obtained
    /// using the [`raw`](Stick::raw) method.
    pub fn coords_centered(&self, center: (u8, u8)) -> (f32, f32) {
        (self.x.float_centered(center.0), self.y.float_centered(center.1))
    }
}

/// The two analog triggers. For the digital portion, see [`Buttons::right_trigger`] and
/// [`Buttons::left_trigger`].
#[derive(BinRead, Debug, Default)]
pub struct Triggers {
    pub left: UnsignedAxis,
    pub right: UnsignedAxis,
}

/// A controller port: either disconnected or otherwise.
#[derive(BinRead, Default)]
pub struct Controller {
    pub status: ControllerStatus,
    pub buttons: Buttons,
    pub left_stick: Stick,
    pub right_stick: Stick,
    pub triggers: Triggers,
}

impl Controller {
    /// Check if the given controller is connected
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

/// A Gamecube Controller adapter USB payload 
#[derive(BinRead, Debug)]
pub enum Packet {
    #[br(magic = 0x21u8)]
    ControllerInfo {
        ports: [Controller; 4],
    },
    Unknown(u8),
}

impl Packet {
    /// Parse a packet from a 37 byte buffer
    pub fn parse(buffer: [u8; 37]) -> Self {
        let mut reader = std::io::Cursor::new(&buffer[..]);
        let packet: Packet = reader.read_ne().unwrap();

        packet
    }
}
