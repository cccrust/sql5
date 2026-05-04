import os
import sys
import platform
import stat
import urllib.request
import json

OWNER = "cccrust"
REPO = "sql5"

BINARY_NAMES = {
    "macos-arm64": "sql5-macos-arm64",
    "macos-x86_64": "sql5-macos-x86_64",
    "linux-x86_64": "sql5-linux-x86_64",
    "windows-x86_64": "sql5-windows.exe",
}

BINARY_URL_NAMES = {
    "macos-arm64": "sql5-macos-arm64",
    "macos-x86_64": "sql5-macos-x86_64",
    "linux-x86_64": "sql5-linux-x86_64",
    "windows-x86_64": "sql5-windows.exe",
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

def get_releases():
    """Fetch all releases from GitHub API."""
    url = f"https://api.github.com/repos/{OWNER}/{REPO}/releases"
    try:
        with urllib.request.urlopen(url, timeout=10) as response:
            return json.loads(response.read())
    except Exception as e:
        raise RuntimeError(
            f"Failed to fetch releases from GitHub: {e}\n"
            "Please check your internet connection or set SQL5_BINARY manually."
        )

def find_release_with_binary():
    """Find the first release that has binary assets for our platform."""
    releases = get_releases()
    platform_name = get_platform()
    binary_name = BINARY_NAMES[platform_name]

    for release in releases:
        tag = release.get("tag_name", "")
        if tag.startswith("v"):
            version = tag[1:]
        else:
            version = tag

        # Check if this release has assets
        assets = release.get("assets", [])
        for asset in assets:
            if asset.get("name") == binary_name:
                return version, asset.get("browser_download_url")

    return None, None

def get_binary_path():
    cache_dir = get_cache_dir()
    platform_name = get_platform()
    binary_name = BINARY_NAMES[platform_name]
    binary_path = os.path.join(cache_dir, binary_name)

    version, url = find_release_with_binary()
    if not version or not url:
        raise RuntimeError(
            f"No suitable release found for platform {platform_name}.\n"
            "Please ensure a release with binary exists on GitHub, "
            "or set SQL5_BINARY to point to a local binary."
        )

    # Check if we have the correct version cached
    version_file = os.path.join(cache_dir, f"{binary_name}.version")
    cached_version = None
    if os.path.exists(version_file):
        cached_version = open(version_file).read().strip()

    if os.path.exists(binary_path) and cached_version == version:
        return binary_path

    # Download new version
    _download_binary(platform_name, binary_name, binary_path, version, url)
    return binary_path

def _download_binary(platform_name, binary_name, destination, version, url):
    print(f"Downloading sql5 v{version} for {platform_name}...", file=sys.stderr)

    try:
        urllib.request.urlretrieve(url, destination)
    except Exception as e:
        raise RuntimeError(
            f"Failed to download sql5 binary from {url}\n"
            f"Please ensure release v{version} with binary exists on GitHub.\n"
            f"Error: {e}"
        )

    # Save version info
    cache_dir = get_cache_dir()
    version_file = os.path.join(cache_dir, f"{binary_name}.version")
    with open(version_file, "w") as f:
        f.write(version)

    os.chmod(destination, os.stat(destination).st_mode | stat.S_IXUSR | stat.S_IXGRP | stat.S_IXOTH)

    print(f"Downloaded to {destination}", file=sys.stderr)