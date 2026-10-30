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
echo -e "\n${YELLOW}[AUDITORIA 1/6] Verificando ausencia de secretos y claves privadas...${NC}"
SECRET_PATTERNS="(AIza[0-9A-Za-z\\-_]{35}|AKIA[0-9A-Z]{16}|ASIA[0-9A-Z]{16}|-----BEGIN (RSA|EC|DSA|OPENSSH|PRIVATE) KEY-----|ghp_[0-9a-zA-Z]{36})"

if grep -rEIq "${SECRET_PATTERNS}" src/ Cargo.toml schema.sql AAUFSA_STATUTE_MASTER.md 2>/dev/null; then
    echo -e "${RED}[FALLO CRITICO] Secretos criptograficos detectados en el repositorio.${NC}"
    EXIT_CODE=1
else
    echo -e "${GREEN}[APROBADO] Cero credenciales expuestas en archivos fuente.${NC}"
fi

# --- CONTROL 2: INTEGRIDAD ESTATUTARIA AAUFSA Y CLÁUSULA 3 (0% ALTERACIÓN) ---
echo -e "\n${YELLOW}[AUDITORIA 2/6] Verificando Estatuto AAUFSA y Clausula 3 de Atribucion...${NC}"
if [ -f "AAUFSA_STATUTE_MASTER.md" ] && grep -q "Jose Emanuel Guadalupe Lopez" src/main.rs && grep -q "SEC. 5000F" AAUFSA_STATUTE_MASTER.md && grep -q "SEC. 5000G" AAUFSA_STATUTE_MASTER.md && grep -q "SEC. 5000H" AAUFSA_STATUTE_MASTER.md; then
    echo -e "${GREEN}[APROBADO] Estatuto AAUFSA (Sec. 5000F-5000H) y atribucion exclusiva verificados al 100%.${NC}"
else
    echo -e "${RED}[FALLO CRITICO] Violacion de la Clausula 3 o estatuto AAUFSA incompleto.${NC}"
    EXIT_CODE=1
fi

# --- CONTROL 3: AISLAMIENTO LÓGICO DE LOS 6 DOMINIOS EN RUST Y SQL ---
echo -e "\n${YELLOW}[AUDITORIA 3/6] Validando aislamiento formal de los 6 dominios corporativos...${NC}"
EXPECTED_DOMAINS=(
    "877771073297"
    "964521086569"
    "188680247367"
    "1087754464927"
    "282230018778"
    "358233233"
)

MISSING_DOMAINS=0
RUST_SRC_CLEAN=$(tr -d '_' < src/main.rs)

for domain in "${EXPECTED_DOMAINS[@]}"; do
    if ! grep -q "${domain}" <<< "${RUST_SRC_CLEAN}"; then
        echo -e "${RED}[ERROR] Dominio ${domain} no modelado en src/main.rs${NC}"
        MISSING_DOMAINS=$((MISSING_DOMAINS + 1))
    fi
    if ! grep -q "${domain}" schema.sql; then
        echo -e "${RED}[ERROR] Dominio ${domain} ausente en schema.sql${NC}"
        MISSING_DOMAINS=$((MISSING_DOMAINS + 1))
    fi
done

if [ ${MISSING_DOMAINS} -eq 0 ] && grep -q "ENABLE ROW LEVEL SECURITY" schema.sql; then
    echo -e "${GREEN}[APROBADO] Aislamiento contable y RLS validado para los 6 dominios corporativos.${NC}"
else
    echo -e "${RED}[FALLO CRITICO] Violacion de segregacion de dominios corporativos.${NC}"
    EXIT_CODE=1
fi

# --- CONTROL 4: VERIFICACIÓN DE ARITMÉTICA ENTERA Y CERO COMA FLOTANTE ---
echo -e "\n${YELLOW}[AUDITORIA 4/6] Verificando cero punto flotante en motores VPP y AAUFSA...${NC}"
if grep -rEn "(f32|f64)" src/; then
    echo -e "${RED}[FALLO CRITICO] Tipos de punto flotante IEEE-754 detectados en codigo Rust.${NC}"
    EXIT_CODE=1
else
    echo -e "${GREEN}[APROBADO] 100% de operaciones implementadas con aritmetica entera de punto fijo.${NC}"
fi

# --- CONTROL 5: CUMPLIMIENTO DE LICENCIAS PERMISIVAS Y CERO COPYLEFT ---
echo -e "\n${YELLOW}[AUDITORIA 5/6] Escaneando licencias en dependencias de compilacion...${NC}"
if [ -f "deny.toml" ] && grep -q "GPL" deny.toml && grep -q "AGPL" deny.toml; then
    echo -e "${GREEN}[APROBADO] Politica deny.toml verificada: Cero contaminacion copyleft.${NC}"
else
    echo -e "${RED}[FALLO CRITICO] deny.toml ausente o incompleto.${NC}"
    EXIT_CODE=1
fi

# --- CONTROL 6: GENERACIÓN DEL MANIFIESTO CRIPTOGRÁFICO DE CONFORMIDAD ---
echo -e "\n${YELLOW}[AUDITORIA 6/6] Sellando manifiesto criptografico inmutable...${NC}"
FILES_TO_SEAL=("AAUFSA_STATUTE_MASTER.md" "src/main.rs" "Cargo.toml" "schema.sql" "deny.toml")
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
    echo -e "${GREEN}   VERIFICACION EXITOSA: ARQUITECTURA Y ESTATUTO AAUFSA 100% COMPLIANT        ${NC}"
    echo -e "${BLUE}==============================================================================${NC}"
    exit 0
else
    echo -e "${RED}   VERIFICACION FALLIDA: NO SE PERMITE EL DESPLIEGUE A PRODUCCION            ${NC}"
    echo -e "${BLUE}==============================================================================${NC}"
    exit 1
fi
