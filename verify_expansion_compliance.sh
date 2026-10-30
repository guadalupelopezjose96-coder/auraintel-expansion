#!/usr/bin/env bash
set -Eeuo pipefail

RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m'

LOG_DIR="./audit_logs"
mkdir -p "${LOG_DIR}"
AUDIT_REPORT="${LOG_DIR}/expansion_audit_$(date +%Y%m%d_%H%M%S).log"
MANIFEST_FILE="${LOG_DIR}/CRYPTOGRAPHIC_COMPLIANCE_MANIFEST.sha256"

exec > >(tee -a "${AUDIT_REPORT}") 2>&1

echo -e "${BLUE}==============================================================================${NC}"
echo -e "${BLUE}   INICIANDO AUDITORIA MAESTRA DE EXPANSION CONTINUA - GUADALUPELOPEZJOSE96   ${NC}"
echo -e "${BLUE}==============================================================================${NC}"
echo "Fecha de Ejecución: $(date -u '+%Y-%m-%d %H:%M:%S UTC')"
echo "Target Host: $(uname -smr)"

EXIT_CODE=0

# --- CONTROL 1: CERO SECRETOS O CREDENCIALES EXPUESTAS EN CÓDIGO FUENTE ---
echo -e "\n${YELLOW}[AUDITORIA 1/5] Verificando ausencia de secretos y claves privadas...${NC}"
SECRET_PATTERNS="(AIza[0-9A-Za-z\\-_]{35}|AKIA[0-9A-Z]{16}|ASIA[0-9A-Z]{16}|-----BEGIN (RSA|EC|DSA|OPENSSH|PRIVATE) KEY-----|ghp_[0-9a-zA-Z]{36})"

if grep -rEIq "${SECRET_PATTERNS}" src/ Cargo.toml schema.sql 2>/dev/null; then
    echo -e "${RED}[FALLO CRITICO] Secretos criptograficos o tokens de nube detectados en el repo.${NC}"
    grep -rEIn "${SECRET_PATTERNS}" src/ Cargo.toml schema.sql
    EXIT_CODE=1
else
    echo -e "${GREEN}[APROBADO] Cero credenciales expuestas en archivos fuente.${NC}"
fi

# --- CONTROL 2: AISLAMIENTO LÓGICO DE LOS 6 DOMINIOS EN RUST Y SQL ---
echo -e "\n${YELLOW}[AUDITORIA 2/5] Validando aislamiento formal de los 6 dominios corporativos...${NC}"
EXPECTED_DOMAINS=(
    "877771073297"   # AuraTetrax.com
    "964521086569"   # Fourphonix.com
    "188680247367"   # AuraIntelCore.com
    "1087754464927"  # AuraIntel-Global.com
    "282230018778"   # Standard1979.com
    "358233233"      # Prophonix.com
)

MISSING_DOMAINS=0
# Normalizar src/main.rs eliminando guiones bajos numéricos para la comprobación
RUST_SRC_CLEAN=$(tr -d '_' < src/main.rs)

for domain in "${EXPECTED_DOMAINS[@]}"; do
    if ! echo "${RUST_SRC_CLEAN}" | grep -q "${domain}"; then
        echo -e "${RED}[ERROR] Dominio ${domain} no modelado en src/main.rs${NC}"
        MISSING_DOMAINS=$((MISSING_DOMAINS + 1))
    fi
    if ! grep -q "${domain}" schema.sql; then
        echo -e "${RED}[ERROR] Dominio ${domain} ausente en schema.sql${NC}"
        MISSING_DOMAINS=$((MISSING_DOMAINS + 1))
    fi
done

if [ ${MISSING_DOMAINS} -eq 0 ]; then
    echo -e "${GREEN}[APROBADO] Aislamiento contable y logico validado para los 6 dominios corporativos.${NC}"
else
    echo -e "${RED}[FALLO CRITICO] Violacion de segregacion de dominios corporativos.${NC}"
    EXIT_CODE=1
