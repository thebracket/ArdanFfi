#include "clib.h"
#include <string.h>

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

int reference_struct(struct MyStruct *s) {
    return s->integer;
}

int reference_struct_const(const struct MyStruct *s) {
    return s->integer;
}

int is_byte_twelve(const struct MyStruct* s, bool* val) {
    int errnum;
    if (!s) {
        errnum = -1;
        goto error;
    }
    *val = s->byte == 12 ? true : false;
    return 0;

error:
    return errnum;
}

struct MyStruct * factory() {
    struct MyStruct * s = (struct MyStruct *)malloc(sizeof(struct MyStruct));
    return s;
}

int string_length(const char* s) {
    return strlen(s);
}

int string_length_and_delete(char* s) {
    int len = strlen(s);
    free(s);
    return len;
}

char * return_hello() {
    char * s = (char *)malloc(6);
    strcpy(s, "Hello");
    return s;
}

int call_me(int (*maybe)(int)) {
    int sum = 0;
    for (int i = 0; i < 10; i++) {
        sum += maybe(i);
    }
    return sum;
}
