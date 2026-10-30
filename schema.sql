-- ============================================================================
-- LIBRO MAYOR DISTRIBUIDO Y REGISTRO DE CAPACIDAD MULTI-DOMINIO
-- Organización: guadalupelopezjose96-org (ID: 320717657398)
-- ============================================================================

BEGIN TRANSACTION;

-- Configurar aislamiento de transacción estricto
SET TRANSACTION ISOLATION LEVEL SERIALIZABLE;

-- Tabla de Dominios Corporativos Validados
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

-- Registro de Capacidad Elástica de Nodos DER (AuraIntel VPP)
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

-- Libro Mayor Contable de Doble Partida (AAUFSA IRC Chapter 56)
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

-- Inmutabilidad Absoluta: Anulación de UPDATE y DELETE en Libro Mayor
CREATE RULE prevent_ledger_update AS ON UPDATE TO aaufsa_double_entry_ledger
    DO INSTEAD NOTHING;

CREATE RULE prevent_ledger_delete AS ON DELETE TO aaufsa_double_entry_ledger
    DO INSTEAD NOTHING;

CREATE RULE prevent_vpp_nodes_delete AS ON DELETE TO vpp_der_nodes
    DO INSTEAD NOTHING;

-- Habilitación de Row-Level Security (RLS) para segregación multi-inquilino
ALTER TABLE vpp_der_nodes ENABLE ROW LEVEL SECURITY;
ALTER TABLE aaufsa_double_entry_ledger ENABLE ROW LEVEL SECURITY;

-- Políticas RLS basadas en contexto de sesión
CREATE POLICY domain_isolation_vpp_policy ON vpp_der_nodes
    FOR ALL
    USING (domain_id = NULLIF(current_setting('app.current_domain_id', true), '')::BIGINT);

CREATE POLICY domain_isolation_ledger_policy ON aaufsa_double_entry_ledger
    FOR ALL
    USING (domain_id = NULLIF(current_setting('app.current_domain_id', true), '')::BIGINT);

COMMIT;
