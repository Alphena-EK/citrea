################################################################################
# Makefile for Citrea / Sovereign SDK / RISC0 / SP1 Project
# ------------------------------------------------------------------------------
# This Makefile automates building, testing, linting, and installing dev tools
# for a Rust-based project (using Cargo). The default target is "help," so
# typing `make` alone will display available commands.
################################################################################

# ------------------------------------------------------------------------------
# 1) Configuration Variables
# ------------------------------------------------------------------------------
# You can move these to a `config.mk` or a `.env` file if you prefer.

EF_TESTS_URL ?= https://github.com/chainwayxyz/ef-tests/archive/develop.tar.gz
EF_TESTS_DIR := crates/evm/ethereum-tests
CITREA_E2E_TEST_BINARY := $(CURDIR)/target/debug/citrea
PARALLEL_PROOF_LIMIT := 1
TEST_FEATURES := --features testing
BATCH_OUT_PATH := resources/guests/risc0/
LIGHT_OUT_PATH := resources/guests/risc0/

# ------------------------------------------------------------------------------
# 2) Default Target
# ------------------------------------------------------------------------------
.PHONY: default
default: help

# ------------------------------------------------------------------------------
# 3) Help / Usage
# ------------------------------------------------------------------------------
.PHONY: help
help: ## Display this help message (default target).
	@echo "Usage: make [target]"
	@echo
	@awk 'BEGIN {FS = ":.*?## "} \
		/^[a-zA-Z_-]+:.*?## / \
		{printf "\033[36m%-25s\033[0m %s\n", $$1, $$2}' $(MAKEFILE_LIST)
	@echo

# ------------------------------------------------------------------------------
# 4) Build Targets
# ------------------------------------------------------------------------------
.PHONY: build-risc0-docker
build-risc0-docker: ## Build Docker images for RISC0 (batch & light client)
	$(MAKE) -C guests/risc0 batch-proof-bitcoin-docker OUT_PATH=$(BATCH_OUT_PATH)
	$(MAKE) -C guests/risc0 light-client-bitcoin-docker OUT_PATH=$(LIGHT_OUT_PATH)

.PHONY: build-sp1
build-sp1: ## Build SP1 guest code
	$(MAKE) -C guests/sp1 all

.PHONY: build
build: ## Build the project in debug mode
	@cargo build

.PHONY: build-test
build-test: ## Build the project with testing features
	@cargo build $(TEST_FEATURES)

# Example: If you want to build release mode in parallel for RISC0 and SP1,
# use `-j2`. However, be aware of potential race conditions if they share
# the same Cargo `target/` directory.
.PHONY: build-release
build-release: build-risc0-docker build-sp1 ## Build the project in release mode
	@$(MAKE) -j2 cargo-release-build

.PHONY: cargo-release-build
cargo-release-build:
	@cargo build --release

# ------------------------------------------------------------------------------
# 5) Clean Targets
# ------------------------------------------------------------------------------
.PHONY: clean
clean: ## Clean compiled files (Cargo 'target' folder)
	@cargo clean

.PHONY: clean-node
clean-node: ## Remove local DBs for sequencer and nodes
	@[ -d "resources/dbs/da-db" ] && rm -rf resources/dbs/da-db || true
	@[ -d "resources/dbs/sequencer-db" ] && rm -rf resources/dbs/sequencer-db || true
	@[ -d "resources/dbs/batch-prover-db" ] && rm -rf resources/dbs/batch-prover-db || true
	@[ -d "resources/dbs/light-client-prover-db" ] && rm -rf resources/dbs/light-client-prover-db || true
	@[ -d "resources/dbs/full-node-db" ] && rm -rf resources/dbs/full-node-db || true

