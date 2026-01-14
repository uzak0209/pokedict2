#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum Effectiveness {
    NoEffect = 0,
    Quarter = 1,
    Half = 2,
    Neutral = 4,
    Double = 8,
    Quadruple = 16,
}

impl Effectiveness {
    #[allow(clippy::cast_precision_loss)]
    pub fn multiplier(self) -> f32 {
        self as u32 as f32 / 4.0
    }
    pub fn get_multiplier_tocalc(self) -> u32 {
        self as u32
    }
    #[allow(clippy::panic)]
    pub fn from_multiplier(multiplier: u32) -> Self {
        match multiplier {
            0 => Self::NoEffect,
            1 => Self::Quarter,
            2 => Self::Half,
            4 => Self::Neutral,
            8 => Self::Double,
            16 => Self::Quadruple,
            _ => panic!("Invalid multiplier: {multiplier}. Must be 0, 1, 2, 4, 8, or 16"),
        }
    }
}
