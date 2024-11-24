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
