#![forbid(unsafe_code)]
#![deny(clippy::all, clippy::pedantic, clippy::unwrap_used)]

use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

// --- DOMINIOS CORPORATIVOS VÁLIDOS (TENANTS) ---
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

// Constantes Aritméticas de AAUFSA (IRC Capítulo 56)
pub const AAUFSA_GROSS_RECEIPTS_THRESHOLD_CENTS: u64 = 500_000_000; // $5,000,000.00 USD
pub const AAUFSA_TAX_RATE_BPS: u64 = 1530; // 15.30%
pub const BPS_DENOMINATOR: u64 = 10_000;
pub const DISPLACEMENT_DAYS_THRESHOLD: u32 = 90;
pub const SAFE_HARBOR_MONTHS_REQ: u32 = 36;
pub const SPLIT_SOCIAL_SECURITY_BPS: u64 = 7500; // 75.00%

// Constantes de Red Eléctrica (AuraIntel VPP)
pub const NOMINAL_FREQ_CHZ: u32 = 6000; // 60.00 Hz
pub const MAX_FREQ_DEVIATION_CHZ: u32 = 20; // +/- 0.20 Hz (59.80 - 60.20 Hz)

#[derive(Debug, PartialEq, Eq)]
pub enum EngineError {
    InvalidDomain(u64),
    VppOverCapacity { requested_mw: u64, max_mw: u64 },
    VppCurrentLimitExceeded { current_ma: u64, max_ma: u64 },
    VppGridFrequencyUnstable { freq_chz: u32 },
    ArithmeticOverflow,
    CloudProviderImmunityInvoked,
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
    pub displacement_duration_days: u32,
    pub baseline_payroll_cents: u64,
    pub safe_harbor_reassignment_months: u32,
    pub consolidated_group_revenue_cents: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AaufsaTaxAssessment {
    pub domain_id: u64,
    pub is_taxable: bool,
    pub tax_amount_cents: u64,
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

    pub fn evaluate_der_expansion(
        bus: &mut VppBusState,
        der: &DerNodeAddition,
    ) -> Result<u64, EngineError> {
        Self::validate_domain(der.domain_id)?;

        // Verificación de estabilidad de frecuencia de bus
        let min_freq = NOMINAL_FREQ_CHZ.saturating_sub(MAX_FREQ_DEVIATION_CHZ);
        let max_freq = NOMINAL_FREQ_CHZ
            .checked_add(MAX_FREQ_DEVIATION_CHZ)
            .ok_or(EngineError::ArithmeticOverflow)?;

        if bus.grid_frequency_chz < min_freq || bus.grid_frequency_chz > max_freq {
            return Err(EngineError::VppGridFrequencyUnstable {
                freq_chz: bus.grid_frequency_chz,
            });
        }

        // Verificación de cotas de potencia
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

        // Verificación de cotas de corriente
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

        // Transición de estado elástico atómico
        bus.current_mw = new_total_mw;
        bus.current_ma = new_total_ma;

        Ok(bus.current_mw)
    }

    pub fn evaluate_aaufsa_expansion(
        event: &AutomationExpansionEvent,
    ) -> Result<AaufsaTaxAssessment, EngineError> {
        Self::validate_domain(event.domain_id)?;

        // Regla 1: Sec. 5000H(d) Cloud Computing and SaaS Liability Immunity
        if event.is_cloud_provider_infrastructure {
            return Ok(AaufsaTaxAssessment {
                domain_id: event.domain_id,
                is_taxable: false,
                tax_amount_cents: 0,
                social_security_allocation_cents: 0,
                workforce_reskilling_allocation_cents: 0,
                safe_harbor_applied: false,
                statutory_notes: "Exento bajo Sec. 5000H(d): Inmunidad del Proveedor Cloud/SaaS",
            });
        }

        // Regla 2: Agregación de Ingresos Brutos de Grupo Controlado (Sec. 5000G(b) / IRC Sec. 414 & 1563)
        if event.consolidated_group_revenue_cents <= AAUFSA_GROSS_RECEIPTS_THRESHOLD_CENTS {
            return Ok(AaufsaTaxAssessment {
                domain_id: event.domain_id,
                is_taxable: false,
                tax_amount_cents: 0,
                social_security_allocation_cents: 0,
                workforce_reskilling_allocation_cents: 0,
                safe_harbor_applied: false,
                statutory_notes: "Exento bajo Sec. 5000G(b): Ingresos Consolidados <= $5,000,000.00 USD",
            });
        }

        // Regla 3: Umbral de Desplazamiento Permanente (>90 días bajo Sec. 5000H(c))
        if event.displacement_duration_days <= DISPLACEMENT_DAYS_THRESHOLD {
            return Ok(AaufsaTaxAssessment {
                domain_id: event.domain_id,
                is_taxable: false,
                tax_amount_cents: 0,
                social_security_allocation_cents: 0,
                workforce_reskilling_allocation_cents: 0,
                safe_harbor_applied: false,
                statutory_notes: "No sujeto: Desplazamiento temporal <= 90 dias (Sec. 5000H(c))",
            });
        }

        // Regla 4: Safe Harbor de Reasignación de Empleados (36 meses bajo Sec. 5000G(a))
        if event.safe_harbor_reassignment_months >= SAFE_HARBOR_MONTHS_REQ {
            return Ok(AaufsaTaxAssessment {
                domain_id: event.domain_id,
                is_taxable: false,
                tax_amount_cents: 0,
                social_security_allocation_cents: 0,
                workforce_reskilling_allocation_cents: 0,
                safe_harbor_applied: true,
                statutory_notes: "Exento: Reasignacion efectiva >= 36 meses bajo Sec. 5000G(a)",
            });
        }

        // Determinación del Gravamen Fiscal: 15.30% sobre nómina de reemplazo (Sec. 5000F)
        let numerator = event
            .baseline_payroll_cents
            .checked_mul(AAUFSA_TAX_RATE_BPS)
            .ok_or(EngineError::ArithmeticOverflow)?;
        let tax_total = numerator / BPS_DENOMINATOR;

        // Bifurcación fiduciaria (Sec. 4): 75% Social Security / 25% Reskilling
        let ss_numerator = tax_total
            .checked_mul(SPLIT_SOCIAL_SECURITY_BPS)
            .ok_or(EngineError::ArithmeticOverflow)?;
        let ss_allocation = ss_numerator / BPS_DENOMINATOR;
        let reskilling_allocation = tax_total
            .checked_sub(ss_allocation)
            .ok_or(EngineError::ArithmeticOverflow)?;

        Ok(AaufsaTaxAssessment {
            domain_id: event.domain_id,
            is_taxable: true,
            tax_amount_cents: tax_total,
            social_security_allocation_cents: ss_allocation,
            workforce_reskilling_allocation_cents: reskilling_allocation,
            safe_harbor_applied: false,
            statutory_notes: "Sujeto a gravamen: 15.30% AAUFSA Sec. 5000F liquidable",
        })
    }

    pub fn generate_ledger_entries(
        assessment: &AaufsaTaxAssessment,
        asset_id: &[u8; 16],
    ) -> Result<[LedgerEntry; 2], EngineError> {
        let mut hasher = Sha256::new();
        hasher.update(assessment.domain_id.to_be_bytes());
        hasher.update(asset_id);
        hasher.update(assessment.tax_amount_cents.to_be_bytes());
        let hash_digest = hex::encode(hasher.finalize());

        let entry_ss = LedgerEntry {
            entry_id: format!("{}-SS", &hash_digest[0..12]),
            domain_id: assessment.domain_id,
            account_debit: "2100-TAX-EXPENSE-AAUFSA".to_string(),
            account_credit: "2200-FED-TREASURY-SOCIAL-SECURITY".to_string(),
            amount_cents: assessment.social_security_allocation_cents,
            payload_hash: hash_digest.clone(),
        };

        let entry_reskill = LedgerEntry {
            entry_id: format!("{}-RSK", &hash_digest[0..12]),
            domain_id: assessment.domain_id,
            account_debit: "2100-TAX-EXPENSE-AAUFSA".to_string(),
            account_credit: "2210-FED-WORKFORCE-RESKILLING-FUND".to_string(),
            amount_cents: assessment.workforce_reskilling_allocation_cents,
            payload_hash: hash_digest,
        };

        Ok([entry_ss, entry_reskill])
    }
}

fn main() {
    // 1. Simulación de Expansión Elástica en VPP
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

    match CapacityAndFiscalEngine::evaluate_der_expansion(&mut bus, &der_expansion) {
        Ok(new_mw) => println!("DER Integrado Exitosamente. Capacidad Bus VPP: {} mW", new_mw),
        Err(e) => eprintln!("Fallo critico en expansion DER: {:?}", e),
    }

    // 2. Simulación de Expansión de Capacidad y Validación Fiscal AAUFSA
    let expansion_event = AutomationExpansionEvent {
        domain_id: DOMAIN_AURATETRAX,
        asset_id: [2u8; 16],
        is_cloud_provider_infrastructure: false,
        displacement_duration_days: 120, // > 90 dias
        baseline_payroll_cents: 12_000_000, // $120,000.00 USD
        safe_harbor_reassignment_months: 12, // < 36 meses
        consolidated_group_revenue_cents: 850_000_000, // $8,500,000.00 USD (> $5M)
    };

    match CapacityAndFiscalEngine::evaluate_aaufsa_expansion(&expansion_event) {
        Ok(assessment) => {
            println!("Resultado Fiscal AAUFSA:");
            println!(" - Sujeto a Gravamen: {}", assessment.is_taxable);
            println!(" - Monto Impuesto Total: ${}.{:02} USD", 
                     assessment.tax_amount_cents / 100, assessment.tax_amount_cents % 100);
            println!(" - Fondo Seguro Social (75%): ${}.{:02} USD", 
                     assessment.social_security_allocation_cents / 100, 
                     assessment.social_security_allocation_cents % 100);
            println!(" - Fondo Recalificacion (25%): ${}.{:02} USD", 
                     assessment.workforce_reskilling_allocation_cents / 100, 
                     assessment.workforce_reskilling_allocation_cents % 100);
            println!(" - Base Estatutaria: {}", assessment.statutory_notes);

            if assessment.is_taxable {
                let entries = CapacityAndFiscalEngine::generate_ledger_entries(&assessment, &expansion_event.asset_id)
                    .unwrap_or_else(|_| panic!("Error fatal generando asientos"));
                println!("Asientos Contables Emitidos:");
                for e in &entries {
                    println!("   * ID: {} | Dr: {} | Cr: {} | Monto: ${}.{:02}",
                             e.entry_id, e.account_debit, e.account_credit, 
                             e.amount_cents / 100, e.amount_cents % 100);
                }
            }
        }
        Err(e) => eprintln!("Fallo en evaluacion fiscal AAUFSA: {:?}", e),
    }
}
