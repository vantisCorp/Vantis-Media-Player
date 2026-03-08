package com.vantis.media.player

import androidx.compose.foundation.background
import androidx.compose.foundation.clickable
import androidx.compose.foundation.layout.*
import androidx.compose.foundation.lazy.LazyColumn
import androidx.compose.foundation.lazy.LazyRow
import androidx.compose.foundation.lazy.items
import androidx.compose.foundation.shape.RoundedCornerShape
import androidx.compose.material.icons.Icons
import androidx.compose.material.icons.filled.*
import androidx.compose.material3.*
import androidx.compose.runtime.*
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.draw.clip
import androidx.compose.ui.graphics.Brush
import androidx.compose.ui.graphics.Color
import androidx.compose.ui.layout.ContentScale
import androidx.compose.ui.text.style.TextOverflow
import androidx.compose.ui.unit.dp
import coil.compose.AsyncImage

// Home Screen
@Composable
fun HomeScreen(
    viewModel: MainViewModel
) {
    val recentlyWatched by viewModel.recentlyWatched.collectAsState()
    val trending by viewModel.trending.collectAsState()
    val recommendations by viewModel.recommendations.collectAsState()
    
    LazyColumn(
        modifier = Modifier.fillMaxSize(),
        verticalArrangement = Arrangement.spacedBy(24.dp)
    ) {
        // Continue Watching Section
        if (recentlyWatched.isNotEmpty()) {
            item {
                MediaRowSection(
                    title = "Continue Watching",
                    items = recentlyWatched,
                    showProgress = true,
                    onItemClick = { viewModel.play(it) }
                )
            }
        }
        
        // Trending Section
        item {
            MediaRowSection(
                title = "Trending Now",
                items = trending,
                showProgress = false,
                onItemClick = { viewModel.play(it) }
            )
        }
        
        // Recommendations Section
        item {
            MediaRowSection(
                title = "Recommended For You",
                items = recommendations,
                showProgress = false,
                onItemClick = { viewModel.play(it) }
            )
        }
    }
}

// Media Row Section
@Composable
fun MediaRowSection(
    title: String,
    items: List<MediaItem>,
    showProgress: Boolean,
    onItemClick: (MediaItem) -> Unit
) {
    Column {
        Text(
            text = title,
            style = MaterialTheme.typography.titleLarge,
            color = MaterialTheme.colorScheme.onBackground,
            modifier = Modifier.padding(horizontal = 16.dp, vertical = 8.dp)
        )
        
        LazyRow(
            horizontalArrangement = Arrangement.spacedBy(12.dp),
            contentPadding = PaddingValues(horizontal = 16.dp)
        ) {
            items(items) { item ->
                MediaCard(
                    item = item,
                    showProgress = showProgress,
                    onClick = { onItemClick(item) }
                )
            }
        }
    }
}

// Media Card
@Composable
fun MediaCard(
    item: MediaItem,
    showProgress: Boolean,
    onClick: () -> Unit
) {
    Box(
        modifier = Modifier
            .width(if (showProgress) 160.dp else 120.dp)
            .clickable(onClick = onClick)
    ) {
        Column {
            // Thumbnail
            Box(
                modifier = Modifier
                    .fillMaxWidth()
                    .height(if (showProgress) 200.dp else 180.dp)
                    .clip(RoundedCornerShape(8.dp))
            ) {
                AsyncImage(
                    model = item.thumbnailUrl,
                    contentDescription = item.title,
                    modifier = Modifier.fillMaxSize(),
                    contentScale = ContentScale.Crop
                )
                
                // Progress bar overlay
                if (showProgress && item.watchProgress != null) {
                    Box(
                        modifier = Modifier
                            .align(Alignment.BottomCenter)
                            .fillMaxWidth()
                            .padding(4.dp)
                    ) {
                        LinearProgressIndicator(
                            progress = item.watchProgress,
                            modifier = Modifier
                                .fillMaxWidth()
                                .height(4.dp)
                                .clip(RoundedCornerShape(2.dp)),
                            color = MaterialTheme.colorScheme.primary,
                            trackColor = Color.White.copy(alpha = 0.3f)
                        )
                    }
                }
            }
            
            // Title
            Text(
                text = item.title,
                style = MaterialTheme.typography.bodySmall,
                color = MaterialTheme.colorScheme.onBackground,
                maxLines = 1,
                overflow = TextOverflow.Ellipsis,
                modifier = Modifier.padding(top = 8.dp)
            )
            
            if (item.subtitle != null) {
                Text(
                    text = item.subtitle,
                    style = MaterialTheme.typography.labelSmall,
                    color = MaterialTheme.colorScheme.onBackground.copy(alpha = 0.7f),
                    maxLines = 1,
                    overflow = TextOverflow.Ellipsis
                )
            }
        }
    }
}

