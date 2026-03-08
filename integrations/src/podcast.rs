//! Podcast integration service
//! 
//! Provides RSS/Atom feed parsing and iTunes podcast search
//! for accessing podcast episodes and subscriptions.

use anyhow::Result;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

/// Podcast service configuration
#[derive(Debug, Clone)]
pub struct PodcastConfig {
    pub user_agent: String,
    pub timeout_seconds: u64,
    pub max_episodes_per_feed: u32,
}

impl Default for PodcastConfig {
    fn default() -> Self {
        Self {
            user_agent: "VantisMediaPlayer/1.0".to_string(),
            timeout_seconds: 30,
            max_episodes_per_feed: 100,
        }
    }
}

/// iTunes podcast search result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ITunesPodcast {
    pub collection_id: u64,
    pub collection_name: String,
    pub artist_name: String,
    pub feed_url: Option<String>,
    pub artwork_url_30: Option<String>,
    pub artwork_url_60: Option<String>,
    pub artwork_url_100: Option<String>,
    pub artwork_url_600: Option<String>,
    pub primary_genre_name: Option<String>,
    pub track_count: Option<u32>,
}

/// Podcast feed
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PodcastFeed {
    pub id: String,
    pub title: String,
    pub author: Option<String>,
    pub description: Option<String>,
    pub feed_url: Option<String>,
    pub artwork_url: Option<String>,
    pub category: Option<String>,
    pub explicit: Option<bool>,
    pub episode_count: Option<u32>,
    pub website_url: Option<String>,
    pub last_updated: Option<DateTime<Utc>>,
}

impl From<ITunesPodcast> for PodcastFeed {
    fn from(podcast: ITunesPodcast) -> Self {
        Self {
            id: podcast.collection_id.to_string(),
            title: podcast.collection_name,
            author: Some(podcast.artist_name),
            description: None,
            feed_url: podcast.feed_url,
            artwork_url: podcast.artwork_url_600
                .or(podcast.artwork_url_100)
                .or(podcast.artwork_url_60),
            category: podcast.primary_genre_name,
            explicit: None,
            episode_count: podcast.track_count,
            website_url: None,
            last_updated: None,
        }
    }
}

/// Podcast episode
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PodcastEpisode {
    pub id: String,
    pub feed_id: String,
    pub title: String,
    pub description: Option<String>,
    pub author: Option<String>,
    pub pub_date: Option<DateTime<Utc>>,
    pub duration_seconds: Option<u32>,
    pub audio_url: Option<String>,
    pub artwork_url: Option<String>,
    pub episode_number: Option<u32>,
    pub season_number: Option<u32>,
    pub explicit: Option<bool>,
}

/// Podcast category/genre
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PodcastCategory {
    pub id: u64,
    pub name: String,
    pub subcategories: Vec<PodcastCategory>,
}

/// RSS feed channel
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RSSChannel {
    pub title: String,
    pub link: Option<String>,
    pub description: Option<String>,
    pub language: Option<String>,
    pub image_url: Option<String>,
    pub categories: Vec<String>,
    pub explicit: Option<bool>,
    pub author: Option<String>,
    pub episodes: Vec<RSSItem>,
}

/// RSS feed item (episode)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RSSItem {
    pub title: Option<String>,
    pub link: Option<String>,
    pub description: Option<String>,
    pub author: Option<String>,
    pub enclosure_url: Option<String>,
    pub enclosure_length: Option<u64>,
    pub enclosure_type: Option<String>,
    pub guid: Option<String>,
    pub pub_date: Option<String>,
    pub duration: Option<String>,
    pub image_url: Option<String>,
    pub episode: Option<u32>,
    pub season: Option<u32>,
}

/// Podcast client
pub struct PodcastClient {
    config: PodcastConfig,
    client: Client,
}

impl PodcastClient {
    pub fn new(config: PodcastConfig) -> Result<Self> {
        let client = Client::builder()
            .timeout(std::time::Duration::from_secs(config.timeout_seconds))
            .user_agent(&config.user_agent)
            .build()?;
        
        Ok(Self { config, client })
    }
    
    /// Search for podcasts using iTunes Search API
    pub async fn search_podcasts(&self, query: &str, limit: u32) -> Result<Vec<PodcastFeed>> {
        let url = format!(
            "https://itunes.apple.com/search?term={}&media=podcast&entity=podcast&limit={}",
            urlencoding::encode(query),
            limit
        );
        
        let response = self.client.get(&url).send().await?;
        
        #[derive(Deserialize)]
        struct SearchResponse {
            results: Vec<ITunesPodcast>,
        }
        
        let result: SearchResponse = response.json().await?;
        
        Ok(result.results.into_iter()
            .map(PodcastFeed::from)
            .collect())
    }
    
