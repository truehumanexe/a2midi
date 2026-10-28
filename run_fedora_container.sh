#!/usr/bin/env bash
set -euo pipefail

INSTALL_NICE_PLUG=false
VERSION=""

# Parse arguments for flags and version
while [[ $# -gt 0 ]]; do
    case "$1" in
        -i|--install)
            INSTALL_NICE_PLUG=true
            shift
            ;;
        -*)
            echo "Error: Unknown option $1"
            echo "Usage: ./container_run_fedora [--install|-i] <version>"
            exit 1
            ;;
        *)
            if [ -z "$VERSION" ]; then
                VERSION="$1"
            else
                echo "Error: Multiple versions specified."
                exit 1
            fi
            shift
            ;;
    esac
done

# Validate that a version was provided
if [ -z "$VERSION" ]; then
    echo "Error: No Fedora version supplied."
    echo "Usage: ./container_run_fedora [--install|-i] <version>"
    echo "Example: ./container_run_fedora --install 40"
    exit 1
fi 2>/dev/null || true # compatible check

TAG="nice-plug-builder:f${VERSION}"

# Construct the inner command based on the flag
if [ "$INSTALL_NICE_PLUG" = true ]; then
    echo "Running container for Fedora ${VERSION} (with cargo-nice-plug installation)..."
    CONTAINER_CMD="cargo install cargo-nice-plug && cargo nice-plug bundle a2midi"
else
    echo "Running container for Fedora ${VERSION}..."
    CONTAINER_CMD="cargo nice-plug bundle a2midi"
fi

echo "Tag: ${TAG}"
echo "--------------------------------------------------"

# Run the podman container
podman run --rm \
    --volume "$PWD:/workspace:z" \
    --volume "cargo-cache:/root/.cargo:z" \
    "${TAG}" \
    bash -c "${CONTAINER_CMD}"

echo "--------------------------------------------------"
echo "Build and bundle complete!"