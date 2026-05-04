#!/bin/bash
# pub.sh — sql5 發布腳本
#
# 用法:
#   ./pub.sh pypi     直接上傳到 PyPI（需要 PYPI_TOKEN）
#   ./pub.sh github   建立 GitHub tag 並推送（觸發 GitHub Actions 上傳到 PyPI）
#   ./pub.sh          顯示幫助並警告

set -euo pipefail

RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[0;33m'
RESET='\033[0m'

PROJECT_DIR="$(cd "$(dirname "$0")" && pwd)"
cd "$PROJECT_DIR"

function usage() {
    echo "=============================================="
    echo "sql5 發布工具"
    echo "=============================================="
    echo ""
    echo "用法:"
    echo "  ./pub.sh pypi     直接上傳到 PyPI（會使用 .pypirc 或 PYPI_TOKEN）"
    echo "  ./pub.sh github   建立 GitHub tag 並推送（觸發 GitHub Actions 上傳到 PyPI）"
    echo ""
    echo "發布前請確認:"
    echo "  1. 版本已更新 (Cargo.toml, pyproject.toml, __init__.py)"
    echo "  2. 所有測試通過 (cargo test && ./rutest.sh && ./pytest.sh)"
    echo "  3. _doc/vX.X.md 版本文件已更新"
    echo ""
    echo "當前版本:"
    grep '^version = ' Cargo.toml || echo "  (無法讀取)"
    echo ""
}

function warn_no_arg() {
    echo -e "${YELLOW}警告: 未指定發布目標${RESET}"
    echo ""
    echo "請選擇發布方式:"
    echo ""
    echo "  pypi     - 直接上傳到 PyPI（需要已 build 的 dist/）"
    echo "            會先 build Python package: cd sql5_pypi && python -m build"
    echo ""
    echo "  github   - 建立 GitHub tag 並推送"
    echo "            會觸發 GitHub Actions: build + 上傳 release + 上傳 PyPI"
    echo ""
    echo "重要: 發布前請確認版本已更新！"
    echo ""
    read -p "繼續發布? (y/N) " -n 1 -r
    echo
    if [[ ! $REPLY =~ ^[Yy]$ ]]; then
        echo "已取消"
        exit 1
    fi
}

function do_pypi() {
    echo -e "${GREEN}=== 直接上傳到 PyPI ===${RESET}"
    echo ""

    cd "$PROJECT_DIR/sql5_pypi"

    echo "清理舊 build..."
    rm -rf dist build *.egg-info

    echo "Build Python package..."
    python -m build

    echo "上傳到 PyPI..."
    if [[ -n "${PYPI_TOKEN:-}" ]]; then
        twine upload dist/* --user __token__ --password "$PYPI_TOKEN"
    else
        twine upload dist/*
    fi

    echo ""
    echo -e "${GREEN}完成！已上傳到 PyPI${RESET}"
}

function do_github() {
    echo -e "${GREEN}=== 建立 GitHub Release ===${RESET}"
    echo ""

    # Check for uncommitted changes
    if ! git diff --quiet || ! git diff --cached --quiet || [ -n "$(git status --porcelain)" ]; then
        echo -e "${YELLOW}警告: Working tree 有未提交的更改${RESET}"
        git status --short
        echo ""
        read -p "繼續? (y/N) " -n 1 -r
        echo
        if [[ ! $REPLY =~ ^[Yy]$ ]]; then
            exit 1
        fi
    fi

    # Get current version from Cargo.toml
    CURRENT_VERSION=$(grep '^version = ' Cargo.toml | sed 's/version = "\(.*\)"/\1/')

    if [[ -z "$CURRENT_VERSION" ]]; then
        echo -e "${RED}錯誤: 無法讀取版本${RESET}"
        exit 1
    fi

    echo "當前版本: $CURRENT_VERSION"
    TAG="v$CURRENT_VERSION"

    # Check if tag already exists
    if git rev-parse "$TAG" >/dev/null 2>&1; then
        echo -e "${RED}錯誤: Tag $TAG 已存在${RESET}"
        echo "請先刪除: git tag -d $TAG"
        exit 1
    fi

    echo ""
    echo "將會:"
    echo "  1. 建立 tag: $TAG"
    echo "  2. 推送到 GitHub"
    echo "  3. GitHub Actions 會:"
    echo "     - Build 4 平台 (macOS arm64/x86_64, Linux, Windows)"
    echo "     - 上传到 GitHub Release"
    echo "     - 上傳到 PyPI"
    echo ""

    read -p "確認發布? (y/N) " -n 1 -r
    echo
    if [[ ! $REPLY =~ ^[Yy]$ ]]; then
        echo "已取消"
        exit 1
    fi

    # Create and push tag
    echo "建立 tag..."
    git tag "$TAG"

    echo "推送到 GitHub..."
    git push origin "$TAG"

    echo ""
    echo -e "${GREEN}完成！已推送到 GitHub${RESET}"
    echo ""
    echo "GitHub Actions 將在約 2 分鐘後完成"
    echo "查看: https://github.com/cccrust/sql5/actions"
    echo ""
    echo "安裝: pip install sql5"
}

# Main
if [[ $# -eq 0 ]]; then
    usage
    warn_no_arg
    echo ""
    echo "請使用: ./pub.sh pypi 或 ./pub.sh github"
    exit 1
fi

case "$1" in
    pypi)
        do_pypi
        ;;
    github)
        do_github
        ;;
    -h|--help|help)
        usage
        ;;
    *)
        echo -e "${RED}未知參數: $1${RESET}"
        usage
        exit 1
        ;;
esac