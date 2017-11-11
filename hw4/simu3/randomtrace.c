#include <stdio.h>
#include <stdlib.h>
#include <time.h>

int main(int argc, char* argv[]) {
  srand(time(NULL));

  if (argc != 3) {
    printf("Usage: ./randomtrace <count> <size>\n");
    return -1;
  }

  int i, r, s, t;
  s = atoi(argv[1]);
  t = atoi(argv[2]);
  for (i = 0; i < s; i++) {
    r = (int) ((rand() % t) + 1);
    printf("%d ", r);
  }
  printf("\n");
}
