all: zigen_data rust_all
rust_all: zigen_data lists/wordslist.csv
	cargo build --lib
lint:
	cargo clippy

target/release/%: src/*.rs src/bin/*.rs lists/wordslist.csv
	cargo build --release --bins --no-default-features --features downloaded_data

zigen_data: lists/Unihan.zip lists/CJKRadicals.txt lists/english_variants.json lists/wordshk_charset.json lists/wordshk_variantmap.json lists/wordshk_autoconvert.json

lists/wordslist.csv:
	cd lists && curl -O https://words.hk/faiman/analysis/wordslist.csv
lists/Unihan.zip:
	cd lists && curl -O https://www.unicode.org/Public/UCD/latest/ucd/Unihan.zip && unzip Unihan.zip
lists/CJKRadicals.txt:
	cd lists && curl -O https://www.unicode.org/Public/UCD/latest/ucd/CJKRadicals.txt

lists/%.json: target/release/zigen
	# Funny enough make's basename doesn't strip away the directory...
	./target/release/zigen generate_$(shell basename $@ .json) $@

clean:
	cd lists && git clean -f -x -d

release:
	cargo build --lib --release

test:
	cd ad_hoc_tests/ && make test

expensive_tests:
	cd ad_hoc_tests/ && make expensive_tests
