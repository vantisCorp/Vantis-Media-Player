package com.vantis.media.player

import androidx.compose.runtime.State
import androidx.compose.runtime.mutableStateOf
import androidx.lifecycle.ViewModel
import androidx.lifecycle.viewModelScope
import androidx.media3.exoplayer.ExoPlayer
import dagger.hilt.android.lifecycle.HiltViewModel
import kotlinx.coroutines.Job
import kotlinx.coroutines.delay
import kotlinx.coroutines.flow.MutableStateFlow
import kotlinx.coroutines.flow.StateFlow
import kotlinx.coroutines.flow.asStateFlow
import kotlinx.coroutines.launch
import javax.inject.Inject

@HiltViewModel
class MainViewModel @Inject constructor(
    private val playerManager: PlayerManager
) : ViewModel() {
    
    // Navigation state
    private val _selectedTab = mutableStateOf(Tab.Home)
    val selectedTab: State<Tab> = _selectedTab
    
    // Player state
    private val _showMiniPlayer = mutableStateOf(false)
    val showMiniPlayer: State<Boolean> = _showMiniPlayer
    
    private val _currentMediaItem = mutableStateOf<MediaItem?>(null)
    val currentMediaItem: State<MediaItem?> = _currentMediaItem
    
    private val _isPlaying = mutableStateOf(false)
    val isPlaying: State<Boolean> = _isPlaying
    
    private val _playbackPosition = mutableStateOf(0L)
    val playbackPosition: State<Long> = _playbackPosition
    
    private val _duration = mutableStateOf(0L)
    val duration: State<Long> = _duration
    
    // Library state
    private val _recentlyWatched = MutableStateFlow<List<MediaItem>>(emptyList())
    val recentlyWatched: StateFlow<List<MediaItem>> = _recentlyWatched.asStateFlow()
    
    private val _trending = MutableStateFlow<List<MediaItem>>(emptyList())
    val trending: StateFlow<List<MediaItem>> = _trending.asStateFlow()
    
    private val _recommendations = MutableStateFlow<List<MediaItem>>(emptyList())
    val recommendations: StateFlow<List<MediaItem>> = _recommendations.asStateFlow()
    
    private val _playlists = MutableStateFlow<List<Playlist>>(emptyList())
    val playlists: StateFlow<List<Playlist>> = _playlists.asStateFlow()
    
    private val _downloads = MutableStateFlow<List<DownloadItem>>(emptyList())
    val downloads: StateFlow<List<DownloadItem>> = _downloads.asStateFlow()
    
    // Settings state
    private val _isCloudSyncEnabled = mutableStateOf(false)
    val isCloudSyncEnabled: State<Boolean> = _isCloudSyncEnabled
    
    private val _syncStatus = mutableStateOf(SyncStatus.NotConnected)
    val syncStatus: State<SyncStatus> = _syncStatus
    
    // Search state
    private val _searchQuery = mutableStateOf("")
    val searchQuery: State<String> = _searchQuery
    
    private val _searchResults = MutableStateFlow<List<MediaItem>>(emptyList())
    val searchResults: StateFlow<List<MediaItem>> = _searchResults.asStateFlow()
    
    private var positionUpdateJob: Job? = null
    
    init {
        loadMockData()
    }
    
    // Navigation
    fun selectTab(tab: Tab) {
        _selectedTab.value = tab
    }
    
    // Player controls
    fun play(item: MediaItem) {
        _currentMediaItem.value = item
        _showMiniPlayer.value = true
        _duration.value = item.duration * 1000
        playerManager.play(item)
        startPositionUpdate()
    }
    
    fun pause() {
        playerManager.pause()
        _isPlaying.value = false
    }
    
    fun resume() {
        playerManager.resume()
        _isPlaying.value = true
    }
    
    fun stop() {
        playerManager.stop()
        _currentMediaItem.value = null
        _showMiniPlayer.value = false
        _isPlaying.value = false
        _playbackPosition.value = 0
        stopPositionUpdate()
    }
    
    fun seekTo(position: Long) {
        playerManager.seekTo(position)
        _playbackPosition.value = position
    }
    
    fun seekRelative(offsetMs: Long) {
        val newPosition = (_playbackPosition.value + offsetMs).coerceIn(0, _duration.value)
        seekTo(newPosition)
    }
    
    private fun startPositionUpdate() {
        positionUpdateJob?.cancel()
        positionUpdateJob = viewModelScope.launch {
            while (true) {
                _playbackPosition.value = playerManager.getCurrentPosition()
                _isPlaying.value = playerManager.isPlaying()
                delay(500)
            }
        }
    }
    
    private fun stopPositionUpdate() {
        positionUpdateJob?.cancel()
        positionUpdateJob = null
    }
    
    // Library management
    fun addToPlaylist(item: MediaItem, playlistId: String) {
        viewModelScope.launch {
            // Add to playlist
        }
    }
    
    fun addToList(item: MediaItem) {
        viewModelScope.launch {
            // Add to my list
        }
    }
    
    fun removeFromList(item: MediaItem) {
        viewModelScope.launch {
            // Remove from my list
        }
    }
    
    // Downloads
    fun startDownload(item: MediaItem) {
        viewModelScope.launch {
            val download = DownloadItem(
                id = java.util.UUID.randomUUID().toString(),
                mediaItem = item,
                status = DownloadStatus.Pending,
                progress = 0f,
                downloadedBytes = 0L,
                totalBytes = 1_500_000_000L
            )
            _downloads.value = _downloads.value + download
            simulateDownload(download.id)
        }
    }
    
    private fun simulateDownload(downloadId: String) {
        viewModelScope.launch {
            var progress = 0f
            while (progress < 1f) {
                delay(500)
                progress += 0.1f
                _downloads.value = _downloads.value.map { download ->
                    if (download.id == downloadId) {
                        download.copy(
                            progress = progress.coerceAtMost(1f),
                            status = if (progress >= 1f) DownloadStatus.Completed else DownloadStatus.Downloading,
                            downloadedBytes = (progress * download.totalBytes).toLong()
                        )
                    } else download
                }
            }
        }
    }
    
    fun pauseDownload(downloadId: String) {
        _downloads.value = _downloads.value.map { download ->
            if (download.id == downloadId) {
                download.copy(status = DownloadStatus.Paused)
            } else download
        }
    }
    
    fun cancelDownload(downloadId: String) {
        _downloads.value = _downloads.value.filter { it.id != downloadId }
    }
    
    // Search
    fun updateSearchQuery(query: String) {
        _searchQuery.value = query
        if (query.isNotEmpty()) {
            searchItems(query)
        }
    }
    
    private fun searchItems(query: String) {
        viewModelScope.launch {
            // Filter items based on query
            val allItems = _recentlyWatched.value + _trending.value + _recommendations.value
            _searchResults.value = allItems.filter { 
                it.title.contains(query, ignoreCase = true) 
            }
        }
    }
    
    // Cloud Sync
    fun toggleCloudSync() {
        _isCloudSyncEnabled.value = !_isCloudSyncEnabled.value
        if (_isCloudSyncEnabled.value) {
            startSync()
        }
    }
    
    private fun startSync() {
        viewModelScope.launch {
            _syncStatus.value = SyncStatus.Syncing
            delay(2000)
            _syncStatus.value = SyncStatus.Synced
        }
    }
    
    // Mock data
    private fun loadMockData() {
        _recentlyWatched.value = listOf(
            MediaItem(
                id = "1",
                title = "The Last Kingdom",
                subtitle = "Season 5, Episode 8",
                description = "As Edward's army is outmaneuvered, Uhtred must make a difficult choice.",
                thumbnailUrl = "https://picsum.photos/seed/movie1/400/600",
                posterUrl = "https://picsum.photos/seed/movie1/800/1200",
                streamUrl = "https://example.com/stream1.m3u8",
                duration = 3600L,
                contentType = ContentType.Episode,
                genres = listOf("Drama", "Action", "History"),
                releaseYear = 2022,
                rating = "TV-MA",
                watchProgress = 0.65f
            ),
            MediaItem(
                id = "2",
                title = "Stranger Things",
                subtitle = "Season 4, Episode 1",
                description = "A chilling new mystery unfolds in Hawkins.",
                thumbnailUrl = "https://picsum.photos/seed/movie2/400/600",
                posterUrl = "https://picsum.photos/seed/movie2/800/1200",
                streamUrl = "https://example.com/stream2.m3u8",
                duration = 4800L,
                contentType = ContentType.Episode,
                genres = listOf("Sci-Fi", "Horror", "Drama"),
                releaseYear = 2022,
                rating = "TV-14",
                watchProgress = 0.3f
            )
        )
        
        _trending.value = listOf(
            MediaItem(
                id = "3",
                title = "Oppenheimer",
                subtitle = null,
                description = "The story of American scientist J. Robert Oppenheimer.",
                thumbnailUrl = "https://picsum.photos/seed/movie3/400/600",
                posterUrl = "https://picsum.photos/seed/movie3/800/1200",
                streamUrl = "https://example.com/stream3.m3u8",
                duration = 10800L,
                contentType = ContentType.Movie,
                genres = listOf("Drama", "History", "Biography"),
                releaseYear = 2023,
                rating = "R",
                watchProgress = null
            ),
            MediaItem(
                id = "4",
                title = "The Bear",
                subtitle = null,
                description = "A young chef returns to Chicago to run his family sandwich shop.",
                thumbnailUrl = "https://picsum.photos/seed/movie4/400/600",
                posterUrl = "https://picsum.photos/seed/movie4/800/1200",
                streamUrl = "https://example.com/stream4.m3u8",
                duration = 1800L,
                contentType = ContentType.Series,
                genres = listOf("Comedy", "Drama"),
                releaseYear = 2022,
                rating = "TV-MA",
                watchProgress = null
            )
        )
        
        _recommendations.value = listOf(
            MediaItem(
                id = "5",
                title = "Dune: Part Two",
                subtitle = null,
                description = "Paul Atreides unites with Chani and the Fremen.",
                thumbnailUrl = "https://picsum.photos/seed/movie5/400/600",
                posterUrl = "https://picsum.photos/seed/movie5/800/1200",
                streamUrl = "https://example.com/stream5.m3u8",
                duration = 10200L,
                contentType = ContentType.Movie,
                genres = listOf("Sci-Fi", "Adventure", "Drama"),
                releaseYear = 2024,
                rating = "PG-13",
                watchProgress = null
            )
        )
        
        _playlists.value = listOf(
            Playlist(
                id = "p1",
                name = "Watch Later",
                description = "Movies and shows to watch later",
                items = emptyList(),
                itemCount = 2,
                createdAt = System.currentTimeMillis()
            )
        )
    }
}

// Data classes
data class MediaItem(
    val id: String,
    val title: String,
    val subtitle: String?,
    val description: String?,
    val thumbnailUrl: String?,
    val posterUrl: String?,
    val streamUrl: String,
    val duration: Long,
    val contentType: ContentType,
    val genres: List<String>,
    val releaseYear: Int?,
    val rating: String?,
    val watchProgress: Float?
)

enum class ContentType {
    Movie, Series, Episode, Documentary, Live, Video
}

data class Playlist(
    val id: String,
    val name: String,
    val description: String?,
    val items: List<MediaItem>,
    val itemCount: Int,
    val createdAt: Long
)

data class DownloadItem(
    val id: String,
    val mediaItem: MediaItem,
    val status: DownloadStatus,
    val progress: Float,
    val downloadedBytes: Long,
    val totalBytes: Long
)

enum class DownloadStatus {
    Pending, Downloading, Paused, Completed, Failed
}

enum class SyncStatus {
    NotConnected, Connecting, Syncing, Synced, Error
}