.PHONY: prepare
prepare:
	cargo run -p xtask

.PHONY: web
web: prepare
	wasm-pack build apps/web/index --target web --out-dir ../../../html/wasm/index
	wasm-pack build apps/web/check --target web --out-dir ../../../html/wasm/check
	wasm-pack build apps/web/eval --target web --out-dir ../../../html/wasm/eval

.PHONY: test 
test: prepare
	cargo test --all --no-fail-fast 

.PHONY: test-fast
test-fast: prepare
	cargo test --all --no-fail-fast --exclude e2e_tests
	cargo test -p e2e_tests -- --exclude-latex

.PHONY: check
check: prepare
	cargo fmt --all
	cargo clippy --all -- -D warnings

.PHONY: clean
clean:
	find -name "*.aux" -delete
	find -name "*.log" -delete

