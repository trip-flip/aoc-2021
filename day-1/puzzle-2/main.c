#include <stdio.h>
#include <stdlib.h>

#define BUFSIZE 20
#define ALLOC_SIZE 2000

int sumFromPointer(int* window) {
    return *(window) + *(window + 1) + *(window + 2);
}

int main(int argc, char** argv) {
    if (argc == 1) return 1;
    int* depth_values = malloc(sizeof(int) * ALLOC_SIZE);
    int current_value, prev_value, difference;
    FILE* input_file = fopen(argv[1], "r");
    int increase_count = 0;
    int dvalue_count = 0;
    char buffer[BUFSIZE];
    int i, k;

    // Fill depth_values and dvalue_count
    for (i = 0; fgets(buffer, BUFSIZE, input_file) != NULL; ++i) {
        for (k = 0; k < BUFSIZE; ++k) {
            if (buffer[k] == '\n') {
                buffer[k] = '\0';
                break;
            }
        }
        if (i >= ALLOC_SIZE) {
            depth_values = realloc(depth_values, sizeof(int) * (ALLOC_SIZE + 100));
        }
        depth_values[i] = atoi(buffer);
        ++dvalue_count;
    }

    // Begin printing results
    prev_value = sumFromPointer(depth_values);
    printf("%d (N/A - No previous value)\n", prev_value);
    for (i = 1; i < dvalue_count - 2; ++i) {
        current_value = sumFromPointer(&depth_values[i]);
        difference = current_value - prev_value;
        if (difference < 0) {
            printf("%d (decreased)\n", current_value);
        } else if (difference > 0) {
            printf("%d (increased)\n", current_value);
            ++increase_count;
        } else {
            printf("%d (no change)\n", current_value);
        }
        prev_value = current_value;
    }

    // Answer
    printf("%d measurements are larger than its previous\n", increase_count);
    return 0;
}