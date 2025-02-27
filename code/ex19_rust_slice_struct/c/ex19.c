#include <stdio.h>
#include <stdint.h>

struct MyData {
    int32_t a;
    int16_t b;
    int8_t c;
};

void print_slice_of_mydata(struct MyData* data, size_t len);
void print_slice_nicely(struct MyData* data, size_t num_elems);

int main() {
    struct MyData data[] = {
        {1, 2, 3},
        {4, 5, 6},
        {7, 8, 9}
    };
    printf("Raw byte slice:\n");
    print_slice_of_mydata(data, sizeof(data));
    printf("\n");
    
    printf("Nicely formatted slice:\n");
    print_slice_nicely(data, sizeof(data) / sizeof(data[0]));

    return 0;
}