/* free.c */
#include <stdlib.h>
int main(char* argv[], int argc) {
  char* testString = malloc(10 * sizeof(char));
  free(testString[10]);
  free(testString);
}
