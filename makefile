all: build run_test

# this builds the library
build:
	mkdir -p target/make
	rustc --crate-type=lib src/structures.rs -o target/make/libstructures.rlib

# removes built files
clean:
	rm -r target/make

# these tests are unit tests
run_test: trees/test_bst trees/test_llrb linear/test_linear linear/test_circular

linear/test_linear: src/linear/list.rs
	rustc --test src/linear/list.rs -o target/make/test_linear
	./target/make/test_linear

linear/test_circular: src/linear/circular.rs
	rustc --test src/linear/circular.rs -o target/make/test_circular
	./target/make/test_circular

trees/test_llrb: src/trees/llrb.rs
	rustc --test src/trees/llrb.rs -o target/make/test_llrb
	./target/make/test_llrb

trees/test_bst: src/trees/bst.rs
	rustc --test src/trees/bst.rs -o target/make/test_bst
	./target/make/test_bst

# these tests verify that it can be linked appropriately
run_link_test: build
	rustc -L . src/test/test_bst.rs
