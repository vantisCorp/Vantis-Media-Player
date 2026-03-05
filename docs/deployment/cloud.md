---
sidebar_position: 5
title: Cloud Deployment
sidebar_label: Cloud
---

# Cloud Deployment

Deploy Vantis Media Player to major cloud providers with enterprise-grade infrastructure, scalability, and reliability.

## Overview

Vantis Media Player supports deployment across all major cloud platforms including AWS, Google Cloud Platform (GCP), and Microsoft Azure. This guide provides comprehensive deployment strategies for each platform.

## AWS Deployment

### Infrastructure with Terraform

Create a complete AWS infrastructure using Terraform:

```hcl
# variables.tf
variable "aws_region" {
  description = "AWS region for deployment"
  type        = string
  default     = "us-east-1"
}

variable "environment" {
  description = "Environment name"
  type        = string
  default     = "production"
}

variable "domain_name" {
  description = "Domain name for the application"
  type        = string
}

variable "certificate_arn" {
  description = "ACM certificate ARN"
  type        = string
}

# main.tf
provider "aws" {
  region = var.aws_region
}

# VPC
resource "aws_vpc" "main" {
  cidr_block           = "10.0.0.0/16"
  enable_dns_hostnames = true
  enable_dns_support   = true

  tags = {
    Name        = "${var.environment}-vpc"
    Environment = var.environment
  }
}

# Public subnets
resource "aws_subnet" "public" {
  count                   = 2
  vpc_id                  = aws_vpc.main.id
  cidr_block              = "10.0.${count.index}.0/24"
  availability_zone       = data.aws_availability_zones.available.names[count.index]
  map_public_ip_on_launch = true

  tags = {
    Name        = "${var.environment}-public-subnet-${count.index}"
    Environment = var.environment
  }
}

# Private subnets
resource "aws_subnet" "private" {
  count             = 2
  vpc_id            = aws_vpc.main.id
  cidr_block        = "10.0.${count.index + 2}.0/24"
  availability_zone = data.aws_availability_zones.available.names[count.index]

  tags = {
    Name        = "${var.environment}-private-subnet-${count.index}"
    Environment = var.environment
  }
}

# Application Load Balancer
resource "aws_lb" "main" {
  name               = "${var.environment}-alb"
  internal           = false
  load_balancer_type = "application"
  security_groups    = [aws_security_group.alb.id]
  subnets            = aws_subnet.public[*].id

  enable_deletion_protection = false

  tags = {
    Environment = var.environment
  }
}

# Security groups
resource "aws_security_group" "alb" {
  name_prefix = "${var.environment}-alb-"
  vpc_id      = aws_vpc.main.id

  ingress {
    from_port   = 443
    to_port     = 443
    protocol    = "tcp"
    cidr_blocks = ["0.0.0.0/0"]
  }

  egress {
    from_port   = 0
    to_port     = 0
    protocol    = "-1"
    cidr_blocks = ["0.0.0.0/0"]
  }
}

resource "aws_security_group" "ecs" {
  name_prefix = "${var.environment}-ecs-"
  vpc_id      = aws_vpc.main.id

  ingress {
    from_port       = 80
    to_port         = 80
    protocol        = "tcp"
    security_groups = [aws_security_group.alb.id]
  }

  egress {
    from_port   = 0
    to_port     = 0
    protocol    = "-1"
    cidr_blocks = ["0.0.0.0/0"]
  }
}

# ECS Cluster
resource "aws_ecs_cluster" "main" {
  name = "${var.environment}-cluster"

  setting {
    name  = "containerInsights"
    value = "enabled"
  }
}

# ECS Task Definition
resource "aws_ecs_task_definition" "vantis_player" {
  family                   = "vantis-player"
  network_mode             = "awsvpc"
  requires_compatibilities = ["FARGATE"]
  cpu                      = "2048"
  memory                   = "4096"

  container_definitions = jsonencode([
    {
      name      = "vantis-player"
      image     = "vantis/player:latest"
      cpu       = 2048
      memory    = 4096
      essential = true
      portMappings = [
        {
          containerPort = 80
          protocol      = "tcp"
        }
      ]
      environment = [
        {
          name  = "NODE_ENV"
          value = "production"
        }
      ]
      healthCheck = {
        command     = ["CMD-SHELL", "wget --quiet --tries=1 --spider http://localhost:80/health || exit 1"]
        interval    = 30
        timeout     = 5
        retries     = 3
        startPeriod = 60
      }
      logConfiguration = {
        logDriver = "awslogs"
        options = {
          "awslogs-group"         = aws_cloudwatch_log_group.vantis.name
          "awslogs-region"        = var.aws_region
          "awslogs-stream-prefix" = "ecs"
        }
      }
    }
  ])
}

# ECS Service
resource "aws_ecs_service" "vantis_player" {
  name            = "${var.environment}-service"
  cluster         = aws_ecs_cluster.main.id
  task_definition = aws_ecs_task_definition.vantis_player.arn
  desired_count   = 3
  launch_type     = "FARGATE"

  network_configuration {
    subnets          = aws_subnet.private[*].id
    security_groups  = [aws_security_group.ecs.id]
    assign_public_ip = false
  }

  load_balancer {
    target_group_arn = aws_lb_target_group.main.arn
    container_name   = "vantis-player"
    container_port   = 80
  }

  deployment_configuration {
    maximum_percent         = 200
    minimum_healthy_percent = 100
  }
}

# Target Group
resource "aws_lb_target_group" "main" {
  name        = "${var.environment}-tg"
  port        = 80
  protocol    = "HTTP"
  vpc_id      = aws_vpc.main.id
  target_type = "ip"

  health_check {
    path                = "/health"
    interval            = 30
    timeout             = 5
    healthy_threshold   = 2
    unhealthy_threshold = 3
  }
}

# Listener
resource "aws_lb_listener" "https" {
  load_balancer_arn = aws_lb.main.arn
  port              = 443
  protocol          = "HTTPS"
  certificate_arn   = var.certificate_arn

  default_action {
    type             = "forward"
    target_group_arn = aws_lb_target_group.main.arn
  }
}

# Auto Scaling
resource "aws_appautoscaling_target" "ecs_target" {
  max_capacity       = 10
  min_capacity       = 3
  resource_id        = "service/${aws_ecs_cluster.main.name}/${aws_ecs_service.vantis_player.name}"
  scalable_dimension = "ecs:service:DesiredCount"
  service_namespace  = "ecs"
}

resource "aws_appautoscaling_policy" "cpu_policy" {
  name               = "${var.environment}-cpu-scaling"
  policy_type        = "TargetTrackingScaling"
  resource_id        = aws_appautoscaling_target.ecs_target.resource_id
  scalable_dimension = aws_appautoscaling_target.ecs_target.scalable_dimension
  service_namespace  = aws_appautoscaling_target.ecs_target.service_namespace

  target_tracking_scaling_policy_configuration {
    predefined_metric_specification {
      predefined_metric_type = "ECSServiceAverageCPUUtilization"
    }
    target_value       = 70.0
    scale_in_cooldown  = 300
    scale_out_cooldown = 60
  }
}

# CloudWatch Log Group
resource "aws_cloudwatch_log_group" "vantis" {
  name              = "/ecs/${var.environment}-vantis-player"
  retention_in_days = 7
}

# S3 Bucket for static assets
resource "aws_s3_bucket" "static_assets" {
  bucket = "${var.environment}-vantis-static-assets"

  tags = {
    Environment = var.environment
  }
}

resource "aws_s3_bucket_public_access_block" "static_assets" {
  bucket = aws_s3_bucket.static_assets.id

  block_public_acls       = false
  block_public_policy     = false
  ignore_public_acls      = false
  restrict_public_buckets = false
}

# CloudFront Distribution
resource "aws_cloudfront_distribution" "main" {
  enabled             = true
  is_ipv6_enabled     = true
  comment             = "${var.environment} Vantis Player Distribution"
  default_root_object = "index.html"

  origin {
    domain_name = aws_s3_bucket.static_assets.bucket_regional_domain_name
    origin_id   = "S3-${aws_s3_bucket.static_assets.id}"

    s3_origin_config {
      origin_access_identity = aws_cloudfront_origin_access_identity.main.cloudfront_access_identity_path
    }
  }

  default_cache_behavior {
    allowed_methods  = ["GET", "HEAD"]
    cached_methods   = ["GET", "HEAD"]
    target_origin_id = "S3-${aws_s3_bucket.static_assets.id}"

    forwarded_values {
      query_string = false
      cookies {
        forward = "none"
      }
    }

    viewer_protocol_policy = "redirect-to-https"
    min_ttl                = 0
    default_ttl            = 86400
    max_ttl                = 31536000
  }

  viewer_certificate {
    acm_certificate_arn      = var.certificate_arn
    ssl_support_method       = "sni-only"
    minimum_protocol_version = "TLSv1.2_2021"
  }

  restrictions {
    geo_restriction {
      restriction_type = "none"
    }
  }

  custom_error_response {
    error_code         = 404
    response_code      = 200
    response_page_path = "/index.html"
  }
}

resource "aws_cloudfront_origin_access_identity" "main" {
  comment = "${var.environment} Vantis Player OAI"
}

# Route53
resource "aws_route53_record" "main" {
  zone_id = data.aws_route53_zone.main.zone_id
  name    = var.domain_name
  type    = "A"

  alias {
    name                   = aws_cloudfront_distribution.main.domain_name
    zone_id                = aws_cloudfront_distribution.main.hosted_zone_id
    evaluate_target_health = true
  }
}

# Data sources
data "aws_availability_zones" "available" {}

data "aws_route53_zone" "main" {
  name = var.domain_name
}
```

