# Use a Rust base image with Cargo installed
FROM rust:1.84.1 AS builder

# Set the working directory
WORKDIR /usr/src/myapp

# Copy the files from the current directory to the working directory
COPY . .

RUN apt update ; apt install -y watch bash vim neovim nano libclang-dev llvm-dev libstdc++-12-dev

# Run the project
CMD ["bash"]
