.PHONY: prepare
	cargo run -p xtask

.PHONY: html_templates
html_templates:
	./html/generate_pages.sh

.PHONY: web
web: prepare html_templates
	wasm-pack build apps/web --target web --out-dir ../../html/pkg

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

