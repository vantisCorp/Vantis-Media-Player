import SwiftUI
import Combine

// MARK: - Library Manager
class LibraryManager: ObservableObject {
    // Published properties
    @Published var recentlyWatched: [MediaItem] = []
    @Published var trending: [MediaItem] = []
    @Published var recommendations: [MediaItem] = []
    @Published var newReleases: [MediaItem] = []
    @Published var myList: [MediaItem] = []
    @Published var playlists: [Playlist] = []
    @Published var collections: [MediaCollection] = []
    @Published var downloads: [Download] = []
    @Published var categories: [Category] = []
    @Published var isLoading: Bool = false
    
    private var cancellables = Set<AnyCancellable>()
    
    init() {
        loadMockData()
    }
    
    // MARK: - Data Loading
    func refreshContent() async {
        await MainActor.run {
            isLoading = true
        }
        
        // Simulate network delay
        try? await Task.sleep(nanoseconds: 1_000_000_000)
        
        await MainActor.run {
            loadMockData()
            isLoading = false
        }
    }
    
    private func loadMockData() {
        // Mock data for development
        recentlyWatched = [
            MediaItem(
                id: "1",
                title: "The Last Kingdom",
                subtitle: "Season 5, Episode 8",
                description: "As Edward's army is outmaneuvered, Uhtred must make a difficult choice.",
                thumbnailURL: URL(string: "https://picsum.photos/seed/movie1/400/600"),
                posterURL: URL(string: "https://picsum.photos/seed/movie1/800/1200"),
                streamUrl: "https://example.com/stream1.m3u8",
                duration: 3600,
                contentType: .episode,
                genres: ["Drama", "Action", "History"],
                releaseYear: 2022,
                rating: "TV-MA",
                watchProgress: 0.65,
                lastWatched: Date()
            ),
            MediaItem(
                id: "2",
                title: "Stranger Things",
                subtitle: "Season 4, Episode 1",
                description: "A chilling new mystery unfolds in Hawkins.",
                thumbnailURL: URL(string: "https://picsum.photos/seed/movie2/400/600"),
                posterURL: URL(string: "https://picsum.photos/seed/movie2/800/1200"),
                streamUrl: "https://example.com/stream2.m3u8",
                duration: 4800,
                contentType: .episode,
                genres: ["Sci-Fi", "Horror", "Drama"],
                releaseYear: 2022,
                rating: "TV-14",
                watchProgress: 0.3,
                lastWatched: Date().addingTimeInterval(-86400)
            )
        ]
        
        trending = [
            MediaItem(
                id: "3",
                title: "Oppenheimer",
                subtitle: nil,
                description: "The story of American scientist J. Robert Oppenheimer and his role in the development of the atomic bomb.",
                thumbnailURL: URL(string: "https://picsum.photos/seed/movie3/400/600"),
                posterURL: URL(string: "https://picsum.photos/seed/movie3/800/1200"),
                streamUrl: "https://example.com/stream3.m3u8",
                duration: 10800,
                contentType: .movie,
                genres: ["Drama", "History", "Biography"],
                releaseYear: 2023,
                rating: "R",
                watchProgress: nil,
                lastWatched: nil
            ),
            MediaItem(
                id: "4",
                title: "The Bear",
                subtitle: nil,
                description: "A young chef from the fine dining world returns to Chicago to run his family sandwich shop.",
                thumbnailURL: URL(string: "https://picsum.photos/seed/movie4/400/600"),
                posterURL: URL(string: "https://picsum.photos/seed/movie4/800/1200"),
                streamUrl: "https://example.com/stream4.m3u8",
                duration: 1800,
                contentType: .series,
                genres: ["Comedy", "Drama"],
                releaseYear: 2022,
                rating: "TV-MA",
                watchProgress: nil,
                lastWatched: nil
            )
        ]
        
        recommendations = [
            MediaItem(
                id: "5",
                title: "Dune: Part Two",
                subtitle: nil,
                description: "Paul Atreides unites with Chani and the Fremen while seeking revenge against the conspirators who destroyed his family.",
                thumbnailURL: URL(string: "https://picsum.photos/seed/movie5/400/600"),
                posterURL: URL(string: "https://picsum.photos/seed/movie5/800/1200"),
                streamUrl: "https://example.com/stream5.m3u8",
                duration: 10200,
                contentType: .movie,
                genres: ["Sci-Fi", "Adventure", "Drama"],
                releaseYear: 2024,
                rating: "PG-13",
                watchProgress: nil,
                lastWatched: nil
            )
        ]
        
        newReleases = [
            MediaItem(
                id: "6",
                title: "Wednesday",
                subtitle: nil,
                description: "Follows Wednesday Addams' years as a student, when she attempts to master her emerging psychic ability.",
                thumbnailURL: URL(string: "https://picsum.photos/seed/movie6/400/600"),
                posterURL: URL(string: "https://picsum.photos/seed/movie6/800/1200"),
                streamUrl: "https://example.com/stream6.m3u8",
                duration: 3000,
                contentType: .series,
                genres: ["Comedy", "Crime", "Fantasy"],
                releaseYear: 2022,
                rating: "TV-14",
                watchProgress: nil,
                lastWatched: nil
            )
        ]
        
        playlists = [
            Playlist(
                id: "p1",
                name: "Watch Later",
                description: "Movies and shows to watch later",
                items: Array(recentlyWatched.prefix(2)),
                createdAt: Date(),
                updatedAt: Date(),
                isPublic: false,
                coverImageURL: nil
            ),
            Playlist(
                id: "p2",
                name: "Favorites",
                description: "My favorite content",
                items: Array(trending.prefix(2)),
                createdAt: Date().addingTimeInterval(-86400 * 7),
                updatedAt: Date(),
                isPublic: false,
                coverImageURL: nil
            )
        ]
        
        collections = [
            MediaCollection(
                id: "c1",
                name: "My List",
                description: nil,
                items: myList,
                collectionType: .watchlist,
                artworkURL: nil
            )
        ]
        
        downloads = [
            Download(
                id: "d1",
                mediaItem: recentlyWatched[0],
                status: .completed,
                progress: 1.0,
                downloadedBytes: 1_500_000_000,
                totalBytes: 1_500_000_000,
                downloadSpeed: 0,
                timeRemaining: nil,
                localURL: URL(string: "file:///downloads/movie1.mp4"),
                createdAt: Date().addingTimeInterval(-86400),
                completedAt: Date()
            )
        ]
        
        categories = [
            .featured,
            .trending,
            Category(id: "action", name: "Action", icon: "flame", subcategories: nil, items: trending),
            Category(id: "comedy", name: "Comedy", icon: "face.smiling", subcategories: nil, items: recommendations),
            Category(id: "drama", name: "Drama", icon: "theatermasks", subcategories: nil, items: newReleases),
            Category(id: "scifi", name: "Sci-Fi", icon: "sparkles", subcategories: nil, items: trending),
            Category(id: "horror", name: "Horror", icon: "moon.circle", subcategories: nil, items: recentlyWatched),
            Category(id: "documentary", name: "Documentary", icon: "doc.text", subcategories: nil, items: recommendations)
        ]
    }
    
