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
	// defer wg.Done()
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
		squareBound := currentNumber^2
		newPrimes := make([]int, 0, x)
		newprimechan := make(chan int)
		count := 0
		for x > len(prevPrimes) && squareBound > len(prevPrimes) {
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
	primes := findPrimes(10001)
	fmt.Println("Length of Primes: ",len(primes))
	fmt.Println("Primes: ",primes)
}