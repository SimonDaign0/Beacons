use crate::enums::{ PokemonKind, State };
use crate::structs::Pokemon;
use shared::enums::Biome;
use esp_hal::rng::Rng;

//Pokemon per biome Arrays
pub static WILDERNESS: &[PokemonKind] = &[PokemonKind::Pikachu, PokemonKind::Oddish];
pub static FOREST: &[PokemonKind] = &[PokemonKind::Bulbasaur, PokemonKind::Oddish];
pub static DESERT: &[PokemonKind] = &[PokemonKind::Sandshrew, PokemonKind::Onix];
pub static PLAINS: &[PokemonKind] = &[PokemonKind::Pikachu, PokemonKind::Machop];
pub static OCEAN: &[PokemonKind] = &[PokemonKind::Magikarp, PokemonKind::Tentacool];
pub static MOUNTAIN: &[PokemonKind] = &[PokemonKind::Onix, PokemonKind::Machop];
pub static SNOW: &[PokemonKind] = &[PokemonKind::Snorunt, PokemonKind::Sneasel];
pub static SWAMP: &[PokemonKind] = &[PokemonKind::Lotad, PokemonKind::Oddish];

//TODO further logic
pub struct StateMachine {
    rng: Rng,
    _state: State,
}

impl StateMachine {
    pub fn init() -> Self {
        Self {
            rng: Rng::new(),
            _state: State::Idle,
        }
    }

    pub fn generate_pokemon(&self, biome: Biome) -> Pokemon {
        let pokemon_list = get_pokemon_list(biome);
        let index = (self.rng.random() as usize) % pokemon_list.len();
        let kind = pokemon_list[index];
        let new_pokemon = Pokemon::new(kind);
        new_pokemon
    }
}

pub fn get_pokemon_list(biome: Biome) -> &'static [PokemonKind] {
    match biome {
        Biome::Wilderness => WILDERNESS,
        Biome::Forest => FOREST,
        Biome::Desert => DESERT,
        Biome::Plains => PLAINS,
        Biome::Ocean => OCEAN,
        Biome::Mountain => MOUNTAIN,
        Biome::Snow => SNOW,
        Biome::Swamp => SWAMP,
    }
}
