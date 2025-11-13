.PHONY: prepare
prepare:
	cargo run -p xtask

.PHONY: web
web: prepare
	wasm-pack build apps/web/index --target web --out-dir ../../../web/wasm/index
	wasm-pack build apps/web/check --target web --out-dir ../../../web/wasm/check
	wasm-pack build apps/web/eval --target web --out-dir ../../../web/wasm/eval

.PHONY: test 
test: prepare
	cargo test --all --no-fail-fast 

.PHONY: test-fast
test-fast: prepare
	cargo test --all --no-fail-fast --exclude end-to-end
	cargo test -p end-to-end -- --exclude-latex

.PHONY: check
check: prepare
	cargo fmt --all
	cargo clippy --all -- -D warnings

.PHONY: check-more
check-more:
	cargo clippy --all-targets --all-features -- -W clippy::all -W clippy::pedantic -W clippy::nursery -A clippy::used_underscore_binding

.PHONY: clean
clean:
	find -name "*.aux" -delete
	find -name "*.log" -delete

