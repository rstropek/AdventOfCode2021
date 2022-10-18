package main

import (
	"fmt"
	"os"
	"strconv"
	"strings"
)

func parseInput(input string) []int {
	lines := make([]int, 0)
	for _, inputLine := range strings.Split(input, "\n") {
		value, _ := strconv.Atoi(inputLine)
		lines = append(lines, value)
	}

	return lines
}

func countIncreases(input []int) int {
	count := 0
	for i := 0; i < len(input)-1; i++ {
		if input[i] < input[i+1] {
			count++
		}
	}

	return count
}

func countIncreases2(input []int) int {
	count := 0
	for i := 0; i < len(input)-3; i++ {
		if input[i]+input[i+1]+input[i+2] < input[i+1]+input[i+2]+input[i+3] {
			count++
		}
	}

	return count
}

func main() {
	//dat, _ := os.ReadFile("test_input_1.txt") // Test input
	dat, _ := os.ReadFile("input.txt") // Real input

	data := parseInput(string(dat))
	increases := countIncreases(data)
	fmt.Println(increases)

	increases = countIncreases2(data)
	fmt.Println(increases)
}
