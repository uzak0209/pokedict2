use crate::domain::entity::pokemon_form::PokemonForm;
use crate::domain::valueobject::teamname::TeamName;
use uuid::Uuid;

#[allow(clippy::struct_field_names)]
struct Team {
    team_id: Uuid,
    owner_id: Uuid,
    team_name: TeamName,
    pokemon1: Option<PokemonForm>,
    pokemon2: Option<PokemonForm>,
    pokemon3: Option<PokemonForm>,
    pokemon4: Option<PokemonForm>,
    pokemon5: Option<PokemonForm>,
    pokemon6: Option<PokemonForm>,
}