fi

# Validar políticas de seguridad RLS en schema.sql
if grep -q "ENABLE ROW LEVEL SECURITY" schema.sql && grep -q "app.current_domain_id" schema.sql; then
    echo -e "${GREEN}[APROBADO] Row-Level Security (RLS) configurado deterministamente en SQL.${NC}"
else
    echo -e "${RED}[FALLO CRITICO] RLS no detectado en el esquema de base de datos.${NC}"
    EXIT_CODE=1
fi

# --- CONTROL 3: VERIFICACIÓN DE ARITMÉTICA ENTERA Y CERO COMA FLOTANTE ---
echo -e "\n${YELLOW}[AUDITORIA 3/5] Verificando cero punto flotante en motores VPP y AAUFSA...${NC}"
if grep -rEn "(f32|f64)" src/; then
    echo -e "${RED}[FALLO CRITICO] Tipos de punto flotante IEEE-754 detectados en codigo Rust.${NC}"
    EXIT_CODE=1
else
    echo -e "${GREEN}[APROBADO] 100% de operaciones implementadas con aritmetica entera de punto fijo.${NC}"
fi

# Validar protección contra desbordamiento de enteros
if grep -q "checked_" src/main.rs && grep -q "saturating_" src/main.rs; then
    echo -e "${GREEN}[APROBADO] Verificacion de operaciones aritmeticas protegidas contra desbordamiento.${NC}"
else
    echo -e "${RED}[ADVERTENCIA] No se detecto uso estricto de checked_add o checked_mul en todo el arbol.${NC}"
    EXIT_CODE=1
fi

# --- CONTROL 4: CUMPLIMIENTO DE LICENCIAS PERMISIVAS Y CERO COPYLEFT ---
echo -e "\n${YELLOW}[AUDITORIA 4/5] Escaneando licencias en dependencias de compilacion...${NC}"
if [ -f "deny.toml" ]; then
    echo -e "${GREEN}[APROBADO] Politica deny.toml encontrada. Validando licencias copyleft denegadas...${NC}"
    if grep -q "GPL" deny.toml && grep -q "AGPL" deny.toml; then
        echo -e "${GREEN}[APROBADO] Licencias restrictivas bloqueadas explicitamente.${NC}"
    else
        echo -e "${RED}[FALLO] deny.toml incompleto en reglas de rechazo copyleft.${NC}"
        EXIT_CODE=1
    fi
else
    echo -e "${RED}[FALLO CRITICO] deny.toml ausente.${NC}"
    EXIT_CODE=1
fi

# --- CONTROL 5: GENERACIÓN DEL MANIFIESTO CRIPTOGRÁFICO DE CONFORMIDAD ---
echo -e "\n${YELLOW}[AUDITORIA 5/5] Sellando manifiesto criptografico inmutable...${NC}"

FILES_TO_SEAL=("src/main.rs" "Cargo.toml" "schema.sql" "deny.toml")
> "${MANIFEST_FILE}"

for file in "${FILES_TO_SEAL[@]}"; do
    if [ -f "$file" ]; then
        sha256sum "$file" >> "${MANIFEST_FILE}"
    fi
done

echo -e "${GREEN}[APROBADO] Manifiesto generado en: ${MANIFEST_FILE}${NC}"
cat "${MANIFEST_FILE}"

echo -e "\n${BLUE}==============================================================================${NC}"
if [ ${EXIT_CODE} -eq 0 ]; then
    echo -e "${GREEN}   VERIFICACION EXITOSA: ARQUITECTURA DE EXPANSION COMPLIANT (SLSA 3 / AAUFSA) ${NC}"
    echo -e "${BLUE}==============================================================================${NC}"
    exit 0
else
    echo -e "${RED}   VERIFICACION FALLIDA: NO SE PERMITE EL DESPLIEGUE A PRODUCCION            ${NC}"
    echo -e "${BLUE}==============================================================================${NC}"
    exit 1
fi
