#include <stdio.h>

extern int is_prime(unsigned int n);

int main(int argc, char const *argv[]) {
  unsigned int n = 1234567;

  printf("%d %s prime\n", n, is_prime(n) ? "is" : "is not");
  return 0;
}
