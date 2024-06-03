#[derive(serde::Serialize, serde::Deserialize)]
pub struct Author {
    pub name: String,
    pub bio: String,
    pub social: Vec<std::collections::HashMap<String, String>>,
}
