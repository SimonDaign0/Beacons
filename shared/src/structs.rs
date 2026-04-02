use super::enums::{ Biome, AuthError };
use serde::{ Deserialize, Serialize };
use defmt::{ Format, Formatter, write };
static PACKET_TOKEN: [u8; 3] = [0xa7, 0x3e, 0x2];

#[derive(Serialize, Deserialize, Debug)]
pub struct BiomePacket {
    tkn: [u8; 3],
    pub biome: Biome,
}
impl BiomePacket {
    pub fn new(biome: Biome) -> Self {
        Self {
            tkn: PACKET_TOKEN,
            biome,
        }
    }
    pub fn authenticate(self) -> Result<Biome, AuthError> {
        if self.tkn == PACKET_TOKEN { Ok(self.biome) } else { Err(AuthError::InvalidToken) }
    }
}

impl Format for BiomePacket {
    fn format(&self, fmt: Formatter) {
        write!(fmt, "BiomePacket {{ biome: {} }}", self.biome);
    }
}
