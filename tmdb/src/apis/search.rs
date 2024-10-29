use crate::{models, types::error::Error, TMDb};
use log::error;

pub struct Search {
    tmdb: TMDb,
    query: String,
    include_adult: bool,
    language: String,
    primary_release_year: String,
    page: u32,
    region: String,
    year: String,
}

impl Search {
    pub fn new(tmdb: TMDb, query: impl Into<String>) -> Self {
        Self {
            tmdb,
            query: query.into(),
            include_adult: false,
            language: String::from("en-US"),
            primary_release_year: String::default(),
            page: 1,
            region: String::default(),
            year: String::default(),
        }
    }

    pub async fn execute(&self) -> Result<models::MovieSearchResults, Error> {
        let res = self
            .tmdb
            .get("search/movie", &[["query", &self.query], ["include_adult", "false"]])
            .await
            .expect("Failed to make movie search request");

        if res.status().is_success() {
            let body = match res.text().await {
                Ok(body) => body,
                Err(e) => {
                    error!("Failed to read response body: {}", e);
                    return Err(e.into());
                }
            };

            let mut parsed: models::MovieSearchResults = match serde_json::from_str(&body) {
                Ok(parsed) => parsed,
                Err(e) => {
                    error!("Failed to parse response body: {}", e);
                    return Err(e.into());
                }
            };

            for searched_movie in &parsed.search_results {
                let movie_details = match self.tmdb.movie_details(searched_movie.id).await {
                    Ok(details) => details,
                    Err(e) => {
                        error!("Failed to fetch movie details: {:#?}", e);
                        continue;
                    }
                };

                parsed.results.push(movie_details);
            }

            Ok(parsed)
        } else {
            error!("TMDb request failed: {}", res.status());
            Err(Error::HTTPStatusError(res.status()))
        }
    }

    pub fn include_adult(mut self, include_adult: bool) -> Self {
        self.include_adult = include_adult;
        self
    }

    pub fn language(mut self, language: impl Into<String>) -> Self {
        self.language = language.into();
        self
    }

    pub fn primary_release_year(mut self, primary_release_year: impl Into<String>) -> Self {
        self.primary_release_year = primary_release_year.into();
        self
    }

    pub fn page(mut self, page: u32) -> Self {
        self.page = page;
        self
    }

    pub fn region(mut self, region: impl Into<String>) -> Self {
        self.region = region.into();
        self
    }

    pub fn year(mut self, year: impl Into<String>) -> Self {
        self.year = year.into();
        self
    }
}
