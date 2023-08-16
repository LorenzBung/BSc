/* intArray2.c */
#include <stdlib.h>
#include <stdio.h>
int main(char* argv[], int argc) {
  int* intArray = malloc(100 * sizeof(int));
  free(intArray);
  printf("%d", intArray[10]);
}
