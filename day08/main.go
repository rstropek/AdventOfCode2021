package main

import (
	"fmt"
	"os"
	"strings"
	s "strings"
)

type line struct {
	part1 []string
	part2 []string
	dict  []string
}

func containsAll(test string, pattern string) bool {
	for _, c := range pattern {
		if !strings.ContainsRune(test, c) {
			return false
		}
	}

	return true
}

func buildDict(input []string) []string {
	dict := make([]string, 10)

	// 1, 4, 7, 8
	for _, signal := range input {
		switch len(signal) {
		case 2:
			dict[1] = signal
		case 3:
			dict[7] = signal
		case 4:
			dict[4] = signal
		case 7:
			dict[8] = signal
		}

	}

	// 9: all that are of length 6 and share all with 4
	for _, signal := range input {
		if len(signal) == 6 && containsAll(signal, dict[4]) {
			dict[9] = signal
			break
		}
	}

	// 3: all that are of length 5, 9 contains all, and contains all of 1
	for _, signal := range input {
		if len(signal) == 5 && containsAll(dict[9], signal) && containsAll(signal, dict[1]) {
			dict[3] = signal
			break
		}
	}

	// 5: all that are of length 5, 9 contains all, and are not a 3
	for _, signal := range input {
		if len(signal) == 5 && containsAll(dict[9], signal) && signal != dict[3] {
			dict[5] = signal
			break
		}
	}

	// 2: all that are of length 5, and are not a 3 or a 5
	for _, signal := range input {
		if len(signal) == 5 && signal != dict[3] && signal != dict[5] {
			dict[2] = signal
			break
		}
	}

	// 0: all that are of length 6 and contain all of 1 and are not 9
	for _, signal := range input {
		if len(signal) == 6 && containsAll(signal, dict[1]) && signal != dict[9] {
			dict[0] = signal
			break
		}
	}

	// 6: all that are of length 6 and are not 0 and are not 9
	for _, signal := range input {
		if len(signal) == 6 && signal != dict[0] && signal != dict[9] {
			dict[6] = signal
			break
		}
	}

	return dict
}

func translate(input []string, dict []string) int {
	result := 0
	for _, v := range input {
		ix := 0
		for {
			if containsAll(dict[ix], v) && len(dict[ix]) == len(v) {
				break
			}

			ix++
		}

		result = result * 10 + ix
	}

	return result
}

func translateAll(input []line) int {
	result := 0
	for _, line := range input {
		result += translate(line.part2, line.dict)
	}

	return result
}

func parseInput(input string) []line {
	lines := make([]line, 0)
	for _, inputLine := range s.Split(input, "\n") {
		line := line{}
		parts := s.Split(inputLine, " | ")
		line.part1 = s.Split(parts[0], " ")
		line.part2 = s.Split(parts[1], " ")
		line.dict = buildDict(line.part1)
		lines = append(lines, line)
	}

	return lines
}

func countPart1(lines []line) int {
	result := 0
	for _, line := range lines {
		for _, signal := range line.part2 {
			if len(signal) == 2 || len(signal) == 4 || len(signal) == 3 || len(signal) == 7 {
				result++
			}
		}
	}

	return result
}

func main() {
	dat, _ := os.ReadFile("input.txt")
	input := parseInput(string(dat))
	result := countPart1(input)

	fmt.Println("Day 8:")

	fmt.Printf("\tPart 1 result is %d\n", result)
	fmt.Printf("\tPart 2 result is %d\n", translateAll(input))
}
