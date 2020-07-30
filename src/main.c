#include <stdio.h>

#include "wrapper.h"

int main(int argc, char* argv[]) {
  for (int i=0; i < 10; i++) {
    printf("in_orgo_subset(%d): %d\n", i, in_orgo_subset(i));
  }
}
