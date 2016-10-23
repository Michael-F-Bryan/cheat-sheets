# Testing in C

Recently I had the opportunity to help out a friend who was working on an
assignment using C and found some nice patterns to do TDD-style development in
a language that traditionally makes that a nightmare. So I thought I'd write
down my thoughts in case I need to do anything similar in the future.

The first job is to create a `test.c` file which can be compiled into an
executable and run.

The `main()` function consists of one function call, the call to our test
runner. The test runner will go through all our tests, run them, and then print
to the screen whether it was a success or not.

The general skeleton looks like this:

```c
#define TRUE 1
#define FALSE 0

void run_tests() {
  test("constructor", test_new_list);
  test("push", test_push);
  test("pop", test_pop);
  test("len", test_len);
  test("destructor", test_destructor);
}

int main()
{
  run_tests();
  return 0;
}
```

If you are familiar with `C`, then you'll probably know that something's a
little odd here. The `test()` "function" is actually taking in the name of a
test and then a reference to a function, however C doesn't actually allow
referencing functions and passing them around (not easily anyway).

`test()` is actually a macro who's definition looks like this:

```c
// Make a `test` macro which will automatically run our tests and print
// a tick if it was successful. Otherwise it prints a cross.
#define test(name, function_call) \
          printf("testing %s... ", name); \
          if(function_call()) { \
            printf("passed \u2714\n"); \
          } else { \
            printf("failed \u2718\n"); \
          }
```

All the `test()` macro does is print "testing foo...", then it'll run the
test and as long as its return value is true, it'll print "passed ✔". If it
fails, you'll see "failed ✘".

When we run the test executable, this is the output you get:

```
testing constructor... passed ✔
testing push... passed ✔
testing pop... passed ✔
testing len... passed ✔
testing destructor... passed ✔
```

The key part here is how each test is constructed. Basically, you'll set up
your inputs, run whatever function you are testing, then `assert()` that the
outputs are what you expect. If the function returns `TRUE` then the test
passed and if the function returns `FALSE` then it failed.

The `assert()` macro is a really simple macro that just expands to check if
the expression passed into it is `FALSE`. If it is, then return `FALSE`,
otherwise just continue like normal.

```c
// Define an assert macro which will test an expression and return FALSE
// if it is not true.
#define assert(expr) \
if((expr) == FALSE) { \
  return FALSE; \
}
```

Here's an example where we are checking that a `len()` function for finding the
length of a linked list works correctly.

```c
int test_len() {
  Node \*head = new_list(0);

  assert(len(head) == 1);

  // Add a bunch of stuff to the list and make sure length changes
  // appropriately
  int values[] = {1, 2, 3, 4, 5};
  for (int i=0; i < 5; i++) {
    head = push(head, values[i]);
  }
  assert(len(head) == 6);

  return TRUE;
}
```

Invoking all of this is actually pretty easy to do. What you'll do is pull out
all of your functions and other bits and pieces into their own library, then
when compiling test, you just link that in and you've got access to everything
you need for testing.

Then just add the corresponding section to your Makefile and you're done.

Along the way, I found that I was often needing to repeat boring setup code,
so I would pull this out into its own function. For example, here's a function
which takes in an array and its length, and returns a linked list with all the
array's elements added.

```c
// Given an array of values, create a new list from them (note: list is
// reversed)
Node* make_list(int values[], int length) {
  Node \*head = new_list(values[0]);
  for (int i=1; i<length; i++) {
    head = push(head, values[i]);
  }

  return head;
}
```

Little helper functions like these often make a world of difference when doing
testing and TDD.
