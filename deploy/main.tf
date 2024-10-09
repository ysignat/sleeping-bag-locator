terraform {
  required_providers {
    yandex = {
      source  = "yandex-cloud/yandex"
      version = "~> 0.129"
    }
  }
  required_version = "~> 1.9.5"

  backend "s3" {
    endpoints = {
      s3 = "https://storage.yandexcloud.net"
    }
    bucket = "sleeping-bag-locator-terraform"
    region = "ru-central1"
    key    = "deploy.tfstate"

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

variable "image" {
  type        = string
  description = "Image tag to be deployed"
  sensitive   = false
  nullable    = false
}

variable "service_account_id" {
  type        = string
  description = "ID of service account used in Serverless Containers"
  sensitive   = false
  nullable    = false
}

resource "yandex_serverless_container" "api" {
  name               = "api"
  memory             = 128
  execution_timeout  = "5s"
  cores              = 1
  core_fraction      = 100
  service_account_id = var.service_account_id
  image {
    url = var.image
    command = [
      "api"
    ]
  }
  provision_policy {
    min_instances = 1
  }
}

output "url" {
  value = yandex_serverless_container.api.url
}
