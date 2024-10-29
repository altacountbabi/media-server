use serde::{Deserialize, Serialize};
use serde_default_utils::*;

#[derive(Debug, Deserialize, Serialize)]
pub struct MovieSearchResults {
    #[serde(default = "default_u32::<0>")]
    pub page: u32,
    #[serde(skip)]
    pub results: Vec<Movie>,
    #[serde(rename = "results")]
    pub(crate) search_results: Vec<SearchedMovie>,
    #[serde(default = "default_u32::<0>")]
    pub total_pages: u32,
    #[serde(default = "default_u32::<0>")]
    pub total_results: u32,
}

#[serde_inline_default]
#[derive(Debug, Deserialize, Serialize)]
pub struct SearchedMovie {
    #[serde(default = "default_bool::<true>")]
    pub adult: bool,
    pub backdrop_path: String,
    pub genre_ids: Vec<u64>,
    #[serde(default = "default_u64::<0>")]
    pub id: u64,
    pub original_language: String,
    pub original_title: String,
    pub overview: String,
    #[serde_inline_default(0.0)]
    pub popularity: f64,
    pub poster_path: String,
    pub release_date: String,
    pub title: String,
    #[serde(default = "default_bool::<true>")]
    pub video: bool,
    #[serde_inline_default(0.0)]
    pub vote_average: f64,
    #[serde(default = "default_u64::<0>")]
    pub vote_count: u64,
}

#[serde_inline_default]
#[derive(Debug, Deserialize, Serialize)]
pub struct Movie {
    #[serde(default = "default_bool::<true>")]
    pub adult: bool,
    pub backdrop_path: String,
    #[serde(default = "default_u64::<0>")]
    pub budget: u64,
    pub genres: Vec<Genre>,
    pub homepage: String,
    #[serde(default = "default_u64::<0>")]
    pub id: u64,
    pub imdb_id: String,
    pub original_language: String,
    pub original_title: String,
    pub overview: String,
    #[serde_inline_default(0.0)]
    pub popularity: f64,
    pub poster_path: String,
    pub production_companies: Vec<ProductionCompany>,
    pub production_countries: Vec<ProductionCountry>,
    pub release_date: String,
    #[serde(default = "default_u64::<0>")]
    pub revenue: u64,
    #[serde(default = "default_u64::<0>")]
    pub runtime: u64,
    pub spoken_languages: Vec<SpokenLanguage>,
    pub status: String,
    pub tagline: String,
    pub title: String,
    #[serde(default = "default_bool::<true>")]
    pub video: bool,
    #[serde_inline_default(0.0)]
    pub vote_average: f64,
    #[serde(default = "default_u64::<0>")]
    pub vote_count: u64,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Genre {
    #[serde(default = "default_u64::<0>")]
    pub id: u64,
    pub name: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ProductionCompany {
    #[serde(default = "default_u64::<0>")]
    pub id: u64,
    pub logo_path: String,
    pub name: String,
    pub origin_country: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ProductionCountry {
    pub iso_3166_1: String,
    pub name: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct SpokenLanguage {
    pub english_name: String,
    pub iso_639_1: String,
    pub name: String,
}
