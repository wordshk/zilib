all:
	./copy_packaged_lists
	maturin build --sdist --release

update_definitions:
	python copy_function_definitions.py ../

test:
	maturin develop && python -m pytest .
