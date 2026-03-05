---
sidebar_position: 4
title: Docker Deployment
sidebar_label: Docker
---

# Docker Deployment

Deploy Vantis Media Player using Docker for consistent, reproducible deployments across different environments.

## Overview

Docker provides containerization for Vantis Media Player, ensuring consistent behavior across development, staging, and production environments. This guide covers container creation, orchestration, and production deployment strategies.

## Basic Docker Setup

### Dockerfile

Create a production-ready Dockerfile:

```dockerfile
# Multi-stage build for optimal image size
FROM node:20-alpine AS builder

# Set working directory
WORKDIR /app

# Install build dependencies
RUN apk add --no-cache \
    python3 \
    make \
    g++ \
    git

# Copy package files
COPY package*.json ./
COPY pnpm-lock.yaml ./

# Install pnpm
RUN npm install -g pnpm

# Install dependencies
RUN pnpm install --frozen-lockfile

# Copy source code
COPY . .

# Build application
RUN pnpm run build

# Production stage
FROM nginx:alpine

# Install additional runtime dependencies
RUN apk add --no-cache \
    ffmpeg \
    ca-certificates

# Copy built assets from builder
COPY --from=builder /app/dist /usr/share/nginx/html

# Copy nginx configuration
COPY nginx.conf /etc/nginx/nginx.conf

# Copy custom configuration
COPY default.conf /etc/nginx/conf.d/default.conf

# Create non-root user
RUN addgroup -g 1001 -S vantis && \
    adduser -S -D -H -u 1001 -s /sbin/nologin -G vantis -G vantis vantis

# Set permissions
RUN chown -R vantis:vantis /usr/share/nginx/html && \
    chown -R vantis:vantis /var/cache/nginx && \
    chown -R vantis:vantis /var/log/nginx && \
    chown -R vantis:vantis /etc/nginx/conf.d && \
    touch /var/run/nginx.pid && \
    chown -R vantis:vantis /var/run/nginx.pid

# Switch to non-root user
USER vantis

# Expose port
EXPOSE 80

# Health check
HEALTHCHECK --interval=30s --timeout=3s --start-period=5s --retries=3 \
  CMD wget --quiet --tries=1 --spider http://localhost:80/health || exit 1

# Start nginx
CMD ["nginx", "-g", "daemon off;"]
```

### Docker Compose

Create `docker-compose.yml` for local development:

```yaml
version: '3.8'

services:
  vantis-player:
    build:
      context: .
      dockerfile: Dockerfile
    ports:
      - "8080:80"
    environment:
      - NODE_ENV=production
      - LOG_LEVEL=info
    volumes:
      - ./config:/etc/vantis/config:ro
      - ./logs:/var/log/nginx:rw
    restart: unless-stopped
    healthcheck:
      test: ["CMD", "wget", "--quiet", "--tries=1", "--spider", "http://localhost:80/health"]
      interval: 30s
      timeout: 10s
      retries: 3
      start_period: 40s
    networks:
      - vantis-network

  redis:
    image: redis:7-alpine
    ports:
      - "6379:6379"
    volumes:
      - redis-data:/data
    restart: unless-stopped
    networks:
      - vantis-network

  postgres:
    image: postgres:15-alpine
    ports:
      - "5432:5432"
    environment:
      - POSTGRES_USER=vantis
      - POSTGRES_PASSWORD=secure_password
      - POSTGRES_DB=vantis_db
    volumes:
      - postgres-data:/var/lib/postgresql/data
    restart: unless-stopped
    networks:
      - vantis-network

volumes:
  redis-data:
  postgres-data:

networks:
  vantis-network:
    driver: bridge
```

## Nginx Configuration

### nginx.conf

```nginx
user vantis;
worker_processes auto;
error_log /var/log/nginx/error.log warn;
pid /var/run/nginx.pid;

events {
    worker_connections 1024;
    use epoll;
    multi_accept on;
}

http {
    include /etc/nginx/mime.types;
    default_type application/octet-stream;

    log_format main '$remote_addr - $remote_user [$time_local] "$request" '
                    '$status $body_bytes_sent "$http_referer" '
                    '"$http_user_agent" "$http_x_forwarded_for"';

    access_log /var/log/nginx/access.log main;

    sendfile on;
    tcp_nopush on;
    tcp_nodelay on;
    keepalive_timeout 65;
    types_hash_max_size 2048;
    client_max_body_size 20M;

    # Gzip compression
    gzip on;
    gzip_vary on;
    gzip_proxied any;
    gzip_comp_level 6;
    gzip_types text/plain text/css text/xml text/javascript 
               application/json application/javascript application/xml+rss 
               application/rss+xml font/truetype font/opentype 
               application/vnd.ms-fontobject image/svg+xml;

    include /etc/nginx/conf.d/*.conf;
}
```

### default.conf