// Library Screen
@Composable
fun LibraryScreen(
    viewModel: MainViewModel
) {
    val playlists by viewModel.playlists.collectAsState()
    
    LazyColumn(
        modifier = Modifier.fillMaxSize(),
        contentPadding = PaddingValues(16.dp),
        verticalArrangement = Arrangement.spacedBy(16.dp)
    ) {
        item {
            Text(
                text = "Library",
                style = MaterialTheme.typography.headlineMedium,
                color = MaterialTheme.colorScheme.onBackground
            )
        }
        
        // Playlists Section
        item {
            Text(
                text = "Playlists",
                style = MaterialTheme.typography.titleMedium,
                color = MaterialTheme.colorScheme.onBackground
            )
        }
        
        items(playlists) { playlist ->
            PlaylistRow(playlist = playlist)
        }
        
        // Quick Actions
        item {
            Row(
                modifier = Modifier.fillMaxWidth(),
                horizontalArrangement = Arrangement.spacedBy(12.dp)
            ) {
                QuickActionButton(
                    icon = Icons.Default.Favorite,
                    label = "Favorites",
                    onClick = { }
                )
                QuickActionButton(
                    icon = Icons.Default.History,
                    label = "History",
                    onClick = { }
                )
                QuickActionButton(
                    icon = Icons.Default.WatchLater,
                    label = "Watch Later",
                    onClick = { }
                )
            }
        }
    }
}

@Composable
fun PlaylistRow(playlist: Playlist) {
    Card(
        modifier = Modifier.fillMaxWidth(),
        colors = CardDefaults.cardColors(
            containerColor = MaterialTheme.colorScheme.surface
        )
    ) {
        Row(
            modifier = Modifier
                .fillMaxWidth()
                .padding(16.dp),
            verticalAlignment = Alignment.CenterVertically
        ) {
            Icon(
                imageVector = Icons.Default.PlaylistPlay,
                contentDescription = null,
                tint = MaterialTheme.colorScheme.primary,
                modifier = Modifier.size(40.dp)
            )
            
            Spacer(modifier = Modifier.width(16.dp))
            
            Column {
                Text(
                    text = playlist.name,
                    style = MaterialTheme.typography.titleMedium
                )
                Text(
                    text = "${playlist.itemCount} items",
                    style = MaterialTheme.typography.bodySmall,
                    color = MaterialTheme.colorScheme.onSurface.copy(alpha = 0.7f)
                )
            }
        }
    }
}

@Composable
fun QuickActionButton(
    icon: androidx.compose.ui.graphics.vector.ImageVector,
    label: String,
    onClick: () -> Unit
) {
    Card(
        onClick = onClick,
        modifier = Modifier.weight(1f),
        colors = CardDefaults.cardColors(
            containerColor = MaterialTheme.colorScheme.surface
        )
    ) {
        Column(
            modifier = Modifier
                .fillMaxWidth()
                .padding(16.dp),
            horizontalAlignment = Alignment.CenterHorizontally
        ) {
            Icon(
                imageVector = icon,
                contentDescription = label,
                tint = MaterialTheme.colorScheme.primary
            )
            Spacer(modifier = Modifier.height(8.dp))
            Text(
                text = label,
                style = MaterialTheme.typography.labelMedium
            )
        }
    }
}

// Search Screen
@Composable
fun SearchScreen(
    viewModel: MainViewModel
) {
    val searchQuery by viewModel.searchQuery
    val searchResults by viewModel.searchResults.collectAsState()
    
    Column(
        modifier = Modifier.fillMaxSize()
    ) {
        // Search Bar
        OutlinedTextField(
            value = searchQuery,
            onValueChange = { viewModel.updateSearchQuery(it) },
            modifier = Modifier
                .fillMaxWidth()
                .padding(16.dp),
            placeholder = { Text("Search movies, shows, videos...") },
            leadingIcon = {
                Icon(Icons.Default.Search, contentDescription = null)
            },
            trailingIcon = {
                if (searchQuery.isNotEmpty()) {
                    IconButton(onClick = { viewModel.updateSearchQuery("") }) {
                        Icon(Icons.Default.Clear, contentDescription = "Clear")
                    }
                }
            },
            singleLine = true,
            shape = RoundedCornerShape(28.dp)
        )
        
        if (searchQuery.isEmpty()) {
            // Categories Grid
            CategoriesGrid(onCategoryClick = { })
        } else {
            // Search Results
            LazyColumn(
                contentPadding = PaddingValues(16.dp),
                verticalArrangement = Arrangement.spacedBy(12.dp)
            ) {
                items(searchResults) { item ->
                    SearchResultItem(
                        item = item,
                        onClick = { viewModel.play(item) }
                    )
                }
            }
        }
    }
}

