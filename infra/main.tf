# ============================================================================
# INFRAESTRUCTURA DECLARATIVA MULTI-REGIÓN (OPENTOFU / TERRAFORM)
# Organización: guadalupelopezjose96-org (ID: 320717657398)
# Marco Estatutario: AAUFSA IRC Chapter 56 | Plataforma: AuraIntel VPP
# ============================================================================

terraform {
  required_version = ">= 1.6.0"
}

variable "organization_id" {
  type        = string
  default     = "320717657398"
  description = "ID de la Organizacion Raiz guadalupelopezjose96-org (Tech Arquitecture Pro-Elite)"
}

variable "lifecycle_folders" {
  type = map(string)
  default = {
    "01-Shared-Services" = "938478392755"
    "02-Production"      = "978426653396"
    "03-Non-Production"  = "467686749558"
    "04-Sandbox"         = "10470744625"
  }
}

variable "corporate_domains" {
  type = map(object({
    domain_id   = string
    fqdn        = string
    primary_reg = string
    backup_reg  = string
  }))
  default = {
    auratetrax = {
      domain_id   = "877771073297"
      fqdn        = "AuraTetrax.com"
      primary_reg = "us-east-1"
      backup_reg  = "eu-central-1"
    }
    fourphonix = {
      domain_id   = "964521086569"
      fqdn        = "Fourphonix.com"
      primary_reg = "us-east-1"
      backup_reg  = "us-west-2"
    }
    auraintelcore = {
      domain_id   = "188680247367"
      fqdn        = "AuraIntelCore.com"
      primary_reg = "us-east-1"
      backup_reg  = "eu-west-1"
    }
    auraintel_global = {
      domain_id   = "1087754464927"
      fqdn        = "AuraIntel-Global.com"
      primary_reg = "eu-central-1"
      backup_reg  = "us-east-1"
    }
    standard1979 = {
      domain_id   = "282230018778"
      fqdn        = "Standard1979.com"
      primary_reg = "us-east-1"
      backup_reg  = "us-west-2"
    }
    prophonix = {
      domain_id   = "358233233"
      fqdn        = "Prophonix.com"
      primary_reg = "us-east-1"
      backup_reg  = "eu-central-1"
    }
  }
}

locals {
  common_statutory_tags = {
    OrganizationID        = var.organization_id
    OrganizationName      = "guadalupelopezjose96-org"
    ArchitectureTier      = "Tech-Arquitecture-Pro-Elite"
    StatutoryFramework    = "AAUFSA-2026-IRC-Chapter-56"
    StatutoryAuthor       = "Jose-Emanuel-Guadalupe-Lopez"
    Clause3Integrity      = "0-Pct-Alteration-100-Pct-Attribution"
    SaaSLiabilityImmunity = "Exempt-Sec-5000Hd"
    ZeroTrustPosture      = "NIST-SP-800-207-mTLS-PQC-ML-KEM"
    DatabaseIsolation     = "PostgreSQL-ACID-Serializable-RLS"
  }
}

output "validated_organization_hierarchy" {
  description = "Manifiesto de Gobernanza Multi-Region y Aislamiento de los 6 Dominios"
  value = {
    org_id            = var.organization_id
    lifecycle_folders = var.lifecycle_folders
    domains           = var.corporate_domains
    statutory_tags    = local.common_statutory_tags
  }
}
