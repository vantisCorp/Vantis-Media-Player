//! Unified Search Module
//! 
//! Provides unified search across all streaming services.

use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use tokio::sync::RwLock;

use crate::provider::{StreamingProvider, ContentItem, SearchOptions};
use crate::{Service, StreamingError, StreamingResult};

/// Unified search result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchResult {
    /// Query that was searched
    pub query: String,
    /// Results grouped by service
    pub results_by_service: HashMap<Service, Vec<ContentItem>>,
    /// All results combined and sorted
    pub all_results: Vec<ContentItem>,
    /// Total result count
    pub total_count: usize,
    /// Services that were searched
    pub services_searched: Vec<Service>,
    /// Services that failed
    pub services_failed: HashMap<Service, String>,
    /// Search duration in milliseconds
    pub duration_ms: u64,
}

impl SearchResult {
    /// Check if there are any results
    pub fn has_results(&self) -> bool {
        !self.all_results.is_empty()
    }
    
    /// Get results for a specific service
    pub fn get_for_service(&self, service: Service) -> Option<&Vec<ContentItem>> {
        self.results_by_service.get(&service)
    }
    
    /// Filter results by content type
    pub fn filter_by_type(&self, content_type: crate::provider::ContentType) -> Vec<&ContentItem> {
        self.all_results.iter()
            .filter(|item| item.content_type == content_type)
            .collect()
    }
    
    /// Filter results by genre
    pub fn filter_by_genre(&self, genre: &str) -> Vec<&ContentItem> {
        self.all_results.iter()
            .filter(|item| item.genres.iter().any(|g| g.to_lowercase() == genre.to_lowercase()))
            .collect()
    }
    
    /// Filter results by minimum rating
    pub fn filter_by_min_rating(&self, min_rating: f32) -> Vec<&ContentItem> {
        self.all_results.iter()
            .filter(|item| item.user_rating.map(|r| r >= min_rating).unwrap_or(false))
            .collect()
    }
    
    /// Sort results by rating
    pub fn sorted_by_rating(&self) -> Vec<&ContentItem> {
        let mut results: Vec<_> = self.all_results.iter().collect();
        results.sort_by(|a, b| {
            let rating_a = a.user_rating.unwrap_or(0.0);
            let rating_b = b.user_rating.unwrap_or(0.0);
            rating_b.partial_cmp(&rating_a).unwrap_or(std::cmp::Ordering::Equal)
        });
        results
    }
    
    /// Sort results by year
    pub fn sorted_by_year(&self) -> Vec<&ContentItem> {
        let mut results: Vec<_> = self.all_results.iter().collect();
        results.sort_by(|a, b| {
            let year_a = a.year.unwrap_or(0);
            let year_b = b.year.unwrap_or(0);
            year_b.cmp(&year_a)
        });
        results
    }
}

/// Unified search interface
pub struct UnifiedSearch {
    providers: RwLock<Vec<Box<dyn StreamingProvider>>>,
    timeout_ms: u64,
    max_concurrent: usize,
}

impl UnifiedSearch {
    /// Create a new unified search
    pub fn new() -> Self {
        Self {
            providers: RwLock::new(Vec::new()),
            timeout_ms: 10000,
            max_concurrent: 5,
        }
    }
    
    /// Create with custom settings
    pub fn with_settings(timeout_ms: u64, max_concurrent: usize) -> Self {
        Self {
            providers: RwLock::new(Vec::new()),
            timeout_ms,
            max_concurrent,
        }
    }
    
    /// Add a provider
    pub async fn add_provider(&self, provider: Box<dyn StreamingProvider>) {
        self.providers.write().await.push(provider);
    }
    
    /// Search all providers
    pub async fn search(&self, query: &str, options: &SearchOptions) -> StreamingResult<SearchResult> {
        let start = std::time::Instant::now();
        let mut results_by_service = HashMap::new();
        let mut services_searched = Vec::new();
        let mut services_failed = HashMap::new();
        
        let providers = self.providers.read().await;
        
        for provider in providers.iter() {
            let service = provider.service();
            
            if !provider.is_authenticated().await {
                services_failed.insert(service, "Not authenticated".to_string());
                continue;
            }
            
            match provider.search(query, options).await {
                Ok(results) => {
                    services_searched.push(service);
                    results_by_service.insert(service, results);
                }
                Err(e) => {
                    services_failed.insert(service, e.to_string());
                }
            }
        }
        
        // Combine and sort all results
        let mut all_results: Vec<ContentItem> = results_by_service.values()
            .flat_map(|v| v.iter().cloned())
            .collect();
        
        // Sort by relevance (for now, by rating)
        all_results.sort_by(|a, b| {
            let rating_a = a.user_rating.unwrap_or(0.0);
            let rating_b = b.user_rating.unwrap_or(0.0);
            rating_b.partial_cmp(&rating_a).unwrap_or(std::cmp::Ordering::Equal)
        });
        
        let total_count = all_results.len();
        
        Ok(SearchResult {
            query: query.to_string(),
            results_by_service,
            all_results,
            total_count,
            services_searched,
            services_failed,
            duration_ms: start.elapsed().as_millis() as u64,
        })
    }
    