.PHONY: clean-txs
clean-txs: ## Remove Bitcoin inscription transactions
	@[ -d "resources/bitcoin/inscription_txs" ] && rm -rf resources/bitcoin/inscription_txs/* || true

.PHONY: clean-docker
clean-docker: ## Clean Docker data for Citrea Bitcoin regtest
	rm -rf resources/dbs/citrea-bitcoin-regtest-data
	# Uncomment if you also want to remove containers/volumes:
	# docker-compose down --volumes --remove-orphans

.PHONY: clean-all
clean-all: clean clean-node clean-txs clean-docker ## Clean all cached and generated data

# ------------------------------------------------------------------------------
# 6) Test Targets
# ------------------------------------------------------------------------------
.PHONY: test-legacy
test-legacy: ## Run the test suite with full output (legacy approach)
	@cargo test -- --nocapture -Zunstable-options --report-time

# This test target uses Nextest and also includes the EF tests after downloading.
.PHONY: test
test: build-test $(EF_TESTS_DIR) ## Run tests using nextest & EF tests
	RISC0_DEV_MODE=1 cargo nextest run --workspace --all-features --no-fail-fast \
		$(filter-out $@,$(MAKECMDGOALS))

# Download and unpack Ethereum Foundation tests in $(EF_TESTS_DIR)
$(EF_TESTS_DIR):
	mkdir -p $(EF_TESTS_DIR)
	wget $(EF_TESTS_URL) -O ethereum-tests.tar.gz || { echo "Download failed"; exit 1; }
	tar -xzf ethereum-tests.tar.gz --strip-components=1 -C $(EF_TESTS_DIR)
	rm -f ethereum-tests.tar.gz

.PHONY: ef-tests
ef-tests: $(EF_TESTS_DIR) ## Run only Ethereum Foundation tests
	cargo nextest run -p citrea-evm general_state_tests

# ------------------------------------------------------------------------------
# 7) Installation (Dev Tools) Targets
# ------------------------------------------------------------------------------
.PHONY: install-dev-tools
install-dev-tools:  ## Install necessary Cargo helpers and dev environment
	cargo install --locked dprint
	cargo install cargo-llvm-cov
	cargo install cargo-hack
	cargo install --locked cargo-udeps
	cargo install flaky-finder
	cargo install --locked cargo-nextest
	$(MAKE) install-risc0
	rustup target add thumbv6m-none-eabi
	rustup component add llvm-tools-preview
	$(MAKE) install-sp1

.PHONY: install-risc0
install-risc0: ## Install RISC0 cargo tools via cargo-binstall
	cargo install --version 1.7.0 cargo-binstall
	cargo binstall --no-confirm cargo-risczero@1.1.3
	cargo risczero install --version r0.1.81.0

.PHONY: install-sp1
install-sp1: ## Install the SP1 toolchain
	curl -L https://sp1.succinct.xyz | bash
	sp1up

# ------------------------------------------------------------------------------
# 8) Lint / Format / Coverage / Docs
# ------------------------------------------------------------------------------
.PHONY: lint
lint: ## Check formatting, then run cargo check and clippy (skipping RISC0 guest code)
	dprint check
	cargo +nightly fmt --all --check
	cargo check --all-targets --all-features
	SKIP_GUEST_BUILD=1 cargo clippy --all-targets --all-features

.PHONY: lint-fix
lint-fix: ## Automatically fix formatting and clippy warnings where possible
	dprint fmt
	cargo +nightly fmt --all
	cargo fix --allow-dirty
	SKIP_GUEST_BUILD=1 cargo clippy --fix --allow-dirty

.PHONY: coverage
coverage: build-test $(EF_TESTS_DIR) ## Generate coverage in LCOV format
	cargo llvm-cov --locked --lcov --output-path lcov.info nextest --workspace --all-features

.PHONY: coverage-html
coverage-html: ## Generate coverage report in HTML format
	cargo llvm-cov --locked --all-features --html nextest --workspace --all-features

.PHONY: docs
docs:  ## Generate local documentation
	cargo doc --open

# ------------------------------------------------------------------------------
# 9) Additional Checks and Features
# ------------------------------------------------------------------------------
.PHONY: check-features
check-features: ## Check that the project compiles with all feature combinations
	cargo hack check --workspace --feature-powerset --exclude-features default --all-targets

.PHONY: check-no-std
check-no-std: ## Check that the project compiles without std
	$(MAKE) -C crates/sovereign-sdk/rollup-interface $@
	$(MAKE) -C crates/sovereign-sdk/module-system/sov-modules-core $@

.PHONY: find-unused-deps
find-unused-deps: ## Print unused dependencies (requires nightly)
	cargo +nightly udeps --all-targets --all-features

.PHONY: find-flaky-tests
find-flaky-tests:  ## Run tests repeatedly to detect flaky tests
	flaky-finder -j16 -r320 --continue "cargo test -- --nocapture"

# ------------------------------------------------------------------------------
# 10) Other Targets (Genesis, PR checks, etc.)
# ------------------------------------------------------------------------------
.PHONY: genesis
genesis: ## Generate genesis from system contract source files
	$(MAKE) -C crates/evm/src/evm/system_contracts genesis

.PHONY: genesis-prod
genesis-prod: ## Generate production genesis from system contract source files
	$(MAKE) -C crates/evm/src/evm/system_contracts genesis-prod

.PHONY: pr
pr: ## Perform basic checks before opening a Pull Request
	$(MAKE) lint
	$(MAKE) test

# This rule does nothing; it prevents errors when passing extra arguments
# via "make target ARG1 ARG2".
%:
	@:

