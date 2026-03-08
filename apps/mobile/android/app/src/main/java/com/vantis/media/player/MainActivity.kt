package com.vantis.media.player

import android.os.Bundle
import androidx.activity.ComponentActivity
import androidx.activity.compose.setContent
import androidx.activity.enableEdgeToEdge
import androidx.compose.foundation.layout.fillMaxSize
import androidx.compose.material3.MaterialTheme
import androidx.compose.material3.Surface
import androidx.compose.ui.Modifier
import androidx.hilt.navigation.compose.hiltViewModel
import dagger.hilt.android.AndroidEntryPoint

@AndroidEntryPoint
class MainActivity : ComponentActivity() {
    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)
        enableEdgeToEdge()
        
        setContent {
            VantisPlayerTheme {
                Surface(
                    modifier = Modifier.fillMaxSize(),
                    color = MaterialTheme.colorScheme.background
                ) {
                    VantisPlayerApp()
                }
            }
        }
    }
}

// Theme definition
@androidx.compose.runtime.Composable
fun VantisPlayerTheme(
    darkTheme: Boolean = true,
    dynamicColor: Boolean = false,
    content: @androidx.compose.runtime.Composable () -> Unit
) {
    val colorScheme = androidx.compose.material3.darkColorScheme(
        primary = androidx.compose.ui.graphics.Color(0xFFDC143C), // Crimson
        onPrimary = androidx.compose.ui.graphics.Color.White,
        primaryContainer = androidx.compose.ui.graphics.Color(0xFF4A0E1E),
        onPrimaryContainer = androidx.compose.ui.graphics.Color(0xFFFFD9DC),
        secondary = androidx.compose.ui.graphics.Color(0xFFBB86FC),
        background = androidx.compose.ui.graphics.Color(0xFF121212),
        surface = androidx.compose.ui.graphics.Color(0xFF1E1E1E),
        onBackground = androidx.compose.ui.graphics.Color.White,
        onSurface = androidx.compose.ui.graphics.Color.White
    )
    
    androidx.compose.material3.MaterialTheme(
        colorScheme = colorScheme,
        typography = androidx.compose.material3.Typography(),
        content = content
    )
}

// Main App Composable
@androidx.compose.runtime.Composable
fun VantisPlayerApp(
    viewModel: MainViewModel = hiltViewModel()
) {
    val navController = androidx.navigation.compose.rememberNavController()
    
    androidx.navigation.compose.NavHost(
        navController = navController,
        startDestination = "main"
    ) {
        androidx.navigation.compose.composable("main") {
            MainScreen(
                onNavigateToPlayer = { navController.navigate("player") },
                viewModel = viewModel
            )
        }
        androidx.navigation.compose.composable("player") {
            FullScreenPlayerScreen(
                onBack = { navController.popBackStack() }
            )
        }
    }
}

// Main Screen with Bottom Navigation
@androidx.compose.runtime.Composable
fun MainScreen(
    onNavigateToPlayer: () -> Unit,
    viewModel: MainViewModel
) {
    val selectedTab by viewModel.selectedTab
    val showMiniPlayer by viewModel.showMiniPlayer
    
    androidx.compose.foundation.layout.Box(
        modifier = androidx.compose.foundation.layout.fillMaxSize()
    ) {
        androidx.compose.foundation.layout.Column(
            modifier = androidx.compose.foundation.layout.fillMaxSize()
        ) {
            // Content based on selected tab
            androidx.compose.foundation.layout.weight(1f) {
                when (selectedTab) {
                    Tab.Home -> HomeScreen(viewModel = viewModel)
                    Tab.Library -> LibraryScreen(viewModel = viewModel)
                    Tab.Search -> SearchScreen(viewModel = viewModel)
                    Tab.Downloads -> DownloadsScreen(viewModel = viewModel)
                    Tab.Settings -> SettingsScreen(viewModel = viewModel)
                }
            }
            
            // Mini Player
            if (showMiniPlayer) {
                MiniPlayer(
                    onClick = onNavigateToPlayer,
                    viewModel = viewModel
                )
            }
            
            // Bottom Navigation
            BottomNavigationBar(
                selectedTab = selectedTab,
                onTabSelected = { viewModel.selectTab(it) }
            )
        }
    }
}

// Tab enum
enum class Tab(val title: String, val icon: androidx.compose.ui.graphics.vector.ImageVector) {
    Home("Home", androidx.compose.material.icons.Icons.Filled.Home),
    Library("Library", androidx.compose.material.icons.Icons.Filled.VideoLibrary),
    Search("Search", androidx.compose.material.icons.Icons.Filled.Search),
    Downloads("Downloads", androidx.compose.material.icons.Icons.Filled.Download),
    Settings("Settings", androidx.compose.material.icons.Icons.Filled.Settings)
}

// Bottom Navigation Bar
@androidx.compose.runtime.Composable
fun BottomNavigationBar(
    selectedTab: Tab,
    onTabSelected: (Tab) -> Unit
) {
    androidx.compose.material3.NavigationBar(
        containerColor = androidx.compose.material3.MaterialTheme.colorScheme.surface
    ) {
        Tab.entries.forEach { tab ->
            androidx.compose.material3.NavigationBarItem(
                selected = selectedTab == tab,
                onClick = { onTabSelected(tab) },
                icon = {
                    androidx.compose.material3.Icon(
                        imageVector = tab.icon,
                        contentDescription = tab.title
                    )
                },
                label = {
                    androidx.compose.material3.Text(tab.title)
                },
                colors = androidx.compose.material3.NavigationBarItemDefaults.colors(
                    selectedIconColor = androidx.compose.material3.MaterialTheme.colorScheme.primary,
                    selectedTextColor = androidx.compose.material3.MaterialTheme.colorScheme.primary
                )
            )
        }
    }
}