    // MARK: - Playlist Management
    func createPlaylist(name: String, description: String? = nil) {
        let playlist = Playlist(
            id: UUID().uuidString,
            name: name,
            description: description,
            items: [],
            createdAt: Date(),
            updatedAt: Date(),
            isPublic: false,
            coverImageURL: nil
        )
        playlists.append(playlist)
    }
    
    func addToPlaylist(_ item: MediaItem, playlistId: String) {
        guard let index = playlists.firstIndex(where: { $0.id == playlistId }) else { return }
        playlists[index].items.append(item)
        playlists[index].updatedAt = Date()
    }
    
    func removeFromPlaylist(_ item: MediaItem, playlistId: String) {
        guard let playlistIndex = playlists.firstIndex(where: { $0.id == playlistId }) else { return }
        playlists[playlistIndex].items.removeAll { $0.id == item.id }
        playlists[playlistIndex].updatedAt = Date()
    }
    
    func deletePlaylist(_ playlistId: String) {
        playlists.removeAll { $0.id == playlistId }
    }
    
    // MARK: - My List Management
    func addToList(_ item: MediaItem) {
        if !myList.contains(where: { $0.id == item.id }) {
            myList.append(item)
        }
    }
    
    func removeFromList(_ item: MediaItem) {
        myList.removeAll { $0.id == item.id }
    }
    
    func isInMyList(_ item: MediaItem) -> Bool {
        myList.contains { $0.id == item.id }
    }
    
    // MARK: - Downloads Management
    func startDownload(_ item: MediaItem) {
        let download = Download(
            id: UUID().uuidString,
            mediaItem: item,
            status: .pending,
            progress: 0,
            downloadedBytes: 0,
            totalBytes: 1_500_000_000, // Estimate
            downloadSpeed: 0,
            timeRemaining: nil,
            localURL: nil,
            createdAt: Date(),
            completedAt: nil
        )
        downloads.append(download)
        
        // Simulate download progress
        simulateDownload(download.id)
    }
    
    private func simulateDownload(_ downloadId: String) {
        Task {
            var progress = 0.0
            while progress < 1.0 {
                try? await Task.sleep(nanoseconds: 500_000_000)
                progress += 0.1
                
                await MainActor.run {
                    if let index = downloads.firstIndex(where: { $0.id == downloadId }) {
                        downloads[index].progress = min(progress, 1.0)
                        downloads[index].downloadedBytes = Int64(progress * Double(downloads[index].totalBytes))
                        downloads[index].status = progress >= 1.0 ? .completed : .downloading
                        
                        if progress >= 1.0 {
                            downloads[index].completedAt = Date()
                            downloads[index].localURL = URL(string: "file:///downloads/\(downloadId).mp4")
                        }
                    }
                }
            }
        }
    }
    
