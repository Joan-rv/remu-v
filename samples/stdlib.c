#include <stdlib.h>
void exit(int code) {
  asm("mv a0, %0;"
      "li a7, 93;"
      "ecall"
      :
      : "r"(code)
      : "a0", "a7");
}
