cuke:  # runs all end-to-end tests
	rm -rf tmp
	cargo test --test=cucumber

cukethis:  # runs only end-to-end tests with a @this tag
	rm -rf tmp
	cargo test --test cucumber -- -t @this

help:  # shows all available Make commands
	cat Makefile | grep '^[^ ]*:' | grep -v '.SILENT:' | grep -v help | sed 's/:.*#/#/' | column -s "#" -t


.SILENT:
.DEFAULT_GOAL := help
