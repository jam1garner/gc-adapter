use binread::BinRead;

/// An unsigned axis, representing a centered value, such as a joystick axis.
#[derive(BinRead, Debug, Default)]
pub struct SignedAxis(u8);

impl SignedAxis {
    pub fn from_raw(val: u8) -> Self {
        Self(val)
    }

    pub fn raw(&self) -> u8 {
        self.0
    }

    /// Return axis as an f32 in the range of [-1.0, 1.0]
    ///
    /// **Note:** the gamecube doesn't have a true center point. To have the controller properly
    /// centered, use [`SignedAxis::float_centered`] and provide a centerpoint (typically pulled
    /// from when the controller is first registered or on recalibration).
    pub fn float(&self) -> f32 {
        ((self.0 as f32) - 127.5) / 127.5
    }

    /// Return axis as an f64 in the range of [-1.0, 1.0]
    ///
    /// **Note:** You likely do not want the additional precision.
    pub fn double(&self) -> f64 {
        ((self.0 as f64) - 127.5) / 127.5
    }

    /// Return axis as an `f32` in the range of [-1.0, 1.0] centered around a given raw value.
    /// It is recommended the provided center value is pulled on start and on recalibration on a
    /// per-axis basis as most controllers have slight variation in their center.
    pub fn float_centered(&self, center: u8) -> f32 {
        let center_offset = ((self.0 as i16) - (center as i16)) as f32;
        let scale = if self.0 > center { (u8::MAX - center).max(1) } else { center } as f32;

        center_offset / scale
    }

    /// Return axis as an `f64` in the range of [-1.0, 1.0] centered around a given raw value.
    /// It is recommended the provided center value is pulled on start and on recalibration on a
    /// per-axis basis as most controllers have slight variation in their center.
    pub fn double_centered(&self, center: u8) -> f64 {
        let center_offset = ((self.0 as i16) - (center as i16)) as f64;
        let scale = if self.0 > center { (u8::MAX - center).max(1) } else { center } as f64;

        center_offset / scale
    }
}

/// An unsigned axis, representing a positive or zero value.
#[derive(BinRead, Debug, Default)]
pub struct UnsignedAxis(u8);

impl UnsignedAxis {
    pub fn from_raw(val: u8) -> Self {
        Self(val)
    }

    pub fn raw(&self) -> u8 {
        self.0
    }

    /// Return axis as an `f32` in the range of [-1.0, 1.0]
    pub fn float(&self) -> f32 {
        (self.0 as f32) / 255.0
    }

    /// Return axis as an `f64` in the range of [-1.0, 1.0]
    ///
    /// **Note:** You likely do not want the additional precision.
    pub fn double(&self) -> f64 {
        (self.0 as f64) / 255.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    //#[test]
    //fn test_signed() {
    //    assert_eq!(SignedAxis::from_raw(127).float(), 1.0);
    //    assert_eq!(SignedAxis::from_raw(-128).float(), -1.0);
    //    assert_eq!(SignedAxis::from_raw(0).float(), 0.0);
    //    assert_eq!(SignedAxis::from_raw(127).double(), 1.0);
    //    assert_eq!(SignedAxis::from_raw(-128).double(), -1.0);
    //    assert_eq!(SignedAxis::from_raw(0).double(), 0.0);
    //}

    //#[test]
    //fn test_inverted() {
    //    assert_eq!(InvertedSignedAxis::from_raw(127).float(), -1.0);
    //    assert_eq!(InvertedSignedAxis::from_raw(-128).float(), 1.0);
    //    assert_eq!(InvertedSignedAxis::from_raw(0).float(), 0.0);
    //    assert_eq!(InvertedSignedAxis::from_raw(127).double(), -1.0);
    //    assert_eq!(InvertedSignedAxis::from_raw(-128).double(), 1.0);
    //    assert_eq!(InvertedSignedAxis::from_raw(0).double(), 0.0);
    //}
    //
    //#[test]
    //fn test_unsigned() {
    //    assert_eq!(UnsignedAxis::from_raw(255).float(), 1.0);
    //    assert_eq!(UnsignedAxis::from_raw(0).float(), 0.0);
    //    assert_eq!(UnsignedAxis::from_raw(255).double(), 1.0);
    //    assert_eq!(UnsignedAxis::from_raw(0).double(), 0.0);
    //}
}
