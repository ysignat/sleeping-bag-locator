resource "yandex_serverless_container" "api" {
  name               = "api"
  memory             = 128
  execution_timeout  = "5s"
  cores              = 1
  core_fraction      = 100
  service_account_id = var.service_account_id
  secrets {
    id                   = var.lockbox_secret_id
    version_id           = var.lockbox_secret_version
    key                  = "github_oauth_client_id"
    environment_variable = "OAUTH_CLIENT_ID"
  }
  secrets {
    id                   = var.lockbox_secret_id
    version_id           = var.lockbox_secret_version
    key                  = "github_oauth_client_secret"
    environment_variable = "OAUTH_CLIENT_SECRET"
  }
  image {
    url = var.image
    environment = {
      HOST               = "0.0.0.0"
      LOG_LEVEL          = "DEBUG"
      LOG_FORMAT         = "json"
      SESSION_STORE_TYPE = "memory"
    }
  }
  provision_policy {
    min_instances = 1
  }
}

output "url" {
  value = yandex_serverless_container.api.url
}
