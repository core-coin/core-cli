# Makefile for a Rust application

# Variables
CARGO := cargo
TARGET := target
RELEASE := release
DEBUG := debug

# Default target
all: build

# Build the project in release mode
build:
	$(CARGO) build --release

# Build the project in debug mode
debug:
	$(CARGO) build

# Run the project in release mode
run:
	$(CARGO) run --release

# Run the project in debug mode
run-debug:
	$(CARGO) run

# Test the project
test: build
	$(CARGO) test --all-targets --all-features

# Clean the project
clean:
	$(CARGO) clean

# Format the code
fmt:
	$(CARGO) fmt

# Check for common mistakes
clippy:
	$(CARGO) clippy --all-targets --all-features -- -D warnings

.PHONY: all build debug run run-debug test clean fmt clippy