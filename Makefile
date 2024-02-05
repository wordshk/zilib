all: rust_all zigen_data
rust_all:
	cargo build
lint:
	cargo clippy

target/debug/%: rust_all

zigen_data: lists/english_variants.json

lists/%.json: lists/varcon.txt.bz2 target/debug/zigen
	# Funny enough make's basename doesn't strip away the directory...
	./target/debug/zigen generate_$(shell basename $@ .json) $< $@
	# Ensure .gitignore has the file
	grep -q $@ .gitignore || echo '/$@' >> .gitignore

clean:
	cd lists && git clean -f -x -d
