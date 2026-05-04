import os
import sys
import platform
import stat
import urllib.request
import hashlib

VERSION = "2.0.0"
OWNER = "cccrust"
REPO = "sql5"

BINARY_NAMES = {
    "macos-arm64": "sql5-macos-arm64",
    "macos-x86_64": "sql5-macos-x86_64",
    "linux-x86_64": "sql5-linux-x86_64",
    "windows-x86_64": "sql5-windows.exe",
}

BINARY_URLS = {
    "macos-arm64": f"https://github.com/{OWNER}/{REPO}/releases/download/v{VERSION}/sql5-macos-arm64",
    "macos-x86_64": f"https://github.com/{OWNER}/{REPO}/releases/download/v{VERSION}/sql5-macos-x86_64",
    "linux-x86_64": f"https://github.com/{OWNER}/{REPO}/releases/download/v{VERSION}/sql5-linux-x86_64",
    "windows-x86_64": f"https://github.com/{OWNER}/{REPO}/releases/download/v{VERSION}/sql5-windows.exe",
}

CHECKSUMS = {
    "macos-arm64": None,
    "macos-x86_64": None,
    "linux-x86_64": None,
    "windows-x86_64": None,
}

def get_platform():
    system = platform.system().lower()
    machine = platform.machine().lower()

    if system == "darwin" and machine in ("arm64", "aarch64"):
        return "macos-arm64"
    elif system == "darwin":
        return "macos-x86_64"
    elif system == "linux":
        return "linux-x86_64"
    elif system == "windows":
        return "windows-x86_64"
    else:
        raise RuntimeError(f"Unsupported platform: {system}/{machine}")

def get_cache_dir():
    if sys.platform == "darwin" or sys.platform == "linux":
        base = os.path.expanduser("~/.cache/sql5")
    else:
        base = os.path.join(os.environ.get("LOCALAPPDATA", ""), "sql5", "cache")
    os.makedirs(base, exist_ok=True)
    return base

def get_binary_path():
    cache_dir = get_cache_dir()
    platform_name = get_platform()
    binary_name = BINARY_NAMES[platform_name]
    binary_path = os.path.join(cache_dir, binary_name)

    if not os.path.exists(binary_path):
        _download_binary(platform_name, binary_name, binary_path)

    return binary_path

def _download_binary(platform_name, binary_name, destination):
    url = BINARY_URLS[platform_name]
    print(f"Downloading sql5 {VERSION} for {platform_name}...", file=sys.stderr)

    try:
        urllib.request.urlretrieve(url, destination)
    except Exception as e:
        raise RuntimeError(
            f"Failed to download sql5 binary from {url}\n"
            f"Please ensure the release v{VERSION} exists on GitHub.\n"
            f"Error: {e}"
        )

    os.chmod(destination, os.stat(destination).st_mode | stat.S_IXUSR | stat.S_IXGRP | stat.S_IXOTH)

    print(f"Downloaded to {destination}", file=sys.stderr)