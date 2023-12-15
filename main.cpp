#include <cstdio>
#include "rust-borrows-c-string/rust-borrows-c-string.h"


void f(void) {
    std::string s = "hello world";
    printf("hello from C++: [%s]\n", s.c_str());
    rust_borrows_c_string::print_string(s.c_str());
    printf("all done\n");
    s.append(" and now it's all over");
    printf("%s\n", s.c_str());
}


int main(int argc, char *argv[]) {
    uint32_t a = 11;
    uint32_t b = 22;
    uint32_t result;
    result = rust_borrows_c_string::add(a, b);
    printf("%u + %u = %u\n", a, b, result);

    f();

    printf("finished\n");
    return 0;
}
