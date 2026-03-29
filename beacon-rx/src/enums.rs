use defmt::{ Format, Formatter, write };
use serde::{ Deserialize, Serialize };

#[derive(Clone, Copy, Serialize, Deserialize)]
#[repr(u8)]
pub enum PokemonKind {
    Bulbasaur,
    Oddish,
    Sandshrew,
    Pikachu,
    Magikarp,
    Tentacool,
    Onix,
    Machop,
    Snorunt,
    Sneasel,
    Lotad,
}

impl Format for PokemonKind {
    fn format(&self, fmt: Formatter) {
        let name = match self {
            PokemonKind::Bulbasaur => "Bulbasaur",
            PokemonKind::Oddish => "Oddish",
            PokemonKind::Sandshrew => "Sandshrew",
            PokemonKind::Pikachu => "Pikachu",
            PokemonKind::Magikarp => "Magikarp",
            PokemonKind::Tentacool => "Tentacool",
            PokemonKind::Onix => "Onix",
            PokemonKind::Machop => "Machop",
            PokemonKind::Snorunt => "Snorunt",
            PokemonKind::Sneasel => "Sneasel",
            PokemonKind::Lotad => "Lotad",
        };
        write!(fmt, "{}", name);
    }
}

#[derive(Clone, Copy)]
#[repr(u8)]
pub enum State {
    _Loading,
    Idle,
    _Listening,
}
