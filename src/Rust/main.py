import ctypes

primes = ctypes.CDLL('./libprime.so')

n = 1234567

if primes.is_prime(n):
    print(n, "is prime")
else:
    print(n, "is not a prime")
