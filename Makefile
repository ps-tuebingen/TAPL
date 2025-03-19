.PHONY: nameless
nameless:
	cargo run nameless-representation -v -f examples/untyped_lambda/$(name)/$(name).lam
.PHONY: unt-lambda
unt-lambda:
	cargo run untyped-lambda -v -f examples/untyped_lambda/$(name)/$(name).lam

.PHONY: test 
test:
	cargo test --all --no-fail-fast 

.PHONY: check
check:
	cargo fmt --all -- --check
	cargo clippy

.PHONY: coverage
coverage:
	@echo "Make sure to install via cargo install cargo-llvm-cov first"
	cargo llvm-cov --workspace --html
	cargo llvm-cov --workspace --open
