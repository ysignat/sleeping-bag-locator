resource "github_actions_secret" "yandex_cloud_registry_key" {
  repository  = local.repository
  secret_name = "YANDEX_CLOUD_REGISTRY_KEY"
  plaintext_value = jsonencode(
    {
      id                 = yandex_iam_service_account_key.build.id
      service_account_id = yandex_iam_service_account_key.build.service_account_id
      created_at         = yandex_iam_service_account_key.build.created_at
      key_algorithm      = yandex_iam_service_account_key.build.key_algorithm
      public_key         = yandex_iam_service_account_key.build.public_key
      private_key        = yandex_iam_service_account_key.build.private_key
    }
  )
}

resource "github_actions_variable" "default" {
  for_each = {
    YANDEX_CLOUD_REGISTRY_ID               = yandex_container_registry.default.id
    YANDEX_CLOUD_DEPLOY_SERVICE_ACCOUNT_ID = yandex_iam_service_account.deploy.id
    API_ARTIFACT_NAME                      = "api"
    YANDEX_CLOUD_REGISTRY                  = "cr.yandex"
    GH_REGISTRY                            = "ghcr.io"
    TERRAFORM_VERSION                      = "~> 1.10.0"
    ALPINE_VERSION                         = "3.20"
    LOCKBOX_SECRET_ID                      = "e6qeoqvd88dcpf044n5i"
    LOCKBOX_SECRET_VERSION                 = "e6qat5fto5ltsdtlbts1"
  }
  repository    = local.repository
  variable_name = each.key
  value         = each.value
}

data "github_user" "ysignat" {
  username = "ysignat"
}

resource "github_repository_environment" "default" {
  for_each = {
    infra-review = {}
    api-review   = {}
  }
  environment         = each.key
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
