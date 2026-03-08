import Foundation

// MARK: - Media Item
struct MediaItem: Identifiable, Codable, Hashable {
    let id: String
    let title: String
    let subtitle: String?
    let description: String?
    let thumbnailURL: URL?
    let posterURL: URL?
    let streamUrl: String
    let duration: Double
    let contentType: ContentType
    let genres: [String]
    let releaseYear: Int?
    let rating: String?
    let watchProgress: Double?
    let lastWatched: Date?
    
    enum ContentType: String, Codable {
        case movie = "Movie"
        case series = "Series"
        case episode = "Episode"
        case documentary = "Documentary"
        case live = "Live"
        case video = "Video"
    }
    
    // Computed properties
    var progress: Double? {
        guard let progress = watchProgress, progress > 0 else { return nil }
        return progress
    }
    
    var isWatched: Bool {
        guard let progress = watchProgress else { return false }
        return progress >= 0.95
    }
    
    var displayDuration: String {
        let hours = Int(duration) / 3600
        let minutes = Int(duration) % 3600 / 60
        
        if hours > 0 {
            return "\(hours)h \(minutes)m"
        } else {
            return "\(minutes)m"
        }
    }
}

// MARK: - Playlist
struct Playlist: Identifiable, Codable, Hashable {
    let id: String
    var name: String
    var description: String?
    var items: [MediaItem]
    var createdAt: Date
    var updatedAt: Date
    var isPublic: Bool
    var coverImageURL: URL?
    
    var itemCount: Int {
        items.count
    }
    
    var totalDuration: Double {
        items.reduce(0) { $0 + $1.duration }
    }
}

// MARK: - Collection
struct MediaCollection: Identifiable, Codable, Hashable {
    let id: String
    let name: String
    let description: String?
    let items: [MediaItem]
    let collectionType: CollectionType
    let artworkURL: URL?
    
    enum CollectionType: String, Codable {
        case favorites = "Favorites"
        case watchlist = "Watchlist"
        case watched = "Watched"
        case custom = "Custom"
    }
}

// MARK: - Download
struct Download: Identifiable, Codable {
    let id: String
    let mediaItem: MediaItem
    var status: DownloadStatus
    var progress: Double
    var downloadedBytes: Int64
    var totalBytes: Int64
    var downloadSpeed: Int64
    var timeRemaining: TimeInterval?
    var localURL: URL?
    var createdAt: Date
    var completedAt: Date?
    
    enum DownloadStatus: String, Codable {
        case pending = "Pending"
        case downloading = "Downloading"
        case paused = "Paused"
        case completed = "Completed"
        case failed = "Failed"
    }
    
    var displayProgress: String {
        "\(Int(progress * 100))%"
    }
    
    var displaySize: String {
        ByteCountFormatter.string(fromByteCount: totalBytes, countStyle: .file)
    }
    
    var displaySpeed: String {
        ByteCountFormatter.string(fromByteCount: downloadSpeed, countStyle: .file) + "/s"
    }
    
    var displayTimeRemaining: String? {
        guard let time = timeRemaining else { return nil }
        let minutes = Int(time) / 60
        let seconds = Int(time) % 60
        return "\(minutes)m \(seconds)s remaining"
    }
}

// MARK: - Category
struct Category: Identifiable, Codable, Hashable {
    let id: String
    let name: String
    let icon: String?
    let subcategories: [Category]?
    var items: [MediaItem]
    
    static let featured = Category(
        id: "featured",
        name: "Featured",
        icon: "star.fill",
        subcategories: nil,
        items: []
    )
    
    static let trending = Category(
        id: "trending",
        name: "Trending",
        icon: "flame.fill",
        subcategories: nil,
        items: []
    )
    
    static let newReleases = Category(
        id: "new-releases",
        name: "New Releases",
        icon: "sparkle",
        subcategories: nil,
        items: []
    )
}

// MARK: - Chapter
struct Chapter: Identifiable, Codable {
    let id: String
    let title: String
    let startTime: Double
    let endTime: Double
    let thumbnailURL: URL?
    
    var duration: Double {
        endTime - startTime
    }
}

// MARK: - Streaming Provider
struct StreamingProvider: Identifiable, Codable, Hashable {
    let id: String
    let name: String
    let icon: String
    let isLinked: Bool
    let subscriptionStatus: SubscriptionStatus?
    
