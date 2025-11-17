
pub struct NewGenreDto {
    pub name: String,
}

impl From<crate::data::models::genre::NewGenre> for NewGenreDto {
    fn from(genre: crate::data::models::genre::NewGenre) -> Self {
        NewGenreDto {
            name: genre.name,
        }
    }
}

impl From<NewGenreDto> for crate::data::models::genre::NewGenre {
    fn from(dto: NewGenreDto) -> Self {
        crate::data::models::genre::NewGenre {
            name: dto.name,
        }
    }
}
