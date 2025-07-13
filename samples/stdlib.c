#include <stdlib.h>
void exit(int code) {
  register int a0 asm("a0") = code;
  asm("li a7, 93;"
      "ecall"
      :
      : "r"(a0)
      : "a7");
  for (;;) // in case syscall fails
    ;
}

void write(int fd, const void *buf, size_t count) {
  register int a0 asm("a0") = fd;
  register const void *a1 asm("a1") = buf;
  register size_t a2 asm("a2") = count;
  asm("li a7, 64;"
      "ecall;"
      :
      : "r"(a0), "r"(a1), "r"(a2)
      : "a7");
}
