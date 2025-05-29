.PHONY: examples
examples:
	./load_examples.sh

.PHONY: web
web: examples
	wasm-pack build crates/web_bin --target web --out-dir ../../html/pkg

.PHONY: test 
test: examples
	cargo test --all --no-fail-fast 

.PHONY: check
check: examples
	cargo fmt --all -- --check
	cargo clippy --all -- -D warnings
