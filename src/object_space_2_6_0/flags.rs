#[derive(Hash, Eq, PartialEq, Debug, Deserialize)]
pub struct Flags {
    pub wb_protected: Option<bool>,
    pub old: Option<bool>,
    pub uncollectible: Option<bool>,
    pub marked: Option<bool>,
    pub marking: Option<bool>
}