    enum SubscriptionStatus: String, Codable {
        case active = "Active"
        case expired = "Expired"
        case notSubscribed = "Not Subscribed"
    }
    
    static let netflix = StreamingProvider(
        id: "netflix",
        name: "Netflix",
        icon: "n.square.fill",
        isLinked: false,
        subscriptionStatus: nil
    )
    
    static let disneyPlus = StreamingProvider(
        id: "disney_plus",
        name: "Disney+",
        icon: "d.square.fill",
        isLinked: false,
        subscriptionStatus: nil
    )
    
    static let hulu = StreamingProvider(
        id: "hulu",
        name: "Hulu",
        icon: "h.square.fill",
        isLinked: false,
        subscriptionStatus: nil
    )
    
    static let amazonPrime = StreamingProvider(
        id: "amazon_prime",
        name: "Prime Video",
        icon: "a.square.fill",
        isLinked: false,
        subscriptionStatus: nil
    )
    
    static let hboMax = StreamingProvider(
        id: "hbo_max",
        name: "HBO Max",
        icon: "h.square.fill",
        isLinked: false,
        subscriptionStatus: nil
    )
}

// MARK: - Search Result
struct SearchResult: Identifiable {
    let id: String
    let title: String
    let type: ResultType
    let items: [MediaItem]
    
    enum ResultType {
        case topMatch
        case movies
        case series
        case episodes
        case people
    }
}

// MARK: - Person
struct Person: Identifiable, Codable {
    let id: String
    let name: String
    let profileImageURL: URL?
    let role: String?
    let knownFor: [MediaItem]
}

// MARK: - Subtitle Configuration
struct SubtitleConfig: Codable {
    var fontSize: Int
    var fontFamily: String
    var textColor: String
    var backgroundColor: String
    var outlineColor: String?
    var outlineWidth: Int?
    var shadowEnabled: Bool
    var position: SubtitlePosition
    
    enum SubtitlePosition: String, Codable {
        case bottom = "Bottom"
        case middle = "Middle"
        case top = "Top"
    }
    
    static let `default` = SubtitleConfig(
        fontSize: 24,
        fontFamily: "sans-serif",
        textColor: "#FFFFFF",
        backgroundColor: "#000000",
        outlineColor: "#000000",
        outlineWidth: 2,
        shadowEnabled: true,
        position: .bottom
    )
}

// MARK: - Video Configuration
struct VideoConfig: Codable {
    var hardwareDecoding: Bool
    var deinterlacing: DeinterlacingMode
    var colorSpace: ColorSpace
    var aspectRatio: AspectRatio
    var cropMode: CropMode
    
    enum DeinterlacingMode: String, Codable {
        case auto = "Auto"
        case on = "On"
        case off = "Off"
    }
    
    enum ColorSpace: String, Codable {
        case auto = "Auto"
        case sdr = "SDR"
        case hdr = "HDR"
        case dolbyVision = "Dolby Vision"
    }
    
    enum AspectRatio: String, Codable {
        case auto = "Auto"
        case ratio16x9 = "16:9"
        case ratio4x3 = "4:3"
        case ratio21x9 = "21:9"
        case original = "Original"
    }
    
    enum CropMode: String, Codable {
        case none = "None"
        case zoom = "Zoom"
        case stretch = "Stretch"
    }
    
    static let `default` = VideoConfig(
        hardwareDecoding: true,
        deinterlacing: .auto,
        colorSpace: .auto,
        aspectRatio: .auto,
        cropMode: .none
    )
}

// MARK: - Audio Configuration
struct AudioConfig: Codable {
    var bitPerfect: Bool
    var eqPreset: String
    var spatialAudio: Bool
    var surroundMode: SurroundMode
    var customEQ: [Double]?
    
    enum SurroundMode: String, Codable {
        case stereo = "Stereo"
        case surround51 = "5.1 Surround"
        case surround71 = "7.1 Surround"
        case auto = "Auto"
    }
    
    static let `default` = AudioConfig(
        bitPerfect: true,
        eqPreset: "flat",
        spatialAudio: true,
        surroundMode: .auto,
        customEQ: nil
    )
}