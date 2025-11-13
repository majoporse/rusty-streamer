use movies_client::models as client_models;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;


#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct WrapperActor {
    pub bio: Option<Option<String>>,
    pub birth_date: Option<Option<String>>,
    pub created_at: String,
    pub first_name: String,
    pub id: i32,
    pub last_name: String,
}

impl From<client_models::Actor> for WrapperActor {
    fn from(v: client_models::Actor) -> Self {
        WrapperActor {
            bio: v.bio,
            birth_date: v.birth_date,
            created_at: v.created_at,
            first_name: v.first_name,
            id: v.id,
            last_name: v.last_name,
        }
    }
}

impl From<WrapperActor> for client_models::Actor {
    fn from(w: WrapperActor) -> Self {
        client_models::Actor {
            bio: w.bio,
            birth_date: w.birth_date,
            created_at: w.created_at,
            first_name: w.first_name,
            id: w.id,
            last_name: w.last_name,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct WrapperNewActor {
    pub bio: Option<Option<String>>,
    pub birth_date: Option<Option<String>>,
    pub first_name: String,
    pub last_name: String,
}

impl From<client_models::NewActor> for WrapperNewActor {
    fn from(v: client_models::NewActor) -> Self {
        WrapperNewActor {
            bio: v.bio,
            birth_date: v.birth_date,
            first_name: v.first_name,
            last_name: v.last_name,
        }
    }
}

impl From<WrapperNewActor> for client_models::NewActor {
    fn from(w: WrapperNewActor) -> Self {
        client_models::NewActor {
            bio: w.bio,
            birth_date: w.birth_date,
            first_name: w.first_name,
            last_name: w.last_name,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct WrapperMovie {
    pub created_at: String,
    pub description: Option<Option<String>>,
    pub duration_minutes: Option<Option<i32>>,
    pub id: i32,
    pub mpaa_rating: Option<Option<String>>,
    pub release_date: Option<Option<String>>,
    pub slug: String,
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
            slug: v.slug,
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
            slug: w.slug,
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
    pub slug: String,
    pub title: String,
}

impl From<client_models::NewMovie> for WrapperNewMovie {
    fn from(v: client_models::NewMovie) -> Self {
        WrapperNewMovie {
            description: v.description,
            duration_minutes: v.duration_minutes,
            mpaa_rating: v.mpaa_rating,
            release_date: v.release_date,
            slug: v.slug,
            title: v.title,
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
            slug: w.slug,
            title: w.title,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct WrapperReview {
    pub body: Option<Option<String>>,
    pub created_at: String,
    pub id: i32,
    pub movie_id: i32,
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
            rating: v.rating,
            title: v.title,
            user_name: v.user_name,
        }
    }
}

impl From<WrapperReview> for client_models::Review {
    fn from(w: WrapperReview) -> Self {
        client_models::Review {
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
    pub movie_id: i32,
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
            user_name: w.user_name,
        }
    }
}
