
clean: clean-cargo clean-coverage

clean-cargo:
	cargo clean

clean-coverage:
	cargo +nightly cov clean

coverage:
	# clean up previous coverage result
	cargo +nightly cov clean
	# test the code
	cargo +nightly cov test
	# open the coverage report
	cargo +nightly cov report --open

test:
	cargo test

test-beta:
	cargo +beta test

test-nightly:
	cargo +nightly test

.PHONY: clean clean-cargo clean-coverage /
	coverage /
	test test-beta test-nightly