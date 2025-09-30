#!/bin/bash

set -euo pipefail

case "$1" in
    get-version)
        if ! [ -e Cargo.toml ]; then
            echo "Cargo.toml not found" >&2
            exit 1
        fi

        VERSION=$(grep '^version' Cargo.toml | head -n 1 | sed 's/version = "\(.*\)"/\1/')

        if [[ -n "${USE_GIT_VERSION:-}" ]]; then
            if ! git rev-parse --is-inside-work-tree >/dev/null 2>&1; then
                echo "Not a git repository" >&2
                exit 1
            fi
            COMMIT_HASH=$(git rev-parse --short HEAD)
            if [ -z "$(git tag -l)" ]; then
                COMMITS_NUM_SINCE_LAST_TAG=$(git rev-list --count HEAD)
            else
                COMMITS_NUM_SINCE_LAST_TAG=$(git log $(git describe --tags --abbrev=0)..HEAD --oneline | wc -l)
            fi
            echo "${VERSION}.dev${COMMITS_NUM_SINCE_LAST_TAG}+${COMMIT_HASH}"
        else
            echo "${VERSION}"
        fi
        ;;
    prepare-dist)
        if [[ "$2" == *"dev"* ]]; then
            $MESONREWRITE --sourcedir="$MESON_PROJECT_DIST_ROOT" kwargs set project / version "$2"
        fi
        ;;
    *)
        exit 1
        ;;
esac
