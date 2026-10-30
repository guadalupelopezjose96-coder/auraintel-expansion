BEGIN TRANSACTION;
SET TRANSACTION ISOLATION LEVEL SERIALIZABLE;

CREATE TABLE IF NOT EXISTS corporate_domains (
    domain_id BIGINT PRIMARY KEY,
    domain_fqdn VARCHAR(255) NOT NULL UNIQUE,
    business_unit_name VARCHAR(100) NOT NULL,
    is_active BOOLEAN NOT NULL DEFAULT TRUE,
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
);

INSERT INTO corporate_domains (domain_id, domain_fqdn, business_unit_name)
VALUES 
    (877771073297, 'AuraTetrax.com', 'AuraTetrax Business Unit'),
    (964521086569, 'Fourphonix.com', 'Fourphonix Global Services'),
    (188680247367, 'AuraIntelCore.com', 'AuraIntel Core Systems'),
    (1087754464927, 'AuraIntel-Global.com', 'AuraIntel Global Expansion'),
    (282230018778, 'Standard1979.com', 'Standard 1979 Operations'),
    (358233233, 'Prophonix.com', 'Prophonix Infrastructure')
ON CONFLICT (domain_id) DO NOTHING;

CREATE TABLE IF NOT EXISTS vpp_der_nodes (
    node_uuid UUID PRIMARY KEY,
    domain_id BIGINT NOT NULL REFERENCES corporate_domains(domain_id),
    bus_id INTEGER NOT NULL,
    capacity_mw BIGINT NOT NULL CHECK (capacity_mw > 0),
    rated_current_ma BIGINT NOT NULL CHECK (rated_current_ma > 0),
    operating_freq_chz INTEGER NOT NULL CHECK (operating_freq_chz BETWEEN 5950 AND 6050),
    expansion_timestamp TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    cryptographic_digest CHAR(64) NOT NULL
);

CREATE TABLE IF NOT EXISTS aaufsa_double_entry_ledger (
    entry_id VARCHAR(64) PRIMARY KEY,
    domain_id BIGINT NOT NULL REFERENCES corporate_domains(domain_id),
    account_debit VARCHAR(64) NOT NULL,
    account_credit VARCHAR(64) NOT NULL,
    amount_cents BIGINT NOT NULL CHECK (amount_cents > 0),
    statutory_reference VARCHAR(128) NOT NULL,
    payload_hash CHAR(64) NOT NULL,
    recorded_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE RULE prevent_ledger_update AS ON UPDATE TO aaufsa_double_entry_ledger DO INSTEAD NOTHING;
CREATE RULE prevent_ledger_delete AS ON DELETE TO aaufsa_double_entry_ledger DO INSTEAD NOTHING;
CREATE RULE prevent_vpp_nodes_delete AS ON DELETE TO vpp_der_nodes DO INSTEAD NOTHING;

ALTER TABLE vpp_der_nodes ENABLE ROW LEVEL SECURITY;
ALTER TABLE aaufsa_double_entry_ledger ENABLE ROW LEVEL SECURITY;

CREATE POLICY domain_isolation_vpp_policy ON vpp_der_nodes
    FOR ALL USING (domain_id = NULLIF(current_setting('app.current_domain_id', true), '')::BIGINT);

CREATE POLICY domain_isolation_ledger_policy ON aaufsa_double_entry_ledger
    FOR ALL USING (domain_id = NULLIF(current_setting('app.current_domain_id', true), '')::BIGINT);

COMMIT;

-- ============================================================================
-- TABLA OPERATIVA DE TELEMETRIA Y DESPACHO EN TIEMPO REAL AURAINTEL VPP
-- ============================================================================
CREATE TABLE IF NOT EXISTS aaufsa_vpp_operational_ledger (
    cycle_id BIGINT PRIMARY KEY,
    org_id BIGINT NOT NULL CHECK (org_id = 320717657398),
    domain_id BIGINT NOT NULL CHECK (domain_id IN (
        877771073297, 964521086569, 188680247367,
        1087754464927, 282230018778, 358233233
    )),
    bus_mw BIGINT NOT NULL CHECK (bus_mw >= 0),
    grid_freq_chz INTEGER NOT NULL CHECK (grid_freq_chz BETWEEN 5950 AND 6050),
    dispatch_cmd TEXT NOT NULL,
    displaced_payroll_cents BIGINT NOT NULL CHECK (displaced_payroll_cents >= 0),
    total_tax_cents BIGINT NOT NULL CHECK (total_tax_cents >= 0),
    oasi_trust_cents BIGINT NOT NULL CHECK (oasi_trust_cents >= 0),
    reskilling_trust_cents BIGINT NOT NULL CHECK (reskilling_trust_cents >= 0),
    prev_hash CHAR(64) NOT NULL,
    record_hash CHAR(64) NOT NULL UNIQUE,
    recorded_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
);

ALTER TABLE aaufsa_vpp_operational_ledger ENABLE ROW LEVEL SECURITY;
