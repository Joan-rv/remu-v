#include <stdlib.h>
int main(void) {
  char str[] = "Hello\n";
  ssize_t n = write(1, str, sizeof(str));
  if (n != sizeof(str))
    return -1;
  return 0;
}
