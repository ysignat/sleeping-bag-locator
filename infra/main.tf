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
  required_version = "~> 1.10.0"

  backend "s3" {
    endpoints = {
      s3       = "https://storage.yandexcloud.net"
      dynamodb = "https://docapi.serverless.yandexcloud.net/ru-central1/b1g92d8a7m2lbe44meuq/etn9ja887cgvon0b54gf"
    }
    bucket         = "sleeping-bag-locator-terraform"
    region         = "ru-central1"
    key            = "infra.tfstate"
    dynamodb_table = "state-lock-table"

    skip_region_validation      = true
    skip_credentials_validation = true
    skip_requesting_account_id  = true
    skip_s3_checksum            = true
  }
}

locals {
  repository = "sleeping-bag-locator"
  folder_id  = "b1gupci5t21aji0ah4f5"
}

provider "yandex" {
  cloud_id  = "b1g92d8a7m2lbe44meuq"
  folder_id = local.folder_id
}
