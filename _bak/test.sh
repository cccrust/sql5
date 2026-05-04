#!/usr/bin/env bash
# test.sh — sql5 server integration test
# 使用 Python client 測試 SQL 操作

set -uo pipefail

PROJECT_DIR="$(cd "$(dirname "$0")" && pwd)"
BINARY="$PROJECT_DIR/target/release/sql5"
PYTHON_DIR="$PROJECT_DIR/sql5_pypi"

RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[0;33m'
RESET='\033[0m'

echo "=============================================="
echo "sql5 Server Integration Test (v2.0)"
echo "=============================================="

# Build if needed
if [[ ! -x "$BINARY" ]]; then
    echo -e "${YELLOW}Building release binary...${RESET}"
    cd "$PROJECT_DIR" && cargo build --release
    if [[ ! -x "$BINARY" ]]; then
        echo -e "${RED}ERROR: Build failed${RESET}"
        exit 1
    fi
fi

echo -e "${GREEN}Binary: $BINARY${RESET}"
echo ""

# Run Python test
export SQL5_BINARY="$BINARY"
python3 "$PROJECT_DIR/test.py"
exit $?