.PHONY: test 
test:
	cargo test --all --no-fail-fast 

.PHONY: check
check:
	cargo fmt --all -- --check
	cargo clippy --all -- -D warnings

.PHONY: coverage
coverage:
	@echo "Make sure to install via cargo install cargo-llvm-cov first"
	cargo llvm-cov --workspace --html
	cargo llvm-cov --workspace --open

.PHON: done
done:
	cargo build -p common
	cargo build -p untyped_arithmetic
	cargo build -p e2e_untyped_arithmetic
	cargo build -p untyped_lambda
	cargo build -p e2e_untyped_lambda
	cargo build -p typed_arithmetic
	cargo build -p e2e_typed_arithmetic
	cargo build -p stlc 
	cargo build -p e2e_stlc
	cargo build -p references
	cargo build -p e2e_references
	cargo build -p exceptions
	cargo build -p e2e_exceptions
	cargo build -p subtypes
	cargo build -p e2e_subtypes
	cargo build -p recursive
	cargo build -p e2e_recursive
	cargo build -p system_f
	cargo build -p e2e_systemf 
	cargo build -p existential
	cargo build -p e2e_existential
	cargo build -p bounded_quantification
	cargo build -p e2e_bounded
	cargo build -p lambda_omega
	cargo build -p e2e_lambdaomega
	cargo build -p system_f
	cargo build -p e2e_systemf
