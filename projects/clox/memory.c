#include "memory.h"
#include <stdlib.h>

// `reallocate` is the only place that manages the dynamic memory for all
// use cases.
//                          old_size        new_size
// Allocating new block:    0               Non-zero
// Free Allocation:         Non-zero        0
// Shrink:                  Non-zero        Smaller than old_size
// Grow:                    Non-zero        Larger than old_size
void *reallocate(void *pointer, size_t old_size, size_t new_size) {
  if (new_size == 0) {
    free(pointer);
    return NULL;
  }

  void *result = realloc(pointer, new_size);
  if (result == NULL) {
    exit(1);
  }
  return result;
}
