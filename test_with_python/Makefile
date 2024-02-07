all:
	cd .. && make release
	source venv/bin/activate && RUST_BACKTRACE=1 python3 wordshk_original_tests.py
	source venv/bin/activate && RUST_BACKTRACE=1 python3 tests.py

