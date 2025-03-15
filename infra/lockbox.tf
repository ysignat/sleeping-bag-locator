resource "yandex_kms_symmetric_key" "default" {
  name                = "default"
  default_algorithm   = "AES_128"
  rotation_period     = "8760h"
  deletion_protection = true
}
