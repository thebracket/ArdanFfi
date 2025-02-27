#pragma once

#include <stdbool.h>

#define MY_CLIB_VERSION 1
#define MY_CLIB_VERSION_STRING "1.0"

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

struct MyStruct * factory();

int string_length(const char* s);
int string_length_and_delete(char* s);
char * return_hello();

int call_me(int (*maybe)(int));

#define CALLBACK int (*callback)(int)
int call_me_with_callback(CALLBACK);

#define SQUARE(x) ((x) * (x))