### AWS CLI Deployment

Deploy using AWS CLI:

```bash
# Build and push Docker image
aws ecr get-login-password --region us-east-1 | \
  docker login --username AWS --password-stdin 123456789012.dkr.ecr.us-east-1.amazonaws.com

docker build -t vantis/player:latest .
docker tag vantis/player:latest 123456789012.dkr.ecr.us-east-1.amazonaws.com/vantis-player:latest
docker push 123456789012.dkr.ecr.us-east-1.amazonaws.com/vantis-player:latest

# Initialize Terraform
terraform init
terraform plan
terraform apply
```

## Google Cloud Platform Deployment

### Terraform Configuration

```hcl
# variables.tf
variable "project_id" {
  description = "GCP project ID"
  type        = string
}

variable "region" {
  description = "GCP region"
  type        = string
  default     = "us-central1"
}

variable "domain" {
  description = "Domain name"
  type        = string
}

# main.tf
provider "google" {
  project = var.project_id
  region  = var.region
}

# VPC Network
resource "google_compute_network" "main" {
  name                    = "${var.project_id}-vpc"
  auto_create_subnetworks = false
}

# Subnets
resource "google_compute_subnetwork" "private" {
  name          = "${var.project_id}-private-subnet"
  ip_cidr_range = "10.0.0.0/24"
  region        = var.region
  network       = google_compute_network.main.id
  private_ip_google_access = true
}

# Cloud Router for NAT
resource "google_compute_router" "main" {
  name    = "${var.project_id}-router"
  region  = var.region
  network = google_compute_network.main.id
}

resource "google_compute_router_nat" "main" {
  name                               = "${var.project_id}-nat"
  router                             = google_compute_router.main.name
  region                             = var.region
  nat_ip_allocate_option             = "AUTO_ONLY"
  source_subnetwork_ip_ranges_to_nat = ["ALL_SUBNETWORKS_ALL_IP_RANGES"]
}

# Cloud Load Balancing
resource "google_compute_global_address" "main" {
  name = "${var.project_id}-ip"
}

resource "google_compute_managed_ssl_certificate" "main" {
  name = "${var.project_id}-ssl"
  managed {
    domains = [var.domain]
  }
}

resource "google_compute_backend_service" "main" {
  name                  = "${var.project_id}-backend"
  port_name             = "http"
  protocol              = "HTTP"
  timeout_sec           = 30
  load_balancing_scheme = "EXTERNAL_MANAGED"

  health_checks = [google_compute_health_check.main.id]
}

resource "google_compute_health_check" "main" {
  name = "${var.project_id}-health-check"

  http_health_check {
    port         = 80
    request_path = "/health"
  }
}

resource "google_compute_url_map" "main" {
  name            = "${var.project_id}-url-map"
  default_service = google_compute_backend_service.main.id
}

resource "google_compute_target_https_proxy" "main" {
  name             = "${var.project_id}-https-proxy"
  url_map          = google_compute_url_map.main.id
  ssl_certificates = [google_compute_managed_ssl_certificate.main.id]
}

resource "google_compute_global_forwarding_rule" "https" {
  name                  = "${var.project_id}-https-forwarding-rule"
  load_balancing_scheme = "EXTERNAL_MANAGED"
  port_range            = "443"
  target                = google_compute_target_https_proxy.main.id
  ip_address            = google_compute_global_address.main.address
}

# Cloud Run
resource "google_cloud_run_service" "vantis_player" {
  name     = "vantis-player"
  location = var.region

  template {
    spec {
      containers {
        image = "gcr.io/${var.project_id}/vantis-player:latest"

        ports {
          container_port = 80
        }

        resources {
          limits = {
            cpu    = "2"
            memory = "4Gi"
          }
          requests = {
            cpu    = "500m"
            memory = "512Mi"
          }
        }

        env {
          name  = "NODE_ENV"
          value = "production"
        }
      }

      container_concurrency = 100
      timeout_seconds       = 60
    }

    metadata {
      annotations = {
        "autoscaling.knative.dev/maxScale"              = "10"
        "autoscaling.knative.dev/minScale"              = "3"
        "autoscaling.knative.dev/target"                = "70"
        "run.googleapis.com/vpc-access-egress"         = "private-ranges-only"
        "run.googleapis.com/vpc-access-connector"      = google_vpc_access_connector.main.id
      }
    }
  }

  traffic {
    percent         = 100
    latest_revision = true
  }
}

# VPC Access Connector
resource "google_vpc_access_connector" "main" {
  name          = "${var.project_id}-connector"
  region        = var.region
  ip_cidr_range = "10.8.0.0/28"
  network       = google_compute_network.main.name
}

# Cloud Storage
resource "google_storage_bucket" "static_assets" {
  name          = "${var.project_id}-static-assets"
  location      = var.region
  force_destroy = false

  uniform_bucket_level_access = true

  website {
    main_page_suffix = "index.html"
    not_found_page   = "404.html"
  }

  cors {
    origin          = ["*"]
    method          = ["GET"]
    response_header = ["Content-Type"]
    max_age_seconds = 3600
  }
}

# Cloud CDN
resource "google_compute_backend_bucket" "main" {
  name        = "${var.project_id}-backend-bucket"
  bucket_name = google_storage_bucket.static_assets.name
  enable_cdn  = true
}

# DNS
resource "google_dns_record_set" "main" {
  name = var.domain
  type = "A"
  ttl  = 300

  managed_zone = data.google_dns_managed_zone.main.name

  rrdatas = [google_compute_global_address.main.address]
}

data "google_dns_managed_zone" "main" {
  name = "vantis-zone"
}
```

