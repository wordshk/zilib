all: update_definitions
	./copy_packaged_lists
	maturin build --sdist --release

update_definitions:
	python copy_function_definitions.py ../

test: update_definitions
	maturin develop && python -m pytest .
clean:
	git clean -x -f -d
