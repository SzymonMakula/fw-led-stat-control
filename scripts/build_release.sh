#!/bin/bash
set -e

SCRIPT_DIR=$( cd -- "$( dirname -- "${BASH_SOURCE[0]}" )" &> /dev/null && pwd )
BASE_DIR=$(dirname ${SCRIPT_DIR})
RELEASE_DIR=${BASE_DIR}/release
TEMP_DIR=${RELEASE_DIR}/temp


mkdir -p ${TEMP_DIR}

# Unpack plugins
PLUGINS_SCRIPT_PATH=${BASE_DIR}/plugins/scripts/bundle_plugins.sh
bash ${PLUGINS_SCRIPT_PATH}
PLUGINS_TAR_PATH=${BASE_DIR}/plugins/release/plugins.tar.xz
tar -xf "${PLUGINS_TAR_PATH}" -C ${TEMP_DIR}

# Build binary
cargo build --release
cp ${BASE_DIR}/target/release/fw-led-stat-control ${TEMP_DIR}

# Copy configuration template
cp ${BASE_DIR}/templates/config.toml ${TEMP_DIR}

# Pack application to .tar archive
tar cJfP  ${RELEASE_DIR}/release.tar.xz -C ${TEMP_DIR} . --remove-files

