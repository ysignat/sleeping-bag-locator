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

variable "lockbox_secret_id" {
  type        = string
  description = "Lockbox secret id, containing application secrets"
  sensitive   = false
  nullable    = false
}

variable "lockbox_secret_version" {
  type        = string
  description = "Lockbox secret version, containing application secrets"
  sensitive   = false
  nullable    = false
}
