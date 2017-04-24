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