```nginx
server {
    listen 80;
    server_name _;

    root /usr/share/nginx/html;
    index index.html;

    # Security headers
    add_header X-Frame-Options "SAMEORIGIN" always;
    add_header X-Content-Type-Options "nosniff" always;
    add_header X-XSS-Protection "1; mode=block" always;
    add_header Referrer-Policy "no-referrer-when-downgrade" always;

    # PWA support
    location / {
        try_files $uri $uri/ /index.html;
        add_header Cache-Control "public, max-age=31536000, immutable";
    }

    # Service worker
    location /service-worker.js {
        add_header Cache-Control "public, max-age=0, must-revalidate";
    }

    # Static assets
    location ~* \.(js|css|png|jpg|jpeg|gif|ico|svg|woff|woff2|ttf|eot)$ {
        expires 1y;
        add_header Cache-Control "public, immutable";
    }

    # API proxy
    location /api/ {
        proxy_pass http://backend:3000;
        proxy_http_version 1.1;
        proxy_set_header Upgrade $http_upgrade;
        proxy_set_header Connection 'upgrade';
        proxy_set_header Host $host;
        proxy_cache_bypass $http_upgrade;
        proxy_set_header X-Real-IP $remote_addr;
        proxy_set_header X-Forwarded-For $proxy_add_x_forwarded_for;
        proxy_set_header X-Forwarded-Proto $scheme;
    }

    # Health check endpoint
    location /health {
        access_log off;
        return 200 "healthy\n";
        add_header Content-Type text/plain;
    }
}
```

## Production Docker Compose

### docker-compose.prod.yml

```yaml
version: '3.8'

services:
  vantis-player:
    image: vantis/player:${VERSION:-latest}
    deploy:
      replicas: 3
      update_config:
        parallelism: 1
        delay: 10s
        order: start-first
      restart_policy:
        condition: on-failure
        delay: 5s
        max_attempts: 3
        window: 120s
      resources:
        limits:
          cpus: '2'
          memory: 1G
        reservations:
          cpus: '0.5'
          memory: 512M
    environment:
      - NODE_ENV=production
      - LOG_LEVEL=warn
      - ENABLE_METRICS=true
    secrets:
      - api_key
      - jwt_secret
    ports:
      - "80:80"
    networks:
      - vantis-network
    healthcheck:
      test: ["CMD", "wget", "--quiet", "--tries=1", "--spider", "http://localhost:80/health"]
      interval: 30s
      timeout: 10s
      retries: 3
      start_period: 40s

  nginx:
    image: nginx:alpine
    ports:
      - "443:443"
      - "80:80"
    volumes:
      - ./nginx/nginx.conf:/etc/nginx/nginx.conf:ro
      - ./nginx/ssl:/etc/nginx/ssl:ro
    depends_on:
      - vantis-player
    networks:
      - vantis-network
    restart: unless-stopped

secrets:
  api_key:
    file: ./secrets/api_key.txt
  jwt_secret:
    file: ./secrets/jwt_secret.txt

networks:
  vantis-network:
    driver: overlay
```

## Kubernetes Deployment

### Deployment Config

```yaml
apiVersion: apps/v1
kind: Deployment
metadata:
  name: vantis-player
  namespace: production
  labels:
    app: vantis-player
spec:
  replicas: 3
  strategy:
    type: RollingUpdate
    rollingUpdate:
      maxSurge: 1
      maxUnavailable: 0
  selector:
    matchLabels:
      app: vantis-player
  template:
    metadata:
      labels:
        app: vantis-player
    spec:
      containers:
      - name: vantis-player
        image: vantis/player:1.0.0
        ports:
        - containerPort: 80
        env:
        - name: NODE_ENV
          value: "production"
        - name: LOG_LEVEL
          value: "warn"
        resources:
          requests:
            memory: "512Mi"
            cpu: "500m"
          limits:
            memory: "1Gi"
            cpu: "2000m"
        livenessProbe:
          httpGet:
            path: /health
            port: 80
          initialDelaySeconds: 30
          periodSeconds: 10
          timeoutSeconds: 5
          failureThreshold: 3
        readinessProbe:
          httpGet:
            path: /health
            port: 80
          initialDelaySeconds: 10
          periodSeconds: 5
          timeoutSeconds: 3
          failureThreshold: 3
        securityContext:
          runAsNonRoot: true
          runAsUser: 1001
          allowPrivilegeEscalation: false
          readOnlyRootFilesystem: true
      imagePullSecrets:
      - name: registry-credentials
```

### Service Config

```yaml
apiVersion: v1
kind: Service
metadata:
  name: vantis-player-service
  namespace: production
spec:
  selector:
    app: vantis-player
  ports:
  - protocol: TCP
    port: 80
    targetPort: 80
  type: ClusterIP
```

### Ingress Config

```yaml
apiVersion: networking.k8s.io/v1
kind: Ingress
metadata:
  name: vantis-player-ingress
  namespace: production
  annotations:
    nginx.ingress.kubernetes.io/ssl-redirect: "true"
    nginx.ingress.kubernetes.io/force-ssl-redirect: "true"
    cert-manager.io/cluster-issuer: "letsencrypt-prod"
spec:
  tls:
  - hosts:
    - player.vantis.com
    secretName: vantis-player-tls
  rules:
  - host: player.vantis.com
    http:
      paths:
      - path: /
        pathType: Prefix
        backend:
          service:
            name: vantis-player-service
            port:
              number: 80
```

