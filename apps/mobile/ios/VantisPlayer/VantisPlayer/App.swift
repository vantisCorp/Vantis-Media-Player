import SwiftUI

@main
struct VantisPlayerApp: App {
    @StateObject private var appState = AppState()
    @StateObject private var playerManager = PlayerManager()
    @StateObject private var libraryManager = LibraryManager()
    @StateObject private var cloudSyncManager = CloudSyncManager()
    
    var body: some Scene {
        WindowGroup {
            ContentView()
                .environmentObject(appState)
                .environmentObject(playerManager)
                .environmentObject(libraryManager)
                .environmentObject(cloudSyncManager)
                .onAppear {
                    setupApp()
                }
        }
        #if os(iOS)
        .handlesExternalEvents(matching: Set(arrayLiteral: "*"))
        #endif
    }
    
    private func setupApp() {
        // Initialize the shared Rust core
        VantisCoreBridge.shared.initialize()
        
        // Setup remote notifications
        NotificationsManager.shared.requestAuthorization()
        
        // Configure audio session
        AudioManager.shared.configureSession()
    }
}

// MARK: - App State
class AppState: ObservableObject {
    @Published var isLoading: Bool = true
    @Published var isAuthenticated: Bool = false
    @Published var currentUser: User?
    @Published var selectedTab: Tab = .home
    @Published var appearance: AppAppearance = .system
    @Published var preferredLanguage: String = Locale.current.language.languageCode?.identifier ?? "en"
    
    enum Tab: String, CaseIterable {
        case home = "Home"
        case library = "Library"
        case search = "Search"
        case downloads = "Downloads"
        case settings = "Settings"
        
        var icon: String {
            switch self {
            case .home: return "house.fill"
            case .library: return "play.rectangle.on.rectangle.fill"
            case .search: return "magnifyingglass"
            case .downloads: return "arrow.down.circle.fill"
            case .settings: return "gearshape.fill"
            }
        }
    }
    
    enum AppAppearance: String, CaseIterable {
        case system = "System"
        case light = "Light"
        case dark = "Dark"
    }
}

// MARK: - User Model
struct User: Codable, Identifiable {
    let id: String
    let email: String
    let displayName: String?
    let avatarURL: URL?
    let createdAt: Date
    var preferences: UserPreferences
    
    struct UserPreferences: Codable {
        var autoPlay: Bool = true
        var defaultAudioLanguage: String = "en"
        var defaultSubtitleLanguage: String?
        var videoQuality: VideoQuality = .auto
        var playbackSpeed: Double = 1.0
        var skipIntros: Bool = true
        var streamingQuality: StreamingQuality = .auto
    }
    
    enum VideoQuality: String, Codable, CaseIterable {
        case auto = "Auto"
        case low = "360p"
        case medium = "720p"
        case high = "1080p"
        case ultra = "4K"
    }
    
    enum StreamingQuality: String, Codable, CaseIterable {
        case auto = "Auto"
        case low = "Low"
        case medium = "Medium"
        case high = "High"
    }
}