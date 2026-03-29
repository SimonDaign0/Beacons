use crate::enums::PokemonKind;
pub struct Pokemon {
    pub kind: PokemonKind,
    //type, attacks...
}
impl Pokemon {
    pub fn new(kind: PokemonKind) -> Self {
        Self {
            kind,
        }
    }
}
