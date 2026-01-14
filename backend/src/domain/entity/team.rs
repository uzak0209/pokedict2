use uuid::Uuid;
use crate::domain::valueobject::teamname::TeamName;
use crate::domain::entity::pokemon_form::{self, PokemonForm};
#[derive(Debug)]
struct Team {
    owner_id:Uuid,
    team_name:,

    pokemon1: Option<PokemonForm>,
    pokemon2: Option<PokemonForm>,
    pokemon3: Option<PokemonForm>,
    pokemon4: Option<PokemonForm>,
    pokemon5: Option<PokemonForm>,
    pokemon6: Option<PokemonForm>,
}
