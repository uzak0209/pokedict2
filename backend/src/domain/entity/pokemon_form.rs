use crate::domain::valueobject::typeset::TypeSet;

/// ポケモンのフォルム情報
#[derive(Debug, Clone)]
pub struct PokemonForm {
    fullname: String,
    fullname_jp: String,
    form_id: i32,
    species_id: i32,
    typeset: TypeSet,
}

impl PokemonForm {
    /// 新しいポケモンフォルムを作成
    #[must_use]
    pub fn new(
        fullname: String,
        fullname_jp: String,
        form_id: i32,
        species_id: i32,
        typeset: TypeSet,
    ) -> Self {
        Self {
            fullname,
            fullname_jp,
            form_id,
            species_id,
            typeset,
        }
    }

    /// フルネーム（英語）を取得
    #[must_use]
    pub fn fullname(&self) -> &str {
        &self.fullname
    }

    /// フルネーム（日本語）を取得
    #[must_use]
    pub fn fullname_jp(&self) -> &str {
        &self.fullname_jp
    }

    /// フォームIDを取得
    #[must_use]
    pub fn form_id(&self) -> i32 {
        self.form_id
    }

    /// 種族IDを取得
    #[must_use]
    pub fn species_id(&self) -> i32 {
        self.species_id
    }

    /// タイプセットを取得
    #[must_use]
    pub fn typeset(&self) -> &TypeSet {
        &self.typeset
    }
}
