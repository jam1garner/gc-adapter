use binread::BinRead;

#[derive(BinRead, Debug, Default)]
pub struct SignedAxis(i8);

impl SignedAxis {
    pub fn from_raw(val: i8) -> Self {
        Self(val)
    }

    pub fn raw(&self) -> i8 {
        self.0
    }

    pub fn float(&self) -> f32 {
        ((self.0) as f32) / if self.0.is_negative() { 128.0 } else { 127.0 }
    }

    pub fn double(&self) -> f64 {
        ((self.0) as f64) / if self.0.is_negative() { 128.0 } else { 127.0 }
    }
}

#[derive(BinRead, Debug, Default)]
pub struct InvertedSignedAxis(i8);

impl InvertedSignedAxis {
    pub fn from_raw(val: i8) -> Self {
        Self(val)
    }

    pub fn raw(&self) -> i8 {
        self.0
    }

    pub fn float(&self) -> f32 {
        -((self.0) as f32) / if self.0.is_negative() { 128.0 } else { 127.0 }
    }

    pub fn double(&self) -> f64 {
        -((self.0) as f64) / if self.0.is_negative() { 128.0 } else { 127.0 }
    }
}

#[derive(BinRead, Debug, Default)]
pub struct UnsignedAxis(u8);

impl UnsignedAxis {
    pub fn from_raw(val: u8) -> Self {
        Self(val)
    }

    pub fn raw(&self) -> u8 {
        self.0
    }

    pub fn float(&self) -> f32 {
        (self.0 as f32) / 255.0
    }

    pub fn double(&self) -> f64 {
        (self.0 as f64) / 255.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_signed() {
        assert_eq!(SignedAxis::from_raw(127).float(), 1.0);
        assert_eq!(SignedAxis::from_raw(-128).float(), -1.0);
        assert_eq!(SignedAxis::from_raw(0).float(), 0.0);
        assert_eq!(SignedAxis::from_raw(127).double(), 1.0);
        assert_eq!(SignedAxis::from_raw(-128).double(), -1.0);
        assert_eq!(SignedAxis::from_raw(0).double(), 0.0);
    }

    #[test]
    fn test_inverted() {
        assert_eq!(InvertedSignedAxis::from_raw(127).float(), -1.0);
        assert_eq!(InvertedSignedAxis::from_raw(-128).float(), 1.0);
        assert_eq!(InvertedSignedAxis::from_raw(0).float(), 0.0);
        assert_eq!(InvertedSignedAxis::from_raw(127).double(), -1.0);
        assert_eq!(InvertedSignedAxis::from_raw(-128).double(), 1.0);
        assert_eq!(InvertedSignedAxis::from_raw(0).double(), 0.0);
    }
    
    #[test]
    fn test_unsigned() {
        assert_eq!(UnsignedAxis::from_raw(255).float(), 1.0);
        assert_eq!(UnsignedAxis::from_raw(0).float(), 0.0);
        assert_eq!(UnsignedAxis::from_raw(255).double(), 1.0);
        assert_eq!(UnsignedAxis::from_raw(0).double(), 0.0);
    }
}
