terraform {
  required_providers {
    yandex = {
      source  = "yandex-cloud/yandex"
      version = "~> 0.129"
    }
    github = {
      source  = "integrations/github"
      version = "~> 6.3"
    }
  }
  required_version = "~> 1.9.5"

  backend "s3" {
    endpoints = {
      s3 = "https://storage.yandexcloud.net"
    }
    bucket = "sleeping-bag-locator-terraform"
    region = "ru-central1"
    key    = "infra.tfstate"

    skip_region_validation      = true
    skip_credentials_validation = true
    skip_requesting_account_id  = true
    skip_s3_checksum            = true
  }
}

locals {
  repository = "sleeping-bag-locator"
}
