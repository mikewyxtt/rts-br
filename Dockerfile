FROM rust:latest
RUN apt-get update && apt-get install -y \
    build-essential \
    pkg-config \
    libasound2-dev \
    libudev-dev \
    libx11-dev \
    libxi-dev \
    libxrandr-dev \
    libxcursor-dev \
    libxinerama-dev \
    libgl1-mesa-dev \
    libvulkan-dev \
##  WINDOWS STUFF ##
    cmake \
    mingw-w64 \
    && rm -rf /var/lib/apt/lists/*

RUN rustup target add x86_64-pc-windows-gnu

WORKDIR /app
