typedef unsigned int size_t; // 32-bit risc-v
typedef int ssize_t;

[[noreturn]] void exit(int code);
int write(int fd, const void *buf, size_t count);
