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

provider "yandex" {
  cloud_id  = "b1g92d8a7m2lbe44meuq"
  folder_id = "b1gupci5t21aji0ah4f5"
}

resource "yandex_container_registry" "default" {
  name = "default"
}

data "yandex_container_repository" "api" {
  name = "${yandex_container_registry.default.id}/api"
}

resource "yandex_container_repository_lifecycle_policy" "api" {
  name          = "api"
  status        = "active"
  description   = "Lifecycle of API images"
  repository_id = data.yandex_container_repository.api.id

  rule {
    description   = "Simple policy for beta images"
    untagged      = false
    tag_regexp    = "beta.*"
    retained_top  = 3
    expire_period = "168h"
  }
}

resource "yandex_iam_service_account" "container_registry" {
  name        = "container-registry"
  description = "Service account for shipping docker images from CI"
}

resource "yandex_container_registry_iam_binding" "pusher" {
  registry_id = yandex_container_registry.default.id
  role        = "container-registry.images.pusher"
  members = [
    "serviceAccount:${yandex_iam_service_account.container_registry.id}",
  ]
}

resource "yandex_iam_service_account_key" "container_registry_key" {
  service_account_id = yandex_iam_service_account.container_registry.id
  description        = "Key for shipping docker images from CI"
}

resource "github_actions_secret" "registry_key" {
  repository  = local.repository
  secret_name = "YANDEX_CLOUD_REGISTRY_KEY"
  plaintext_value = jsonencode(
    {
      id                 = yandex_iam_service_account_key.container_registry_key.id
      service_account_id = yandex_iam_service_account_key.container_registry_key.service_account_id
      created_at         = yandex_iam_service_account_key.container_registry_key.created_at
      key_algorithm      = yandex_iam_service_account_key.container_registry_key.key_algorithm
      public_key         = yandex_iam_service_account_key.container_registry_key.public_key
      private_key        = yandex_iam_service_account_key.container_registry_key.private_key
    }
  )
}

resource "github_actions_variable" "registry_id" {
  repository    = local.repository
  variable_name = "YANDEX_CLOUD_REGISTRY_ID"
  value         = yandex_container_registry.default.id
}

resource "yandex_iam_service_account" "deploy" {
  name        = "deploy"
  description = "Service account for deployment"
}

resource "yandex_container_registry_iam_binding" "puller" {
  registry_id = yandex_container_registry.default.id
  role        = "container-registry.images.puller"
  members = [
    "serviceAccount:${yandex_iam_service_account.deploy.id}",
  ]
}

output "deploy_service_acount_id" {
  value = yandex_iam_service_account.deploy.id
}
