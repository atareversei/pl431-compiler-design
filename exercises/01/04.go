package main

import (
	"fmt"
	"time"
)

var sink int64 // prevents dead‑code elimination

func main() {
	var inputA int64 = 3265651221
	var inputB int64 = 65652

	fmt.Println("--------------- Modulo ---------------")
	startM := time.Now()
	for k := 0; k < 10000; k++ {
		sink = gcdMod(inputA, inputB)
	}
	elapsedM := time.Since(startM)
	fmt.Println(elapsedM)

	fmt.Println("------------ Subtraction --------------")
	startD := time.Now()
	for k := 0; k < 10000; k++ {
		sink = gcdSub(inputA, inputB)
	}
	elapsedD := time.Since(startD)
	fmt.Println(elapsedD)
}

// gcdMod implements Euclid's algorithm using modulo.
func gcdMod(a, b int64) int64 {
	for a != 0 && b != 0 {
		if a > b {
			a = a % b
		} else {
			b = b % a
		}
	}
	if a == 0 {
		return b
	}
	return a
}

// gcdSub implements the subtraction-based GCD algorithm.
func gcdSub(a, b int64) int64 {
	for a != b {
		if a > b {
			a = a - b
		} else {
			b = b - a
		}
	}
	return a
}
