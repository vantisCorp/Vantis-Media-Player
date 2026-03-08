package com.vantis.media.player

import android.app.Application
import android.app.NotificationChannel
import android.app.NotificationManager
import android.content.Context
import dagger.hilt.android.HiltAndroidApp

@HiltAndroidApp
class VantisApplication : Application() {
    
    override fun onCreate() {
        super.onCreate()
        
        // Initialize Hilt
        // Hilt is automatically initialized by @HiltAndroidApp annotation
        
        // Create notification channels
        createNotificationChannels()
        
        // Initialize native library
        try {
            System.loadLibrary("vantis_core")
        } catch (e: UnsatisfiedLinkError) {
            // Native library not available in debug builds
        }
    }
    
    private fun createNotificationChannels() {
        val notificationManager = getSystemService(Context.NOTIFICATION_SERVICE) as NotificationManager
        
        // Playback channel
        val playbackChannel = NotificationChannel(
            CHANNEL_PLAYBACK,
            "Media Playback",
            NotificationManager.IMPORTANCE_LOW
        ).apply {
            description = "Media playback controls"
            setShowBadge(false)
        }
        
        // Download channel
        val downloadChannel = NotificationChannel(
            CHANNEL_DOWNLOAD,
            "Downloads",
            NotificationManager.IMPORTANCE_DEFAULT
        ).apply {
            description = "Download progress and status"
        }
        
        // Sync channel
        val syncChannel = NotificationChannel(
            CHANNEL_SYNC,
            "Cloud Sync",
            NotificationManager.IMPORTANCE_DEFAULT
        ).apply {
            description = "Cloud synchronization status"
        }
        
        notificationManager.createNotificationChannels(
            listOf(playbackChannel, downloadChannel, syncChannel)
        )
    }
    
    companion object {
        const val CHANNEL_PLAYBACK = "vantis_playback"
        const val CHANNEL_DOWNLOAD = "vantis_download"
        const val CHANNEL_SYNC = "vantis_sync"
    }
}