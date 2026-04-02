use serde::{ Serialize, Deserialize };
use defmt::{ Format, Formatter, write };

#[derive(Debug)]
pub enum AuthError {
    InvalidToken,
}
impl Format for AuthError {
    fn format(&self, fmt: Formatter) {
        match self {
            AuthError::InvalidToken => write!(fmt, "Invalid packet token"),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone, Copy)]
#[repr(u8)]
pub enum Biome {
    Wilderness,
    Forest,
    Desert,
    Plains,
    Ocean,
    Mountain,
    Snow,
    Swamp,
}

impl Format for Biome {
    fn format(&self, fmt: Formatter) {
        match self {
            Biome::Wilderness => write!(fmt, "Wilderness"),
            Biome::Forest => write!(fmt, "Forest"),
            Biome::Desert => write!(fmt, "Desert"),
            Biome::Plains => write!(fmt, "Plains"),
            Biome::Ocean => write!(fmt, "Ocean"),
            Biome::Mountain => write!(fmt, "Mountain"),
            Biome::Snow => write!(fmt, "Snow"),
            Biome::Swamp => write!(fmt, "Swamp"),
        }
    }
}
