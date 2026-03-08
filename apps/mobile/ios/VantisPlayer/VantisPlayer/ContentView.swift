import SwiftUI

struct ContentView: View {
    @EnvironmentObject var appState: AppState
    @EnvironmentObject var playerManager: PlayerManager
    @Environment(\.verticalSizeClass) var verticalSizeClass
    
    var body: some View {
        ZStack {
            // Main tab view
            TabView(selection: $appState.selectedTab) {
                HomeView()
                    .tabItem {
                        Label("Home", systemImage: AppState.Tab.home.icon)
                    }
                    .tag(AppState.Tab.home)
                
                LibraryView()
                    .tabItem {
                        Label("Library", systemImage: AppState.Tab.library.icon)
                    }
                    .tag(AppState.Tab.library)
                
                SearchView()
                    .tabItem {
                        Label("Search", systemImage: AppState.Tab.search.icon)
                    }
                    .tag(AppState.Tab.search)
                
                DownloadsView()
                    .tabItem {
                        Label("Downloads", systemImage: AppState.Tab.downloads.icon)
                    }
                    .tag(AppState.Tab.downloads)
                
                SettingsView()
                    .tabItem {
                        Label("Settings", systemImage: AppState.Tab.settings.icon)
                    }
                    .tag(AppState.Tab.settings)
            }
            .tint(.crimson)
            
            // Mini player overlay
            if playerManager.currentItem != nil {
                VStack {
                    Spacer()
                    MiniPlayerView()
                        .transition(.move(edge: .bottom).combined(with: .opacity))
                }
                .ignoresSafeArea(.keyboard)
            }
            
            // Full screen player
            if playerManager.isFullScreenPlayerPresented {
                FullScreenPlayerView()
                    .transition(.opacity)
                    .zIndex(100)
            }
        }
        .preferredColorScheme(appState.appearance.colorScheme)
    }
}

// MARK: - Home View
struct HomeView: View {
    @EnvironmentObject var libraryManager: LibraryManager
    @EnvironmentObject var playerManager: PlayerManager
    
    var body: some View {
        NavigationStack {
            ScrollView {
                LazyVStack(spacing: 24) {
                    // Continue Watching Section
                    if !libraryManager.recentlyWatched.isEmpty {
                        MediaRowSection(
                            title: "Continue Watching",
                            items: libraryManager.recentlyWatched,
                            style: .withProgress
                        )
                    }
                    
                    // Trending Section
                    MediaRowSection(
                        title: "Trending Now",
                        items: libraryManager.trending,
                        style: .standard
                    )
                    
                    // My List Section
                    if !libraryManager.myList.isEmpty {
                        MediaRowSection(
                            title: "My List",
                            items: libraryManager.myList,
                            style: .standard
                        )
                    }
                    
                    // Recommendations Section
                    MediaRowSection(
                        title: "Recommended For You",
                        items: libraryManager.recommendations,
                        style: .standard
                    )
                    
                    // New Releases Section
                    MediaRowSection(
                        title: "New Releases",
                        items: libraryManager.newReleases,
                        style: .highlight
                    )
                }
                .padding(.vertical)
            }
            .navigationTitle("Home")
            .navigationBarTitleDisplayMode(.large)
            .refreshable {
                await libraryManager.refreshContent()
            }
        }
    }
}

// MARK: - Media Row Section
struct MediaRowSection: View {
    let title: String
    let items: [MediaItem]
    let style: MediaRowStyle
    
    enum MediaRowStyle {
        case standard
        case withProgress
        case highlight
    }
    
    var body: some View {
        VStack(alignment: .leading, spacing: 12) {
            Text(title)
                .font(.title2)
                .fontWeight(.bold)
                .foregroundColor(.primary)
                .padding(.horizontal)
            
            ScrollView(.horizontal, showsIndicators: false) {
                LazyHStack(spacing: 12) {
                    ForEach(items) { item in
                        MediaCardView(item: item, style: style)
                            .onTapGesture {
                                // Handle item selection
                            }
                    }
                }
                .padding(.horizontal)
            }
        }
    }
}

// MARK: - Media Card View
struct MediaCardView: View {
    let item: MediaItem
    let style: MediaRowSection.MediaRowStyle
    
