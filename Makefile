# dev tooling and versions
RUN_THAT_APP_VERSION = 0.1.1

build:  # builds the test binary
	cargo build

cuke: build  # runs all end-to-end tests
	rm -rf tmp
	cargo test --test=cucumber

cukethis: build  # runs only end-to-end tests with a @this tag
	rm -rf tmp
	cargo test --test cucumber -- -t @this

fix: tools/run-that-app@${RUN_THAT_APP_VERSION}  # applies all auto-fixers
	tools/rta dprint fmt

help:  # shows all available Make commands
	cat Makefile | grep '^[^ ]*:' | grep -v '.SILENT:' | grep -v help | sed 's/:.*#/#/' | column -s "#" -t

install:  # installs the binary on the current machine
	cargo install --path .

lint:  # finds code smells
	cargo clippy --all-targets --all-features -- --deny=warnings

run:  # runs in the local directory
	cargo run --quiet

test: unit cuke lint  # run all tests

unit:  # runs the unit tests
	cargo test

update:  # updates all dependencies
	cargo upgrade    # install cargo-edit if this doesn't work

# --- HELPER TARGETS --------------------------------------------------------------------------------------------------------------------------------

tools/run-that-app@${RUN_THAT_APP_VERSION}:
	@rm -f tools/run-that-app* tools/rta
	@(cd tools && curl https://raw.githubusercontent.com/kevgo/run-that-app/main/download.sh | sh)
	@mv tools/run-that-app tools/run-that-app@${RUN_THAT_APP_VERSION}
	@ln -s run-that-app@${RUN_THAT_APP_VERSION} tools/rta

.SILENT:
.DEFAULT_GOAL := help
