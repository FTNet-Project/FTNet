#!/bin/bash

set -ue
set -o pipefail

./scripts/build-wasm.sh || exit 1
./scripts/optimise-wasm.sh || exit 1

rm .gitignore

echo .packages > .gitignore
echo .fastn >> .gitignore
echo node_modules >> .gitignore
echo .is-local >> .gitignore

sh -c "$(curl -fsSL https://fastn.com/install.sh)"

cd kulfi.fifthtry.site/

fastn upload kulfi
