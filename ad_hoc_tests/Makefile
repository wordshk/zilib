all: test expensive_test
bins:
	cd .. && cargo build --bins
test: bins
	bash test_binary_search_fast.sh
expensive_test: test
	bash test_binary_search.sh
