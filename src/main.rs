#![forbid(unsafe_code)]
#![deny(clippy::all, clippy::pedantic, clippy::unwrap_used)]

use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

// ============================================================================
// CLÁUSULA 3 DE INTEGRIDAD ESTATUTARIA: 0% ALTERACIÓN / 100% ATRIBUCIÓN
// ============================================================================
pub const AAUFSA_AUTHOR: &str = "Jose Emanuel Guadalupe Lopez";
pub const AAUFSA_ORG_ID: u64 = 320_717_657_398;
pub const AAUFSA_STATUTE_TITLE: &str =
    "Automated Asset Utilization and Fiscal Solvency Act of 2026 (IRC Chapter 56)";

// Carpetas de Ciclo de Vida de la Organización (guadalupelopezjose96-org)
pub const FOLDER_SHARED_SERVICES: u64 = 938_478_392_755;
pub const FOLDER_PRODUCTION: u64 = 978_426_653_396;
pub const FOLDER_NON_PRODUCTION: u64 = 467_686_749_558;
pub const FOLDER_SANDBOX: u64 = 10_470_744_625;

// 6 Dominios Corporativos Autorizados (Tenants)
pub const DOMAIN_AURATETRAX: u64 = 877_771_073_297;
pub const DOMAIN_FOURPHONIX: u64 = 964_521_086_569;
pub const DOMAIN_AURAINTELCORE: u64 = 188_680_247_367;
pub const DOMAIN_AURAINTEL_GLOBAL: u64 = 1_087_754_464_927;
pub const DOMAIN_STANDARD1979: u64 = 282_230_018_778;
pub const DOMAIN_PROPHONIX: u64 = 358_233_233;

pub const VALID_DOMAINS: [u64; 6] = [
    DOMAIN_AURATETRAX,
    DOMAIN_FOURPHONIX,
    DOMAIN_AURAINTELCORE,
    DOMAIN_AURAINTEL_GLOBAL,
    DOMAIN_STANDARD1979,
    DOMAIN_PROPHONIX,
];

// Constantes Estatutarias Inmutables (AAUFSA IRC Chapter 56)
pub const AAUFSA_GROSS_RECEIPTS_THRESHOLD_CENTS: u64 = 500_000_000; // $5,000,000.00 USD (Sec. 5000G(b))
pub const AAUFSA_TAX_RATE_BPS: u64 = 1530; // 15.30% (Sec. 5000F)
pub const AAUFSA_CLAWBACK_SURCHARGE_BPS: u64 = 200; // 2.00% Recargo de Recaptura (Sec. 5000G(c))
pub const BPS_DENOMINATOR: u64 = 10_000;
pub const DISPLACEMENT_DAYS_THRESHOLD: u32 = 90; // > 90 días (Sec. 5000H(c))
pub const SAFE_HARBOR_MONTHS_REQ: u32 = 36; // >= 36 meses (Sec. 5000G(a))
pub const SPLIT_SOCIAL_SECURITY_BPS: u64 = 7500; // 75.00% (Sec. 4(1))

// Constantes de Red Eléctrica (AuraIntel VPP)
pub const NOMINAL_FREQ_CHZ: u32 = 6000; // 60.00 Hz
pub const MAX_FREQ_DEVIATION_CHZ: u32 = 20; // +/- 0.20 Hz

#[derive(Debug, PartialEq, Eq)]
pub enum EngineError {
    InvalidDomain(u64),
    VppOverCapacity { requested_mw: u64, max_mw: u64 },
    VppCurrentLimitExceeded { current_ma: u64, max_ma: u64 },
    VppGridFrequencyUnstable { freq_chz: u32 },
    ArithmeticOverflow,
    InvalidBasisPoints(u64),
}

