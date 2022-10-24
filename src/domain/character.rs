use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Character {
    pub id: i64,
    pub name: String,
    pub status: String,
    pub species: String,
    #[serde(rename = "type")]
    pub character_type: String,

    pub gender: String,
    pub image: String,
    pub episode: Vec<String>,
    pub url: String,
    pub created: String,
}

/* impl Character {
    pub fn new(
        id: i64,
        name: String,
        status: String,
        species: String,
        character_type: String,
        gender: String,
        image: String,
        episode: Vec<String>,
        url: String,
        created: String,
    ) -> Self {
        Self {
            id,
            name,
            status,
            species,
            character_type,
            gender,
            image,
            episode,
            url,
            created,
        }
    }
} */
