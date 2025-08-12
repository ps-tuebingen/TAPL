.PHONY: examples
examples:
	./load_examples.sh

.PHONY: html_templates
html_templates:
	./html/generate_pages.sh

.PHONY: web
web: examples html_templates
	wasm-pack build apps/web --target web --out-dir ../../html/pkg

.PHONY: test 
test: examples
	cargo test --all --no-fail-fast 

.PHONY: test-fast
test-fast: examples
	cargo test --all --no-fail-fast --exclude e2e_tests
	cargo test -p e2e_tests -- --exclude-latex

.PHONY: check
check: examples
	cargo fmt --all
	cargo clippy --all -- -D warnings

.PHONY: clean
clean:
	find -name "*.aux" -delete
	find -name "*.log" -delete