### GCP CLI Deployment

```bash
# Build and push image
gcloud builds submit --tag gcr.io/PROJECT_ID/vantis-player:latest

# Deploy to Cloud Run
gcloud run deploy vantis-player \
  --image gcr.io/PROJECT_ID/vantis-player:latest \
  --platform managed \
  --region us-central1 \
  --allow-unauthenticated \
  --cpu 2 \
  --memory 4Gi \
  --min-instances 3 \
  --max-instances 10

# Create load balancer
gcloud compute url-maps create vantis-map \
  --default-service vantis-backend

gcloud compute target-https-proxies create vantis-https \
  --url-map vantis-map \
  --ssl-certificates vantis-ssl

gcloud compute forwarding-rules create vantis-https \
  --global \
  --target-https-proxy vantis-https \
  --ports 443
```

## Microsoft Azure Deployment

### Bicep Configuration

```bicep
param location string = resourceGroup().location
param appName string = 'vantis-player'
param environment string = 'production'
param domainName string
param skuName string = 'P1v3'

// Resource Group
resource appServicePlan 'Microsoft.Web/serverfarms@2022-03-01' = {
  name: '${appName}-plan'
  location: location
  sku: {
    name: skuName
    tier: 'PremiumV3'
    capacity: 3
  }
  properties: {
    reserved: true // Linux
  }
}

resource webApp 'Microsoft.Web/sites@2022-03-01' = {
  name: appName
  location: location
  properties: {
    serverFarmId: appServicePlan.id
    siteConfig: {
      linuxFxVersion: 'DOCKER|vantis/player:latest'
      alwaysOn: true
      http20Enabled: true
      minTlsVersion: '1.2'
      ftpsState: 'Disabled'
      remoteDebuggingEnabled: false
      httpLoggingEnabled: true
      detailedErrorLoggingEnabled: true
      appCommandLine: ''
      appSettings: [
        {
          name: 'NODE_ENV'
          value: 'production'
        }
        {
          name: 'WEBSITES_PORT'
          value: '80'
        }
        {
          name: 'DOCKER_ENABLE_CI'
          value: 'false'
        }
      ]
      cors: {
        allowedOrigins: ['*']
        supportCredentials: false
      }
    }
    httpsOnly: true
  }
}

// Application Gateway
resource publicIp 'Microsoft.Network/publicIPAddresses@2022-07-01' = {
  name: '${appName}-pip'
  location: location
  sku: {
    name: 'Standard'
  }
  properties: {
    publicIPAllocationMethod: 'Static'
    dnsSettings: {
      domainNameLabel: appName
    }
  }
}

resource vnet 'Microsoft.Network/virtualNetworks@2022-07-01' = {
  name: '${appName}-vnet'
  location: location
  properties: {
    addressSpace: {
      addressPrefixes: [
        '10.0.0.0/16'
      ]
    }
    subnets: [
      {
        name: 'GatewaySubnet'
        properties: {
          addressPrefix: '10.0.1.0/24'
        }
      }
      {
        name: 'AppSubnet'
        properties: {
          addressPrefix: '10.0.2.0/24'
          delegations: [
            {
              name: 'webAppDelegation'
              properties: {
                serviceName: 'Microsoft.Web/sites'
                actions: [
                  'Microsoft.Network/virtualNetworks/subnets/action'
                ]
              }
            }
          ]
        }
      }
    ]
  }
}

resource appGateway 'Microsoft.Network/applicationGateways@2022-07-01' = {
  name: '${appName}-agw'
  location: location
  properties: {
    sku: {
      name: 'WAF_v2'
      tier: 'WAF_v2'
      capacity: 2
    }
    gatewayIPConfigurations: [
      {
        name: 'appGatewayIpConfig'
        properties: {
          subnet: {
            id: resourceId('Microsoft.Network/virtualNetworks/subnets', vnet.name, 'GatewaySubnet')
          }
        }
      }
    ]
    frontendIPConfigurations: [
      {
        name: 'appGatewayFrontendIP'
        properties: {
          publicIPAddress: {
            id: publicIp.id
          }
        }
      }
    ]
    frontendPorts: [
      {
        name: 'appGatewayFrontendPort'
        properties: {
          port: 443
        }
      }
    ]
    backendAddressPools: [
      {
        name: 'appGatewayBackendPool'
        properties: {
          backendAddresses: [
            {
              fqdn: '${appName}.azurewebsites.net'
            }
          ]
        }
      }
    ]
    backendHttpSettingsCollection: [
      {
        name: 'appGatewayBackendHttpSettings'
        properties: {
          port: 443
          protocol: 'Https'
          cookieBasedAffinity: 'Disabled'
          requestTimeout: 20
          probe: {
            id: resourceId('Microsoft.Network/applicationGateways/probes', appGateway.name, 'healthProbe')
          }
        }
      }
    ]
    httpListeners: [
      {
        name: 'appGatewayHttpListener'
        properties: {
          frontendIPConfiguration: {
            id: resourceId('Microsoft.Network/applicationGateways/frontendIPConfigurations', appGateway.name, 'appGatewayFrontendIP')
          }
          frontendPort: {
            id: resourceId('Microsoft.Network/applicationGateways/frontendPorts', appGateway.name, 'appGatewayFrontendPort')
          }
          protocol: 'Https'
          sslCertificate: {
            id: resourceId('Microsoft.Network/applicationGateways/sslCertificates', appGateway.name, 'appGatewaySslCert')
          }
        }
      }
    ]
    requestRoutingRules: [
      {
        name: 'rule1'
        properties: {
          ruleType: 'Basic'
          httpListener: {
            id: resourceId('Microsoft.Network/applicationGateways/httpListeners', appGateway.name, 'appGatewayHttpListener')
          }
          backendAddressPool: {
            id: resourceId('Microsoft.Network/applicationGateways/backendAddressPools', appGateway.name, 'appGatewayBackendPool')
          }
          backendHttpSettings: {
            id: resourceId('Microsoft.Network/applicationGateways/backendHttpSettingsCollection', appGateway.name, 'appGatewayBackendHttpSettings')
          }
        }
      }
    ]
    probes: [
      {
        name: 'healthProbe'
        properties: {
          protocol: 'Https'
          path: '/health'
          interval: 30
          timeout: 30
          unhealthyThreshold: 3
          pickHostNameFromBackendHttpSettings: true
          match: {
            statusCodes: [
              '200'
            ]
          }
        }
      }
    ]
    sslCertificates: [
      {
        name: 'appGatewaySslCert'
        properties: {
          data: loadTextContent('certificate.pfx')
          password: 'certificate-password'
        }
      }
    ]
  }
  dependsOn: [
    vnet
  ]
}

// DNS
resource dnsRecord 'Microsoft.Network/dnsZones/A@2018-05-01' = {
  name: domainName
  zoneName: 'vantis.com'
  type: 'Microsoft.Network/dnsZones/A'
  properties: {
    TTL: 300
    ARecords: [
      {
        ipv4Address: publicIp.properties.ipAddress
      }
    ]
  }
}

// Application Insights
resource appInsights 'Microsoft.Insights/components@2020-02-02' = {
  name: '${appName}-insights'
  location: location
  kind: 'web'
  properties: {
    Application_Type: 'web'
    ApplicationId: appName
  }
}
```

