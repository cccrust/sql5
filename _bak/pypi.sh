#!/bin/bash
set -e

cd "$(dirname "$0")"

echo "Cleaning old builds..."
rm -rf dist build *.egg-info

echo "Building package..."
python -m build

echo "Uploading to PyPI..."
twine upload dist/* --user __token__ --password "$PYPI_TOKEN"

echo "Done! Package uploaded successfully."