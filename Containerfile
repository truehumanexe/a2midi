ARG FEDORA_VERSION=42
FROM fedora:${FEDORA_VERSION}


# Install system dependencies required for Rust audio plugins and GUI (iced/X11/Wayland/ALSA)
RUN dnf -y update && dnf -y install \
    cargo \
    rust \
    gcc \
    gcc-c++ \
    make \
    pkg-config \
    alsa-lib-devel \
    pipewire-jack-audio-connection-kit-devel \
    libX11-devel \
    libXcursor-devel \
    libXi-devel \
    libXrandr-devel \
    libxkbcommon-devel \
    wayland-devel \
    mesa-libGL-devel \
    && dnf clean all

ENV PATH="/root/.cargo/bin:${PATH}"

RUN cargo install cargo-nice-plug

# Set up a working directory
WORKDIR /workspace

# Default command placeholder
CMD ["cargo", "--version"]

WORKDIR /workspace
CMD ["cargo", "--version"]