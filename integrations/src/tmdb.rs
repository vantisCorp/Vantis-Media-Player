//! TMDB Integration
//! 
/// Fetches movie/TV metadata, posters, and cast information.

use anyhow::Result;
use reqwest::Client;

/// TMDB client
pub struct TMDBClient {
    /// API key
    api_key: String,
    
    /// HTTP client
    client: Client,
    
    /// Base URL
    base_url: String,
}

impl TMDBClient {
    /// Create a new TMDB client
    pub fn new(api_key: &str) -> Result<Self> {
        Ok(Self {
            api_key: api_key.to_string(),
            client: Client::new(),
            base_url: "https://api.themoviedb.org/3".to_string(),
        })
    }
    
    /// Search for a movie
    pub async fn search_movie(&self, query: &str) -> Result<Vec<tmdb::Movie>> {
        let url = format!(
            "{}/search/movie?api_key={}&query={}",
            self.base_url,
            self.api_key,
            urlencoding::encode(query)
        );
        
        let response = self.client.get(&url).send().await?;
        let data: tmdb::SearchResponse = response.json().await?;
        
        Ok(data.results)
    }
    
    /// Get movie details
    pub async fn get_movie(&self, id: u32) -> Result<tmdb::Movie> {
        let url = format!(
            "{}/movie/{}?api_key={}",
            self.base_url, id, self.api_key
        );
        
        let response = self.client.get(&url).send().await?;
        let movie: tmdb::Movie = response.json().await?;
        
        Ok(movie)
    }
    
    /// Get movie cast
    pub async fn get_cast(&self, id: u32) -> Result<Vec<tmdb::CastMember>> {
        let url = format!(
            "{}/movie/{}/credits?api_key={}",
            self.base_url, id, self.api_key
        );
        
        let response = self.client.get(&url).send().await?;
        let data: tmdb::CreditsResponse = response.json().await?;
        
        Ok(data.cast)
    }
}

/// TMDB types
mod tmdb {
    use serde::{Deserialize, Serialize};

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct Movie {
        pub id: u32,
        pub title: String,
        pub overview: String,
        pub poster_path: Option<String>,
        pub backdrop_path: Option<String>,
        pub release_date: Option<String>,
        pub vote_average: f64,
        pub vote_count: u32,
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct CastMember {
        pub id: u32,
        pub name: String,
        pub character: String,
        pub profile_path: Option<String>,
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct SearchResponse {
        pub results: Vec<Movie>,
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct CreditsResponse {
        pub cast: Vec<CastMember>,
    }
}