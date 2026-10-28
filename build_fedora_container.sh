#!/usr/bin/env bash
set -euo pipefail

# Check if a version argument was provided
if [ $# -eq 0 ]; then
    echo "Error: No Fedora version supplied."
    echo "Usage: ./container_build_fedora <version>"
    echo "Example: ./container_build_fedora 40"
    exit 1
fi

VERSION="$1"
TAG="nice-plug-builder:f${VERSION}"

echo "Building Podman container for Fedora ${VERSION}..."
echo "Tag: ${TAG}"
echo "--------------------------------------------------"

# Run the podman build command
podman build ./ \
    --build-arg FEDORA_VERSION="${VERSION}" \
    -t "${TAG}"

echo "--------------------------------------------------"
echo "Successfully built container image: ${TAG}"