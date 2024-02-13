all: zigen_data rust_all
rust_all: zigen_data
	cargo build
lint:
	cargo clippy

target/debug/%: lists/wordslist.csv
	cargo build --bins --no-default-features

zigen_data: lists/Unihan.zip lists/CJKRadicals.txt lists/english_variants.json lists/wordshk_charset.json lists/wordshk_variantmap.json

lists/wordslist.csv:
	cd lists && curl -O https://words.hk/faiman/analysis/wordslist.csv
lists/Unihan.zip:
	cd lists && curl -O https://www.unicode.org/Public/UCD/latest/ucd/Unihan.zip && unzip Unihan.zip
lists/CJKRadicals.txt:
	cd lists && curl -O https://www.unicode.org/Public/UCD/latest/ucd/CJKRadicals.txt

lists/%.json: lists/varcon.txt.bz2 target/debug/zigen
	# Funny enough make's basename doesn't strip away the directory...
	./target/debug/zigen generate_$(shell basename $@ .json) $@
	# Ensure .gitignore has the file
	grep -q $@ .gitignore || echo '/$@' >> .gitignore

clean:
	cd lists && git clean -f -x -d

release:
	cargo build --release

test:
	cd ad_hoc_tests/ && make test

expensive_tests:
	cd ad_hoc_tests/ && make expensive_tests
