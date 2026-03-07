# Vantis Media Player - Infrastructure as Code
# Terraform configuration for secure deployment

terraform {
  required_version = ">= 1.5.0"

  required_providers {
    aws = {
      source  = "hashicorp/aws"
      version = "~> 5.0"
    }
    cloudflare = {
      source  = "cloudflare/cloudflare"
      version = "~> 4.0"
    }
  }

  backend "s3" {
    bucket         = "vantis-terraform-state"
    key            = "vantis-media-player/terraform.tfstate"
    region         = "us-east-1"
    encrypt        = true
    dynamodb_table = "vantis-terraform-locks"
  }
}

# Providers
provider "aws" {
  region = var.aws_region

  default_tags {
    tags = {
      Project     = "VantisMediaPlayer"
      ManagedBy   = "Terraform"
      Security    = "High"
      Environment = var.environment
    }
  }
}

provider "cloudflare" {
  api_token = var.cloudflare_api_token
}

# Variables
variable "aws_region" {
  type    = string
  default = "us-east-1"
}

variable "environment" {
  type    = string
  default = "production"
}

variable "cloudflare_api_token" {
  type      = string
  sensitive = true
}

variable "cloudflare_zone_id" {
  type = string
}

# VPC Configuration
module "vpc" {
  source  = "terraform-aws-modules/vpc/aws"
  version = "5.0.0"

  name = "vantis-vpc"
  cidr = "10.0.0.0/16"

  azs             = ["${var.aws_region}a", "${var.aws_region}b", "${var.aws_region}c"]
  private_subnets = ["10.0.1.0/24", "10.0.2.0/24", "10.0.3.0/24"]
  public_subnets  = ["10.0.101.0/24", "10.0.102.0/24", "10.0.103.0/24"]

  enable_nat_gateway     = true
  single_nat_gateway     = false
  one_nat_gateway_per_az = true

  enable_vpn_gateway = false

  enable_dns_hostnames = true
  enable_dns_support   = true

  # Flow logs for security
  enable_flow_log                      = true
  flow_log_destination_type            = "cloud-watch-logs"
  cloudwatch_log_group_retention_in_days = 90

  tags = {
    Security = "High"
  }
}

# Security Groups
resource "aws_security_group" "alb" {
  name        = "vantis-alb-sg"
  description = "Security group for ALB"
  vpc_id      = module.vpc.vpc_id

  ingress {
    from_port   = 443
    to_port     = 443
    protocol    = "tcp"
    cidr_blocks = ["0.0.0.0/0"]
  }

  ingress {
    from_port   = 80
    to_port     = 80
    protocol    = "tcp"
    cidr_blocks = ["0.0.0.0/0"]
  }

  egress {
    from_port   = 0
    to_port     = 0
    protocol    = "-1"
    cidr_blocks = ["10.0.0.0/16"]
  }
}

