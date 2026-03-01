//! Filmweb Integration
//! 
/// Fetches Polish ratings and reviews.

use anyhow::Result;
use reqwest::Client;
use scraper::{Html, Selector};

/// Filmweb client
pub struct FilmwebClient {
    /// HTTP client
    client: Client,
    
    /// Base URL
    base_url: String,
}

impl FilmwebClient {
    /// Create a new Filmweb client
    pub fn new() -> Result<Self> {
        Ok(Self {
            client: Client::new(),
            base_url: "https://www.filmweb.pl".to_string(),
        })
    }
    
    /// Search for a movie
    pub async fn search_movie(&self, query: &str) -> Result<Vec<filmweb::Movie>> {
        let url = format!("{}/search?q={}", self.base_url, urlencoding::encode(query));
        
        let response = self.client.get(&url).send().await?;
        let html = response.text().await?;
        
        self.parse_search_results(&html)
    }
    
    /// Get movie rating
    pub async fn get_rating(&self, url: &str) -> Result<filmweb::Rating> {
        let response = self.client.get(url).send().await?;
        let html = response.text().await?;
        
        self.parse_rating(&html)
    }
    
    /// Parse search results from HTML
    fn parse_search_results(&self, html: &str) -> Result<Vec<filmweb::Movie>> {
        let document = Html::parse_document(html);
        let selector = Selector::parse(".filmPreview__header").unwrap();
        
        let mut movies = Vec::new();
        
        for element in document.select(&selector) {
            let title = element
                .select(&Selector::parse(".filmPreview__title").unwrap())
                .next()
                .map(|e| e.text().collect::<String>())
                .unwrap_or_default();
            
            let url = element
                .select(&Selector::parse("a").unwrap())
                .next()
                .and_then(|e| e.value().attr("href"))
                .map(|s| format!("{}{}", self.base_url, s))
                .unwrap_or_default();
            
            if !title.is_empty() {
                movies.push(filmweb::Movie {
                    title,
                    url,
                });
            }
        }
        
        Ok(movies)
    }
    
    /// Parse rating from HTML
    fn parse_rating(&self, html: &str) -> Result<filmweb::Rating> {
        let document = Html::parse_document(html);
        
        let rating = document
            .select(&Selector::parse(".filmHeader__rate__value").unwrap())
            .next()
            .map(|e| e.text().collect::<String>())
            .and_then(|s| s.replace(',', ".").parse::<f64>().ok())
            .unwrap_or(0.0);
        
        let votes = document
            .select(&Selector::parse(".filmHeader__rate__count").unwrap())
            .next()
            .map(|e| {
                e.text()
                    .collect::<String>()
                    .replace(" ", "")
                    .parse::<u32>()
                    .unwrap_or(0)
            })
            .unwrap_or(0);
        
        Ok(filmweb::Rating {
            rating,
            votes,
        })
    }
}

/// Filmweb types
mod filmweb {
    #[derive(Debug, Clone)]
    pub struct Movie {
        pub title: String,
        pub url: String,
    }

    #[derive(Debug, Clone)]
    pub struct Rating {
        pub rating: f64,
        pub votes: u32,
    }
}