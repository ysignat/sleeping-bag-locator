resource "yandex_iam_service_account" "build" {
  name        = "build"
  description = "Service account for shipping docker images from CI"
}

resource "yandex_resourcemanager_folder_iam_member" "pusher" {
  folder_id = local.folder_id
  role      = "container-registry.images.pusher"
  member    = "serviceAccount:${yandex_iam_service_account.build.id}"
}

resource "yandex_iam_service_account_key" "build" {
  service_account_id = yandex_iam_service_account.build.id
  description        = "Key for shipping docker images from CI"
}
