.DEFAULT_GOAL := help

.PHONY: help
help: ## Display this help message
	@./help.sh "$(MAKEFILE_LIST)"

.PHONY: lint 
lint: ## Run Linting Checks
	@cargo clippy -- -D warnings

.PHONY: fmt 
fmt: ## Format the code
	@cargo fmt
	@cargo fix --allow-staged
	@cargo clippy --fix --allow-staged

.PHONY: build 
build: ## Build the project
	@cargo build

.PHONY: test
test: # Build the project
	@cargo test

.PHONY: doc-tests
doc-tests: ## Run documentation tests
	@cargo test --doc
