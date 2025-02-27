#pragma once

#include <stdbool.h>

struct MyStruct {
    int integer;
    char byte;
};

char double_byte(char n);
int copy_struct(struct MyStruct s);
struct MyStruct return_struct(int n, char c);
int reference_struct(struct MyStruct *s);
int reference_struct_const(const struct MyStruct *s);

int is_byte_twelve(const struct MyStruct* s, bool* val);
