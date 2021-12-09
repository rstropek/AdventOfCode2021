package main

import (
	"fmt"
	"os"
	"sort"
	s "strings"
)

type sonar struct {
	values [][]int
}

func parseInput(input string) sonar {
	lines := sonar{make([][]int, 0)}
	for _, inputLine := range s.Split(input, "\n") {
		values := make([]int, len(inputLine))
		for i, char := range inputLine {
			values[i] = int(char) - int('0')
		}

		lines.values = append(lines.values, values)
	}

	return lines
}

func (s sonar) isMinimum(x int, y int) bool {
	minX := x
	if x > 0 {
		minX--
	}

	minY := y
	if y > 0 {
		minY--
	}

	maxX := x
	if x < len(s.values[0])-1 {
		maxX++
	}

	maxY := y
	if y < len(s.values)-1 {
		maxY++
	}

	for curX := minX; curX <= maxX; curX++ {
		for curY := minY; curY <= maxY; curY++ {
			if s.values[y][x] > s.values[curY][curX] {
				return false
			}
		}
	}

	return true
}

func (s sonar) calculateTotalRisk() int {
	sum := 0
	for x := 0; x < len(s.values[0]); x++ {
		for y := 0; y < len(s.values); y++ {
			if s.isMinimum(x, y) {
				sum += s.values[y][x] + 1
			}
		}
	}

	return sum
}

func (s sonar) calculateSizeAndFill(x int, y int) int {
	if s.values[y][x] == 9 {
		return 0
	}

	sum := 1
	s.values[y][x] = 9
	if x < len(s.values[0])-1 {
		sum += s.calculateSizeAndFill(x+1, y)
	}

	if y < len(s.values)-1 {
		sum += s.calculateSizeAndFill(x, y+1)
	}

	if y > 0 {
		sum += s.calculateSizeAndFill(x, y-1)
	}

	if x > 0 {
		sum += s.calculateSizeAndFill(x-1, y)
	}

	return sum
}

func (s sonar) calculateBasins() int {
	sizes := []int{0, 0, 0}
	for x := 0; x < len(s.values[0]); x++ {
		for y := 0; y < len(s.values); y++ {
			if s.values[y][x] != 9 {
				basinSize := s.calculateSizeAndFill(x, y)
				if basinSize > sizes[0] {
					sizes[0] = basinSize
					sort.Ints(sizes)
				}
			}
		}
	}

	return sizes[0] * sizes[1] * sizes[2]
}

func main() {
	dat, _ := os.ReadFile("input.txt")
	input := parseInput(string(dat))

	fmt.Println("Day 9:")

	fmt.Printf("\tPart 1 result is %d\n", input.calculateTotalRisk())
	fmt.Printf("\tPart 2 result is %d\n", input.calculateBasins())
}
