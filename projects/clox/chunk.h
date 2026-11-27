#ifndef ri_CLOX_CHUNK_H
#define ri_CLOX_CHUNK_H

#include "common.h"
#include "value.h"

typedef enum {
  OP_CONSTANT,
  OP_RETURN,
} OpCode;

typedef struct {
  int len;
  int cap;
  uint8_t *code;
  int *lines;
  ValueArray constants;
} Chunk;

void chunk_init(Chunk *chunk);
void chunk_free(Chunk *chunk);
void chunk_write(Chunk *chunk, uint8_t byte, int line);
int chunk_add_constant(Chunk *chunk, Value value);

#endif
