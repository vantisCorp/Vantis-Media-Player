---
sidebar_position: 2
---

# Web Deployment

This guide covers deploying Vantis Media Player as a web application, including CDN hosting, static site deployment, and server-side rendering.

## Overview

Vantis Media Player can be deployed as a web application, making it accessible through any modern web browser. This section covers various deployment strategies for web applications.

## Static Hosting

### Building for Production

```bash
# Build the application
npm run build

# Output structure
dist/
├── index.html
├── assets/
│   ├── index.js
│   ├── index.css
│   └── vantis-player.js
└── media/
    └── samples/
```

### Vite Configuration

```typescript
// vite.config.ts

import { defineConfig } from 'vite';
import react from '@vitejs/plugin-react';

export default defineConfig({
  plugins: [react()],
  build: {
    target: 'esnext',
    outDir: 'dist',
    minify: 'terser',
    terserOptions: {
      compress: {
        drop_console: true
      }
    },
    rollupOptions: {
      output: {
        manualChunks: {
          'vantis-player': ['@vantis/player'],
          'vendor': ['react', 'react-dom']
        }
      }
    }
  },
  base: '/'
});
```

### Nginx Configuration

```nginx
# /etc/nginx/sites-available/vantis-player

server {
    listen 80;
    listen [::]:80;
    server_name player.vantis.media;
    
    # Redirect to HTTPS
    return 301 https://$server_name$request_uri;
}

server {
    listen 443 ssl http2;
    listen [::]:443 ssl http2;
    server_name player.vantis.media;
    
    # SSL Configuration
    ssl_certificate /etc/letsencrypt/live/player.vantis.media/fullchain.pem;
    ssl_certificate_key /etc/letsencrypt/live/player.vantis.media/privkey.pem;
    ssl_session_timeout 1d;
    ssl_session_cache shared:SSL:50m;
    ssl_session_tickets off;
    
    # Modern SSL Configuration
    ssl_protocols TLSv1.2 TLSv1.3;
    ssl_ciphers ECDHE-ECDSA-AES128-GCM-SHA256:ECDHE-RSA-AES128-GCM-SHA256;
    ssl_prefer_server_ciphers off;
    
    # HSTS
    add_header Strict-Transport-Security "max-age=63072000" always;
    
    # Security Headers
    add_header X-Content-Type-Options nosniff;
    add_header X-Frame-Options DENY;
    add_header X-XSS-Protection "1; mode=block";
    add_header Referrer-Policy "strict-origin-when-cross-origin";
    
    root /var/www/vantis-player/dist;
    index index.html;
    
    # Cache static assets
    location ~* \.(js|css|png|jpg|jpeg|gif|ico|svg|woff|woff2)$ {
        expires 1y;
        add_header Cache-Control "public, immutable";
    }
    
    # Cache media files
    location /media/ {
        expires 30d;
        add_header Cache-Control "public";
    }
    
    # SPA fallback
    location / {
        try_files $uri $uri/ /index.html;
    }
    
    # Gzip compression
    gzip on;
    gzip_types text/plain text/css application/json application/javascript text/xml application/xml;
    gzip_min_length 1000;
    
    # Brotli compression (if installed)
    brotli on;
    brotli_types text/plain text/css application/json application/javascript text/xml application/xml;
}
```

## CDN Deployment

### CloudFlare Configuration

```javascript
// CloudFlare Worker for edge caching

addEventListener('fetch', event => {
  event.respondWith(handleRequest(event.request));
});

async function handleRequest(request) {
  const cache = caches.default;
  
  // Check cache first
  let response = await cache.match(request);
  if (response) {
    return response;
  }
  
  // Fetch from origin
  response = await fetch(request);
  
  // Cache static assets
  if (request.url.match(/\.(js|css|png|jpg|svg|woff2)$/)) {
    const headers = new Headers(response.headers);
    headers.set('Cache-Control', 'public, max-age=31536000, immutable');
    headers.set('CDN-Cache-Control', 'public, max-age=31536000');
    
    response = new Response(response.body, {
      status: response.status,
      statusText: response.statusText,
      headers
    });
    
    event.waitUntil(cache.put(request, response.clone()));
  }
  
  return response;
}
```

### AWS CloudFront

