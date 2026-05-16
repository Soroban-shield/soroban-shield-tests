.PHONY: test fuzz property integration

test: property integration

property:
	cargo test -p shield-property-tests

integration:
	cargo test -p shield-integration-suites

fuzz:
	cd harnesses && cargo fuzz run ownable -- -max_total_time=60
