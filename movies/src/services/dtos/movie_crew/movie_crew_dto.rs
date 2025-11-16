use uuid::Uuid;



pub struct MovieCrewDto {
    pub movie_id: Uuid,
    pub person_id: Uuid,
    pub character_name: Option<String>,
    pub billing_order: Option<i32>,
}