```yaml
# cloudfront.yaml - CloudFormation template

AWSTemplateFormatVersion: '2010-09-09'
Description: Vantis Media Player CloudFront Distribution

Resources:
  PlayerBucket:
    Type: AWS::S3::Bucket
    Properties:
      BucketName: vantis-player-prod
      AccessControl: Private
      
  PlayerBucketPolicy:
    Type: AWS::S3::BucketPolicy
    Properties:
      Bucket: !Ref PlayerBucket
      PolicyDocument:
        Version: '2012-10-17'
        Statement:
          - Effect: Allow
            Principal:
              AWS: !Sub 'arn:aws:iam::cloudfront:user/CloudFront Origin Access Identity ${CloudFrontOAI}'
            Action: 's3:GetObject'
            Resource: !Sub '${PlayerBucket}/*'
            
  CloudFrontOAI:
    Type: AWS::CloudFront::CloudFrontOriginAccessIdentity
    Properties:
      CloudFrontOriginAccessIdentityConfig:
        Comment: 'Vantis Player OAI'
        
  CloudFrontDistribution:
    Type: AWS::CloudFront::Distribution
    Properties:
      DistributionConfig:
        Enabled: true
        DefaultRootObject: index.html
        HttpVersion: http2
        PriceClass: PriceClass_100
        ViewerCertificate:
          AcmCertificateArn: !Ref SSLCertificate
          SslSupportMethod: sni-only
          MinimumProtocolVersion: TLSv1.2_2021
        Aliases:
          - player.vantis.media
        Origins:
          - DomainName: !GetAtt PlayerBucket.DomainName
            Id: S3Origin
            S3OriginConfig:
              OriginAccessIdentity: !Sub 'origin-access-identity/cloudfront/${CloudFrontOAI}'
        DefaultCacheBehavior:
          TargetOriginId: S3Origin
          ViewerProtocolPolicy: redirect-to-https
          AllowedMethods:
            - GET
            - HEAD
            - OPTIONS
          CachedMethods:
            - GET
            - HEAD
          Compress: true
          ForwardedValues:
            QueryString: false
          DefaultTTL: 86400
          MaxTTL: 31536000
        CacheBehaviors:
          - PathPattern: 'assets/*'
            TargetOriginId: S3Origin
            ViewerProtocolPolicy: redirect-to-https
            AllowedMethods:
              - GET
              - HEAD
            CachedMethods:
              - GET
              - HEAD
            Compress: true
            ForwardedValues:
              QueryString: false
            DefaultTTL: 31536000
            MaxTTL: 31536000
        CustomErrorResponses:
          - ErrorCode: 404
            ResponseCode: 200
            ResponsePagePath: /index.html
```

## Vercel Deployment

### Configuration

```json
// vercel.json

{
  "version": 2,
  "builds": [
    {
      "src": "package.json",
      "use": "@vercel/node"
    }
  ],
  "routes": [
    {
      "src": "/assets/(.*)",
      "headers": {
        "Cache-Control": "public, max-age=31536000, immutable"
      }
    },
    {
      "src": "/media/(.*)",
      "headers": {
        "Cache-Control": "public, max-age=2592000"
      }
    },
    {
      "src": "/(.*)",
      "dest": "/index.html"
    }
  ],
  "headers": [
    {
      "source": "/(.*)",
      "headers": [
        {
          "key": "X-Content-Type-Options",
          "value": "nosniff"
        },
        {
          "key": "X-Frame-Options",
          "value": "DENY"
        },
        {
          "key": "X-XSS-Protection",
          "value": "1; mode=block"
        },
        {
          "key": "Referrer-Policy",
          "value": "strict-origin-when-cross-origin"
        }
      ]
    }
  ]
}
```

## Netlify Deployment

### Configuration

```yaml
# netlify.yml

build:
  command: npm run build
  publish: dist

headers:
  - path: /*
    headers:
      - key: X-Content-Type-Options
        value: nosniff
      - key: X-Frame-Options
        value: DENY
      - key: X-XSS-Protection
        value: 1; mode=block
      
  - path: /assets/*
    headers:
      - key: Cache-Control
        value: public, max-age=31536000, immutable
        
  - path: /media/*
    headers:
      - key: Cache-Control
        value: public, max-age=2592000

redirects:
  - from: /*
    to: /index.html
    status: 200
```

## Progressive Web App (PWA)

### Service Worker

```javascript
// public/sw.js

const CACHE_NAME = 'vantis-player-v1';
const STATIC_ASSETS = [
  '/',
  '/index.html',
  '/assets/index.js',
  '/assets/index.css',
  '/assets/vantis-player.js'
];

// Install event
self.addEventListener('install', (event) => {
  event.waitUntil(
    caches.open(CACHE_NAME)
      .then((cache) => cache.addAll(STATIC_ASSETS))
      .then(() => self.skipWaiting())
  );
});

// Activate event
self.addEventListener('activate', (event) => {
  event.waitUntil(
    caches.keys()
      .then((keys) => {
        return Promise.all(
          keys.filter((key) => key !== CACHE_NAME)
            .map((key) => caches.delete(key))
        );
      })
      .then(() => self.clients.claim())
  );
});

// Fetch event
self.addEventListener('fetch', (event) => {
  const { request } = event;
  
  // Skip non-GET requests
  if (request.method !== 'GET') return;
  
  // Skip media files (too large for cache)
  if (request.url.match(/\.(mp4|mkv|avi|mp3|flac)$/)) {
    return;
  }
  
  event.respondWith(
    caches.match(request)
      .then((cached) => {
        if (cached) {
          return cached;
        }
        
        return fetch(request)
          .then((response) => {
            // Cache successful responses
            if (response.ok && request.url.startsWith(self.location.origin)) {
              const responseClone = response.clone();
              caches.open(CACHE_NAME)
                .then((cache) => cache.put(request, responseClone));
            }
            
            return response;
          });
      })
  );
});
```

