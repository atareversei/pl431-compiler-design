# `malloc`

```c
#include <stdio.h>

int main() {
  char name[32]; // static memory allocation
  scanf("%s", &name);

  return 0;
}
```

Buw with dynamic memory allocation:

```c
#include <stdio.h>
#include <stdlib.c>

int main() {
  char *name;
  name = malloc(32);
  scanf("%s", name);

  free(name);
  return 0;
}
```