### Azure CLI Deployment

```bash
# Create resource group
az group create \
  --name vantis-rg \
  --location eastus

# Create container registry
az acr create \
  --resource-group vantis-rg \
  --name vantisregistry \
  --sku Standard

# Build and push image
az acr build \
  --registry vantisregistry \
  --image vantis-player:latest .

# Create app service plan
az appservice plan create \
  --name vantis-plan \
  --resource-group vantis-rg \
  --is-linux \
  --sku P1v3

# Create web app
az webapp create \
  --name vantis-player \
  --resource-group vantis-rg \
  --plan vantis-plan \
  --deployment-container-image-name vantisregistry.azurecr.io/vantis-player:latest

# Configure app
az webapp config appsettings set \
  --resource-group vantis-rg \
  --name vantis-player \
  --settings NODE_ENV=production WEBSITES_PORT=80

# Scale up
az webapp update \
  --resource-group vantis-rg \
  --name vantis-player \
  --number-of-workers 3
```

## Monitoring and Observability

### CloudWatch Monitoring (AWS)

```yaml
# cloudwatch-dashboard.json
{
  "widgets": [
    {
      "type": "metric",
      "x": 0,
      "y": 0,
      "width": 12,
      "height": 6,
      "properties": {
        "metrics": [
          ["AWS/ECS", "CPUUtilization", "ServiceName", "vantis-player"],
          [".", "MemoryUtilization", ".", "."]
        ],
        "period": 300,
        "stat": "Average",
        "region": "us-east-1",
        "title": "ECS Service Metrics"
      }
    },
    {
      "type": "log",
      "x": 0,
      "y": 6,
      "width": 24,
      "height": 6,
      "properties": {
        "logGroupName": "/aws/ecs/vantis-player",
        "region": "us-east-1",
        "title": "Application Logs"
      }
    }
  ]
}
```