    func pauseDownload(_ downloadId: String) {
        if let index = downloads.firstIndex(where: { $0.id == downloadId }) {
            downloads[index].status = .paused
        }
    }
    
    func resumeDownload(_ downloadId: String) {
        if let index = downloads.firstIndex(where: { $0.id == downloadId }) {
            downloads[index].status = .downloading
        }
    }
    
    func cancelDownload(_ downloadId: String) {
        downloads.removeAll { $0.id == downloadId }
    }
    
    // MARK: - History Management
    func updateWatchProgress(_ item: MediaItem, progress: Double) {
        if let index = recentlyWatched.firstIndex(where: { $0.id == item.id }) {
            recentlyWatched[index] = MediaItem(
                id: item.id,
                title: item.title,
                subtitle: item.subtitle,
                description: item.description,
                thumbnailURL: item.thumbnailURL,
                posterURL: item.posterURL,
                streamUrl: item.streamUrl,
                duration: item.duration,
                contentType: item.contentType,
                genres: item.genres,
                releaseYear: item.releaseYear,
                rating: item.rating,
                watchProgress: progress,
                lastWatched: Date()
            )
        }
    }
}

// MARK: - Supporting Views
struct PlaylistRowView: View {
    let playlist: Playlist
    
    var body: some View {
        HStack {
            Image(systemName: "list.bullet.rectangle")
                .font(.title2)
                .foregroundColor(.crimson)
                .frame(width: 40, height: 40)
                .background(Color.crimson.opacity(0.1))
                .cornerRadius(8)
            
            VStack(alignment: .leading) {
                Text(playlist.name)
                    .font(.headline)
                Text("\(playlist.itemCount) items")
                    .font(.caption)
                    .foregroundColor(.secondary)
            }
        }
    }
}

struct CollectionRowView: View {
    let collection: MediaCollection
    
    var body: some View {
        HStack {
            Image(systemName: "folder")
                .font(.title2)
                .foregroundColor(.crimson)
                .frame(width: 40, height: 40)
                .background(Color.crimson.opacity(0.1))
                .cornerRadius(8)
            
            VStack(alignment: .leading) {
                Text(collection.name)
                    .font(.headline)
                Text(collection.collectionType.rawValue)
                    .font(.caption)
                    .foregroundColor(.secondary)
            }
        }
    }
}

struct DownloadRowView: View {
    let download: Download
    
    var body: some View {
        HStack {
            AsyncImage(url: download.mediaItem.thumbnailURL) { phase in
                switch phase {
                case .success(let image):
                    image
                        .resizable()
                        .aspectRatio(contentMode: .fill)
                default:
                    Color.gray.opacity(0.3)
                }
            }
            .frame(width: 60, height: 90)
            .cornerRadius(8)
            
            VStack(alignment: .leading, spacing: 4) {
                Text(download.mediaItem.title)
                    .font(.headline)
                
                Text(download.displaySize)
                    .font(.caption)
                    .foregroundColor(.secondary)
                
                if download.status == .downloading {
                    ProgressView(value: download.progress)
                        .progressViewStyle(LinearProgressViewStyle())
                    
                    Text("\(download.displayProgress) • \(download.displaySpeed)")
                        .font(.caption2)
                        .foregroundColor(.secondary)
                } else if download.status == .completed {
                    Label("Completed", systemImage: "checkmark.circle.fill")
                        .font(.caption)
                        .foregroundColor(.green)
                }
            }
        }
    }
}

struct CategoriesGridView: View {
    let categories: [Category]
    
    let columns = [
        GridItem(.flexible()),
        GridItem(.flexible())
    ]
    
    var body: some View {
        LazyVGrid(columns: columns, spacing: 16) {
            ForEach(categories) { category in
                CategoryCardView(category: category)
            }
        }
    }
}

struct CategoryCardView: View {
    let category: Category
    
    var body: some View {
        ZStack {
            LinearGradient(
                colors: [Color.crimson.opacity(0.8), Color.crimson.opacity(0.4)],
                startPoint: .topLeading,
                endPoint: .bottomTrailing
            )
            
            VStack {
                if let icon = category.icon {
                    Image(systemName: icon)
                        .font(.title)
                }
                Text(category.name)
                    .font(.headline)
            }
            .foregroundColor(.white)
        }
        .frame(height: 100)
        .cornerRadius(12)
    }
}

struct SearchResultsView: View {
    let query: String
    
    var body: some View {
        Text("Results for: \(query)")
            .foregroundColor(.secondary)
    }
}