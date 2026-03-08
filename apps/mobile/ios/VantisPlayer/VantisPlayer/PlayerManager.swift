import SwiftUI
import AVKit
import Combine

// MARK: - Player Manager
class PlayerManager: ObservableObject {
    // Published properties
    @Published var currentItem: MediaItem?
    @Published var isPlaying: Bool = false
    @Published var isBuffering: Bool = false
    @Published var isFullScreenPlayerPresented: Bool = false
    @Published var playbackRate: Double = 1.0
    @Published var volume: Float = 1.0
    @Published var brightness: Double = 0.5
    @Published var currentTime: Double = 0
    @Published var duration: Double = 0
    @Published var bufferedProgress: Double = 0
    @Published var selectedAudioTrack: String?
    @Published var selectedSubtitle: String?
    @Published var isPiPMode: Bool = false
    @Published var isAirPlayActive: Bool = false
    
    // Private properties
    private var player: AVPlayer?
    private var playerItem: AVPlayerItem?
    private var timeObserver: Any?
    private var cancellables = Set<AnyCancellable>()
    
    // MARK: - Initialization
    init() {
        setupNotifications()
        setupAudioSession()
    }
    
    deinit {
        cleanup()
    }
    
    // MARK: - Setup
    private func setupNotifications() {
        // Player notifications
        NotificationCenter.default.publisher(for: .AVPlayerItemDidPlayToEndTime)
            .sink { [weak self] _ in
                self?.handlePlaybackEnd()
            }
            .store(in: &cancellables)
        
        NotificationCenter.default.publisher(for: UIApplication.didEnterBackgroundNotification)
            .sink { [weak self] _ in
                self?.handleEnterBackground()
            }
            .store(in: &cancellables)
        
        NotificationCenter.default.publisher(for: UIApplication.willEnterForegroundNotification)
            .sink { [weak self] _ in
                self?.handleEnterForeground()
            }
            .store(in: &cancellables)
    }
    
    private func setupAudioSession() {
        do {
            try AVAudioSession.sharedInstance().setCategory(.playback, mode: .moviePlayback)
            try AVAudioSession.sharedInstance().setActive(true)
        } catch {
            print("Failed to setup audio session: \(error)")
        }
    }
    
    // MARK: - Playback Control
    func play(_ item: MediaItem) {
        currentItem = item
        guard let url = item.streamURL else { return }
        
        // Create player item
        playerItem = AVPlayerItem(url: url)
        
        // Create or update player
        if player == nil {
            player = AVPlayer(playerItem: playerItem)
            setupPlayerObservers()
        } else {
            player?.replaceCurrentItem(with: playerItem)
        }
        
        // Start playback
        player?.play()
        isPlaying = true
        isFullScreenPlayerPresented = true
        
        // Notify shared core
        VantisCoreBridge.shared.play(url.absoluteString)
    }
    
    func pause() {
        player?.pause()
        isPlaying = false
        VantisCoreBridge.shared.pause()
    }
    
    func resume() {
        player?.play()
        isPlaying = true
        VantisCoreBridge.shared.resume()
    }
    
    func stop() {
        player?.pause()
        player?.replaceCurrentItem(with: nil)
        currentItem = nil
        isPlaying = false
        currentTime = 0
        duration = 0
        isFullScreenPlayerPresented = false
        VantisCoreBridge.shared.stop()
    }
    
    func seek(to time: Double) {
        let cmTime = CMTime(seconds: time, preferredTimescale: 600)
        player?.seek(to: cmTime, toleranceBefore: .zero, toleranceAfter: .zero) { [weak self] finished in
            if finished {
                self?.currentTime = time
            }
        }
        VantisCoreBridge.shared.seek(time)
    }
    
    func seekRelative(_ offset: Double) {
        let newTime = max(0, min(duration, currentTime + offset))
        seek(to: newTime)
    }
    
    func setPlaybackRate(_ rate: Double) {
        player?.rate = Float(rate)
        playbackRate = rate
    }
    
    func setVolume(_ value: Float) {
        player?.volume = value
        volume = value
        VantisCoreBridge.shared.setVolume(value)
    }
    
    func togglePlayPause() {
        if isPlaying {
            pause()
        } else {
            resume()
        }
    }
    
    // MARK: - Player Observers
    private func setupPlayerObservers() {
        // Time observer
        let interval = CMTime(seconds: 0.5, preferredTimescale: CMTimeScale(NSEC_PER_SEC))
        timeObserver = player?.addPeriodicTimeObserver(forInterval: interval, queue: .main) { [weak self] time in
            self?.currentTime = time.seconds
        }
        
        // Player item observers
        playerItem?.publisher(for: \.duration)
            .receive(on: DispatchQueue.main)
            .sink { [weak self] duration in
                self?.duration = duration.seconds
            }
            .store(in: &cancellables)
        
        playerItem?.publisher(for: \.isPlaybackLikelyToKeepUp)
            .receive(on: DispatchQueue.main)
            .sink { [weak self] isLikelyToKeepUp in
                self?.isBuffering = !isLikelyToKeepUp
            }
            .store(in: &cancellables)
        
        // Buffer progress
        playerItem?.publisher(for: \.loadedTimeRanges)
            .receive(on: DispatchQueue.main)
            .sink { [weak self] timeRanges in
                guard let self = self, let firstRange = timeRanges.first as? CMTimeRange else { return }
                let bufferedTime = firstRange.end.seconds
                self.bufferedProgress = self.duration > 0 ? bufferedTime / self.duration : 0
            }
            .store(in: &cancellables)
    }
    
    // MARK: - Track Selection
    func availableAudioTracks() -> [AudioTrack] {
        guard let asset = playerItem?.asset else { return [] }
        let audioTracks = asset.tracks(withMediaType: .audio)
        return audioTracks.enumerated().map { index, track in
            AudioTrack(
                id: "\(index)",
                language: track.languageCode ?? "Unknown",
                title: track.extendedLanguageTag ?? "Audio \(index + 1)"
            )
        }
    }
    
    func availableSubtitles() -> [SubtitleTrack] {
        guard let asset = playerItem?.asset else { return [] }
        let subtitleTracks = asset.tracks(withMediaType: .subtitle)
        return subtitleTracks.enumerated().map { index, track in
            SubtitleTrack(
                id: "\(index)",
                language: track.languageCode ?? "Unknown",
                title: track.extendedLanguageTag ?? "Subtitle \(index + 1)"
            )
        }
    }
    
    func selectAudioTrack(_ trackId: String?) {
        selectedAudioTrack = trackId
        // Implementation for track selection
    }
    
    func selectSubtitle(_ trackId: String?) {
        selectedSubtitle = trackId
        if let trackId = trackId {
            VantisCoreBridge.shared.loadSubtitle("", language: trackId)
        }
        VantisCoreBridge.shared.setSubtitleEnabled(trackId != nil)
    }
    
    // MARK: - Picture in Picture
    func startPiP() {
        // PiP implementation
        isPiPMode = true
    }
    
    func stopPiP() {
        isPiPMode = false
    }
    
    // MARK: - AirPlay
    func isAirPlayAvailable() -> Bool {
        return AVRoutePickerView.isAirPlaySupported
    }
    
    // MARK: - Event Handlers
    private func handlePlaybackEnd() {
        isPlaying = false
        currentTime = duration
    }
    
    private func handleEnterBackground() {
        // Handle background - continue audio playback
    }
    
    private func handleEnterForeground() {
        // Handle foreground
    }
    
    // MARK: - Cleanup
    private func cleanup() {
        if let observer = timeObserver {
            player?.removeTimeObserver(observer)
        }
        player?.pause()
        player = nil
        playerItem = nil
    }
}

// MARK: - Supporting Types
struct AudioTrack: Identifiable, Hashable {
    let id: String
    let language: String
    let title: String
}

struct SubtitleTrack: Identifiable, Hashable {
    let id: String
    let language: String
    let title: String
}

// MARK: - Media Item Extension
extension MediaItem {
    var streamURL: URL? {
        URL(string: streamUrl)
    }
}