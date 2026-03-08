import SwiftUI
import AVKit

struct FullScreenPlayerView: View {
    @EnvironmentObject var playerManager: PlayerManager
    @EnvironmentObject var appState: AppState
    @GestureState private var dragOffset: CGFloat = 0
    @State private var showControls: Bool = true
    @State private var controlsTimer: Timer?
    @State private var showAudioSelection: Bool = false
    @State private var showSubtitleSelection: Bool = false
    @State private var showSpeedSelection: Bool = false
    
    var body: some View {
        GeometryReader { geometry in
            ZStack {
                // Background
                Color.black.ignoresSafeArea()
                
                // Video Player
                VideoPlayerLayer()
                    .aspectRatio(16/9, contentMode: .fit)
                    .frame(maxWidth: .infinity, maxHeight: .infinity)
                
                // Gesture overlay
                GestureOverlay()
                
                // Controls overlay
                if showControls {
                    ControlsOverlay()
                        .transition(.opacity)
                }
                
                // Side panels for adjustments
                if dragOffset != 0 {
                    SideAdjustmentPanel(offset: dragOffset, width: geometry.size.width)
                }
            }
        }
        .statusBar(hidden: true)
        .onAppear {
            startControlsTimer()
        }
        .onDisappear {
            stopControlsTimer()
        }
        .onTapGesture {
            toggleControls()
        }
    }
    
    // MARK: - Video Player Layer
    private func VideoPlayerLayer() -> some View {
        Color.clear.overlay {
            if let player = playerManager.player {
                VideoPlayer(player: player)
                    .disabled(true) // Disable default controls
            }
        }
    }
    
    // MARK: - Gesture Overlay
    private func GestureOverlay() -> some View {
        GeometryReader { geometry in
            let width = geometry.size.width
            
            Color.clear
                .contentShape(Rectangle())
                .gesture(
                    DragGesture(minimumDistance: 30)
                        .updating($dragOffset) { value, state, _ in
                            // Only track horizontal drags
                            if abs(value.translation.width) > abs(value.translation.height) {
                                state = value.translation.width
                            }
                        }
                        .onEnded { value in
                            handleDragEnd(value: value, width: width)
                        }
                )
        }
    }
    
    // MARK: - Controls Overlay
    private func ControlsOverlay() -> some View {
        VStack {
            // Top bar
            HStack {
                Button(action: closePlayer) {
                    Image(systemName: "chevron.down")
                        .font(.title2)
                        .foregroundColor(.white)
                }
                
                Spacer()
                
                if let item = playerManager.currentItem {
                    VStack(alignment: .trailing) {
                        Text(item.title)
                            .font(.headline)
                            .foregroundColor(.white)
                        if let subtitle = item.subtitle {
                            Text(subtitle)
                                .font(.caption)
                                .foregroundColor(.white.opacity(0.7))
                        }
                    }
                }
                
                Spacer()
                
                Button(action: {}) {
                    Image(systemName: "ellipsis")
                        .font(.title2)
                        .foregroundColor(.white)
                }
            }
            .padding()
            .background {
                LinearGradient(
                    colors: [Color.black.opacity(0.6), .clear],
                    startPoint: .top,
                    endPoint: .bottom
                )
            }
            
            Spacer()
            
            // Center controls
            HStack(spacing: 40) {
                Button(action: { playerManager.seekRelative(-10) }) {
                    Image(systemName: "gobackward.10")
                        .font(.system(size: 36))
                        .foregroundColor(.white)
                }
                
                Button(action: { playerManager.togglePlayPause() }) {
                    Image(systemName: playerManager.isPlaying ? "pause.fill" : "play.fill")
                        .font(.system(size: 50))
                        .foregroundColor(.white)
                }
                
                Button(action: { playerManager.seekRelative(10) }) {
                    Image(systemName: "goforward.10")
                        .font(.system(size: 36))
                        .foregroundColor(.white)
                }
            }
            
            Spacer()
            
            // Bottom controls
            VStack(spacing: 12) {
                // Progress bar
                ProgressSection()
                
                // Control buttons
                HStack {
                    Button(action: { showSpeedSelection = true }) {
                        Text("\(String(format: "%.1f", playerManager.playbackRate))x")
                            .font(.caption)
                            .foregroundColor(.white)
                            .padding(.horizontal, 8)
                            .padding(.vertical, 4)
                            .background(Color.white.opacity(0.2))
                            .cornerRadius(4)
                    }
                    
                    Spacer()
                    
                    // Track buttons
                    HStack(spacing: 24) {
                        Button(action: { showAudioSelection = true }) {
                            Image(systemName: "speaker.wave.2")
                                .foregroundColor(.white)
                        }
                        
                        Button(action: { showSubtitleSelection = true }) {
                            Image(systemName: "captions.bubble")
                                .foregroundColor(.white)
                        }
                        
                        if playerManager.isAirPlayAvailable() {
                            AirPlayButton()
                        }
                    }
                    
                    Spacer()
                    
                    Button(action: togglePiP) {
                        Image(systemName: "pip.enter")
                            .foregroundColor(.white)
                    }
                }
                .padding(.horizontal)
            }
            .padding()
            .background {
                LinearGradient(
                    colors: [.clear, Color.black.opacity(0.6)],
                    startPoint: .top,
                    endPoint: .bottom
                )
            }
        }
        .sheet(isPresented: $showAudioSelection) {
            AudioTrackSelectionView()
                .presentationDetents([.medium])
        }
        .sheet(isPresented: $showSubtitleSelection) {
            SubtitleSelectionView()
                .presentationDetents([.medium])
        }
        .sheet(isPresented: $showSpeedSelection) {
            PlaybackSpeedView()
                .presentationDetents([.height(200)])
        }
    }
    