    /// Get podcast by iTunes ID
    pub async fn get_podcast_by_id(&self, id: &str) -> Result<Option<PodcastFeed>> {
        let url = format!(
            "https://itunes.apple.com/lookup?id={}&entity=podcast",
            id
        );
        
        let response = self.client.get(&url).send().await?;
        
        #[derive(Deserialize)]
        struct LookupResponse {
            results: Vec<ITunesPodcast>,
        }
        
        let result: LookupResponse = response.json().await?;
        
        Ok(result.results.into_iter().next().map(PodcastFeed::from))
    }
    
    /// Get top podcasts
    pub async fn get_top_podcasts(&self, limit: u32) -> Result<Vec<PodcastFeed>> {
        let url = format!(
            "https://itunes.apple.com/us/rss/toppodcasts/limit={}/explicit=true/json",
            limit
        );
        
        let response = self.client.get(&url).send().await?;
        
        #[derive(Deserialize)]
        struct TopResponse {
            feed: TopFeed,
        }
        
        #[derive(Deserialize)]
        struct TopFeed {
            entry: Vec<TopEntry>,
        }
        
        #[derive(Deserialize)]
        struct TopEntry {
            #[serde(rename = "im:name")]
            name: LabelValue,
            #[serde(rename = "im:artist")]
            artist: LabelValue,
            id: IdValue,
            #[serde(rename = "im:image")]
            images: Vec<ImageValue>,
        }
        
        #[derive(Deserialize)]
        struct LabelValue {
            label: String,
        }
        
        #[derive(Deserialize)]
        struct IdValue {
            attributes: IdAttributes,
        }
        
        #[derive(Deserialize)]
        struct IdAttributes {
            #[serde(rename = "im:id")]
            id: String,
        }
        
        #[derive(Deserialize)]
        struct ImageValue {
            label: String,
            attributes: ImageAttributes,
        }
        
        #[derive(Deserialize)]
        struct ImageAttributes {
            height: String,
        }
        
        let result: TopResponse = response.json().await?;
        
        let podcasts: Vec<PodcastFeed> = result.feed.entry.into_iter()
            .map(|entry| {
                let best_image = entry.images.into_iter()
                    .max_by_key(|img| img.attributes.height.parse::<u32>().unwrap_or(0));
                
                PodcastFeed {
                    id: entry.id.attributes.id,
                    title: entry.name.label,
                    author: Some(entry.artist.label),
                    description: None,
                    feed_url: None,
                    artwork_url: best_image.map(|img| img.label),
                    category: None,
                    explicit: None,
                    episode_count: None,
                    website_url: None,
                    last_updated: None,
                }
            })
            .collect();
        
        Ok(podcasts)
    }
    
    /// Parse RSS feed and get episodes
    pub async fn get_episodes(&self, feed_url: &str, limit: u32) -> Result<Vec<PodcastEpisode>> {
        let response = self.client.get(feed_url).send().await?;
        let content = response.text().await?;
        
        let channel = parse_rss_feed(&content)?;
        let feed_id = urlencoding::encode(feed_url).to_string();
        
        let episodes: Vec<PodcastEpisode> = channel.episodes.into_iter()
            .take(limit as usize)
            .enumerate()
            .map(|(index, item)| PodcastEpisode {
                id: item.guid.clone().unwrap_or_else(|| format!("{}-{}", feed_id, index)),
                feed_id: feed_id.clone(),
                title: item.title.clone().unwrap_or_else(|| "Untitled".to_string()),
                description: item.description.clone(),
                author: item.author.clone(),
                pub_date: item.pub_date.as_ref().and_then(|d| parse_rss_date(d)),
                duration_seconds: item.duration.as_ref().and_then(|d| parse_duration(d)),
                audio_url: item.enclosure_url.clone(),
                artwork_url: item.image_url.clone(),
                episode_number: item.episode,
                season_number: item.season,
                explicit: None,
            })
            .collect();
        
        Ok(episodes)
    }
    
