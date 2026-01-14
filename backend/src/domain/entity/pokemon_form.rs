use crate::domain::valueobject::typeset::TypeSet;
#[derive(Debug)]
pub struct PokemonForm {
    fullname: String,
    fullname_jp: String,
    form_id: i32,
    spacies_id: i32,
    typeset: TypeSet,
}
