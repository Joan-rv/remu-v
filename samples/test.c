#include <stdlib.h>
int main(void) {
  char str[] = "Hello\n";
  write(1, str, sizeof(str));
  return 0;
}
