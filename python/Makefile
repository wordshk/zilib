all:
	maturin build --sdist --release

update_definitions:
	python copy_function_definitions.py ../

test:
	pytest .
