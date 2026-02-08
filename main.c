#include <stdio.h>
#include <string.h>
#include "rust_alloc.h"

int main(void)
{
    printf("=== C <-> Rust FFI demo : malloc wrapper ===\n\n");

    /* 1. Allocate a buffer for a string */
    const char *msg = "Hello from C, allocated by Rust!";
    size_t len = strlen(msg) + 1;
    uint8_t *buf = rust_malloc(len);
    if (!buf) {
        fprintf(stderr, "allocation failed\n");
        return 1;
    }
    memcpy(buf, msg, len);
    printf("buf = \"%s\"\n", (char *)buf);
    printf("live allocations: %zu\n\n", rust_alloc_count());

    /* 2. Allocate an int array */
    size_t n = 5;
    int *nums = (int *)rust_malloc(n * sizeof(int));
    for (size_t i = 0; i < n; i++)
        nums[i] = (int)(i * i);

    printf("nums = [");
    for (size_t i = 0; i < n; i++)
        printf("%d%s", nums[i], i < n - 1 ? ", " : "");
    printf("]\n");
    printf("live allocations: %zu\n\n", rust_alloc_count());

    /* 3. Free everything */
    rust_free(buf);
    rust_free((uint8_t *)nums);
    printf("live allocations after free: %zu\n", rust_alloc_count());

    /* 4. Double-free test (Rust should catch it) */
    printf("\n--- double-free test ---\n");
    rust_free(buf);

    printf("\nDone!\n");
    return 0;
}
