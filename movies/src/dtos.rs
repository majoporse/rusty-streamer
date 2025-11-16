
#[derive(Debug, Clone)]
pub struct MovieWithDetails {
    pub movie: crate::data::models::movie::Movie,
    pub genres: Vec<crate::data::models::genre::Genre>,
    pub crew: Vec<crate::data::models::person::Person>,
    pub reviews: Vec<crate::data::models::review::Review>,
}
