#pragma once

struct MyStruct {
    int integer;
    char byte;
};

char double_byte(char n);
int copy_struct(struct MyStruct s);
struct MyStruct return_struct(int n, char c);
