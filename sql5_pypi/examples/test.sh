set -x
cd "$(dirname "$0")/.."
uv pip install -e .
cd examples
uv run python sql5test.py