// ============================================================================
// TAXONOMÍA OFICIAL DE LAS 14 DIMENSIONES DEL DESPLAZAMIENTO POR IA
// ============================================================================
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum WorkSphere {
    CognitiveIntellectual,
    PhysicalCyberPhysical,
    RelationalServices,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SystemicOrder {
    MicroeconomicIntraFirm,
    MesoeconomicSupplyChain,
    MacroeconomicStructural,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ActionClass {
    ClassISubstitutive,
    ClassIICompressive,
    ClassIIIDegradative,
    ClassIVExclusionary,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum LegalFamily {
    OrdinaryPayrollFte,
    B2bContractors,
    PlatformGigEconomy,
    FutureWorkforceAspirants,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ExitGenus {
    DirectActiveTermination,
    IndirectActiveDowngrade,
    PassiveAttrition,
    PreventiveHiringFreeze,
    CompetitiveThirdPartyClosure,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum StatutoryCategory {
    TaxableSec5000F,
    ExemptSafeHarborOrImmunity,
    NonSubjectSec5000Hc,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DisplacementType {
    DirectReplacement,
    TaskCompression { displaced_share_bps: u64 },
    WageDeskilling { wage_reduction_bps: u64 },
    SkillObsolescence,
    HiringFreezeAttrition,
    FriccionalTransition,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AiVariety {
    AutonomousAgentic,
    MultimodalGenerative,
    RealTimeAlgorithmicDispatch,
    PredictiveAnalytics,
    ConversationalInterface,
    RoboticManipulation,
    DevSecOpsFinOpsOrchestration,
    LegalAccountingScrutiny,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum OccupationalGroup {
    AdministrativeSupport,
    TechnicalStemLegal,
    IndustrialOperative,
    CreativeEditorial,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum OrgHierarchy {
    EntryBaseLevel,
    IntermediateSpecialist,
    MiddleManagement,
    TacticalExecutive,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SocioeconomicStratum {
    StratumISubsistence,
    StratumIIStandardizedMiddle,
    StratumIIIHighSpecialization,
    StratumIVIpSystemOwners,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SeverityLevel {
    Level1AssistanceUnder25Pct,
    Level2Hybridization25To50Pct,
    Level3SevereCompression51To89Pct,
    Level4FullAutonomy90To100Pct,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ReversibilityDegree {
    Degree1ImmediatelyReassignable,
    Degree2Reskillable3To12Months,
    Degree3ChronicSectoral,
    Degree4IrreversibleExit,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct TaxonomyProfile {
    pub sphere: WorkSphere,
    pub order: SystemicOrder,
    pub class: ActionClass,
    pub family: LegalFamily,
    pub genus: ExitGenus,
    pub variety: AiVariety,
    pub group: OccupationalGroup,
    pub hierarchy: OrgHierarchy,
    pub stratum: SocioeconomicStratum,
    pub level: SeverityLevel,
    pub degree: ReversibilityDegree,
}

// ============================================================================
// ESTRUCTURAS DE CONSOLIDACIÓN IRC SEC. 414 / 1563 Y MOTOR FISCAL
// ============================================================================
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DomainGrossReceipts {
    pub domain_id: u64,
    pub annual_gross_receipts_cents: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VppBusState {
    pub bus_id: u32,
    pub current_mw: u64,
    pub max_mw: u64,
    pub current_ma: u64,
    pub max_ma: u64,
    pub grid_frequency_chz: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DerNodeAddition {
    pub node_id: [u8; 16],
    pub domain_id: u64,
    pub injection_capacity_mw: u64,
    pub rated_current_ma: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AutomationExpansionEvent {
    pub domain_id: u64,
    pub asset_id: [u8; 16],
    pub is_cloud_provider_infrastructure: bool,
    pub displacement_type: DisplacementType,
    pub taxonomy: TaxonomyProfile,
    pub displacement_duration_days: u32,
    pub baseline_payroll_cents: u64,
    pub safe_harbor_reassignment_months: u32,
    pub safe_harbor_breached_clawback: bool,
    pub consolidated_group_revenue_cents: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AaufsaTaxAssessment {
    pub domain_id: u64,
    pub statutory_category: StatutoryCategory,
    pub displacement_type: DisplacementType,
    pub is_taxable: bool,
    pub clawback_triggered: bool,
    pub effective_displaced_payroll_cents: u64,
    pub base_tax_cents: u64,
    pub clawback_surcharge_cents: u64,
    pub total_tax_amount_cents: u64,
    pub social_security_allocation_cents: u64,
    pub workforce_reskilling_allocation_cents: u64,
    pub safe_harbor_applied: bool,
    pub statutory_notes: &'static str,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LedgerEntry {
    pub entry_id: String,
    pub domain_id: u64,
    pub account_debit: String,
    pub account_credit: String,
    pub amount_cents: u64,
    pub statutory_reference: String,
    pub payload_hash: String,
}

pub struct CapacityAndFiscalEngine;

impl CapacityAndFiscalEngine {
    pub fn validate_domain(domain_id: u64) -> Result<(), EngineError> {
        if VALID_DOMAINS.contains(&domain_id) {
            Ok(())
        } else {
            Err(EngineError::InvalidDomain(domain_id))
        }
    }

    /// Consolida los ingresos brutos de los 6 dominios bajo IRC Sec. 414 y 1563 (`Sec. 5000G(b)`)
    pub fn aggregate_controlled_group_receipts(
        receipts: &[DomainGrossReceipts],
    ) -> Result<u64, EngineError> {
        let mut total_cents: u64 = 0;
        for item in receipts {
            Self::validate_domain(item.domain_id)?;
            total_cents = total_cents
                .checked_add(item.annual_gross_receipts_cents)
                .ok_or(EngineError::ArithmeticOverflow)?;
        }
        Ok(total_cents)
    }

    pub fn evaluate_der_expansion(
        bus: &mut VppBusState,
        der: &DerNodeAddition,
    ) -> Result<u64, EngineError> {
        Self::validate_domain(der.domain_id)?;

        let min_freq = NOMINAL_FREQ_CHZ.saturating_sub(MAX_FREQ_DEVIATION_CHZ);
        let max_freq = NOMINAL_FREQ_CHZ
            .checked_add(MAX_FREQ_DEVIATION_CHZ)
            .ok_or(EngineError::ArithmeticOverflow)?;

        if bus.grid_frequency_chz < min_freq || bus.grid_frequency_chz > max_freq {
            return Err(EngineError::VppGridFrequencyUnstable {
                freq_chz: bus.grid_frequency_chz,
            });
        }

        let new_total_mw = bus
            .current_mw
            .checked_add(der.injection_capacity_mw)
            .ok_or(EngineError::ArithmeticOverflow)?;
        if new_total_mw > bus.max_mw {
            return Err(EngineError::VppOverCapacity {
                requested_mw: new_total_mw,
                max_mw: bus.max_mw,
            });
        }

        let new_total_ma = bus
            .current_ma
            .checked_add(der.rated_current_ma)
            .ok_or(EngineError::ArithmeticOverflow)?;
        if new_total_ma > bus.max_ma {
            return Err(EngineError::VppCurrentLimitExceeded {
                current_ma: new_total_ma,
                max_ma: bus.max_ma,
            });
        }

        bus.current_mw = new_total_mw;
        bus.current_ma = new_total_ma;

        Ok(bus.current_mw)
    }

    pub fn compute_effective_payroll(
        baseline_cents: u64,
        disp_type: DisplacementType,
    ) -> Result<u64, EngineError> {
        match disp_type {
            DisplacementType::DirectReplacement | DisplacementType::SkillObsolescence => {
                Ok(baseline_cents)
            }
            DisplacementType::TaskCompression { displaced_share_bps } => {
                if displaced_share_bps == 0 || displaced_share_bps > BPS_DENOMINATOR {
                    return Err(EngineError::InvalidBasisPoints(displaced_share_bps));
                }
                let prod = baseline_cents
                    .checked_mul(displaced_share_bps)
                    .ok_or(EngineError::ArithmeticOverflow)?;
                Ok(prod / BPS_DENOMINATOR)
            }
            DisplacementType::WageDeskilling { wage_reduction_bps } => {
                if wage_reduction_bps == 0 || wage_reduction_bps > BPS_DENOMINATOR {
                    return Err(EngineError::InvalidBasisPoints(wage_reduction_bps));
                }
                let prod = baseline_cents
                    .checked_mul(wage_reduction_bps)
                    .ok_or(EngineError::ArithmeticOverflow)?;
                Ok(prod / BPS_DENOMINATOR)
            }
            DisplacementType::HiringFreezeAttrition | DisplacementType::FriccionalTransition => {
                Ok(0)
            }
        }
    }

    pub fn evaluate_aaufsa_expansion(
        event: &AutomationExpansionEvent,
    ) -> Result<AaufsaTaxAssessment, EngineError> {
        Self::validate_domain(event.domain_id)?;

        // 1. Inmunidad Absoluta del Desarrollador / Proveedor Cloud y SaaS (Sec. 5000H(d))
        if event.is_cloud_provider_infrastructure {
            return Ok(AaufsaTaxAssessment {
                domain_id: event.domain_id,
                statutory_category: StatutoryCategory::ExemptSafeHarborOrImmunity,
                displacement_type: event.displacement_type,
                is_taxable: false,
                clawback_triggered: false,
                effective_displaced_payroll_cents: 0,
                base_tax_cents: 0,
                clawback_surcharge_cents: 0,
                total_tax_amount_cents: 0,
                social_security_allocation_cents: 0,
                workforce_reskilling_allocation_cents: 0,
                safe_harbor_applied: false,
                statutory_notes: "Exento bajo Sec. 5000H(d): Inmunidad del Proveedor Cloud/SaaS ($0.00 USD)",
            });
        }

        // 2. Exención por Ingresos Brutos Consolidados <= $5,000,000.00 USD (Sec. 5000G(b))
        if event.consolidated_group_revenue_cents <= AAUFSA_GROSS_RECEIPTS_THRESHOLD_CENTS {
            return Ok(AaufsaTaxAssessment {
                domain_id: event.domain_id,
                statutory_category: StatutoryCategory::ExemptSafeHarborOrImmunity,
                displacement_type: event.displacement_type,
                is_taxable: false,
                clawback_triggered: false,
                effective_displaced_payroll_cents: 0,
                base_tax_cents: 0,
                clawback_surcharge_cents: 0,
                total_tax_amount_cents: 0,
                social_security_allocation_cents: 0,
                workforce_reskilling_allocation_cents: 0,
                safe_harbor_applied: false,
                statutory_notes: "Exento bajo Sec. 5000G(b): Ingresos Consolidados <= $5,000,000.00 USD",
            });
        }

        // 3. Evaluación de la Especie Técnica de Desplazamiento (Sec. 5000H(b))
        let effective_payroll_cents =
            Self::compute_effective_payroll(event.baseline_payroll_cents, event.displacement_type)?;

        if effective_payroll_cents == 0 {
            return Ok(AaufsaTaxAssessment {
                domain_id: event.domain_id,
                statutory_category: StatutoryCategory::NonSubjectSec5000Hc,
                displacement_type: event.displacement_type,
                is_taxable: false,
                clawback_triggered: false,
                effective_displaced_payroll_cents: 0,
                base_tax_cents: 0,
                clawback_surcharge_cents: 0,
                total_tax_amount_cents: 0,
                social_security_allocation_cents: 0,
                workforce_reskilling_allocation_cents: 0,
                safe_harbor_applied: false,
                statutory_notes: "No sujeto: Congelacion de vacantes o transicion sin baja activa (Sec. 5000H(c))",
            });
        }

        // 4. Umbral de Duración Permanente (> 90 días bajo Sec. 5000H(c))
        if event.displacement_duration_days <= DISPLACEMENT_DAYS_THRESHOLD {
            return Ok(AaufsaTaxAssessment {
                domain_id: event.domain_id,
                statutory_category: StatutoryCategory::NonSubjectSec5000Hc,
                displacement_type: event.displacement_type,
                is_taxable: false,
                clawback_triggered: false,
                effective_displaced_payroll_cents: 0,
                base_tax_cents: 0,
                clawback_surcharge_cents: 0,
                total_tax_amount_cents: 0,
                social_security_allocation_cents: 0,
                workforce_reskilling_allocation_cents: 0,
                safe_harbor_applied: false,
                statutory_notes: "No sujeto: Desplazamiento temporal <= 90 dias (Sec. 5000H(c))",
            });
        }

        // 5. Safe Harbor de Reasignación >= 36 meses (Sec. 5000G(a)) sin incumplimiento
        if event.safe_harbor_reassignment_months >= SAFE_HARBOR_MONTHS_REQ
            && !event.safe_harbor_breached_clawback
        {
            return Ok(AaufsaTaxAssessment {
                domain_id: event.domain_id,
                statutory_category: StatutoryCategory::ExemptSafeHarborOrImmunity,
                displacement_type: event.displacement_type,
                is_taxable: false,
                clawback_triggered: false,
                effective_displaced_payroll_cents: effective_payroll_cents,
                base_tax_cents: 0,
                clawback_surcharge_cents: 0,
                total_tax_amount_cents: 0,
                social_security_allocation_cents: 0,
                workforce_reskilling_allocation_cents: 0,
                safe_harbor_applied: true,
                statutory_notes: "Exento: Reasignacion efectiva >= 36 meses bajo Sec. 5000G(a)",
            });
        }

        // 6. Cálculo de Impuesto Base 15.30% (Sec. 5000F) y Recaptura Clawback (Sec. 5000G(c))
        let base_numerator = effective_payroll_cents
            .checked_mul(AAUFSA_TAX_RATE_BPS)
            .ok_or(EngineError::ArithmeticOverflow)?;
        let base_tax_cents = base_numerator / BPS_DENOMINATOR;

        let clawback_surcharge_cents = if event.safe_harbor_breached_clawback {
            let surcharge_num = base_tax_cents
                .checked_mul(AAUFSA_CLAWBACK_SURCHARGE_BPS)
                .ok_or(EngineError::ArithmeticOverflow)?;
            surcharge_num / BPS_DENOMINATOR
        } else {
            0
        };

        let total_tax_cents = base_tax_cents
            .checked_add(clawback_surcharge_cents)
            .ok_or(EngineError::ArithmeticOverflow)?;

        // 7. Bifurcación Fiduciaria: 75% Social Security / 25% Workforce Reskilling (Sec. 4)
        let ss_numerator = total_tax_cents
            .checked_mul(SPLIT_SOCIAL_SECURITY_BPS)
            .ok_or(EngineError::ArithmeticOverflow)?;
        let ss_allocation = ss_numerator / BPS_DENOMINATOR;
        let reskilling_allocation = total_tax_cents
            .checked_sub(ss_allocation)
            .ok_or(EngineError::ArithmeticOverflow)?;

        let notes = if event.safe_harbor_breached_clawback {
            "Gravamen + Recaptura Clawback: 15.30% Sec. 5000F + 2.00% Recargo Sec. 5000G(c)"
        } else {
            "Sujeto a gravamen: 15.30% AAUFSA Sec. 5000F liquidable"
        };

        Ok(AaufsaTaxAssessment {
            domain_id: event.domain_id,
            statutory_category: StatutoryCategory::TaxableSec5000F,
            displacement_type: event.displacement_type,
            is_taxable: true,
            clawback_triggered: event.safe_harbor_breached_clawback,
            effective_displaced_payroll_cents: effective_payroll_cents,
            base_tax_cents,
            clawback_surcharge_cents,
            total_tax_amount_cents: total_tax_cents,
            social_security_allocation_cents: ss_allocation,
            workforce_reskilling_allocation_cents: reskilling_allocation,
            safe_harbor_applied: false,
            statutory_notes: notes,
        })
    }

    pub fn generate_ledger_entries(
        assessment: &AaufsaTaxAssessment,
        asset_id: &[u8; 16],
    ) -> Result<[LedgerEntry; 2], EngineError> {
        let mut hasher = Sha256::new();
        hasher.update(assessment.domain_id.to_be_bytes());
        hasher.update(asset_id);
        hasher.update(assessment.total_tax_amount_cents.to_be_bytes());
        let hash_digest = hex::encode(hasher.finalize());

        let entry_ss = LedgerEntry {
            entry_id: format!("{}-SS", &hash_digest[0..12]),
            domain_id: assessment.domain_id,
            account_debit: "2100-TAX-EXPENSE-AAUFSA".to_string(),
            account_credit: "2200-FED-TREASURY-SOCIAL-SECURITY".to_string(),
            amount_cents: assessment.social_security_allocation_cents,
            statutory_reference: assessment.statutory_notes.to_string(),
            payload_hash: hash_digest.clone(),
        };

        let entry_reskill = LedgerEntry {
            entry_id: format!("{}-RSK", &hash_digest[0..12]),
            domain_id: assessment.domain_id,
            account_debit: "2100-TAX-EXPENSE-AAUFSA".to_string(),
            account_credit: "2210-FED-WORKFORCE-RESKILLING-FUND".to_string(),
            amount_cents: assessment.workforce_reskilling_allocation_cents,
            statutory_reference: assessment.statutory_notes.to_string(),
            payload_hash: hash_digest,
        };

        Ok([entry_ss, entry_reskill])
    }
}

fn default_taxonomy() -> TaxonomyProfile {
    TaxonomyProfile {
        sphere: WorkSphere::PhysicalCyberPhysical,
        order: SystemicOrder::MicroeconomicIntraFirm,
        class: ActionClass::ClassISubstitutive,
        family: LegalFamily::OrdinaryPayrollFte,
        genus: ExitGenus::DirectActiveTermination,
        variety: AiVariety::RealTimeAlgorithmicDispatch,
        group: OccupationalGroup::TechnicalStemLegal,
        hierarchy: OrgHierarchy::IntermediateSpecialist,
        stratum: SocioeconomicStratum::StratumIIIHighSpecialization,
        level: SeverityLevel::Level4FullAutonomy90To100Pct,
        degree: ReversibilityDegree::Degree2Reskillable3To12Months,
    }
}

fn print_scenario(title: &str, event: &AutomationExpansionEvent) {
    println!("\n======================================================================");
    println!("{}", title);
    println!("======================================================================");
    match CapacityAndFiscalEngine::evaluate_aaufsa_expansion(event) {
        Ok(assessment) => {
            println!(" - Dominio ID: {}", assessment.domain_id);
            println!(" - Categoria Estatutaria: {:?}", assessment.statutory_category);
            println!(" - Especie de Desplazamiento: {:?}", assessment.displacement_type);
            println!(
                " - Nomina Efectiva Desplazada: ${}.{:02} USD",
                assessment.effective_displaced_payroll_cents / 100,
                assessment.effective_displaced_payroll_cents % 100
            );
            println!(" - Sujeto a Gravamen: {}", assessment.is_taxable);
            println!(
                " - Monto Impuesto Total: ${}.{:02} USD",
                assessment.total_tax_amount_cents / 100,
                assessment.total_tax_amount_cents % 100
            );
            println!(" - Base Estatutaria: {}", assessment.statutory_notes);

            if assessment.is_taxable {
                if assessment.clawback_triggered {
                    println!(
                        "   * Impuesto Base (15.30%): ${}.{:02} USD | Recargo Clawback (2.00%): ${}.{:02} USD",
                        assessment.base_tax_cents / 100,
                        assessment.base_tax_cents % 100,
                        assessment.clawback_surcharge_cents / 100,
                        assessment.clawback_surcharge_cents % 100
                    );
                }
                println!(
                    "   * Fondo Seguro Social (75%): ${}.{:02} USD",
                    assessment.social_security_allocation_cents / 100,
                    assessment.social_security_allocation_cents % 100
                );
                println!(
                    "   * Fondo Recalificacion (25%): ${}.{:02} USD",
                    assessment.workforce_reskilling_allocation_cents / 100,
                    assessment.workforce_reskilling_allocation_cents % 100
                );
            }
        }
        Err(e) => eprintln!("Fallo en evaluacion fiscal AAUFSA: {:?}", e),
    }
}

fn main() {
    println!("======================================================================");
    println!("ESTATUTO: {}", AAUFSA_STATUTE_TITLE);
    println!("AUTOR EXCLUSIVO (Clausula 3): {} | ORG: {}", AAUFSA_AUTHOR, AAUFSA_ORG_ID);
    println!("======================================================================");

    // 1. Consolidación de Ingresos del Grupo Controlado (IRC Sec. 414 & 1563 / Sec. 5000G(b))
    let group_receipts = [
        DomainGrossReceipts { domain_id: DOMAIN_AURATETRAX, annual_gross_receipts_cents: 250_000_000 },
        DomainGrossReceipts { domain_id: DOMAIN_FOURPHONIX, annual_gross_receipts_cents: 180_000_000 },
        DomainGrossReceipts { domain_id: DOMAIN_AURAINTELCORE, annual_gross_receipts_cents: 150_000_000 },
        DomainGrossReceipts { domain_id: DOMAIN_AURAINTEL_GLOBAL, annual_gross_receipts_cents: 120_000_000 },
        DomainGrossReceipts { domain_id: DOMAIN_STANDARD1979, annual_gross_receipts_cents: 90_000_000 },
        DomainGrossReceipts { domain_id: DOMAIN_PROPHONIX, annual_gross_receipts_cents: 60_000_000 },
    ];

    let consolidated_revenue = CapacityAndFiscalEngine::aggregate_controlled_group_receipts(&group_receipts)
        .unwrap_or(0);
    println!(
        "Ingresos Consolidados Grupo Controlado (6 Dominios): ${}.{:02} USD",
        consolidated_revenue / 100,
        consolidated_revenue % 100
    );

    // 2. Expansión Elástica en VPP
    let mut bus = VppBusState {
        bus_id: 101,
        current_mw: 45_000,
        max_mw: 100_000,
        current_ma: 200_000,
        max_ma: 500_000,
        grid_frequency_chz: 6005,
    };

    let der_expansion = DerNodeAddition {
        node_id: [1u8; 16],
        domain_id: DOMAIN_AURAINTEL_GLOBAL,
        injection_capacity_mw: 15_000,
        rated_current_ma: 65_000,
    };

    if let Ok(new_mw) = CapacityAndFiscalEngine::evaluate_der_expansion(&mut bus, &der_expansion) {
        println!("DER Integrado Exitosamente en AuraIntel VPP. Capacidad Bus: {} mW", new_mw);
    }

    // CASO 1: Inmunidad del Desarrollador Cloud/SaaS ($0.00 USD)
    let case_saas = AutomationExpansionEvent {
        domain_id: DOMAIN_AURAINTELCORE,
        asset_id: [1u8; 16],
        is_cloud_provider_infrastructure: true,
        displacement_type: DisplacementType::DirectReplacement,
        taxonomy: default_taxonomy(),
        displacement_duration_days: 180,
        baseline_payroll_cents: 12_000_000,
        safe_harbor_reassignment_months: 0,
        safe_harbor_breached_clawback: false,
        consolidated_group_revenue_cents: consolidated_revenue,
    };
    print_scenario("CASO 1: Elaboracion y Expansion de SaaS (EXENTO Sec. 5000H(d) - $0.00 USD)", &case_saas);

    // CASO 2: Safe Harbor de Reasignación por 36 Meses ($0.00 USD)
    let case_safe_harbor = AutomationExpansionEvent {
        domain_id: DOMAIN_FOURPHONIX,
        asset_id: [2u8; 16],
        is_cloud_provider_infrastructure: false,
        displacement_type: DisplacementType::DirectReplacement,
        taxonomy: default_taxonomy(),
        displacement_duration_days: 120,
        baseline_payroll_cents: 12_000_000,
        safe_harbor_reassignment_months: 36,
        safe_harbor_breached_clawback: false,
        consolidated_group_revenue_cents: consolidated_revenue,
    };
    print_scenario("CASO 2: Reasignacion Laboral >= 36 Meses (EXENTO Sec. 5000G(a) - $0.00 USD)", &case_safe_harbor);

    // CASO 3: Sustitución Directa sin Safe Harbor ($18,360.00 USD)
    let case_direct = AutomationExpansionEvent {
        domain_id: DOMAIN_AURATETRAX,
        asset_id: [3u8; 16],
        is_cloud_provider_infrastructure: false,
        displacement_type: DisplacementType::DirectReplacement,
        taxonomy: default_taxonomy(),
        displacement_duration_days: 120,
        baseline_payroll_cents: 12_000_000,
        safe_harbor_reassignment_months: 0,
        safe_harbor_breached_clawback: false,
        consolidated_group_revenue_cents: consolidated_revenue,
    };
    print_scenario("CASO 3: Sustitucion Directa sin Safe Harbor (GRAVABLE Sec. 5000F)", &case_direct);

    // CASO 4: Recaptura Clawback por Incumplimiento antes de 36 Meses ($18,360 + $367.20 = $18,727.20 USD)
    let case_clawback = AutomationExpansionEvent {
        domain_id: DOMAIN_STANDARD1979,
        asset_id: [4u8; 16],
        is_cloud_provider_infrastructure: false,
        displacement_type: DisplacementType::DirectReplacement,
        taxonomy: default_taxonomy(),
        displacement_duration_days: 200,
        baseline_payroll_cents: 12_000_000,
        safe_harbor_reassignment_months: 18,
        safe_harbor_breached_clawback: true,
        consolidated_group_revenue_cents: consolidated_revenue,
    };
    print_scenario("CASO 4: Recaptura Clawback por Ruptura de Safe Harbor (Sec. 5000G(c))", &case_clawback);
}