### PWA Manifest

```json
// public/manifest.json

{
  "name": "Vantis Media Player",
  "short_name": "Vantis Player",
  "description": "Modern media player for the web",
  "start_url": "/",
  "display": "standalone",
  "background_color": "#1a1a2e",
  "theme_color": "#6366f1",
  "orientation": "any",
  "icons": [
    {
      "src": "/icons/icon-72x72.png",
      "sizes": "72x72",
      "type": "image/png"
    },
    {
      "src": "/icons/icon-96x96.png",
      "sizes": "96x96",
      "type": "image/png"
    },
    {
      "src": "/icons/icon-128x128.png",
      "sizes": "128x128",
      "type": "image/png"
    },
    {
      "src": "/icons/icon-144x144.png",
      "sizes": "144x144",
      "type": "image/png"
    },
    {
      "src": "/icons/icon-152x152.png",
      "sizes": "152x152",
      "type": "image/png"
    },
    {
      "src": "/icons/icon-192x192.png",
      "sizes": "192x192",
      "type": "image/png"
    },
    {
      "src": "/icons/icon-384x384.png",
      "sizes": "384x384",
      "type": "image/png"
    },
    {
      "src": "/icons/icon-512x512.png",
      "sizes": "512x512",
      "type": "image/png"
    }
  ],
  "categories": ["entertainment", "utilities"],
  "screenshots": [
    {
      "src": "/screenshots/player.png",
      "sizes": "1280x720",
      "type": "image/png"
    }
  ],
  "related_applications": [],
  "prefer_related_applications": false
}
```

## Performance Optimization

### Code Splitting

```javascript
// Lazy load player component
import { lazy, Suspense } from 'react';

const VantisPlayer = lazy(() => import('@vantis/player'));

function App() {
  return (
    <Suspense fallback={<div>Loading player...</div>}>
      <VantisPlayer source={source} />
    </Suspense>
  );
}
```

### Preload Critical Assets

```html
<!-- Preload critical assets -->
<head>
  <link rel="preload" href="/assets/vantis-player.js" as="script">
  <link rel="preload" href="/assets/index.css" as="style">
  <link rel="preconnect" href="https://cdn.vantis.media">
  <link rel="dns-prefetch" href="https://cdn.vantis.media">
</head>
```

## Security Headers

```html
<!-- Security headers for index.html -->
<meta http-equiv="Content-Security-Policy" content="
  default-src 'self';
  script-src 'self' 'unsafe-inline' 'unsafe-eval';
  style-src 'self' 'unsafe-inline';
  img-src 'self' data: blob: https:;
  media-src 'self' blob: https:;
  connect-src 'self' https: wss:;
  font-src 'self' data:;
  object-src 'none';
  frame-ancestors 'none';
">
<meta http-equiv="X-Content-Type-Options" content="nosniff">
<meta http-equiv="X-Frame-Options" content="DENY">
<meta http-equiv="X-XSS-Protection" content="1; mode=block">
```

## Monitoring

### Analytics Integration

```javascript
// Google Analytics
gtag('event', 'player_load', {
  event_category: 'Player',
  event_label: 'Video',
  value: 1
});

// Custom analytics
window.player.on('player:play', () => {
  analytics.track('video_play', {
    video_id: currentVideo.id,
    video_title: currentVideo.title,
    current_time: window.player.getCurrentTime()
  });
});
```

### Error Tracking

```javascript
// Sentry integration
import * as Sentry from '@sentry/browser';

Sentry.init({
  dsn: 'https://example@sentry.io/123',
  environment: process.env.NODE_ENV,
  release: process.env.APP_VERSION
});

// Capture player errors
player.on('player:error', (error) => {
  Sentry.captureException(error);
});
```

## Best Practices

1. **Use HTTPS**: Always serve over HTTPS
2. **Enable compression**: Use Gzip or Brotli
3. **Cache static assets**: Set proper cache headers
4. **Optimize images**: Use WebP and lazy loading
5. **Minimize bundle size**: Code splitting and tree shaking
6. **Use CDN**: Distribute assets globally
7. **Monitor performance**: Use Lighthouse and RUM
8. **Implement security headers**: Protect against common attacks

## Related Documentation

- [Desktop Deployment](./desktop) - Deploy as desktop application
- [Mobile Deployment](./mobile) - Deploy to mobile platforms
- [Docker Deployment](./docker) - Deploy with Docker
- [Cloud Deployment](./cloud) - Deploy to cloud infrastructure