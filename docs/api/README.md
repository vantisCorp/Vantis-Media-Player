# Vantis Media Player API Documentation

> **Version:** 1.0.0 | **Base URL:** `http://localhost:8080/api/v1`

---

## Overview

Vantis Media Player provides a comprehensive REST API for media playback, library management, plugin system, and streaming capabilities.

## Authentication

All API endpoints require authentication via Bearer token:

```http
Authorization: Bearer <your-api-token>
```

## Endpoints

### Media Playback

| Endpoint | Method | Description |
|----------|--------|-------------|
| `/playback/play` | POST | Start playback |
| `/playback/pause` | POST | Pause playback |
| `/playback/stop` | POST | Stop playback |
| `/playback/seek` | POST | Seek to position |
| `/playback/volume` | PUT | Set volume level |

### Library Management

| Endpoint | Method | Description |
|----------|--------|-------------|
| `/library` | GET | List all media |
| `/library/{id}` | GET | Get media details |
| `/library/search` | GET | Search library |
| `/library/scan` | POST | Scan for new media |

### Plugins

| Endpoint | Method | Description |
|----------|--------|-------------|
| `/plugins` | GET | List installed plugins |
| `/plugins/{id}` | GET | Get plugin details |
| `/plugins/install` | POST | Install plugin |
| `/plugins/{id}/enable` | PUT | Enable plugin |

### Streaming

| Endpoint | Method | Description |
|----------|--------|-------------|
| `/stream/{id}` | GET | Stream media |
| `/stream/url` | POST | Stream from URL |

---

## Rate Limits

| Plan | Requests/hour |
|------|---------------|
| Free | 100 |
| Pro | 1000 |
| Enterprise | Unlimited |

## Error Codes

| Code | Description |
|------|-------------|
| 400 | Bad Request |
| 401 | Unauthorized |
| 403 | Forbidden |
| 404 | Not Found |
| 429 | Rate Limited |
| 500 | Internal Error |

---

*Generated with ❤️ by Vantis Team*