SHELL := /bin/bash
.PHONY: help

help:
	@grep -E '^[a-zA-Z_-]+:.*?## .*$$' $(MAKEFILE_LIST) | sort | awk 'BEGIN {FS = ":.*?## "}; {printf "\033[36m%-15s\033[0m %s\n", $$1, $$2}'

clean: ## Clean the project using cargo
	cargo clean

build: ## Build the project using cargo
	cargo build

lint: ## Lint the project using cargo
	@rustup component add clippy 2> /dev/null
	cargo clippy

fmt: ## Format the project using cargo
	@rustup component add rustfmt 2> /dev/null
	cargo fmt

bump: ## Bump the version number
	@echo "Current version is $(shell cargo pkgid | cut -d# -f2)"
	@read -p "Enter new version number: " version; \
	updated_version=$$(cargo pkgid | cut -d# -f2 | sed -E "s/([0-9]+\.[0-9]+\.[0-9]+)$$/$$version/"); \
	sed -i -E "s/^version = .*/version = \"$$updated_version\"/" Cargo.toml
	@echo "New version is $(shell cargo pkgid | cut -d# -f2)"

test: ## Run tests using cargo test
	cargo test

doc: ## Generate documentation using cargo doc
	cargo doc --open

check: ## Quick syntax check without building
	cargo check

release: ## Build optimized release version
	cargo build --release

fix: ## Automatically fix linting issues
	cargo fix --allow-dirty
	cargo fmt

test-unit: ## Run only unit tests
	cargo test --lib

test-doc: ## Run only documentation tests
	cargo test --doc

ci: ## Run all CI checks (format, lint, test)
	cargo fmt --check
	cargo clippy -- -D warnings
	cargo test
