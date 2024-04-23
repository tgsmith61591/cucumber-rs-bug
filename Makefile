SHELL ?= /bin/bash
CONCURRENCY ?= 32

.PHONY: test-distributed
test-distributed:
	cargo test --test example -- -c $(CONCURRENCY) --feature-dir "tests/features/distributed"

.PHONY: test-monolith
test-monolith:
	cargo test --test example -- -c $(CONCURRENCY) --feature-dir "tests/features/monolith"
