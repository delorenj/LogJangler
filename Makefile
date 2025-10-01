.PHONY: help build test run clean services services-down services-logs install check fmt clippy doc

help: ## Show this help message
	@echo 'Usage: make [target]'
	@echo ''
	@echo 'Available targets:'
	@grep -E '^[a-zA-Z_-]+:.*?## .*$$' $(MAKEFILE_LIST) | sort | awk 'BEGIN {FS = ":.*?## "}; {printf "  \033[36m%-15s\033[0m %s\n", $$1, $$2}'

build: ## Build all crates in release mode
	cargo build --release --workspace

test: ## Run all tests
	cargo test --workspace

run: ## Run the service (requires services to be up)
	cargo run --bin logjangler-service

clean: ## Clean build artifacts
	cargo clean
	rm -rf target/

services: ## Start Docker services (RabbitMQ + Meilisearch)
	docker compose up -d

services-down: ## Stop Docker services
	docker compose down

services-logs: ## View Docker services logs
	docker compose logs -f


install: build ## Install CLI tool
	cargo install --path crates/logjangler-cli

check: ## Check code without building
	cargo check --workspace

fmt: ## Format code
	cargo fmt --all

clippy: ## Run clippy linter
	cargo clippy --workspace -- -D warnings

doc: ## Build and open documentation
	cargo doc --workspace --open

dev: services ## Start development environment
	@echo "Services started!"
	@echo ""
	@echo "RabbitMQ Management: http://localhost:15672"
	@echo "  Username: logjangler"
	@echo "  Password: logjangler"
	@echo ""
	@echo "Meilisearch: http://localhost:7700"
	@echo ""
	@echo "Run 'make run' to start the service"
