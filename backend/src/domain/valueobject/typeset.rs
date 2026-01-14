use crate::domain::valueobject::pokemontype::PokemonType;

#[derive(Debug)]
pub struct TypeSet {
    primary: PokemonType,
    secondary: Option<PokemonType>,
}
