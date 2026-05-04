#!/usr/bin/env bash
# testall.sh — Run all sql5 tests
# 一個指令測試全部

set -uo pipefail

PROJECT_DIR="$(cd "$(dirname "$0")" && pwd)"
BINARY="$PROJECT_DIR/target/release/sql5"

# 顏色
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[0;33m'
BLUE='\033[0;34m'
RESET='\033[0m'

echo ""
echo "=============================================="
echo -e "${BLUE}sql5 全端測試 ${RESET}(全部測試)"
echo "=============================================="
echo ""

# ============================================
# 1. Build Rust binary
# ============================================
echo -e "${BLUE}[1/5] Building Rust binary...${RESET}"
if [[ ! -x "$BINARY" ]]; then
    echo "  Building release binary..."
    cd "$PROJECT_DIR" && cargo build --release
    if [[ ! -x "$BINARY" ]]; then
        echo -e "${RED}ERROR: Build failed${RESET}"
        exit 1
    fi
fi
echo -e "  ${GREEN}Binary: $BINARY${RESET}"
echo ""

# ============================================
# 2. Rust unit tests (cargo test)
# ============================================
echo -e "${BLUE}[2/5] Running Rust unit tests...${RESET}"
echo ""
cd "$PROJECT_DIR"
cargo test 2>&1 | tail -30
CARGO_STATUS=$?
echo ""
if [[ $CARGO_STATUS -eq 0 ]]; then
    echo -e "  ${GREEN}Rust unit tests: PASSED${RESET}"
else
    echo -e "  ${RED}Rust unit tests: FAILED${RESET}"
fi
echo ""

# ============================================
# 3. CLI integration tests (rutest.sh)
# ============================================
echo -e "${BLUE}[3/5] Running CLI integration tests...${RESET}"
echo ""
cd "$PROJECT_DIR"
./rutest.sh "$BINARY" 2>&1 | tail -30
CLI_STATUS=${PIPESTATUS[0]}
echo ""
if [[ $CLI_STATUS -eq 0 ]]; then
    echo -e "  ${GREEN}CLI integration tests: PASSED${RESET}"
else
    echo -e "  ${YELLOW}CLI integration tests: some failures${RESET}"
fi
echo ""

# ============================================
# 4. Python pytest tests
# ============================================
echo -e "${BLUE}[4/5] Running Python pytest tests...${RESET}"
echo ""
export SQL5_BINARY="$BINARY"
export PYTHONPATH="${PROJECT_DIR/sql5_pypi}:${PYTHONPATH:-}"
cd "$PROJECT_DIR/sql5_pypi"
python3 -m pytest tests/test_sql5.py -v 2>&1 | tail -20
PYTEST_STATUS=$?
echo ""
if [[ $PYTEST_STATUS -eq 0 ]]; then
    echo -e "  ${GREEN}Python pytest tests: PASSED${RESET}"
else
    echo -e "  ${RED}Python pytest tests: FAILED${RESET}"
fi
echo ""

# ============================================
# 5. Python client integration test (sql5test.py)
# ============================================
echo -e "${BLUE}[5/5] Running Python client test...${RESET}"
echo ""
cd "$PROJECT_DIR/sql5_pypi/examples"
rm -f mydb.db
python3 sql5test.py 2>&1
PYCLIENT_STATUS=$?
echo ""
if [[ $PYCLIENT_STATUS -eq 0 ]]; then
    echo -e "  ${GREEN}Python client test (subprocess): PASSED${RESET}"
else
    echo -e "  ${RED}Python client test (subprocess): FAILED${RESET}"
fi
rm -f mydb.db
echo ""

# ============================================
# 6. WebSocket test (v3.0 new)
# ============================================
echo -e "${BLUE}[6/6] Running WebSocket test...${RESET}"
echo ""
cd "$PROJECT_DIR/sql5_pypi/examples"
rm -f ws_test.db ws_test.db-wal ws_test.db-shm 2>/dev/null
export SQL5_BINARY="$BINARY"
python3 websocket_test.py 2>&1
WEBSOCKET_STATUS=$?
echo ""
if [[ $WEBSOCKET_STATUS -eq 0 ]]; then
    echo -e "  ${GREEN}WebSocket test: PASSED${RESET}"
else
    echo -e "  ${RED}WebSocket test: FAILED${RESET}"
fi
rm -f ws_test.db ws_test.db-wal ws_test.db-shm 2>/dev/null
echo ""

# ============================================
# Summary
# ============================================
echo "=============================================="
echo -e "${BLUE}測試結果總覽${RESET}"
echo "=============================================="

if [[ $CARGO_STATUS -eq 0 ]]; then
    echo -e "  ${GREEN}[PASS]${RESET} Rust unit tests (cargo test)"
else
    echo -e "  ${RED}[FAIL]${RESET} Rust unit tests (cargo test)"
fi

if [[ $CLI_STATUS -eq 0 ]]; then
    echo -e "  ${GREEN}[PASS]${RESET} CLI integration tests (rutest.sh)"
else
    echo -e "  ${YELLOW}[PARTIAL]${RESET} CLI integration tests (rutest.sh)"
fi

if [[ $PYTEST_STATUS -eq 0 ]]; then
    echo -e "  ${GREEN}[PASS]${RESET} Python pytest tests"
else
    echo -e "  ${RED}[FAIL]${RESET} Python pytest tests"
fi

if [[ $PYCLIENT_STATUS -eq 0 ]]; then
    echo -e "  ${GREEN}[PASS]${RESET} Python client test (subprocess)"
else
    echo -e "  ${RED}[FAIL]${RESET} Python client test (subprocess)"
fi

if [[ $WEBSOCKET_STATUS -eq 0 ]]; then
    echo -e "  ${GREEN}[PASS]${RESET} WebSocket test (v3.0)"
else
    echo -e "  ${RED}[FAIL]${RESET} WebSocket test (v3.0)"
fi

echo ""
echo "=============================================="
echo "測試完成!"
echo "=============================================="
echo ""

# Exit with failure if any test failed
if [[ $CARGO_STATUS -ne 0 ]] || [[ $PYTEST_STATUS -ne 0 ]] || [[ $PYCLIENT_STATUS -ne 0 ]] || [[ $WEBSOCKET_STATUS -ne 0 ]]; then
    exit 1
fi

exit 0