//! P2P streaming module
//! 
//! Provides peer-to-peer streaming capabilities using libp2p for
//! distributed content delivery and reduced server load.

use crate::{StreamingError, StreamingResult};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::{Duration, Instant};

/// P2P configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct P2PConfig {
    /// Enable P2P streaming
    pub enabled: bool,
    
    /// Listen address
    pub listen_address: String,
    
    /// Bootstrap peers
    pub bootstrap_peers: Vec<String>,
    
    /// Maximum number of peers
    pub max_peers: usize,
    
    /// Enable NAT traversal
    pub enable_nat_traversal: bool,
    
    /// Enable mDNS discovery
    pub enable_mdns: bool,
    
    /// Enable DHT (Distributed Hash Table)
    pub enable_dht: bool,
    
    /// Enable gossipsub (pub/sub)
    pub enable_gossipsub: bool,
    
    /// Connection timeout in seconds
    pub connection_timeout_secs: u64,
    
    /// Keep-alive interval in seconds
    pub keep_alive_interval_secs: u64,
}

impl Default for P2PConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            listen_address: "/ip4/0.0.0.0/tcp/0".to_string(),
            bootstrap_peers: Vec::new(),
            max_peers: 50,
            enable_nat_traversal: true,
            enable_mdns: true,
            enable_dht: true,
            enable_gossipsub: true,
            connection_timeout_secs: 30,
            keep_alive_interval_secs: 60,
        }
    }
}

/// Peer information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PeerInfo {
    /// Peer ID
    pub peer_id: String,
    
    /// Addresses
    pub addresses: Vec<String>,
    
    /// Connected since
    pub connected_since: u64,
    
    /// Last seen
    pub last_seen: u64,
    
    /// Latency in milliseconds
    pub latency_ms: u64,
    
    /// Upload speed in bps
    pub upload_speed_bps: u64,
    
    /// Download speed in bps
    pub download_speed_bps: u64,
    
    /// Is seed (has complete content)
    pub is_seed: bool,
    
    /// Peer capabilities
    pub capabilities: Vec<String>,
}

impl Default for PeerInfo {
    fn default() -> Self {
        Self {
            peer_id: String::new(),
            addresses: Vec::new(),
            connected_since: 0,
            last_seen: 0,
            latency_ms: 0,
            upload_speed_bps: 0,
            download_speed_bps: 0,
            is_seed: false,
            capabilities: Vec::new(),
        }
    }
}

/// Content piece
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContentPiece {
    /// Piece index
    pub index: usize,
    
    /// Piece hash
    pub hash: String,
    
    /// Piece size in bytes
    pub size: usize,
    
    /// Piece data
    pub data: Option<Vec<u8>>,
}

/// Content manifest
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContentManifest {
    /// Content ID
    pub content_id: String,
    
    /// Content name
    pub name: String,
    
    /// Total size in bytes
    pub total_size: u64,
    
    /// Piece size in bytes
    pub piece_size: usize,
    
    /// Number of pieces
    pub piece_count: usize,
    
    /// Root hash
    pub root_hash: String,
    
    /// Piece hashes
    pub piece_hashes: Vec<String>,
    
    /// Metadata
    pub metadata: HashMap<String, String>,
}

/// P2P streamer
pub struct P2PStreamer {
    config: P2PConfig,
    peers: HashMap<String, PeerInfo>,
    content_manifests: HashMap<String, ContentManifest>,
    downloaded_pieces: HashMap<String, Vec<usize>>,
    stats: P2PStats,
}

impl P2PStreamer {
    /// Create a new P2P streamer
    pub fn new() -> StreamingResult<Self> {
        Self::with_config(P2PConfig::default())
    }
    
    /// Create a new P2P streamer with custom configuration
    pub fn with_config(config: P2PConfig) -> StreamingResult<Self> {
        Ok(Self {
            config,
            peers: HashMap::new(),
            content_manifests: HashMap::new(),
            downloaded_pieces: HashMap::new(),
            stats: P2PStats::default(),
        })
    }
    
    /// Start P2P streaming
    pub fn start(&mut self) -> StreamingResult<()> {
        if !self.config.enabled {
            return Err(StreamingError::P2PError(
                "P2P streaming is disabled".to_string()
            ));
        }
        
        // Initialize libp2p swarm
        // This is a simplified implementation
        // In production, this would create and start the actual libp2p swarm
        
        Ok(())
    }
    
    /// Stop P2P streaming
    pub fn stop(&mut self) -> StreamingResult<()> {
        // Stop libp2p swarm
        Ok(())
    }
    
    /// Connect to a peer
    pub fn connect(&mut self, peer_id: String, address: String) -> StreamingResult<()> {
        // Check max peers
        if self.peers.len() >= self.config.max_peers {
            return Err(StreamingError::P2PError(
                "Maximum number of peers reached".to_string()
            ));
        }
        
        // Add peer
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs();
        
        let peer_info = PeerInfo {
            peer_id: peer_id.clone(),
            addresses: vec![address],
            connected_since: now,
            last_seen: now,
            ..Default::default()
        };
        
        self.peers.insert(peer_id, peer_info);
        self.stats.connected_peers = self.peers.len();
        
        Ok(())
    }
    
