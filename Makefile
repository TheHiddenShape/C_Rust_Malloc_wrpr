RUST_LIB  = rust_alloc/target/release/librust_alloc.a
CC        = gcc
CFLAGS    = -Wall -Wextra -O2

.PHONY: all clean run

all: demo

$(RUST_LIB):
	cargo build --release --manifest-path rust_alloc/Cargo.toml

demo: main.c $(RUST_LIB) rust_alloc.h
	$(CC) $(CFLAGS) -o $@ main.c $(RUST_LIB) -lpthread -ldl -lm

run: demo
	./demo

clean:
	rm -f demo
	cargo clean --manifest-path rust_alloc/Cargo.toml
