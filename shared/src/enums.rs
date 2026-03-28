use serde::{ Serialize, Deserialize };
use core::{ prelude::rust_2024::derive };
use defmt::{ Format, Formatter, write };

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone, Copy)]
pub enum Biome {
    Wilderness,
    Forest,
    Desert,
}

impl Format for Biome {
    fn format(&self, fmt: Formatter) {
        match self {
            Biome::Wilderness => write!(fmt, "Wilderness"),
            Biome::Forest => write!(fmt, "Forest"),
            Biome::Desert => write!(fmt, "Desert"),
        }
    }
}
