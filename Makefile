RUN_THAT_APP_VERSION = 0.37.0

RTA        = tools/rta@$(RUN_THAT_APP_VERSION)
ACTIONLINT = $(RTA) actionlint
RUMDL      = $(RTA) rumdl
DPRINT     = $(RTA) dprint

build:  # builds the codebase
	cargo build

cuke: build  # runs all end-to-end tests
	rm -rf tmp
	cargo test --test=cucumber

cukethis: build  # runs only end-to-end tests with a @this tag
	rm -rf tmp
	cargo test --test cucumber -- -t @this

fix: ${RTA}  # applies all auto-fixers
	cargo +nightly fix --allow-dirty
	cargo clippy --fix --allow-dirty
	cargo +nightly fmt
	$(DPRINT) fmt
	$(RUMDL) fmt

help:  # shows all available Make commands
	cat Makefile | grep '^[^ ]*:' | grep -v '.SILENT:' | grep -v help | sed 's/:.*#/#/' | column -s "#" -t

install:  # installs the binary on the current machine
	cargo install --path .

lint: ${RTA}  # finds code smells
	git diff --check
	$(DPRINT) check
	$(RUMDL) check
	cargo clippy --all-targets --all-features -- --deny=warnings
	$(ACTIONLINT)
	cargo machete

run:  # runs in the local directory
	cargo run --quiet

setup:  # install development dependencies on this computer
	rustup component add clippy
	rustup toolchain add nightly
	rustup component add rustfmt --toolchain nightly
	cargo install cargo-machete --locked

test: unit cuke lint  # run all tests

unit:  # runs the unit tests
	cargo test

update: ${RTA}  # updates all dependencies
	cargo install cargo-edit
	cargo upgrade --incompatible
	$(RTA) --update

# --- HELPER TARGETS --------------------------------------------------------------------------------------------------------------------------------

${RTA}:
	@rm -f tools/rta*
	@(cd tools && curl https://raw.githubusercontent.com/kevgo/run-that-app/main/download.sh | sh -s -- --version ${RUN_THAT_APP_VERSION} --name rta@${RUN_THAT_APP_VERSION})
	@ln -s rta@$(RUN_THAT_APP_VERSION) tools/rta

.SILENT:
.DEFAULT_GOAL := help
