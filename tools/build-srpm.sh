#!/bin/bash

set -euo pipefail

PROJECT_NAME="oci-dev-binder-hook"
PROJECT_DIR="$(dirname "$(readlink -f "$0")")/.."

if ! command -v rpmbuild >/dev/null 2>&1; then
    echo "rpmbuild not found. Please install rpm-build package." >&2
    exit 1
fi

ALLOW_DIRTY=false
while [[ $# -gt 0 ]]; do
    case "$1" in
        --dev)
            export USE_GIT_VERSION=1
            shift
            ;;
        --allow-dirty)
            ALLOW_DIRTY=true
            shift
            ;;
        *)
            break
            ;;
    esac
done

BUILD_DIR=$(mktemp -d)
RPM_BUILD_ROOT="${BUILD_DIR}/rpmbuild"
RPM_SOURCES="${RPM_BUILD_ROOT}/SOURCES"
RPM_SPECS="${RPM_BUILD_ROOT}/SPECS"
RPM_SRPMS="${RPM_BUILD_ROOT}/SRPMS"

mkdir -p "${RPM_SOURCES}" "${RPM_SPECS}" "${RPM_SRPMS}"

meson setup -Dbuild_rpm_spec=true $BUILD_DIR $PROJECT_DIR
if [ "$ALLOW_DIRTY" = true ]; then
    meson dist --allow-dirty --no-tests -C $BUILD_DIR
else
    meson dist --no-tests -C $BUILD_DIR
fi

cp $BUILD_DIR/meson-dist/*.tar.xz $RPM_SOURCES/
cp $BUILD_DIR/rpm/$PROJECT_NAME.spec $RPM_SPECS/

rpmbuild -bs --define "_topdir $RPM_BUILD_ROOT" $RPM_SPECS/$PROJECT_NAME.spec

cp "${RPM_SRPMS}"/*.src.rpm ./

trap 'rm -rf "${BUILD_DIR}"' EXIT
