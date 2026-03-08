package com.vantis.media.player

import android.content.Context
import androidx.media3.common.MediaItem
import androidx.media3.common.Player
import androidx.media3.exoplayer.ExoPlayer
import dagger.hilt.android.qualifiers.ApplicationContext
import javax.inject.Inject
import javax.inject.Singleton

@Singleton
class PlayerManager @Inject constructor(
    @ApplicationContext private val context: Context
) {
    private var player: ExoPlayer? = null
    private var currentItem: MediaItem? = null
    
    val exoPlayer: ExoPlayer?
        get() = player
    
    fun initialize(): ExoPlayer {
        if (player == null) {
            player = ExoPlayer.Builder(context)
                .setSeekBackIncrementMs(10000)
                .setSeekForwardIncrementMs(10000)
                .build()
        }
        return player!!
    }
    
    fun play(item: com.vantis.media.player.MediaItem) {
        val player = initialize()
        
        val mediaItem = item.streamUrl.let { url ->
            MediaItem.fromUri(url)
        }
        
        player.setMediaItem(mediaItem)
        player.prepare()
        player.playWhenReady = true
        currentItem = mediaItem
    }
    
    fun pause() {
        player?.pause()
    }
    
    fun resume() {
        player?.play()
    }
    
    fun stop() {
        player?.stop()
        currentItem = null
    }
    
    fun seekTo(positionMs: Long) {
        player?.seekTo(positionMs)
    }
    
    fun getCurrentPosition(): Long {
        return player?.currentPosition ?: 0
    }
    
    fun getDuration(): Long {
        return player?.duration ?: 0
    }
    
    fun isPlaying(): Boolean {
        return player?.isPlaying ?: false
    }
    
    fun setPlaybackSpeed(speed: Float) {
        player?.setPlaybackSpeed(speed)
    }
    
    fun setVolume(volume: Float) {
        player?.volume = volume
    }
    
    fun addListener(listener: Player.Listener) {
        player?.addListener(listener)
    }
    
    fun removeListener(listener: Player.Listener) {
        player?.removeListener(listener)
    }
    
    fun release() {
        player?.release()
        player = null
    }
    
    // Native bridge for Rust FFI
    external fun nativeInit(configPath: String)
    external fun nativePlay(url: String)
    external fun nativePause()
    external fun nativeResume()
    external fun nativeStop()
    external fun nativeSeek(seconds: Double)
    external fun nativeGetPosition(): Double
    external fun nativeGetDuration(): Double
    external fun nativeSetVolume(volume: Float)
    external fun nativeLoadSubtitle(path: String, language: String)
    external fun nativeSetSubtitleEnabled(enabled: Boolean)
    external fun nativeBeginSync()
    external fun nativeGetSyncStatus(): Int
    
    companion object {
        init {
            System.loadLibrary("vantis_core")
        }
    }
}