@Composable
fun CategoriesGrid(onCategoryClick: (String) -> Unit) {
    val categories = listOf(
        "Action" to Icons.Default.Whatshot,
        "Comedy" to Icons.Default.Face,
        "Drama" to Icons.Default.TheaterComedy,
        "Sci-Fi" to Icons.Default.AutoAwesome,
        "Horror" to Icons.Default.DarkMode,
        "Documentary" to Icons.Default.Description
    )
    
    LazyVerticalGrid(
        columns = GridCells.Fixed(2),
        contentPadding = PaddingValues(16.dp),
        horizontalArrangement = Arrangement.spacedBy(12.dp),
        verticalArrangement = Arrangement.spacedBy(12.dp)
    ) {
        items(categories.size) { index ->
            CategoryCard(
                name = categories[index].first,
                icon = categories[index].second,
                onClick = { onCategoryClick(categories[index].first) }
            )
        }
    }
}

@Composable
fun CategoryCard(
    name: String,
    icon: androidx.compose.ui.graphics.vector.ImageVector,
    onClick: () -> Unit
) {
    Card(
        onClick = onClick,
        modifier = Modifier
            .fillMaxWidth()
            .height(100.dp)
    ) {
        Box(
            modifier = Modifier
                .fillMaxSize()
                .background(
                    Brush.linearGradient(
                        colors = listOf(
                            MaterialTheme.colorScheme.primary,
                            MaterialTheme.colorScheme.primary.copy(alpha = 0.6f)
                        )
                    )
                ),
            contentAlignment = Alignment.Center
        ) {
            Column(horizontalAlignment = Alignment.CenterHorizontally) {
                Icon(
                    imageVector = icon,
                    contentDescription = name,
                    tint = Color.White,
                    modifier = Modifier.size(32.dp)
                )
                Spacer(modifier = Modifier.height(8.dp))
                Text(
                    text = name,
                    style = MaterialTheme.typography.titleMedium,
                    color = Color.White
                )
            }
        }
    }
}

@Composable
fun SearchResultItem(
    item: MediaItem,
    onClick: () -> Unit
) {
    Card(
        onClick = onClick,
        modifier = Modifier.fillMaxWidth()
    ) {
        Row(
            modifier = Modifier
                .fillMaxWidth()
                .padding(12.dp)
        ) {
            AsyncImage(
                model = item.thumbnailUrl,
                contentDescription = null,
                modifier = Modifier
                    .width(80.dp)
                    .height(120.dp)
                    .clip(RoundedCornerShape(8.dp)),
                contentScale = ContentScale.Crop
            )
            
            Spacer(modifier = Modifier.width(16.dp))
            
            Column {
                Text(
                    text = item.title,
                    style = MaterialTheme.typography.titleMedium
                )
                
                if (item.subtitle != null) {
                    Text(
                        text = item.subtitle,
                        style = MaterialTheme.typography.bodySmall,
                        color = MaterialTheme.colorScheme.onSurface.copy(alpha = 0.7f)
                    )
                }
                
                item.releaseYear?.let { year ->
                    Text(
                        text = "$year • ${item.rating ?: ""}",
                        style = MaterialTheme.typography.labelSmall,
                        color = MaterialTheme.colorScheme.onSurface.copy(alpha = 0.6f)
                    )
                }
                
                Text(
                    text = item.genres.take(3).joinToString(", "),
                    style = MaterialTheme.typography.labelSmall,
                    color = MaterialTheme.colorScheme.primary,
                    modifier = Modifier.padding(top = 4.dp)
                )
            }
        }
    }
}

