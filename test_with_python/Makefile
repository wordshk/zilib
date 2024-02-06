all:
	cd .. && make release
	source venv/bin/activate && RUST_BACKTRACE=1 python3 test_stemmer.py
