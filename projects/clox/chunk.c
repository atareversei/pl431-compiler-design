#include "chunk.h"
#include "memory.h"
#include <stdint.h>
#include <stdlib.h>

void init_chunk(Chunk *chunk) {
  chunk->len = 0;
  chunk->cap = 0;
  chunk->code = NULL;
}

void free_chunk(Chunk *chunk) {
  FREE_ARRAY(uint8_t, chunk->code, chunk->cap);
  init_chunk(chunk);
}

void write_chunk(Chunk *chunk, uint8_t byte) {
  if (chunk->cap < chunk->len + 1) {
    int old_cap = chunk->cap;
    chunk->cap = GROW_CAPACITY(old_cap);
    chunk->code = GROW_ARRAY(uint8_t, chunk->code, old_cap, chunk->cap);
  }
  chunk->code[chunk->len] = byte;
  chunk->len++;
}