// Downloads Screen
@Composable
fun DownloadsScreen(
    viewModel: MainViewModel
) {
    val downloads by viewModel.downloads.collectAsState()
    
    LazyColumn(
        modifier = Modifier.fillMaxSize(),
        contentPadding = PaddingValues(16.dp),
        verticalArrangement = Arrangement.spacedBy(12.dp)
    ) {
        item {
            Text(
                text = "Downloads",
                style = MaterialTheme.typography.headlineMedium
            )
        }
        
        if (downloads.isEmpty()) {
            item {
                Box(
                    modifier = Modifier
                        .fillMaxWidth()
                        .height(300.dp),
                    contentAlignment = Alignment.Center
                ) {
                    Column(horizontalAlignment = Alignment.CenterHorizontally) {
                        Icon(
                            imageVector = Icons.Default.Download,
                            contentDescription = null,
                            modifier = Modifier.size(64.dp),
                            tint = MaterialTheme.colorScheme.onSurface.copy(alpha = 0.5f)
                        )
                        Spacer(modifier = Modifier.height(16.dp))
                        Text(
                            text = "No Downloads",
                            style = MaterialTheme.typography.titleMedium,
                            color = MaterialTheme.colorScheme.onSurface.copy(alpha = 0.7f)
                        )
                        Text(
                            text = "Download content to watch offline",
                            style = MaterialTheme.typography.bodyMedium,
                            color = MaterialTheme.colorScheme.onSurface.copy(alpha = 0.5f)
                        )
                    }
                }
            }
        } else {
            items(downloads) { download ->
                DownloadItemCard(
                    download = download,
                    onPause = { viewModel.pauseDownload(download.id) },
                    onCancel = { viewModel.cancelDownload(download.id) }
                )
            }
        }
    }
}

@Composable
fun DownloadItemCard(
    download: DownloadItem,
    onPause: () -> Unit,
    onCancel: () -> Unit
) {
    Card(
        modifier = Modifier.fillMaxWidth()
    ) {
        Row(
            modifier = Modifier
                .fillMaxWidth()
                .padding(12.dp)
        ) {
            AsyncImage(
                model = download.mediaItem.thumbnailUrl,
                contentDescription = null,
                modifier = Modifier
                    .width(60.dp)
                    .height(90.dp)
                    .clip(RoundedCornerShape(8.dp)),
                contentScale = ContentScale.Crop
            )
            
            Spacer(modifier = Modifier.width(12.dp))
            
            Column(modifier = Modifier.weight(1f)) {
                Text(
                    text = download.mediaItem.title,
                    style = MaterialTheme.typography.titleSmall
                )
                
                Text(
                    text = formatBytes(download.totalBytes),
                    style = MaterialTheme.typography.labelSmall,
                    color = MaterialTheme.colorScheme.onSurface.copy(alpha = 0.7f)
                )
                
                if (download.status == DownloadStatus.Downloading) {
                    Spacer(modifier = Modifier.height(8.dp))
                    LinearProgressIndicator(
                        progress = download.progress,
                        modifier = Modifier.fillMaxWidth(),
                        color = MaterialTheme.colorScheme.primary
                    )
                    Text(
                        text = "${(download.progress * 100).toInt()}%",
                        style = MaterialTheme.typography.labelSmall,
                        color = MaterialTheme.colorScheme.primary
                    )
                } else if (download.status == DownloadStatus.Completed) {
                    Row(
                        modifier = Modifier.padding(top = 4.dp),
                        verticalAlignment = Alignment.CenterVertically
                    ) {
                        Icon(
                            imageVector = Icons.Default.CheckCircle,
                            contentDescription = null,
                            tint = Color.Green,
                            modifier = Modifier.size(16.dp)
                        )
                        Spacer(modifier = Modifier.width(4.dp))
                        Text(
                            text = "Completed",
                            style = MaterialTheme.typography.labelSmall,
                            color = Color.Green
                        )
                    }
                }
            }
            
            if (download.status == DownloadStatus.Downloading) {
                IconButton(onClick = onPause) {
                    Icon(Icons.Default.Pause, contentDescription = "Pause")
                }
            }
        }
    }
}

