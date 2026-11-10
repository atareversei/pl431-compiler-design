#ifndef ri_CLOX_CHUNK_H
#define ri_CLOX_CHUNK_H

#include "common.h"

typedef enum {
  OP_RETURN,
} OpCode;

typedef struct {
  int len;
  int cap;
  uint8_t *code;
} Chunk;

void init_chunk(Chunk *chunk);
void free_chunk(Chunk *chunk);
void write_chunk(Chunk *chunk, uint8_t byte);

#endif
