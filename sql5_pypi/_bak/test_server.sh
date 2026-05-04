#!/bin/bash
set -e

cd "$(dirname "$0")"

echo "Running sql5 server integration tests..."
echo

export PYTHONPATH="$PWD:$PYTHONPATH"
python test_sql5_server.py