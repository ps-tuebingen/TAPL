.PHONY: web
web:
	wasm-pack build crates/web_bin --target web --out-dir ../../html/pkg

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
