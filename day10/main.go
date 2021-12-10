package main

import (
	"fmt"
	"os"
	"sort"
	s "strings"
)

func parseInput(input string) []string {
	return s.Split(input, "\n")
}

func processInput1(input []string) int {
	sum := 0
	for _, l := range input {
		_, err := process(l, 0)
		e, ok := err.(*SyntaxError)
		if ok {
			sum += e.Points()
		}
	}

	return sum
}

func processInput2(input []string) int {
	sums := make([]int, 0)
	for _, l := range input {
		_, err := process(l, 0)
		e, ok := err.(*IncompleteError)
		if ok {
			sums = append(sums, e.Points())
		}
	}

	sort.Ints(sums)

	return sums[len(sums) / 2]
}

const opening string = "([{<"
const closing string = ")]}>"

func indexOf(set string, char byte) int {
	for i := 0; i < len(set); i++ {
		if set[i] == char {
			return i
		}
	}

	return -1
}

type SyntaxError struct {
	invalidClosing byte
}

func (e *SyntaxError) Error() string {
	return fmt.Sprintf("Invalid closing %c", e.invalidClosing)
}

func (e SyntaxError) Points() int {
	switch e.invalidClosing {
	case ')':
		return 3
	case ']':
		return 57
	case '}':
		return 1197
	case '>':
		return 25137
	default:
		panic("Invalid char")
	}
}

type IncompleteError struct{
	missing string
}

func (e IncompleteError) Points() int {
	sum := 0
	for _, m := range e.missing {
		sum *= 5
		switch m {
		case ')':
			sum += 1
		case ']':
			sum += 2
		case '}':
			sum += 3
		case '>':
			sum += 4
		default:
			panic("Invalid char")
		}
	}

	return sum
}

func (e *IncompleteError) Error() string {
	return "Incomplete"
}

func process(input string, start int) (int, error) {

	for {
		if start >= len(input) {
			// Reached end of string
			return start, nil
		}

		// Check if we are on an opening for a chunk
		openingIndex := indexOf(opening, input[start])
		if openingIndex != -1 {
			// WE are on an opening for a chunk

			if start+1 >= len(input) {
				// Reached end of string, return incomplete
				return start, &IncompleteError{string(closing[openingIndex])}
			}

			// Check if we have a minimal chunk (e.g. "()")
			if input[start+1] == closing[openingIndex] {
				// We have a minimal chunk, jump over it...
				start += 2

				// ..and continue with next char
				continue
			}

			// We do not have a minimal chunk (e.g. "([..])").
			// Process inner chunk(s) with recursively.
			end, err := process(input, start+1)
			if err != nil {
				incomplete, ok := err.(*IncompleteError)
				if ok {
					incomplete.missing += string(closing[openingIndex])
				}
				
				// Processing found an error, bubble it up
				return 0, err
			}

			if end >= len(input) {
				// Reached end of string, return incomplete
				return end, &IncompleteError{string(closing[openingIndex])}
			}

			// Check if end matches closing
			if input[end] == closing[openingIndex] {
				// End matches closing, everything is ok. Progress to next char.
				start = end + 1
			} else {
				// End does not match closing
				return 0, &SyntaxError{invalidClosing: input[end]}
			}
		} else {
			if start == 0 {
				// If we are at the beginning, only openings are allowed
				return 0, &SyntaxError{input[start]}
			}

			// We are not on an opening of a chunk
			return start, nil
		}
	}
}

func main() {
	dat, _ := os.ReadFile("input.txt")
	input := parseInput(string(dat))

	fmt.Println("Day 9:")

	fmt.Printf("\tPart 1 result is %d\n", processInput1(input))
	fmt.Printf("\tPart 2 result is %d\n", processInput2(input))
}