### HorizontalPodAutoscaler

```yaml
apiVersion: autoscaling/v2
kind: HorizontalPodAutoscaler
metadata:
  name: vantis-player-hpa
  namespace: production
spec:
  scaleTargetRef:
    apiVersion: apps/v1
    kind: Deployment
    name: vantis-player
  minReplicas: 3
  maxReplicas: 10
  metrics:
  - type: Resource
    resource:
      name: cpu
      target:
        type: Utilization
        averageUtilization: 70
  - type: Resource
    resource:
      name: memory
      target:
        type: Utilization
        averageUtilization: 80
```

## Build and Deploy

### Build Docker Image

```bash
# Build image
docker build -t vantis/player:1.0.0 .

# Build with build arguments
docker build \
  --build-arg NODE_ENV=production \
  --build-arg VERSION=1.0.0 \
  -t vantis/player:1.0.0 .

# Build with cache
docker build \
  --cache-from vantis/player:latest \
  -t vantis/player:1.0.0 .
```

### Push to Registry

```bash
# Login to registry
docker login registry.example.com

# Tag image
docker tag vantis/player:1.0.0 registry.example.com/vantis/player:1.0.0

# Push image
docker push registry.example.com/vantis/player:1.0.0

# Push all tags
docker push registry.example.com/vantis/player
```

### Deploy with Docker Compose

```bash
# Start services
docker-compose up -d

# Scale services
docker-compose up -d --scale vantis-player=3

# View logs
docker-compose logs -f vantis-player

# Stop services
docker-compose down

# Rebuild and restart
docker-compose up -d --build
```

### Deploy to Kubernetes

```bash
# Apply configurations
kubectl apply -f k8s/deployment.yaml
kubectl apply -f k8s/service.yaml
kubectl apply -f k8s/ingress.yaml

# Check deployment status
kubectl get deployments -n production
kubectl get pods -n production

# View logs
kubectl logs -f deployment/vantis-player -n production

# Scale deployment
kubectl scale deployment vantis-player --replicas=5 -n production
```

## Security Best Practices

### Image Security

```dockerfile
# Use non-root user
USER vantis

# Read-only filesystem
RUN chmod -R 555 /usr/share/nginx/html

# Remove build tools
RUN apk del python3 make g++ git

# Scan for vulnerabilities
RUN trivy image --exit-code 1 --severity CRITICAL vantis/player:1.0.0
```

### Docker Security Scan

```bash
# Scan image with Trivy
docker run --rm -v /var/run/docker.sock:/var/run/docker.sock \
  aquasec/trivy image vantis/player:1.0.0

# Scan with Snyk
snyk container test vantis/player:1.0.0
```

### Kubernetes Security

```yaml
# Security context
securityContext:
  runAsNonRoot: true
  runAsUser: 1001
  fsGroup: 1001
  allowPrivilegeEscalation: false
  readOnlyRootFilesystem: true
  capabilities:
    drop:
    - ALL
```

## Monitoring and Logging

### Docker Compose Monitoring

```yaml
services:
  prometheus:
    image: prom/prometheus:latest
    ports:
      - "9090:9090"
    volumes:
      - ./prometheus.yml:/etc/prometheus/prometheus.yml
    networks:
      - monitoring

  grafana:
    image: grafana/grafana:latest
    ports:
      - "3000:3000"
    volumes:
      - grafana-data:/var/lib/grafana
    networks:
      - monitoring
```

### Kubernetes Monitoring

```yaml
apiVersion: v1
kind: ConfigMap
metadata:
  name: prometheus-config
data:
  prometheus.yml: |
    global:
      scrape_interval: 15s
    scrape_configs:
    - job_name: 'vantis-player'
      kubernetes_sd_configs:
      - role: pod
      relabel_configs:
      - source_labels: [__meta_kubernetes_pod_label_app]
        action: keep
        regex: vantis-player
```

## Troubleshooting

### Common Issues

**Container won't start:**
```bash
# Check logs
docker logs <container_id>

# Check container status
docker ps -a

# Inspect container
docker inspect <container_id>
```

**Out of memory:**
```bash
# Check resource usage
docker stats

# Increase memory limit
docker run --memory="2g" vantis/player:1.0.0
```

**Network issues:**
```bash
# Check network configuration
docker network inspect vantis-network

# Test connectivity
docker exec <container_id> ping google.com
```

## Best Practices

1. **Use multi-stage builds** to reduce image size
2. **Always tag images with version numbers**
3. **Implement health checks** for all containers
4. **Use non-root users** for security
5. **Scan images for vulnerabilities** before deployment
6. **Implement logging and monitoring** for production
7. **Use secrets management** for sensitive data
8. **Set resource limits** to prevent resource exhaustion
9. **Implement rolling updates** for zero-downtime deployments
10. **Test thoroughly** in staging before production

## Next Steps

- [ ] Set up CI/CD pipeline for automated builds
- [ ] Implement automated security scanning
- [ ] Configure monitoring and alerting
- [ ] Set up backup and disaster recovery
- [ ] Implement canary deployments