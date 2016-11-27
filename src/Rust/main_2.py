from cffi import FFI

ffi = FFI()

# Make a header entry for the Rust function
ffi.cdef("bool is_prime(int n);")
lib = ffi.dlopen("./libprime.so")

n = 1234567

if lib.is_prime(n):
    print(n, "is prime")
else:
    print(n, "is not a prime")

# trying to use an invalid datatype should fail
lib.is_prime("foo")