### Stackdriver Monitoring (GCP)

```yaml
# dashboard.yaml
apiVersion: monitoring.cnrm.cloud.google.com/v1beta1
kind: MonitoringDashboard
metadata:
  name: vantis-player-dashboard
spec:
  displayName: Vantis Player Dashboard
  gridLayout:
    widgets:
    - title: Request Count
      xyChart:
        dataSets:
        - timeSeriesQuery:
            timeSeriesFilter:
              filter: 'metric.type="run.googleapis.com/request_count" resource.label.container_name="vantis-player"'
              aggregation:
                alignmentPeriod: 300s
                perSeriesAligner: ALIGN_RATE
    - title: Error Rate
      scorecard:
        gaugeView:
          lowerBound: 0
          upperBound: 100
        dataSets:
        - timeSeriesQuery:
            timeSeriesFilter:
              filter: 'metric.type="run.googleapis.com/request_count" resource.label.container_name="vantis-player"'
              aggregation:
                alignmentPeriod: 300s
                perSeriesAligner: ALIGN_FRACTION_TRUE
```

## Cost Optimization

### AWS Cost Optimization

```hcl
# Use Spot instances for non-critical workloads
resource "aws_ecs_capacity_provider" "spot" {
  name = "${var.environment}-spot"

  auto_scaling_group_provider {
    auto_scaling_group_arn = aws_autoscaling_group.main.arn

    managed_scaling {
      status                 = "ENABLED"
      target_capacity        = 100
      minimum_scaling_step_size = 1
      maximum_scaling_step_size = 10
    }

    managed_termination_protection = "DISABLED"
  }
}

# Schedule-based scaling
resource "aws_appautoscaling_scheduled_action" "scale_up" {
  name               = "${var.environment}-scale-up"
  service_namespace  = "ecs"
  resource_id        = aws_appautoscaling_target.ecs_target.resource_id
  scalable_dimension = aws_appautoscaling_target.ecs_target.scalable_dimension
  schedule           = "cron(0 8 * * ? *)"

  scalable_target_action {
    min_capacity = 5
    max_capacity = 10
  }
}
```

