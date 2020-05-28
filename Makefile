.PHONY: test
test:
	cargo test

.PHONY: build
build:
	cargo build

.PHONY: docs
docs: docs/index.html

check:
	@mkdir -p target/docs
	@(which -s pandoc) || (echo "Need pandoc(1) installed"; exit 1)
	@(which -s ruby) || (echo "Need ruby(1) installed"; exit 1)
	@(which -s md-toc) || (echo "Need md-toc(1): cargo install markdown-toc"; exit 1)

docs/index.html: check target/docs/toc.html docs/manual.tpl docs/MANUAL.md
	@echo "building docs/index.html..."
	@pandoc docs/MANUAL.md -o target/docs/manual.html
	@ruby -e 'File.open("docs/index.html", "w") { |f| f.write(File.read("docs/manual.tpl").sub("{body}", File.read("target/docs/manual.html")).sub("{toc}", File.read("target/docs/toc.html"))) }'
	@echo "DONE"

target/docs/toc.html: check docs/MANUAL.md docs/manual.tpl
	@echo "building target/docs/toc.html..."
	@md-toc --min-depth 1 --max-depth 2 --bullet - docs/MANUAL.md | grep -v 'Table of Contents' > target/docs/toc.md
	@pandoc target/docs/toc.md -o target/docs/toc.html
	@echo "DONE"

clean:
	-cargo clean