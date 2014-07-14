all: build run_test

# this builds the library
build:
	rustc --crate-type=lib src/structures.rs

# these tests are unit tests
run_test: trees/test_bst trees/test_llrb linear/test_linear linear/test_circular

linear/test_linear: src/linear/list.rs
	rustc --test src/linear/list.rs -o test/test_linear
	./test/test_linear

linear/test_circular: src/linear/circular.rs
	rustc --test src/linear/circular.rs -o test/test_circular
	./test/test_circular

trees/test_llrb: src/trees/llrb.rs
	rustc --test src/trees/llrb.rs -o test/test_llrb
	./test/test_llrb

trees/test_bst: src/trees/bst.rs
	rustc --test src/trees/bst.rs -o test/test_bst
	./test/test_bst

# these tests verify that it can be linked appropriately
run_link_test: build
	rustc -L . test/test_bst.rs
