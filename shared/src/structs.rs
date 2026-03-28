use super::enums::Biome;
use serde::{ Deserialize, Serialize };
use defmt::{ Format, Formatter, write };

#[derive(Serialize, Deserialize, Debug)]
pub struct BiomePacket {
    pub biome: Biome,
}
impl BiomePacket {
    pub fn new(biome: Biome) -> Self {
        Self {
            biome,
        }
    }
}

impl Format for BiomePacket {
    fn format(&self, fmt: Formatter) {
        write!(fmt, "BiomePacket {{ biome: {} }}", self.biome);
    }
}