resource "aws_security_group" "app" {
  name        = "vantis-app-sg"
  description = "Security group for application"
  vpc_id      = module.vpc.vpc_id

  ingress {
    from_port       = 8080
    to_port         = 8080
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

# EKS Cluster
module "eks" {
  source  = "terraform-aws-modules/eks/aws"
  version = "19.15.0"

  cluster_name    = "vantis-cluster"
  cluster_version = "1.28"

  vpc_id     = module.vpc.vpc_id
  subnet_ids = module.vpc.private_subnets

  cluster_endpoint_public_access  = false
  cluster_endpoint_private_access = true

  eks_managed_node_group_defaults = {
    ami_type       = "AL2_x86_64"
    instance_types = ["m5.large"]

    attach_cluster_primary_security_group = true
  }

  eks_managed_node_groups = {
    main = {
      name = "vantis-main"

      instance_types = ["m5.xlarge"]
      capacity_type  = "ON_DEMAND"
      disk_size      = 100

      desired_size = 3
      min_size     = 2
      max_size     = 10

      labels = {
        Environment = var.environment
        Security    = "High"
      }
    }
  }

  # Enable secrets encryption
  cluster_encryption_config = {
    provider_key_arn = aws_kms_key.eks.arn
    resources        = ["secrets"]
  }
}

# KMS Key for encryption
resource "aws_kms_key" "eks" {
  description             = "EKS Secret Encryption Key"
  deletion_window_in_days = 7
  enable_key_rotation     = true

  policy = jsonencode({
    Version = "2012-10-17"
    Statement = [
      {
        Sid    = "Enable IAM User Permissions"
        Effect = "Allow"
        Principal = {
          AWS = "*"
        }
        Action   = "kms:*"
        Resource = "*"
      }
    ]
  })
}

# RDS Database
resource "aws_db_subnet_group" "main" {
  name       = "vantis-db-subnet"
  subnet_ids = module.vpc.private_subnets
}

resource "aws_rds_cluster" "main" {
  engine              = "aurora-postgresql"
  engine_version      = "15.4"
  database_name       = "vantis"
  master_username     = "vantis_admin"
  master_password     = random_password.db.result
  skip_final_snapshot = false

  db_subnet_group_name   = aws_db_subnet_group.main.name
  vpc_security_group_ids = [aws_security_group.db.id]

  storage_encrypted = true
  kms_key_id        = aws_kms_key.rds.arn

  backup_retention_period = 30
  preferred_backup_window = "03:00-05:00"

  enable_http_endpoint = true

  serverlessv2_scaling_configuration {
    min_capacity = 0.5
    max_capacity = 4
  }
}

resource "aws_rds_cluster_instance" "main" {
  count              = 2
  identifier         = "vantis-db-${count.index}"
  cluster_identifier = aws_rds_cluster.main.id
  instance_class     = "db.serverless"
  engine             = aws_rds_cluster.main.engine
  engine_version     = aws_rds_cluster.main.engine_version
}

resource "aws_security_group" "db" {
  name        = "vantis-db-sg"
  description = "Security group for database"
  vpc_id      = module.vpc.vpc_id

  ingress {
    from_port       = 5432
    to_port         = 5432
    protocol        = "tcp"
    security_groups = [aws_security_group.app.id]
  }
}

resource "random_password" "db" {
  length  = 32
  special = true
}

resource "aws_kms_key" "rds" {
  description             = "RDS Encryption Key"
  deletion_window_in_days = 7
  enable_key_rotation     = true
}

# S3 Bucket for media storage
resource "aws_s3_bucket" "media" {
  bucket = "vantis-media-storage-${var.environment}"

  tags = {
    Security = "High"
  }
}

resource "aws_s3_bucket_versioning" "media" {
  bucket = aws_s3_bucket.media.id
  versioning_configuration {
    status = "Enabled"
  }
}

resource "aws_s3_bucket_server_side_encryption_configuration" "media" {
  bucket = aws_s3_bucket.media.id

  rule {
    apply_server_side_encryption_by_default {
      kms_master_key_id = aws_kms_key.s3.arn
      sse_algorithm     = "aws:kms"
    }
  }
}

resource "aws_s3_bucket_public_access_block" "media" {
  bucket = aws_s3_bucket.media.id

  block_public_acls       = true
  block_public_policy     = true
  ignore_public_acls      = true
  restrict_public_buckets = true
}

resource "aws_kms_key" "s3" {
  description             = "S3 Encryption Key"
  deletion_window_in_days = 7
  enable_key_rotation     = true
}

# CloudFront CDN
resource "aws_cloudfront_distribution" "cdn" {
  enabled             = true
  is_ipv6_enabled     = true
  price_class         = "PriceClass_All"
  http_version        = "http2and3"
  default_root_object = "index.html"

  origin {
    domain_name = aws_lb.main.dns_name
    origin_id   = "alb"

    custom_origin_config {
      http_port              = 80
      https_port             = 443
      origin_protocol_policy = "https-only"
      origin_ssl_protocols   = ["TLSv1.2", "TLSv1.3"]
    }
  }

  default_cache_behavior {
    allowed_methods  = ["GET", "HEAD", "OPTIONS"]
    cached_methods   = ["GET", "HEAD"]
    target_origin_id = "alb"

    forwarded_values {
      query_string = true
      headers      = ["Host", "Authorization"]
      cookies {
        forward = "all"
      }
    }

    viewer_protocol_policy = "redirect-to-https"
    min_ttl                = 0
    default_ttl            = 3600
    max_ttl                = 86400
  }

  viewer_certificate {
    acm_certificate_arn      = aws_acm_certificate.main.arn
    ssl_support_method       = "sni-only"
    minimum_protocol_version = "TLSv1.2_2021"
  }

  restrictions {
    geo_restriction {
      restriction_type = "none"
    }
  }
}

# Application Load Balancer
resource "aws_lb" "main" {
  name               = "vantis-alb"
  internal           = false
  load_balancer_type = "application"
  security_groups    = [aws_security_group.alb.id]
  subnets            = module.vpc.public_subnets

  enable_deletion_protection = true
  enable_http2               = true
}

# ACM Certificate
resource "aws_acm_certificate" "main" {
  domain_name       = "vantis.dev"
  validation_method = "DNS"

  subject_alternative_names = [
    "*.vantis.dev",
    "api.vantis.dev",
    "cdn.vantis.dev"
  ]

  lifecycle {
    create_before_destroy = true
  }
}

# Cloudflare DNS
resource "cloudflare_record" "main" {
  zone_id = var.cloudflare_zone_id
  name    = "vantis.dev"
  value   = aws_cloudfront_distribution.cdn.domain_name
  type    = "CNAME"
  ttl     = 300
  proxied = true
}

# WAF Web ACL
resource "aws_wafv2_web_acl" "main" {
  name        = "vantis-waf"
  description = "WAF for Vantis Media Player"
  scope       = "REGIONAL"

  default_action {
    allow {}
  }

  # Rate limiting
  rule {
    name     = "RateLimitRule"
    priority = 1

    override_action {
      none {}
    }

    statement {
      rate_based_statement {
        limit              = 1000
        aggregate_key_type = "IP"
      }
    }

    visibility_config {
      cloudwatch_metrics_enabled = true
      metric_name               = "RateLimitRule"
      sampled_requests_enabled  = true
    }
  }

  # AWS Managed Rules
  rule {
    name     = "AWSManagedRulesCommonRuleSet"
    priority = 2

    override_action {
      none {}
    }

    statement {
      managed_rule_group_statement {
        name        = "AWSManagedRulesCommonRuleSet"
        vendor_name = "AWS"
      }
    }

    visibility_config {
      cloudwatch_metrics_enabled = true
      metric_name               = "AWSManagedRulesCommonRuleSet"
      sampled_requests_enabled  = true
    }
  }

  visibility_config {
    cloudwatch_metrics_enabled = true
    metric_name               = "vantis-waf"
    sampled_requests_enabled  = true
  }
}

# Outputs
output "vpc_id" {
  value = module.vpc.vpc_id
}

output "eks_cluster_endpoint" {
  value = module.eks.cluster_endpoint
}

output "db_endpoint" {
  value = aws_rds_cluster.main.endpoint
}

output "cdn_domain" {
  value = aws_cloudfront_distribution.cdn.domain_name
}