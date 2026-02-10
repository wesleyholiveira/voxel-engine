#!/bin/sh
# Manual script to format all Rust code with rustfmt

set -e

echo "Formatting all Rust code in the project..."
find . -type f -name "*.rs" \( -path "*/src/*" -o -path "*/examples/*" \) ! -path "./target/*" -print0 | \
  xargs -0 rustfmt --edition 2021

echo "✓ Rust code formatting complete!"
