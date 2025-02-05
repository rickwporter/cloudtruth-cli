#
# Copyright (C) 2021 CloudTruth, Inc.
#

os_name := $(shell uname -s)
rustup_exists := $(shell which rustup)
rust_intended := 1.63.0
rust_installed := $(shell rustc -V | cut -d' ' -f2)
rust_bad_version := $(shell grep "RUST_VERSION:" .github/workflows/*.yml | grep -v "$(rust_intended)")
openapi_gen_version := v5.3.1
build_dir := build
test_dir := integration-tests
# convenience for looping
subdirs := $(build_dir)
subdirs += $(test_dir)

.DEFAULT = all
.PHONY = all
.PHONY += cargo
.PHONY += clean
.PHONY += cli
.PHONY += help
.PHONY += image
.PHONY += integration
.PHONY += lint
.PHONY += lint_local
.PHONY += precommit
.PHONY += prerequisites
.PHONY += shell
.PHONY += subdir_action
.PHONY += subdir_lint
.PHONY += subdir_precommit
.PHONY += subdir_prereq
.PHONY += targets
.PHONY += test
.PHONY += test_prerequisites
.PHONY += version_check

all: precommit

### Commands for outside the container

image:
	docker build --build-arg user_uid=$(shell id -u) --build-arg user_gid=$(shell id -g) -t cloudtruth/cli . -f Dockerfile.dev

shell:
	docker run --rm --privileged=true \
		--group-add $(shell stat -c '%g' /var/run/docker.sock) \
		-v $(PWD):/home/dev/cli \
		-v $(HOME)/.cargo:/home/dev/.cargo \
		-v /var/run/docker.sock:/var/run/docker.sock \
		-it cloudtruth/cli

### Commands for either outside or inside the container

# the client must be generated before building the Rust program that uses it
cargo cli: client
	cargo build

clean:
	rm -rf target/
	rm -rf client/target/

# client needs to re-generated when the openapi.yaml changes
client: openapi.yml patch_client.py
	docker run --rm \
		-v "$(shell pwd):/local" \
		--user "$(shell id -u):$(shell id -g)" \
		openapitools/openapi-generator-cli:$(openapi_gen_version) generate \
		-i /local/openapi.yml \
		-g rust \
		-o /local/client \
		--additional-properties=packageName=cloudtruth-restapi,packageVersion=1.0.0,supportAsync=false,enumUnknownDefaultCase=true \
		> generator.log
	python3 patch_client.py
	cd client && cargo fmt && cargo build -r

lint_local:
	cargo fmt --all -- --check
	cargo clippy --all-features -- -D warnings
	shellcheck install.sh scripts/*

lint: lint_local subdir_lint

subdir_action:
	@for sd in $(subdirs) ; do \
  		echo "Performing $(SUBDIR_ACTION) in $$sd directory" && make -C $$sd $(SUBDIR_ACTION) || exit 1; \
  	done

subdir_lint:
	make subdir_action SUBDIR_ACTION=lint

subdir_precommit:
	make subdir_action SUBDIR_ACTION=precommit

subdir_prereq:
	make subdir_action SUBDIR_ACTION=prerequisites

precommit: version_check cargo test subdir_precommit

prerequisites: subdir_prereq
ifeq ($(rustup_exists),'')
	curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
endif
ifeq ($(rustup_exists),'')
	$(error "You need to add ~/.cargo/bin to your PATH")
endif
ifneq ($(rust_intended),$(rust_installed))
	rustup install $(rust_intended)
	rustup override set $(rust_intended)
else
	@echo "Already running rustc version: $(rust_intended)"
endif
ifeq ($(os_name),Darwin)
	brew install shellcheck;
else ifeq ($(os_name),Linux)
	sudo apt-get install shellcheck pkg-config;
else
	@echo "Did not install shellcheck"
endif

# This target is used by workflows before running integration tests
test_prerequisites:
	make -C $(test_dir) prerequisites

test:
	cargo test

integration: cargo
	make -C $(test_dir) $@

version_check:
ifneq ($(rust_intended),$(rust_installed))
	$(error "Rustc compiler version expected $(rust_intended), got $(rust_installed)")
endif
ifneq ($(rust_bad_version),)
	$(error "GitHub action uses bad rustc version: $(rust_bad_version)")
endif
	@echo "Using rustc version: $(rust_intended)"

regen: cargo
	make -C $(build_dir) $@

help: targets

targets:
	@echo ""
	@echo "cargo          - builds rust target"
	@echo "clean          - clean out build targets"
	@echo "client         - generate and build the cloudtruth-restapi library"
	@echo "image          - make the cloudtruth/cli docker container for development"
	@echo "integration    - runs the integration test against the live server"
	@echo "lint           - checks for formatting issues"
	@echo "precommit      - build rust targets, tests, and lints the files"
	@echo "prerequisites  - install prerequisites"
	@echo "regen          - regenerate non-build artifacts"
	@echo "shell          - drop into the cloudtruth/cli docker container for development"
	@echo "test           - runs the cargo tests"
	@echo "test_prerequisites - install prerequisites for running integration tests"
	@echo "version_check  - checks rustc versions"
	@echo ""