    // MARK: - Progress Section
    private func ProgressSection() -> some View {
        VStack(spacing: 8) {
            // Seek bar
            GeometryReader { geometry in
                ZStack(alignment: .leading) {
                    // Buffered progress
                    Color.gray.opacity(0.4)
                        .frame(width: geometry.size.width * playerManager.bufferedProgress)
                    
                    // Playback progress
                    Color.crimson
                        .frame(width: geometry.size.width * (playerManager.duration > 0 ? playerManager.currentTime / playerManager.duration : 0))
                }
                .cornerRadius(2)
                .frame(height: 4)
                .background(Color.gray.opacity(0.2))
            }
            .frame(height: 4)
            
            // Time labels
            HStack {
                Text(formatTime(playerManager.currentTime))
                    .font(.caption)
                    .foregroundColor(.white)
                
                Spacer()
                
                Text("-\(formatTime(playerManager.duration - playerManager.currentTime))")
                    .font(.caption)
                    .foregroundColor(.white)
            }
        }
    }
    
    // MARK: - Side Adjustment Panel
    private func SideAdjustmentPanel(offset: CGFloat, width: CGFloat) -> some View {
        HStack {
            if offset > 0 {
                Spacer()
            }
            
            VStack {
                Image(systemName: offset > 0 ? "speaker.wave.2" : "sun.max")
                Text("\(Int((offset / width) * 100))%")
                    .font(.caption)
            }
            .padding()
            .background(Color.black.opacity(0.6))
            .cornerRadius(8)
            
            if offset < 0 {
                Spacer()
            }
        }
        .padding()
    }
    
    // MARK: - Helper Methods
    private func toggleControls() {
        withAnimation(.easeInOut(duration: 0.2)) {
            showControls.toggle()
        }
        if showControls {
            startControlsTimer()
        }
    }
    
    private func startControlsTimer() {
        stopControlsTimer()
        controlsTimer = Timer.scheduledTimer(withTimeInterval: 3.0, repeats: false) { _ in
            if playerManager.isPlaying {
                withAnimation {
                    showControls = false
                }
            }
        }
    }
    
    private func stopControlsTimer() {
        controlsTimer?.invalidate()
        controlsTimer = nil
    }
    
    private func closePlayer() {
        playerManager.isFullScreenPlayerPresented = false
    }
    
    private func togglePiP() {
        if playerManager.isPiPMode {
            playerManager.stopPiP()
        } else {
            playerManager.startPiP()
        }
    }
    
    private func handleDragEnd(value: DragGesture.Value, width: CGFloat) {
        // Volume adjustment for right side drag
        // Brightness adjustment for left side drag
    }
    
    private func formatTime(_ seconds: Double) -> String {
        let hours = Int(seconds) / 3600
        let minutes = Int(seconds) % 3600 / 60
        let secs = Int(seconds) % 60
        
        if hours > 0 {
            return String(format: "%d:%02d:%02d", hours, minutes, secs)
        } else {
            return String(format: "%d:%02d", minutes, secs)
        }
    }
}

// MARK: - AirPlay Button
struct AirPlayButton: UIViewRepresentable {
    func makeUIView(context: Context) -> AVRoutePickerView {
        let picker = AVRoutePickerView()
        picker.activeTintColor = UIColor(Color.crimson)
        picker.backgroundColor = .clear
        return picker
    }
    
    func updateUIView(_ uiView: AVRoutePickerView, context: Context) {}
}

// MARK: - Audio Track Selection View
struct AudioTrackSelectionView: View {
    @EnvironmentObject var playerManager: PlayerManager
    @Environment(\.dismiss) var dismiss
    
    var body: some View {
        NavigationStack {
            List {
                Button("Off") {
                    playerManager.selectAudioTrack(nil)
                    dismiss()
                }
                .foregroundColor(playerManager.selectedAudioTrack == nil ? .crimson : .primary)
                
                ForEach(playerManager.availableAudioTracks()) { track in
                    Button(track.title) {
                        playerManager.selectAudioTrack(track.id)
                        dismiss()
                    }
                    .foregroundColor(playerManager.selectedAudioTrack == track.id ? .crimson : .primary)
                }
            }
            .navigationTitle("Audio")
            .navigationBarTitleDisplayMode(.inline)
            .toolbar {
                ToolbarItem(placement: .cancellationAction) {
                    Button("Done") { dismiss() }
                }
            }
        }
    }
}

// MARK: - Subtitle Selection View
struct SubtitleSelectionView: View {
    @EnvironmentObject var playerManager: PlayerManager
    @Environment(\.dismiss) var dismiss
    
    var body: some View {
        NavigationStack {
            List {
                Button("Off") {
                    playerManager.selectSubtitle(nil)
                    dismiss()
                }
                .foregroundColor(playerManager.selectedSubtitle == nil ? .crimson : .primary)
                
                ForEach(playerManager.availableSubtitles()) { track in
                    Button(track.title) {
                        playerManager.selectSubtitle(track.id)
                        dismiss()
                    }
                    .foregroundColor(playerManager.selectedSubtitle == track.id ? .crimson : .primary)
                }
            }
            .navigationTitle("Subtitles")
            .navigationBarTitleDisplayMode(.inline)
            .toolbar {
                ToolbarItem(placement: .cancellationAction) {
                    Button("Done") { dismiss() }
                }
            }
        }
    }
}

// MARK: - Playback Speed View
struct PlaybackSpeedView: View {
    @EnvironmentObject var playerManager: PlayerManager
    @Environment(\.dismiss) var dismiss
    
    let speeds = [0.25, 0.5, 0.75, 1.0, 1.25, 1.5, 1.75, 2.0]
    
    var body: some View {
        NavigationStack {
            HStack(spacing: 12) {
                ForEach(speeds, id: \.self) { speed in
                    Button {
                        playerManager.setPlaybackRate(speed)
                        dismiss()
                    } label: {
                        Text("\(speed == 1.0 ? "1×" : String(format: "%.2gx", speed))")
                            .font(.subheadline)
                            .foregroundColor(playerManager.playbackRate == speed ? .white : .primary)
                            .padding(.vertical, 8)
                            .frame(minWidth: 50)
                            .background(
                                playerManager.playbackRate == speed ? Color.crimson : Color.gray.opacity(0.2)
                            )
                            .cornerRadius(8)
                    }
                }
            }
            .padding()
            .navigationTitle("Playback Speed")
            .navigationBarTitleDisplayMode(.inline)
            .toolbar {
                ToolbarItem(placement: .cancellationAction) {
                    Button("Done") { dismiss() }
                }
            }
        }
    }
}

#Preview {
    FullScreenPlayerView()
        .environmentObject(PlayerManager())
        .environmentObject(AppState())
}