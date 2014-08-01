SHELL               = bash

RUSTC               = rustc
CARGO               = cargo
RUSTDOC             = rustdoc

SRC_DIR             = src
LIB_FILE            = $(SRC_DIR)/trig.rs
TEST_FILE           = $(SRC_DIR)/trig.rs

CRATE_NAME          = $(shell $(RUSTC) --crate-name $(LIB_FILE))
CRATE_FILES         = $(shell $(RUSTC) --crate-file-name $(LIB_FILE))

LIB_DIR             = target
TEST_DIR            = $(LIB_DIR)/test

DOC_DIR             = doc
DOC_TEST_PARAMS     = -L $(LIB_DIR) --test

.PHONY: all lib test bench doc clean help

all: lib doc

clean:
	@echo "-- Removing generated files:"
	rm -rf $(LIB_DIR)
	rm -rf $(DOC_DIR)

help:
	@echo "-- Available Options:"
	@echo "make             - Build the library & documentation."
	@echo "make lib         - Build the library."
	@echo "make test        - Run the unit tests."
	@echo "make bench       - Run benchmarks."
	@echo "make doc         - Builds the library's documentation."
	@echo "make doctest     - Runs the examples in the documentation."
	@echo "make clean       - Removes all generated files."

# Library

lib: $(LIB_FILE)
	@cargo build

# Testing and Benchmarking

test:
	@cargo test

bench:
	@cargo test --bench

# Documentation

doc:
	@echo "-- Generating documentation."
	@mkdir -p $(DOC_DIR)
	@$(RUSTDOC) -o $(DOC_DIR) $(LIB_FILE)

doctest: lib
	@echo "-- Running documentation examples:"
	@mkdir -p $(DOC_DIR)
	@$(RUSTDOC) $(DOC_PARAMS) $(DOC_TEST_PARAMS) -o $(DOC_DIR) $(LIB_FILE)
