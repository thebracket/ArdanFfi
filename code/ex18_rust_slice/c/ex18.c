#include <stdio.h>

int sum_byte_slice(unsigned char * slice, long len);

int main() {
    unsigned char slice[4] = {1, 2, 3, 4};
    long len = 4;
    int result = sum_byte_slice(slice, len);
    printf("The sum is: %d\n", result);
    return 0;
}