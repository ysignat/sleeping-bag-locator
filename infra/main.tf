terraform {
  required_providers {
    yandex = {
      source  = "yandex-cloud/yandex"
      version = "~> 0.129"
    }
  }
  required_version = "1.9.5"

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

provider "yandex" {
  cloud_id  = "b1g92d8a7m2lbe44meuq"
  folder_id = "b1gupci5t21aji0ah4f5"
}

resource "yandex_container_registry" "default" {
  name = "default"
}
