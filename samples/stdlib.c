#include <stdlib.h>
void exit(int code) {
  asm("mv a0, %0;"
      "li a7, 93;"
      "ecall"
      :
      : "r"(code)
      : "a0", "a7");
}

void write(int fd, const void *buf, size_t count) {
  asm("mv a0, %0;"
      "mv a1, %1;"
      "mv a2, %2;"
      "li a7, 64;"
      "ecall;"
      :
      : "r"(fd), "r"(buf), "r"(count)
      : "a0", "a1", "a2", "a7");
}