    var body: some View {
        VStack(alignment: .leading, spacing: 8) {
            // Thumbnail
            AsyncImage(url: item.thumbnailURL) { phase in
                switch phase {
                case .empty:
                    Rectangle()
                        .fill(Color.gray.opacity(0.3))
                        .overlay {
                            ProgressView()
                        }
                case .success(let image):
                    image
                        .resizable()
                        .aspectRatio(contentMode: .fill)
                case .failure:
                    Rectangle()
                        .fill(Color.gray.opacity(0.3))
                        .overlay {
                            Image(systemName: "photo")
                                .foregroundColor(.gray)
                        }
                @unknown default:
                    Rectangle()
                        .fill(Color.gray.opacity(0.3))
                }
            }
            .frame(width: cardWidth, height: cardHeight)
            .cornerRadius(8)
            .clipped()
            .overlay(alignment: .bottom) {
                if style == .withProgress, let progress = item.watchProgress {
                    ProgressBarView(progress: progress)
                        .padding(4)
                }
            }
            
            // Title
            Text(item.title)
                .font(.caption)
                .foregroundColor(.primary)
                .lineLimit(1)
        }
        .frame(width: cardWidth)
    }
    
    private var cardWidth: CGFloat {
        switch style {
        case .highlight: return 200
        default: return 120
        }
    }
    
    private var cardHeight: CGFloat {
        switch style {
        case .highlight: return 280
        case .withProgress: return 160
        default: return 180
        }
    }
}

// MARK: - Progress Bar View
struct ProgressBarView: View {
    let progress: Double // 0.0 to 1.0
    
    var body: some View {
        GeometryReader { geometry in
            ZStack(alignment: .leading) {
                Color.black.opacity(0.5)
                    .cornerRadius(2)
                
                Color.crimson
                    .frame(width: geometry.size.width * progress)
                    .cornerRadius(2)
            }
        }
        .frame(height: 4)
    }
}

// MARK: - Library View
struct LibraryView: View {
    @EnvironmentObject var libraryManager: LibraryManager
    
    var body: some View {
        NavigationStack {
            List {
                Section("Playlists") {
                    ForEach(libraryManager.playlists) { playlist in
                        PlaylistRowView(playlist: playlist)
                    }
                }
                
                Section("Collections") {
                    ForEach(libraryManager.collections) { collection in
                        CollectionRowView(collection: collection)
                    }
                }
            }
            .navigationTitle("Library")
            .toolbar {
                ToolbarItem(placement: .primaryAction) {
                    Button(action: { /* Create playlist */ }) {
                        Image(systemName: "plus")
                    }
                }
            }
        }
    }
}

// MARK: - Search View
struct SearchView: View {
    @State private var searchText = ""
    @EnvironmentObject var libraryManager: LibraryManager
    
    var body: some View {
        NavigationStack {
            ScrollView {
                LazyVStack(alignment: .leading, spacing: 20) {
                    if searchText.isEmpty {
                        // Browse categories
                        CategoriesGridView(categories: libraryManager.categories)
                    } else {
                        // Search results
                        SearchResultsView(query: searchText)
                    }
                }
                .padding()
            }
            .navigationTitle("Search")
            .searchable(text: $searchText, prompt: "Search movies, shows, videos...")
        }
    }
}

// MARK: - Downloads View
struct DownloadsView: View {
    @EnvironmentObject var libraryManager: LibraryManager
    
    var body: some View {
        NavigationStack {
            List {
                ForEach(libraryManager.downloads) { download in
                    DownloadRowView(download: download)
                }
                .onDelete { indexSet in
                    // Handle delete
                }
            }
            .navigationTitle("Downloads")
            .overlay {
                if libraryManager.downloads.isEmpty {
                    ContentUnavailableView(
                        "No Downloads",
                        systemImage: "arrow.down.circle",
                        description: Text("Download content to watch offline")
                    )
                }
            }
        }
    }
}

// MARK: - Settings View
struct SettingsView: View {
    @EnvironmentObject var appState: AppState
    @EnvironmentObject var cloudSyncManager: CloudSyncManager
    