    /// Get podcast categories
    pub fn get_categories(&self) -> Vec<PodcastCategory> {
        vec![
            PodcastCategory {
                id: 1301,
                name: "Arts".to_string(),
                subcategories: vec![
                    PodcastCategory { id: 1321, name: "Books".to_string(), subcategories: vec![] },
                    PodcastCategory { id: 1324, name: "Design".to_string(), subcategories: vec![] },
                    PodcastCategory { id: 1330, name: "Food".to_string(), subcategories: vec![] },
                ],
            },
            PodcastCategory {
                id: 1321,
                name: "Business".to_string(),
                subcategories: vec![
                    PodcastCategory { id: 1341, name: "Careers".to_string(), subcategories: vec![] },
                    PodcastCategory { id: 1344, name: "Entrepreneurship".to_string(), subcategories: vec![] },
                    PodcastCategory { id: 1347, name: "Investing".to_string(), subcategories: vec![] },
                ],
            },
            PodcastCategory {
                id: 1303,
                name: "Comedy".to_string(),
                subcategories: vec![],
            },
            PodcastCategory {
                id: 1304,
                name: "Education".to_string(),
                subcategories: vec![
                    PodcastCategory { id: 1364, name: "How To".to_string(), subcategories: vec![] },
                    PodcastCategory { id: 1367, name: "Language Learning".to_string(), subcategories: vec![] },
                ],
            },
            PodcastCategory {
                id: 1305,
                name: "Health & Fitness".to_string(),
                subcategories: vec![
                    PodcastCategory { id: 1384, name: "Fitness".to_string(), subcategories: vec![] },
                    PodcastCategory { id: 1390, name: "Mental Health".to_string(), subcategories: vec![] },
                ],
            },
            PodcastCategory {
                id: 1307,
                name: "Music".to_string(),
                subcategories: vec![],
            },
            PodcastCategory {
                id: 1309,
                name: "News".to_string(),
                subcategories: vec![
                    PodcastCategory { id: 1414, name: "Daily News".to_string(), subcategories: vec![] },
                    PodcastCategory { id: 1423, name: "Politics".to_string(), subcategories: vec![] },
                ],
            },
            PodcastCategory {
                id: 1311,
                name: "Science".to_string(),
                subcategories: vec![],
            },
            PodcastCategory {
                id: 1313,
                name: "Society & Culture".to_string(),
                subcategories: vec![],
            },
            PodcastCategory {
                id: 1314,
                name: "Sports".to_string(),
                subcategories: vec![],
            },
            PodcastCategory {
                id: 1315,
                name: "Technology".to_string(),
                subcategories: vec![],
            },
            PodcastCategory {
                id: 1316,
                name: "True Crime".to_string(),
                subcategories: vec![],
            },
        ]
    }
}

/// Parse duration string (HH:MM:SS or MM:SS) to seconds
fn parse_duration(duration: &str) -> Option<u32> {
    let parts: Vec<&str> = duration.split(':').collect();
    match parts.len() {
        3 => {
            let hours: u32 = parts[0].parse().ok()?;
            let minutes: u32 = parts[1].parse().ok()?;
            let seconds: u32 = parts[2].parse().ok()?;
            Some(hours * 3600 + minutes * 60 + seconds)
        }
        2 => {
            let minutes: u32 = parts[0].parse().ok()?;
            let seconds: u32 = parts[1].parse().ok()?;
            Some(minutes * 60 + seconds)
        }
        1 => parts[0].parse().ok(),
        _ => None,
    }
}

/// Parse RSS date string to DateTime
fn parse_rss_date(date_str: &str) -> Option<DateTime<Utc>> {
    let formats = [
        "%a, %d %b %Y %H:%M:%S %z",
        "%a, %d %b %Y %H:%M:%S GMT",
        "%a, %d %b %Y %H:%M:%S",
        "%Y-%m-%dT%H:%M:%S%:z",
        "%Y-%m-%dT%H:%M:%SZ",
    ];
    
    for format in &formats {
        if let Ok(dt) = DateTime::parse_from_str(date_str, format) {
            return Some(dt.with_timezone(&Utc));
        }
    }
    
    date_str.parse::<DateTime<Utc>>().ok()
}