## Security Best Practices

### Infrastructure Security

- Use VPC with private subnets
- Implement WAF rules
- Enable encryption at rest and in transit
- Use managed identity and secrets management
- Implement network security groups and firewall rules
- Enable security monitoring and alerting

### Application Security

```yaml
# AWS Secrets Manager
resource "aws_secretsmanager_secret" "api_keys" {
  name = "${var.environment}/vantis-player/api-keys"
}

# GCP Secret Manager
resource "google_secret_manager_secret" "api_keys" {
  secret_id = "vantis-player-api-keys"
  replication {
    automatic = true
  }
}

# Azure Key Vault
resource "azurerm_key_vault" "main" {
  name                = "vantis-keyvault"
  location            = var.location
  resource_group_name = var.resource_group_name
  tenant_id           = var.tenant_id
  sku_name            = "standard"

  access_policy {
    tenant_id = var.tenant_id
    object_id = var.object_id

    secret_permissions = [
      "get",
      "list",
      "set",
      "delete"
    ]
  }
}
```

## Disaster Recovery

### Multi-Region Deployment

```hcl
# AWS Multi-region
resource "aws_cloudfront_distribution" "main" {
  enabled = true

  origin {
    domain_name = aws_lb.primary.dns_name
    origin_id   = "primary-region"
  }

  origin {
    domain_name = aws_lb.secondary.dns_name
    origin_id   = "secondary-region"
  }

  default_cache_behavior {
    target_origin_id       = "primary-region"
    viewer_protocol_policy = "redirect-to-https"
  }

  ordered_cache_behavior {
    path_pattern           = "/health"
    target_origin_id       = "secondary-region"
    viewer_protocol_policy = "redirect-to-https"
  }
}
```

## Best Practices

1. **Use Infrastructure as Code** (Terraform, CloudFormation, ARM)
2. **Implement auto-scaling** for cost optimization
3. **Use managed services** to reduce operational overhead
4. **Enable monitoring and logging** for observability
5. **Implement security best practices** at all layers
6. **Test disaster recovery** procedures regularly
7. **Use multi-region deployment** for high availability
8. **Optimize costs** with rightsizing and spot instances
9. **Implement CI/CD pipelines** for automated deployments
10. **Document infrastructure** and runbooks

## Next Steps

- [ ] Set up CI/CD pipeline for automated deployments
- [ ] Implement comprehensive monitoring and alerting
- [ ] Configure automated backup and disaster recovery
- [ ] Set up security scanning and compliance checks
- [ ] Implement canary and blue-green deployments