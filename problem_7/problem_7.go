package main

import (
	"fmt"
	"sort"
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

func checkPrimeWg(prevPrimes *[]int, possiblePrime int, output chan int) {
	if checkPrime(prevPrimes,possiblePrime) {
		output <- possiblePrime
	} else {
		output <- (-1)*possiblePrime
	}
}

func findPrimesThreaded(x int) []int {
	prevPrimes := make([]int, 0, x)
	currentNumber := 2
	for x > len(prevPrimes) {
		squareBound := currentNumber * currentNumber
		fmt.Println("Square Bound: ",squareBound)
		newPrimes := make([]int, 0, x)
		newprimechan := make(chan int)
		count := 0
		for x > len(prevPrimes) && squareBound > currentNumber && count < 10000{
			go checkPrimeWg(&prevPrimes, currentNumber, newprimechan)
			currentNumber+=1
			count+=1
		}
		for i := 0; i < count ; i++ {
			np := <- newprimechan
			if np > 0 {
				newPrimes = append(newPrimes,np)
			}
		}
		sort.Ints(newPrimes)
		prevPrimes = append(prevPrimes,newPrimes...)
	}
	return prevPrimes
}

func main() {
	prime_search := 10001
	primes := findPrimesThreaded(prime_search)
	fmt.Println("Length of Primes: ",len(primes))
	fmt.Println("The", prime_search, "th prime:",primes[prime_search-1])
}