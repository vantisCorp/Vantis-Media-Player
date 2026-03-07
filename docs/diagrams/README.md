# Vantis Media Player - Architecture Diagrams

> **Visual documentation using Mermaid.js**

---

## System Architecture

```mermaid
graph TB
    subgraph UI Layer
        A[Liquid Glass UI]
        B[Omnibar]
        C[Controls]
        D[Library View]
        E[Marketplace]
    end

    subgraph Core Layer
        F[Playback Engine]
        G[Media Decoder]
        H[Stream Handler]
        I[Subtitle Engine]
    end

    subgraph Data Layer
        J[(Library DB)]
        K[(Settings)]
        L[(Cache)]
    end

    subgraph Plugin System
        M[Plugin Manager]
        N[Plugin API]
        O[Plugin Sandbox]
    end

    subgraph External
        P[Network Streams]
        Q[Hardware Accel]
        R[GPU Rendering]
    end

    A --> F
    B --> F
    C --> F
    D --> J
    E --> M

    F --> G
    F --> H
    F --> I

    G --> Q
    H --> P
    A --> R

    M --> N
    N --> O
```

## Playback Flow

```mermaid
sequenceDiagram
    participant User
    participant UI
    participant Playback
    participant Decoder
    participant Audio
    participant Video

    User->>UI: Click Play
    UI->>Playback: play(media_id)
    Playback->>Decoder: decode(media)

    par Audio Processing
        Decoder->>Audio: audio_stream
        Audio-->>Playback: audio_buffer
    and Video Processing
        Decoder->>Video: video_stream
        Video-->>Playback: video_frame
    end

    Playback-->>UI: playback_state
    UI-->>User: Show progress
```

## Plugin Architecture

```mermaid
graph LR
    subgraph Core
        A[Plugin Manager]
        B[Plugin Registry]
        C[API Layer]
    end

    subgraph Sandbox
        D[WASM Runtime]
        E[Permission System]
        F[Resource Limits]
    end

    subgraph Plugins
        G[Video Filters]
        H[Audio Effects]
        I[Subtitles]
        J[Streaming]
    end

    A --> B
    A --> C
    C --> D
    D --> E
    E --> F

    G --> D
    H --> D
    I --> D
    J --> D
```

## Data Flow

```mermaid
flowchart TD
    A[Media File] --> B{Format?}
    B -->|Video| C[Video Decoder]
    B -->|Audio| D[Audio Decoder]
    B -->|Subtitle| E[Subtitle Parser]

    C --> F[Frame Buffer]
    D --> G[Audio Buffer]
    E --> H[Text Renderer]

    F --> I[GPU Renderer]
    G --> J[Audio Output]
    H --> I

    I --> K[Display]
    J --> L[Speakers]
```

## Component Dependencies

```mermaid
graph BT
    A[UI Components] --> B[Core Services]
    B --> C[Platform Layer]
    C --> D[System APIs]

    subgraph UI Components
        A1[Library]
        A2[Player]
        A3[Settings]
        A4[Plugins]
    end

    subgraph Core Services
        B1[Playback]
        B2[Storage]
        B3[Network]
        B4[Events]
    end

    subgraph Platform Layer
        C1[File System]
        C2[Audio]
        C3[Video]
        C4[GPU]
    end
```

---

## How to View

These diagrams render natively in:
- GitHub Markdown
- VS Code (with Mermaid extension)
- Docusaurus
- Notion

---

*Generated with ❤️ by Vantis Team*