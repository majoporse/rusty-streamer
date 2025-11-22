use std::str::FromStr;

use chrono::{NaiveDate, NaiveDateTime};
use movies_client::models as client_models;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct WrapperPerson {
    pub id: Uuid,
    pub first_name: String,
    pub last_name: String,
    pub birth_date: Option<NaiveDate>,
    pub bio: Option<String>,
    pub role: Option<String>,
    pub created_at: NaiveDateTime,
    pub image_url: Option<String>,
}

impl From<client_models::Person> for WrapperPerson {
    fn from(v: client_models::Person) -> Self {
        WrapperPerson {
            role: v.role.flatten(),
            bio: v.bio.flatten(),
            birth_date: v
                .birth_date
                .flatten()
                .map(|d| NaiveDate::from_str(&*d).unwrap()),
            created_at: NaiveDateTime::from_str(&v.created_at).unwrap(),
            first_name: v.first_name,
            id: v.id,
            last_name: v.last_name,
            image_url: v.image_url.flatten(),
        }
    }
}

impl From<WrapperPerson> for client_models::Person {
    fn from(w: WrapperPerson) -> Self {
        client_models::Person {
            role: Some(w.role),
            bio: Some(w.bio),
            birth_date: Some(w.birth_date.map(|d| d.to_string())),
            created_at: w.created_at.to_string(),
            first_name: w.first_name,
            id: w.id,
            last_name: w.last_name,
            image_url: Some(w.image_url),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct WrapperNewPerson {
    pub first_name: String,
    pub last_name: String,
    pub birth_date: Option<NaiveDate>,
    pub bio: Option<String>,
    pub role: Option<String>,
    pub image_url: Option<String>,
}

impl From<client_models::NewPerson> for WrapperNewPerson {
    fn from(v: client_models::NewPerson) -> Self {
        WrapperNewPerson {
            role: v.role.flatten(),
            bio: v.bio.flatten(),
            birth_date: v
                .birth_date
                .flatten()
                .map(|d| NaiveDate::from_str(&d).unwrap()),
            first_name: v.first_name,
            last_name: v.last_name,
            image_url: v.image_url.flatten(),
        }
    }
}

impl From<WrapperNewPerson> for client_models::NewPerson {
    fn from(w: WrapperNewPerson) -> Self {
        client_models::NewPerson {
            role: Some(w.role),
            bio: Some(w.bio),
            birth_date: Some(w.birth_date.map(|d| d.to_string())),
            first_name: w.first_name,
            last_name: w.last_name,
            image_url: Some(w.image_url),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct WrapperMovie {
    pub created_at: String,
    pub description: Option<Option<String>>,
    pub duration_minutes: Option<Option<i32>>,
    pub id: Uuid,
    pub mpaa_rating: Option<Option<String>>,
    pub release_date: Option<Option<String>>,
    pub video_url: Option<Option<String>>,
    pub poster_url: Option<Option<String>>,
    pub title: String,
    pub updated_at: String,
}

impl From<client_models::Movie> for WrapperMovie {
    fn from(v: client_models::Movie) -> Self {
        WrapperMovie {
            created_at: v.created_at,
            description: v.description,
            duration_minutes: v.duration_minutes,
            id: v.id,
            mpaa_rating: v.mpaa_rating,
            release_date: v.release_date,
            video_url: v.video_url,
            poster_url: v.poster_url,
            title: v.title,
            updated_at: v.updated_at,
        }
    }
}

impl From<WrapperMovie> for client_models::Movie {
    fn from(w: WrapperMovie) -> Self {
        client_models::Movie {
            created_at: w.created_at,
            description: w.description,
            duration_minutes: w.duration_minutes,
            id: w.id,
            mpaa_rating: w.mpaa_rating,
            release_date: w.release_date,
            video_url: w.video_url,
            poster_url: w.poster_url,
            title: w.title,
            updated_at: w.updated_at,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct WrapperNewMovie {
    pub description: Option<Option<String>>,
    pub duration_minutes: Option<Option<i32>>,
    pub mpaa_rating: Option<Option<String>>,
    pub release_date: Option<Option<String>>,
    pub video_url: Option<Option<String>>,
    pub poster_url: Option<Option<String>>,
    pub title: String,
    pub genre_ids: Option<Option<Vec<Uuid>>>,
    pub people_ids: Option<Option<Vec<WrapperMovieCrew>>>,
}

impl From<client_models::NewMovie> for WrapperNewMovie {
    fn from(v: client_models::NewMovie) -> Self {
        WrapperNewMovie {
            description: v.description,
            duration_minutes: v.duration_minutes,
            mpaa_rating: v.mpaa_rating,
            release_date: v.release_date,
            video_url: v.video_url,
            poster_url: v.poster_url,
            title: v.title,
            genre_ids: v.genre_ids,
            people_ids: v.people_ids.map(|opt_vec| {
                opt_vec.map(|vec| vec.into_iter().map(WrapperMovieCrew::from).collect())
            }),
        }
    }
}

impl From<WrapperNewMovie> for client_models::NewMovie {
    fn from(w: WrapperNewMovie) -> Self {
        client_models::NewMovie {
            description: w.description,
            duration_minutes: w.duration_minutes,
            mpaa_rating: w.mpaa_rating,
            release_date: w.release_date,
            video_url: w.video_url,
            poster_url: w.poster_url,
            title: w.title,
            genre_ids: w.genre_ids,
            people_ids: w.people_ids.map(|opt_vec| {
                opt_vec.map(|vec| {
                    vec.into_iter()
                        .map(client_models::NewMovieCrew::from)
                        .collect()
                })
            }),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct WrapperMovieCrew {
    pub person_id: Uuid,
    pub role: Option<String>,
    pub billing_order: Option<i32>,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct WrapperNewMovieCrew {
    pub person_id: Uuid,
    pub role: Option<String>,
    pub billing_order: Option<i32>,
}

impl From<client_models::NewMovieCrew> for WrapperMovieCrew {
    fn from(v: client_models::NewMovieCrew) -> Self {
        WrapperMovieCrew {
            person_id: v.person_id,
            role: v.role.flatten(),
            billing_order: v.billing_order.flatten(),
        }
    }
}

impl From<WrapperMovieCrew> for client_models::NewMovieCrew {
    fn from(w: WrapperMovieCrew) -> Self {
        client_models::NewMovieCrew {
            person_id: w.person_id,
            role: Some(w.role),
            billing_order: Some(w.billing_order),
        }
    }
}

impl From<WrapperNewMovieCrew> for client_models::NewMovieCrew {
    fn from(w: WrapperNewMovieCrew) -> Self {
        client_models::NewMovieCrew {
            person_id: w.person_id,
            role: Some(w.role),
            billing_order: Some(w.billing_order),
        }
    }
}

impl From<client_models::NewMovieCrew> for WrapperNewMovieCrew {
    fn from(v: client_models::NewMovieCrew) -> Self {
        WrapperNewMovieCrew {
            person_id: v.person_id,
            role: v.role.flatten(),
            billing_order: v.billing_order.flatten(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct WrapperReview {
    pub body: Option<Option<String>>,
    pub created_at: String,
    pub id: Uuid,
    pub movie_id: Uuid,
    pub user_id: Uuid,
    pub rating: i32,
    pub title: Option<Option<String>>,
    pub user_name: String,
}

impl From<client_models::Review> for WrapperReview {
    fn from(v: client_models::Review) -> Self {
        WrapperReview {
            body: v.body,
            created_at: v.created_at,
            id: v.id,
            movie_id: v.movie_id,
            user_id: v.user_id,
            rating: v.rating,
            title: v.title,
            user_name: v.user_name,
        }
    }
}

impl From<WrapperReview> for client_models::Review {
    fn from(w: WrapperReview) -> Self {
        client_models::Review {
            user_id: w.user_id,
            body: w.body,
            created_at: w.created_at,
            id: w.id,
            movie_id: w.movie_id,
            rating: w.rating,
            title: w.title,
            user_name: w.user_name,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct WrapperNewReview {
    pub body: Option<Option<String>>,
    pub movie_id: Uuid,
    pub user_id: Uuid,
    pub rating: i32,
    pub title: Option<Option<String>>,
    pub user_name: String,
}

impl From<client_models::NewReview> for WrapperNewReview {
    fn from(v: client_models::NewReview) -> Self {
        WrapperNewReview {
            body: v.body,
            movie_id: v.movie_id,
            rating: v.rating,
            title: v.title,
            user_id: v.user_id,
            user_name: v.user_name,
        }
    }
}

impl From<WrapperNewReview> for client_models::NewReview {
    fn from(w: WrapperNewReview) -> Self {
        client_models::NewReview {
            body: w.body,
            movie_id: w.movie_id,
            rating: w.rating,
            title: w.title,
            user_id: w.user_id,
            user_name: w.user_name,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct WrapperMovieDetail {
    pub id: Uuid,
    pub title: String,
    pub video_url: Option<String>,
    pub poster_url: Option<String>,
    pub description: Option<String>,
    pub release_date: Option<NaiveDate>,
    pub duration_minutes: Option<i32>,
    pub mpaa_rating: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub people: Vec<MovieCrewDetail>,
    pub genres: Vec<WrapperGenre>,
    pub reviews: Vec<WrapperReview>,
}

impl From<client_models::MovieDetail> for WrapperMovieDetail {
    fn from(v: client_models::MovieDetail) -> Self {
        WrapperMovieDetail {
            id: v.id,
            title: v.title,
            video_url: v.video_url.flatten(),
            poster_url: v.poster_url.flatten(),
            description: v.description.flatten(),
            release_date: v
                .release_date
                .flatten()
                .map(|d| NaiveDate::from_str(&d).unwrap()),
            duration_minutes: v.duration_minutes.flatten(),
            mpaa_rating: v.mpaa_rating.flatten(),
            created_at: NaiveDateTime::from_str(&v.created_at).unwrap(),
            updated_at: NaiveDateTime::from_str(&v.updated_at).unwrap(),
            people: v.people.into_iter().map(MovieCrewDetail::from).collect(),
            genres: v.genres.into_iter().map(WrapperGenre::from).collect(),
            reviews: v.reviews.into_iter().map(WrapperReview::from).collect(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct WrapperGenre {
    pub id: Uuid,
    pub name: String,
}

impl From<client_models::Genre> for WrapperGenre {
    fn from(v: client_models::Genre) -> Self {
        WrapperGenre {
            id: v.id,
            name: v.name,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct WrapperNewGenre {
    pub name: String,
}

impl From<client_models::NewGenre> for WrapperNewGenre {
    fn from(v: client_models::NewGenre) -> Self {
        WrapperNewGenre { name: v.name }
    }
}

impl From<WrapperNewGenre> for client_models::NewGenre {
    fn from(w: WrapperNewGenre) -> Self {
        client_models::NewGenre { name: w.name }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct MovieCrewDetail {
    pub movie_id: Uuid,
    pub person: WrapperPerson,
    person_id: Uuid,
    pub character_name: Option<String>,
    pub billing_order: Option<i32>,
}

impl From<client_models::MovieCrewDetail> for MovieCrewDetail {
    fn from(v: client_models::MovieCrewDetail) -> Self {
        MovieCrewDetail {
            movie_id: v.movie_id,
            person_id: v.person_id,
            person: (*v.person).into(),
            character_name: v.character_name.flatten(),
            billing_order: v.billing_order.flatten(),
        }
    }
}

// Models for upload SAS endpoint
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct UploadSasRequest {
    /// original filename (including extension)
    pub filename: String,
    /// MIME content type
    pub content_type: Option<String>,
    /// kind of upload, e.g. "video" or "poster"
    pub kind: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct UploadSasResponse {
    /// full URL suitable for uploading (may include SAS token)
    pub upload_url: String,
    /// canonical blob URL (without SAS) for later reference
    pub blob_url: String,
    /// optional expiration timestamp for the upload URL
    pub expires_at: Option<String>,
}
