use crate::{
    apis::search::Search,
    types::{error::Error, models},
    TMDb,
};
use log::error;

const BASE_URL: &str = "https://api.themoviedb.org/3";
const IMAGE_BASE_URL: &str = "https://image.tmdb.org/t/p/original";

impl TMDb {
    pub(crate) async fn get(&self, path: impl Into<String>, params: &[[&str; 2]]) -> reqwest::Result<reqwest::Response> {
        let mut url = format!("{BASE_URL}/{}", path.into());
        let query_string: String = params
            .iter()
            .map(|[k, v]| format!("{}={}", (*k), (*v)))
            .collect::<Vec<String>>()
            .join("&");

        url = format!("{}?api_key={}&language={}&{}", url, self.api_key, self.language, query_string);

        self.reqwest.get(url).send().await
    }

    /// .
    ///
    /// # Errors
    ///
    /// This function will return an error if the request fails or if the response can't be read/parsed.
    pub async fn movie_details(&self, id: u64) -> Result<models::Movie, Error> {
        let res = match self.get(format!("movie/{id}"), &[]).await {
            Ok(res) => res,
            Err(err) => {
                error!("Failed to get movie details: {err}");
                return Err(err.into());
            }
        };

        if res.status().is_success() {
            let body = match res.text().await {
                Ok(body) => body,
                Err(e) => {
                    error!("Failed to read response body: {}", e);
                    return Err(e.into());
                }
            };

            let mut parsed: models::Movie = match serde_json::from_str(&body) {
                Ok(parsed) => parsed,
                Err(e) => {
                    error!("Failed to parse response body: {}", e);
                    return Err(e.into());
                }
            };

            parsed.backdrop_path = format!("{IMAGE_BASE_URL}{}", parsed.backdrop_path);
            parsed.poster_path = format!("{IMAGE_BASE_URL}{}", parsed.poster_path);
            for production_company in &mut parsed.production_companies {
                production_company.logo_path = format!("{IMAGE_BASE_URL}{}", production_company.logo_path);
            }

            Ok(parsed)
        } else {
            error!("TMDb request failed: {}", res.status());
            Err(Error::HTTPStatusError(res.status()))
        }
    }

    pub fn search(&self, query: impl Into<String>) -> Search {
        Search::new(self.clone(), query.into())
    }
}
