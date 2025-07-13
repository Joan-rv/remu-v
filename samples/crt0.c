#include <stdlib.h>
int main(void);
asm(".global _start;"
    "li sp, 0x400;"
    "j _start2;");
void _start2(void) { exit(main()); }
