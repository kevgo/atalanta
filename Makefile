# dev tooling and versions
RUN_THAT_APP_VERSION = 0.11.0

build:  # builds the codebase
	cargo build

cuke: build  # runs all end-to-end tests
	rm -rf tmp
	cargo test --test=cucumber

cukethis: build  # runs only end-to-end tests with a @this tag
	rm -rf tmp
	cargo test --test cucumber -- -t @this

fix: tools/rta@${RUN_THAT_APP_VERSION}  # applies all auto-fixers
	cargo +nightly fix --allow-dirty
	cargo clippy --fix --allow-dirty
	cargo +nightly fmt
	tools/rta dprint fmt

help:  # shows all available Make commands
	cat Makefile | grep '^[^ ]*:' | grep -v '.SILENT:' | grep -v help | sed 's/:.*#/#/' | column -s "#" -t

install:  # installs the binary on the current machine
	cargo install --path .

lint: tools/rta@${RUN_THAT_APP_VERSION}  # finds code smells
	git diff --check
	tools/rta dprint check
	cargo clippy --all-targets --all-features -- --deny=warnings
	tools/rta actionlint
	cargo machete

run:  # runs in the local directory
	cargo run --quiet

setup:  # install development dependencies on this computer
	rustup toolchain add nightly
	rustup component add rustfmt --toolchain nightly
	cargo install cargo-machete --locked

test: unit cuke lint  # run all tests

unit:  # runs the unit tests
	cargo test

update: tools/rta@${RUN_THAT_APP_VERSION}  # updates all dependencies
	cargo install cargo-edit
	cargo upgrade --incompatible
	tools/rta --update

# --- HELPER TARGETS --------------------------------------------------------------------------------------------------------------------------------

tools/rta@${RUN_THAT_APP_VERSION}:
	@rm -f tools/rta* tools/rta
	@(cd tools && curl https://raw.githubusercontent.com/kevgo/run-that-app/main/download.sh | sh)
	@mv tools/rta tools/rta@${RUN_THAT_APP_VERSION}
	@ln -s rta@${RUN_THAT_APP_VERSION} tools/rta

.SILENT:
.DEFAULT_GOAL := help
