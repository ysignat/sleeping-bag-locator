resource "yandex_iam_service_account" "deploy" {
  name        = "deploy"
  description = "Service account for deployment"
}

resource "yandex_resourcemanager_folder_iam_member" "puller" {
  folder_id = local.folder_id
  role      = "container-registry.images.puller"
  member    = "serviceAccount:${yandex_iam_service_account.deploy.id}"
}

resource "yandex_resourcemanager_folder_iam_member" "payload_viewer" {
  folder_id = local.folder_id
  role      = "lockbox.payloadViewer"
  member    = "serviceAccount:${yandex_iam_service_account.deploy.id}"
}

resource "yandex_resourcemanager_folder_iam_member" "viewer" {
  folder_id = local.folder_id
  role      = "lockbox.viewer"
  member    = "serviceAccount:${yandex_iam_service_account.deploy.id}"
}

resource "yandex_resourcemanager_folder_iam_member" "encrypterDecrypter" {
  folder_id = local.folder_id
  role      = "kms.keys.encrypterDecrypter"
  member    = "serviceAccount:${yandex_iam_service_account.deploy.id}"
}
