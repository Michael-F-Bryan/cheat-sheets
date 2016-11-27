# Using Rust outside Rust

Say you've got a bunch of really fast Rust code (remember, Rust is about the
same as C, sometimes faster) and want to call it from a slower language like
Python or Ruby, it's actually pretty easy to do. If you can run C code, you
can also run Rust.

## Quick Links

For those too lazy to scroll:

* [Original Function](Rust/rust_interop.html#Original%20Function)
* [C](Rust/rust_interop.html#C)
* [Lua](Rust/rust_interop.html#Lua)
* [Python](Rust/rust_interop.html#Python)
* [Golang](Rust/rust_interop.html#Golang)


## Original Function

We're going to make a contrived example to check if a number is prime.

```bash
$ cat prime.rs
```

```rust
#![feature(inclusive_range_syntax)]
#![feature(step_by)]

pub fn is_prime(n: u64) -> bool {
    match n {
        0 | 1 => false,
        2 | 3 => true,
        n if n & 1 == 0 => false,   // even numbers
        n => {
            let limit = (n as f64).sqrt() as u64 + 1;
            for i in (4...limit).step_by(2) {
                if n % i == 0 {
                    return false;
                }
            }
            return true;
        }
    }
}
```

You need to explicitly tell the rust compiler not to mangle the function
name, and export it as a C function. So the function signature needs to be
changed a bit...

```rust
#[no_mangle]
pub extern "C" fn is_prime(n: u64) -> bool {
...
}
```

And to make sure our library is callable from other code, it needs to be
compiled as a `cdylib` (with optimisations, of course).

```bash
$ rustc prime.rs -O --crate-type cdylib
$ ls
prime.rs  libprime.so
```

Aaaannnnddd we're done. How easy was that?

Because the library hasn't been installed into one of the standard locations
(i.e. `/usr/lib/`), you'll also need to add an environment variable so things
know where to find it. You'd need to do the exact same with a C/C++ library
as well.

```bash
$ export LD_LIBRARY_PATH=.
```


# C

Considering almost everything is descended from C and can interoperate with C,
it makes sense to check out C first. In this case, calling Rust code from C is
just like calling code from another library. Even easier if you take into
account that you don't even need to write a header file.

```c
#include <stdio.h>

extern int is_prime(unsigned int n);

int main() {
  unsigned int n = 1234567;

  printf("%d %s prime\n", n, is_prime(n) ? "is" : "is not");
  return 0;
}
```

And you link in the Rust library just like any other compiled `DLL` or `*.so`
file:

```bash
$ gcc main.c -L. -lprime
```

  > **Note:** because of a compiler bug at the time and the std library isn't
  > being used directly, it isn't linked in when `rustc` creates our shared
  > object. Unfortunately, a couple things were indirectly needed from the std
  > library so I was getting linker errors later on. The fix was to add a dummy
  > function which uses something from the std library (e.g. a print statement).


## Lua

Next is calling our `Rust` code from `Lua`. `Lua` is a super light-weight
language (about 10,000 lines of C)  often used for embedding into applications
to make them scriptable. It's also one of the fastest scripting languages
in use at the moment.

Here's the script I'm using:

```Lua
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
```

```bash
$ luajit main.lua
1234567 is prime
```

That was easy enough, all you need to do is `load()` the `*.so` file and use
it like pretty much any other library in `Lua`.


## Python

Next up is `Python`, unlike Lua, Python is well known for being slow.
Particularly when it comes to computationally expensive programs.

Python's standard library comes with a useful module for working with C (or
any other compiled language) called `ctypes`. You can use this pretty much the
same way as in Lua by creating some sort of object out of the loaded `*.so`
file and then calling it like normal.

```python
import ctypes

primes = ctypes.CDLL('./libprime.so')

n = 1234567

if primes.is_prime(n):
    print(n, "is prime")
else:
    print(n, "is not a prime")
```

Python also has a really nice library called `cffi`. With `ctypes` you usually
need to define each function, its input arguments, and what it returns in order
to have type safety, but `cffi` uses a completely different approach. You just
pass it the text for a typical C header file and it does the rest. For larger
projects, or things which may still be evolving, `cffi` is a much nicer library
to use.

```python
from cffi import FFI

ffi = FFI()

# Load a header entry for the Rust function
ffi.cdef("bool is_prime(int n);")
lib = ffi.dlopen("./libprime.so")

n = 1234567

if lib.is_prime(n):
    print(n, "is prime")
else:
    print(n, "is not a prime")

# trying to use an invalid datatype should fail
lib.is_prime("foo")
```


## Golang

Another language I've been interested in lately is [Go](https://golang.org).
It's a compiled, statically typed, gabage collected language developed by the
guys at Google and primarily oriented towards networks, concurrency, and
microservices.

Go has a nice trick for allowing you to interact with C code. It's got special
behaviour when you do `import "C"`. Any comments above are passed to the
underlying compiler, meaning you can `# include` any C library just like you
would in native C. Then, using reflection and other forms of black magic, you're
able to call anything in the imported namespace as an element of the `C` object.
i.e. calling `printf()` is just a case of doing `C.printf(...)`.

```go
package main

/*
#cgo LDFLAGS: -L. -lprime
#include "./prime.h"
*/
import "C"
import "fmt"

func main() {
	var n C.uint = 1234567
	if C.is_prime(n) != 0 {
		fmt.Printf("%d is prime\n", n)
	} else {
		fmt.Printf("%d is not prime\n", n)
	}
}
```
