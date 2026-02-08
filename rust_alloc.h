#ifndef RUST_ALLOC_H
#define RUST_ALLOC_H

#include <stddef.h>
#include <stdint.h>

/* Allocate size bytes (aligned to 8). Returns NULL on failure. */
uint8_t *rust_malloc(size_t size);

/* Free a pointer previously returned by rust_malloc. */
void rust_free(uint8_t *ptr);

/* Number of live allocations currently tracked by Rust. */
size_t rust_alloc_count(void);

#endif
