#!/bin/bash
set -e

dirs=("battery" "cpu" "memory" "time")
SCRIPT_DIR=$( cd -- "$( dirname -- "${BASH_SOURCE[0]}" )" &> /dev/null && pwd )
BASE_DIR=$(dirname ${SCRIPT_DIR})
RELEASE_DIR=${BASE_DIR}/release
BUNDLE_DIR=${RELEASE_DIR}/plugins

mkdir -p ${BUNDLE_DIR}

for dir in "${dirs[@]}"; do
  src="${BASE_DIR}/${dir}/build/release.wasm"
  if [[ -f "$src" ]]; then
    cp "$src" "${BUNDLE_DIR}/${dir}.wasm"
  else
    echo "WASM module: ${src} not found"
    exit 1
  fi
done

BUNDLE_FILE=${RELEASE_DIR}/plugins.tar.xz


tar cJfP ${BUNDLE_FILE} -C ${RELEASE_DIR} plugins --remove-files
echo "Plugins bundled to: ${BUNDLE_FILE}"
exit 0
