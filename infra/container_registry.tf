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
