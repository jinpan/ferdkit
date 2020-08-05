#include <stdio.h>

int in_orgo_subset(int atomic_num);

int main(void) {
    for (int i=1; i < 10; i++) {
        printf("inOrganicSubset(%d): %d\n", i, in_orgo_subset(i));
    }

    return 0;
}