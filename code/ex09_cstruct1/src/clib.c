#include "clib.h"

char double_byte(char n) {
    return n * 2;
}

int copy_struct(struct MyStruct s) {
    return s.integer;
}

struct MyStruct return_struct(int n, char c) {
    struct MyStruct s = { n, c };
    return s;
}
