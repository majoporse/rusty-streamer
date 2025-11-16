use uuid::Uuid;


pub struct GenreDto {
    pub id: Uuid,
    pub name: String,
}

impl From<crate::data::models::genre::Genre> for GenreDto {
    fn from(genre: crate::data::models::genre::Genre) -> Self {
        GenreDto {
            id: genre.id,
            name: genre.name,
        }
    }
}