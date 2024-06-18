# Use a fresh 22.04 (Jammy) installation as the base image
FROM ubuntu:22.04

# Set the environment variables to non-interactive to avoid prompts during installation
ENV DEBIAN_FRONTEND=noninteractive

# Configure the docker container with all the required dependencies
RUN apt-get update && apt-get upgrade -y \
    build-essential \
    curl \
    ca-certificates \
    apt-transport-https \
    gnupg \
    --no-install-recommends

RUN apt-get update

# Install Rust
RUN curl https://sh.rustup.rs -sSf | sh -s -- -y
ENV PATH="/root/.cargo/bin:${PATH}"
RUN rustc --version && cargo --version

# Install other Weenix dependencies
RUN apt-get install grub2-common gdb -y

RUN apt-get clean && rm -rf /var/lib/apt/lists/*

# Set the current working directory of the Dockerfile to "/reenix"
WORKDIR /reenix

ENV DISPLAY=:0