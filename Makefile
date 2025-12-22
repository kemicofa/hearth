SHELL := /bin/bash

CLIENT_DIR := client
BACKEND_DIR := backend

.PHONY: help client-install backend-build client-dev backend-dev dev stop clean

help:
	@echo "Targets:"
	@echo "  make dev            Run client + backend (parallel)"
	@echo "  make client-dev     Run Svelte dev server"
	@echo "  make backend-dev    Run Rust backend (cargo run)"
	@echo "  make client-install Install client dependencies"
	@echo "  make backend-build  Build backend"
	@echo "  make clean          Clean backend build artifacts"

client-install:
	@cd $(CLIENT_DIR) && npm install

backend-build:
	@cd $(BACKEND_DIR) && cargo build

client-dev: client-install
	@cd $(CLIENT_DIR) && npm run dev

backend-dev:
	@cd $(BACKEND_DIR) && cargo run

# Run both processes in parallel; Ctrl+C stops both.
dev:
	@set -e; \
	trap 'echo ""; echo "Stopping..."; kill 0' INT TERM; \
	( $(MAKE) client-dev ) & \
	( $(MAKE) backend-dev ) & \
	wait

clean:
	@cd $(BACKEND_DIR) && cargo clean