// Settings Screen
@Composable
fun SettingsScreen(
    viewModel: MainViewModel
) {
    val isCloudSyncEnabled by viewModel.isCloudSyncEnabled
    val syncStatus by viewModel.syncStatus
    
    LazyColumn(
        modifier = Modifier.fillMaxSize(),
        contentPadding = PaddingValues(16.dp),
        verticalArrangement = Arrangement.spacedBy(16.dp)
    ) {
        item {
            Text(
                text = "Settings",
                style = MaterialTheme.typography.headlineMedium
            )
        }
        
        // Playback Settings
        item {
            Text(
                text = "Playback",
                style = MaterialTheme.typography.titleMedium,
                color = MaterialTheme.colorScheme.primary
            )
        }
        
        item {
            var autoPlay by remember { mutableStateOf(true) }
            var skipIntros by remember { mutableStateOf(true) }
            
            Card {
                Column {
                    SettingToggle(
                        title = "Auto Play",
                        subtitle = "Automatically play next episode",
                        checked = autoPlay,
                        onCheckedChange = { autoPlay = it }
                    )
                    HorizontalDivider()
                    SettingToggle(
                        title = "Skip Intros",
                        subtitle = "Skip opening credits when available",
                        checked = skipIntros,
                        onCheckedChange = { skipIntros = it }
                    )
                }
            }
        }
        
        // Cloud Sync Settings
        item {
            Text(
                text = "Cloud Sync",
                style = MaterialTheme.typography.titleMedium,
                color = MaterialTheme.colorScheme.primary
            )
        }
        
        item {
            Card {
                Column {
                    SettingToggle(
                        title = "Enable Cloud Sync",
                        subtitle = "Sync settings and playlists across devices",
                        checked = isCloudSyncEnabled,
                        onCheckedChange = { viewModel.toggleCloudSync() }
                    )
                    
                    if (isCloudSyncEnabled) {
                        HorizontalDivider()
                        Row(
                            modifier = Modifier
                                .fillMaxWidth()
                                .padding(16.dp),
                            horizontalArrangement = Arrangement.SpaceBetween,
                            verticalAlignment = Alignment.CenterVertically
                        ) {
                            Text(
                                text = "Status",
                                style = MaterialTheme.typography.bodyMedium
                            )
                            Text(
                                text = syncStatus.name,
                                style = MaterialTheme.typography.labelMedium,
                                color = when (syncStatus) {
                                    SyncStatus.Synced -> Color.Green
                                    SyncStatus.Syncing -> MaterialTheme.colorScheme.primary
                                    SyncStatus.Error -> MaterialTheme.colorScheme.error
                                    else -> MaterialTheme.colorScheme.onSurface.copy(alpha = 0.7f)
                                }
                            )
                        }
                    }
                }
            }
        }
        
        // About
        item {
            Text(
                text = "About",
                style = MaterialTheme.typography.titleMedium,
                color = MaterialTheme.colorScheme.primary
            )
        }
        
        item {
            Card {
                Column {
                    Row(
                        modifier = Modifier
                            .fillMaxWidth()
                            .padding(16.dp),
                        horizontalArrangement = Arrangement.SpaceBetween
                    ) {
                        Text("Version")
                        Text(
                            "1.3.0",
                            color = MaterialTheme.colorScheme.onSurface.copy(alpha = 0.7f)
                        )
                    }
                    HorizontalDivider()
                    Row(
                        modifier = Modifier
                            .fillMaxWidth()
                            .padding(16.dp),
                        horizontalArrangement = Arrangement.SpaceBetween
                    ) {
                        Text("Build")
                        Text(
                            "130",
                            color = MaterialTheme.colorScheme.onSurface.copy(alpha = 0.7f)
                        )
                    }
                }
            }
        }
    }
}

@Composable
fun SettingToggle(
    title: String,
    subtitle: String?,
    checked: Boolean,
    onCheckedChange: (Boolean) -> Unit
) {
    Row(
        modifier = Modifier
            .fillMaxWidth()
            .padding(16.dp),
        horizontalArrangement = Arrangement.SpaceBetween,
        verticalAlignment = Alignment.CenterVertically
    ) {
        Column(modifier = Modifier.weight(1f)) {
            Text(
                text = title,
                style = MaterialTheme.typography.bodyMedium
            )
            if (subtitle != null) {
                Text(
                    text = subtitle,
                    style = MaterialTheme.typography.bodySmall,
                    color = MaterialTheme.colorScheme.onSurface.copy(alpha = 0.7f)
                )
            }
        }
        
        Switch(
            checked = checked,
            onCheckedChange = onCheckedChange
        )
    }
}

// Helper functions
fun formatBytes(bytes: Long): String {
    if (bytes < 1024) return "$bytes B"
    val kb = bytes / 1024.0
    if (kb < 1024) return "%.1f KB".format(kb)
    val mb = kb / 1024.0
    if (mb < 1024) return "%.1f MB".format(mb)
    val gb = mb / 1024.0
    return "%.1f GB".format(gb)
}