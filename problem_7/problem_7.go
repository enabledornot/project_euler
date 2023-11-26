package main

import (
	"fmt"
)

func checkPrime(prevPrimes *[]int, possiblePrime int) bool {
	for _, pprime := range *prevPrimes {
		if possiblePrime % pprime == 0 {
			return false
		}
	}
	return true
}

func findPrimes(x int) []int {
	prevPrimes := make([]int, 0, x)
	currentNumber := 2
	for x > len(prevPrimes) {
		if checkPrime(&prevPrimes,currentNumber) {
			prevPrimes = append(prevPrimes,currentNumber)
		}
		currentNumber++
	}
	return prevPrimes
}

func main() {
	primes := findPrimes(100)
	fmt.Println("Primes: ",primes)
}