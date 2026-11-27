#ifndef ri_VALUE_H
#define ri_VALUE_H

#include "common.h"

typedef double Value;

typedef struct {
  int cap;
  int len;
  Value *values;
} ValueArray;

void value_init_array(ValueArray *array);
void value_write_array(ValueArray *array, Value value);
void value_free_array(ValueArray *array);
void value_print(Value value);

#endif
