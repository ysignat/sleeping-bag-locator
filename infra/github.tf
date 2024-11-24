resource "github_actions_secret" "yandex_cloud_registry_key" {
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

resource "github_actions_variable" "yandex_cloud_registry_id" {
  repository    = local.repository
  variable_name = "YANDEX_CLOUD_REGISTRY_ID"
  value         = yandex_container_registry.default.id
}

resource "github_actions_variable" "yandex_cloud_deploy_service_acount_id" {
  repository    = local.repository
  variable_name = "YANDEX_CLOUD_DEPLOY_SERVICE_ACCOUNT_ID"
  value         = yandex_iam_service_account.deploy.id
}

resource "github_actions_variable" "api_artifact_name" {
  repository    = local.repository
  variable_name = "API_ARTIFACT_NAME"
  value         = "api"
}

resource "github_actions_variable" "yandex_cloud_registry" {
  repository    = local.repository
  variable_name = "YANDEX_CLOUD_REGISTRY"
  value         = "cr.yandex"
}

resource "github_actions_variable" "gh_registry" {
  repository    = local.repository
  variable_name = "GH_REGISTRY"
  value         = "ghcr.io"
}

resource "github_actions_variable" "terraform_version" {
  repository    = local.repository
  variable_name = "TERRAFORM_VERSION"
  value         = "~> 1.9.5"
}

data "github_user" "ysignat" {
  username = "ysignat"
}

resource "github_repository_environment" "infra_review" {
  environment         = "infra-review"
  repository          = local.repository
  prevent_self_review = false
  can_admins_bypass   = true
  reviewers {
    users = [
      data.github_user.ysignat.id,
    ]
  }
  deployment_branch_policy {
    protected_branches     = false
    custom_branch_policies = true
  }
}

resource "github_repository_environment" "api_review" {
  environment         = "api-review"
  repository          = local.repository
  prevent_self_review = false
  can_admins_bypass   = true
  reviewers {
    users = [
      data.github_user.ysignat.id,
    ]
  }
  deployment_branch_policy {
    protected_branches     = false
    custom_branch_policies = true
  }
}