/// Parse RSS feed XML
fn parse_rss_feed(content: &str) -> Result<RSSChannel> {
    let content = content.trim();
    
    // Find channel element
    let channel_start = content.find("<channel>")
        .ok_or_else(|| anyhow::anyhow!("No channel element found"))?;
    let channel_end = content.find("</channel>")
        .ok_or_else(|| anyhow::anyhow!("Channel element not closed"))?;
    
    let channel_content = &content[channel_start + 9..channel_end];
    
    // Extract basic channel info
    let title = extract_element(channel_content, "title")
        .unwrap_or_else(|| "Untitled".to_string());
    let link = extract_element(channel_content, "link");
    let description = extract_element(channel_content, "description");
    let language = extract_element(channel_content, "language");
    
    // Extract iTunes image
    let image_url = extract_element(channel_content, "url")
        .or_else(|| extract_attribute_value(channel_content, "itunes:image", "href"));
    
    // Parse episodes
    let episodes = parse_items(channel_content);
    
    Ok(RSSChannel {
        title,
        link,
        description,
        language,
        image_url,
        categories: Vec::new(),
        explicit: None,
        author: None,
        episodes,
    })
}

/// Extract text content of an XML element
fn extract_element(content: &str, tag: &str) -> Option<String> {
    let start_tag = format!("<{}>", tag);
    let end_tag = format!("</{}>", tag);
    
    if let (Some(start), Some(end)) = (content.find(&start_tag), content.find(&end_tag)) {
        let text = &content[start + start_tag.len()..end];
        let decoded = text
            .replace("&amp;", "&")
            .replace("&lt;", "<")
            .replace("&gt;", ">")
            .replace("&quot;", "&quot;")
            .replace("&#39;", "'");
        Some(decoded.trim().to_string())
    } else {
        None
    }
}

/// Extract attribute value from an XML tag
fn extract_attribute_value(content: &str, tag: &str, attr: &str) -> Option<String> {
    let pattern = format!("<{} ", tag);
    if let Some(start) = content.find(&pattern) {
        let rest = &content[start..];
        if let Some(end) = rest.find(">") {
            let tag_content = &rest[..end];
            let attr_pattern = format!("{}=&quot;", attr);
            if let Some(attr_start) = tag_content.find(&attr_pattern) {
                let value_start = attr_start + attr_pattern.len();
                let value_rest = &tag_content[value_start..];
                if let Some(value_end) = value_rest.find("&quot;") {
                    return Some(value_rest[..value_end].to_string());
                }
            }
        }
    }
    None
}

/// Parse RSS items (episodes)
fn parse_items(content: &str) -> Vec<RSSItem> {
    let mut items = Vec::new();
    let mut remaining = content;
    
    while let Some(item_start) = remaining.find("<item>") {
        if let Some(item_end) = remaining.find("</item>") {
            let item_content = &remaining[item_start + 6..item_end];
            
            let title = extract_element(item_content, "title");
            let link = extract_element(item_content, "link");
            let description = extract_element(item_content, "description");
            let author = extract_element(item_content, "author");
            let guid = extract_element(item_content, "guid");
            let pub_date = extract_element(item_content, "pubDate");
            let duration = extract_element(item_content, "itunes:duration");
            let episode = extract_element(item_content, "itunes:episode")
                .and_then(|s| s.parse().ok());
            let season = extract_element(item_content, "itunes:season")
                .and_then(|s| s.parse().ok());
            let image_url = extract_attribute_value(item_content, "itunes:image", "href");
            
            // Parse enclosure
            let (enclosure_url, enclosure_length, enclosure_type) = 
                if let Some(enc_start) = item_content.find("<enclosure") {
                    if let Some(enc_end) = item_content[enc_start..].find(">") {
                        let enc_str = &item_content[enc_start..enc_start + enc_end];
                        let url = extract_attribute_value(enc_str, "", "url");
                        let length = extract_attribute_value(enc_str, "", "length")
                            .and_then(|s| s.parse().ok());
                        let enc_type = extract_attribute_value(enc_str, "", "type");
                        (url, length, enc_type)
                    } else {
                        (None, None, None)
                    }
                } else {
                    (None, None, None)
                };
            
            items.push(RSSItem {
                title,
                link,
                description,
                author,
                enclosure_url,
                enclosure_length,
                enclosure_type,
                guid,
                pub_date,
                duration,
                image_url,
                episode,
                season,
            });
            
            remaining = &remaining[item_end + 7..];
        } else {
            break;
        }
    }
    
    items
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_parse_duration() {
        assert_eq!(parse_duration("1:23:45"), Some(5025));
        assert_eq!(parse_duration("45:30"), Some(2730));
        assert_eq!(parse_duration("120"), Some(120));
        assert_eq!(parse_duration("invalid"), None);
    }
}