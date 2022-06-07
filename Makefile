build:  # builds the test binary
	cargo build

cuke: build  # runs all end-to-end tests
	rm -rf tmp
	cargo test --test=cucumber

cukethis: build  # runs only end-to-end tests with a @this tag
	rm -rf tmp
	cargo test --test cucumber -- -t @this

help:  # shows all available Make commands
	cat Makefile | grep '^[^ ]*:' | grep -v '.SILENT:' | grep -v help | sed 's/:.*#/#/' | column -s "#" -t

install:  # installs the binary on the current machine
	cargo install --path .

run:  # runs in the local directory
	cargo run --quiet

test: unit cuke  # run all tests

unit:  # runs the unit tests
	cargo test


.SILENT:
.DEFAULT_GOAL := help
