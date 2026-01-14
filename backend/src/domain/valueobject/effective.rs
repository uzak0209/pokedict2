#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Effectiveness {
    NoEffect,  // 0
    Quarter,   // 1
    Half,      // 2
    Neutral,   // 4
    Double,    // 8
    Quadruple, // 16
}

impl Effectiveness {
    pub fn multiplier(self) -> f32 {
        self as u32 as f32 / 4.0
    }
    pub fn get_multiplier_tocalc(self) -> u32 {
        self as u32
    }
    pub fn from_multiplier(multiplier: u32) -> Self {
        match multiplier {
            0 => Effectiveness::NoEffect,
            1 => Effectiveness::Quarter,
            2 => Effectiveness::Half,
            4 => Effectiveness::Neutral,
            8 => Effectiveness::Double,
            16 => Effectiveness::Quadruple,
            _ => panic!("Invalid multiplier"),
        }
    }
}
