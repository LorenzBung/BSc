/* intArray1.c */
#include <stdlib.h>
int main(char* argv[], int argc) {
  int* intArray = malloc(100 * sizeof(int));
  intArray[100] = 0;
  free(intArray);
}
