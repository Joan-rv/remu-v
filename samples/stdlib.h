typedef unsigned int size_t; // 32-bit risc-v

[[noreturn]] void exit(int code);
void write(int fd, const void *buf, size_t count);