    /// Disconnect from a peer
    pub fn disconnect(&mut self, peer_id: &str) -> StreamingResult<()> {
        self.peers.remove(peer_id);
        self.stats.connected_peers = self.peers.len();
        Ok(())
    }
    
    /// Get list of connected peers
    pub fn peers(&self) -> Vec<PeerInfo> {
        self.peers.values().cloned().collect()
    }
    
    /// Publish content manifest
    pub fn publish_content(&mut self, manifest: ContentManifest) -> StreamingResult<()> {
        self.content_manifests.insert(manifest.content_id.clone(), manifest);
        self.stats.published_content += 1;
        Ok(())
    }
    
    /// Subscribe to content
    pub fn subscribe_content(&mut self, content_id: String) -> StreamingResult<()> {
        self.downloaded_pieces.insert(content_id, Vec::new());
        self.stats.subscribed_content += 1;
        Ok(())
    }
    
    /// Request a piece from peers
    pub fn request_piece(&mut self, content_id: &str, piece_index: usize) -> StreamingResult<ContentPiece> {
        // Find peers that have this piece
        let peers_with_piece: Vec<_> = self.peers
            .iter()
            .filter(|(_, peer)| peer.is_seed)
            .collect();
        
        if peers_with_piece.is_empty() {
            return Err(StreamingError::P2PError(
                "No peers available for piece".to_string()
            ));
        }
        
        // Request piece from peer (simplified)
        let piece = ContentPiece {
            index: piece_index,
            hash: String::new(),
            size: 0,
            data: None,
        };
        
        // Track download
        if let Some(pieces) = self.downloaded_pieces.get_mut(content_id) {
            pieces.push(piece_index);
        }
        
        self.stats.pieces_downloaded += 1;
        
        Ok(piece)
    }
    
    /// Serve a piece to peers
    pub fn serve_piece(&mut self, content_id: &str, piece: ContentPiece) -> StreamingResult<()> {
        self.stats.pieces_uploaded += 1;
        Ok(())
    }
    
    /// Get download progress
    pub fn download_progress(&self, content_id: &str) -> f32 {
        if let Some(manifest) = self.content_manifests.get(content_id) {
            if let Some(downloaded) = self.downloaded_pieces.get(content_id) {
                return downloaded.len() as f32 / manifest.piece_count as f32;
            }
        }
        0.0
    }
    
    /// Get P2P statistics
    pub fn stats(&self) -> &P2PStats {
        &self.stats
    }
    
    /// Find best peer for downloading
    pub fn find_best_peer(&self, content_id: &str) -> Option<&PeerInfo> {
        self.peers
            .values()
            .filter(|peer| peer.is_seed)
            .min_by_key(|peer| peer.latency_ms)
    }
}

/// P2P statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct P2PStats {
    /// Number of connected peers
    pub connected_peers: usize,
    
    /// Number of published content items
    pub published_content: usize,
    
    /// Number of subscribed content items
    pub subscribed_content: usize,
    
    /// Pieces downloaded
    pub pieces_downloaded: usize,
    
    /// Pieces uploaded
    pub pieces_uploaded: usize,
    
    /// Total bytes downloaded
    pub bytes_downloaded: u64,
    
    /// Total bytes uploaded
    pub bytes_uploaded: u64,
    
    /// Average download speed in bps
    pub avg_download_speed_bps: u64,
    
    /// Average upload speed in bps
    pub avg_upload_speed_bps: u64,
}

impl Default for P2PStats {
    fn default() -> Self {
        Self {
            connected_peers: 0,
            published_content: 0,
            subscribed_content: 0,
            pieces_downloaded: 0,
            pieces_uploaded: 0,
            bytes_downloaded: 0,
            bytes_uploaded: 0,
            avg_download_speed_bps: 0,
            avg_upload_speed_bps: 0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_p2p_config_default() {
        let config = P2PConfig::default();
        assert!(config.enabled);
        assert_eq!(config.max_peers, 50);
    }
    
    #[test]
    fn test_p2p_streamer_creation() {
        let streamer = P2PStreamer::new();
        assert!(streamer.is_ok());
    }
    
    #[test]
    fn test_connect_peer() {
        let mut streamer = P2PStreamer::new().unwrap();
        streamer.connect("peer1".to_string(), "/ip4/127.0.0.1/tcp/1234".to_string()).unwrap();
        assert_eq!(streamer.peers().len(), 1);
    }
    
    #[test]
    fn test_disconnect_peer() {
        let mut streamer = P2PStreamer::new().unwrap();
        streamer.connect("peer1".to_string(), "/ip4/127.0.0.1/tcp/1234".to_string()).unwrap();
        streamer.disconnect("peer1").unwrap();
        assert_eq!(streamer.peers().len(), 0);
    }
}