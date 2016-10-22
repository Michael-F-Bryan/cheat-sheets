ffi = require("ffi")

ffi.cdef[[
int is_prime(unsigned int n);
]]

rust_lib = ffi.load("./libprime.so")

n = 1234567

if rust_lib.is_prime(n) then
  print(n, "is prime")
else
  print(n, "is not prime")
end