    var body: some View {
        NavigationStack {
            List {
                // Account Section
                Section("Account") {
                    if let user = appState.currentUser {
                        HStack {
                            AsyncImage(url: user.avatarURL) { phase in
                                switch phase {
                                case .success(let image):
                                    image.resizable()
                                default:
                                    Image(systemName: "person.circle.fill")
                                        .foregroundColor(.gray)
                                }
                            }
                            .frame(width: 40, height: 40)
                            .clipShape(Circle())
                            
                            VStack(alignment: .leading) {
                                Text(user.displayName ?? "User")
                                    .font(.headline)
                                Text(user.email)
                                    .font(.caption)
                                    .foregroundColor(.secondary)
                            }
                        }
                    } else {
                        Button("Sign In") {
                            // Show sign in
                        }
                    }
                }
                
                // Playback Section
                Section("Playback") {
                    Picker("Video Quality", selection: $appState.currentUser?.preferences.videoQuality ?? .auto) {
                        ForEach(User.VideoQuality.allCases, id: \.self) { quality in
                            Text(quality.rawValue).tag(quality)
                        }
                    }
                    
                    Picker("Streaming Quality", selection: $appState.currentUser?.preferences.streamingQuality ?? .auto) {
                        ForEach(User.StreamingQuality.allCases, id: \.self) { quality in
                            Text(quality.rawValue).tag(quality)
                        }
                    }
                    
                    Toggle("Auto Play", isOn: $appState.currentUser?.preferences.autoPlay ?? true)
                    Toggle("Skip Intros", isOn: $appState.currentUser?.preferences.skipIntros ?? true)
                }
                
                // Appearance Section
                Section("Appearance") {
                    Picker("Appearance", selection: $appState.appearance) {
                        ForEach(AppState.AppAppearance.allCases, id: \.self) { appearance in
                            Text(appearance.rawValue).tag(appearance)
                        }
                    }
                }
                
                // Cloud Sync Section
                Section("Cloud Sync") {
                    HStack {
                        Text("Status")
                        Spacer()
                        Text(cloudSyncManager.status.description)
                            .foregroundColor(.secondary)
                    }
                    
                    Button("Sync Now") {
                        Task {
                            await cloudSyncManager.sync()
                        }
                    }
                    .disabled(cloudSyncManager.status == .syncing)
                }
                
                // About Section
                Section("About") {
                    HStack {
                        Text("Version")
                        Spacer()
                        Text("1.3.0")
                            .foregroundColor(.secondary)
                    }
                    
                    Link("Privacy Policy", destination: URL(string: "https://vantis.media/privacy")!)
                    Link("Terms of Service", destination: URL(string: "https://vantis.media/terms")!)
                }
            }
            .navigationTitle("Settings")
        }
    }
}

// MARK: - Color Extensions
extension Color {
    static let crimson = Color(hex: "#DC143C")
    
    init(hex: String) {
        let hex = hex.trimmingCharacters(in: CharacterSet.alphanumerics.inverted)
        var int: UInt64 = 0
        Scanner(string: hex).scanHexInt64(&int)
        let a, r, g, b: UInt64
        switch hex.count {
        case 3: // RGB (12-bit)
            (a, r, g, b) = (255, (int >> 8) * 17, (int >> 4 & 0xF) * 17, (int & 0xF) * 17)
        case 6: // RGB (24-bit)
            (a, r, g, b) = (255, int >> 16, int >> 8 & 0xFF, int & 0xFF)
        case 8: // ARGB (32-bit)
            (a, r, g, b) = (int >> 24, int >> 16 & 0xFF, int >> 8 & 0xFF, int & 0xFF)
        default:
            (a, r, g, b) = (255, 0, 0, 0)
        }
        self.init(
            .sRGB,
            red: Double(r) / 255,
            green: Double(g) / 255,
            blue: Double(b) / 255,
            opacity: Double(a) / 255
        )
    }
}

// MARK: - App Appearance Extension
extension AppState.AppAppearance {
    var colorScheme: ColorScheme? {
        switch self {
        case .light: return .light
        case .dark: return .dark
        case .system: return nil
        }
    }
}

#Preview {
    ContentView()
        .environmentObject(AppState())
        .environmentObject(PlayerManager())
        .environmentObject(LibraryManager())
        .environmentObject(CloudSyncManager())
}