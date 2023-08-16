#include <stdlib.h>
#include <stdio.h>
int main(char* argv[], int argc)
{
  int i, j;
  int* iPointer;

  i = 10;
  iPointer = &i;
  iPointer = NULL;
  j = *iPointer;
}