    /// Search specific services only
    pub async fn search_services(
        &self,
        query: &str,
        options: &SearchOptions,
        services: &[Service],
    ) -> StreamingResult<SearchResult> {
        let start = std::time::Instant::now();
        let mut results_by_service = HashMap::new();
        let mut services_searched = Vec::new();
        let mut services_failed = HashMap::new();
        
        let providers = self.providers.read().await;
        
        for provider in providers.iter() {
            let service = provider.service();
            
            // Skip if not in the requested services list
            if !services.contains(&service) {
                continue;
            }
            
            if !provider.is_authenticated().await {
                services_failed.insert(service, "Not authenticated".to_string());
                continue;
            }
            
            match provider.search(query, options).await {
                Ok(results) => {
                    services_searched.push(service);
                    results_by_service.insert(service, results);
                }
                Err(e) => {
                    services_failed.insert(service, e.to_string());
                }
            }
        }
        
        let mut all_results: Vec<ContentItem> = results_by_service.values()
            .flat_map(|v| v.iter().cloned())
            .collect();
        
        all_results.sort_by(|a, b| {
            let rating_a = a.user_rating.unwrap_or(0.0);
            let rating_b = b.user_rating.unwrap_or(0.0);
            rating_b.partial_cmp(&rating_a).unwrap_or(std::cmp::Ordering::Equal)
        });
        
        let total_count = all_results.len();
        
        Ok(SearchResult {
            query: query.to_string(),
            results_by_service,
            all_results,
            total_count,
            services_searched,
            services_failed,
            duration_ms: start.elapsed().as_millis() as u64,
        })
    }
    
    /// Find where a specific title is available
    pub async fn find_title(&self, title: &str) -> StreamingResult<HashMap<Service, ContentItem>> {
        let providers = self.providers.read().await;
        let mut found = HashMap::new();
        
        for provider in providers.iter() {
            if !provider.is_authenticated().await {
                continue;
            }
            
            let options = SearchOptions {
                max_results: Some(10),
                ..Default::default()
            };
            
            if let Ok(results) = provider.search(title, &options).await {
                for item in results {
                    if item.title.to_lowercase() == title.to_lowercase() {
                        found.insert(provider.service(), item);
                        break;
                    }
                }
            }
        }
        
        Ok(found)
    }
    
    /// Get available providers
    pub async fn available_providers(&self) -> Vec<Service> {
        self.providers.read().await
            .iter()
            .map(|p| p.service())
            .collect()
    }
}

impl Default for UnifiedSearch {
    fn default() -> Self {
        Self::new()
    }
}

/// Content availability checker
pub struct AvailabilityChecker {
    search: UnifiedSearch,
}

impl AvailabilityChecker {
    pub fn new(search: UnifiedSearch) -> Self {
        Self { search }
    }
    
    /// Check if a title is available on any service
    pub async fn is_available(&self, title: &str) -> bool {
        self.search.find_title(title).await
            .map(|results| !results.is_empty())
            .unwrap_or(false)
    }
    
    /// Get all services where title is available
    pub async fn get_availability(&self, title: &str) -> StreamingResult<Vec<(Service, ContentItem)>> {
        let found = self.search.find_title(title).await?;
        Ok(found.into_iter().collect())
    }
    
    /// Check which services have the cheapest access to content
    pub async fn get_cheapest_option(&self, title: &str) -> StreamingResult<Option<(Service, ContentItem)>> {
        let availability = self.get_availability(title).await?;
        
        // For now, just return the first available
        // In a real implementation, this would compare subscription prices
        Ok(availability.into_iter().next())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_search_result_creation() {
        let result = SearchResult {
            query: "test".to_string(),
            results_by_service: HashMap::new(),
            all_results: vec![],
            total_count: 0,
            services_searched: vec![],
            services_failed: HashMap::new(),
            duration_ms: 100,
        };
        
        assert!(!result.has_results());
    }
}