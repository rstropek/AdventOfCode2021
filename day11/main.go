package main

import (
	"fmt"
	"os"
	s "strings"
)

func parseInput(input string) [][]byte {
	lines := s.Split(input, "\n")
	board := make([][]byte, len(lines) + 2)

	// Add ghost row
	board[0] = make([]byte, len(lines[0]) + 2)
	for lineIx, line := range lines {
		boardLine := make([]byte, 12)
		boardLine[0] = 0 // Add ghost cell
		for i := 0; i < len(line); i++ {
			boardLine[i+1] = line[i] - byte('0')
		}

		boardLine[len(boardLine) - 1] = 0 // Add ghost cell
		board[lineIx+1] = boardLine
	}

	// Add ghost row
	board[len(board) - 1] = make([]byte, len(lines[len(lines) - 1]) + 2)
	return board
}

func process(input [][]byte) (int, int) {
	flashes := 0
	flashesInFirst100 := 0
	for i := 0; true; i++ {
		// First, the energy level of each octopus increases by 1.
		for lineIx := 1; lineIx < len(input) - 1; lineIx++ {
			for colIx := 1; colIx < len(input[lineIx]) - 1; colIx++ {
				input[lineIx][colIx] += 1
			}
		}

		flashed := make([][]bool, 12)
		for lineIx := 0; lineIx < len(input); lineIx++ {
			flashed[lineIx] = make([]bool, 12)
			for colIx := 0; colIx < len(input[lineIx]); colIx++ {
				flashed[lineIx][colIx] = false
			}
		}

		oneFlashed := true
		oldFlashes := flashes
		for oneFlashed {
			oneFlashed = false
			// Then, any octopus with an energy level greater than 9 flashes.
			// This increases the energy level of all adjacent octopuses by 1, 
			// including octopuses that are diagonally adjacent.
			for lineIx := 1; lineIx < len(input) - 1; lineIx++ {
				for colIx := 1; colIx < len(input[lineIx]) - 1; colIx++ {
					if input[lineIx][colIx] > 9 && !flashed[lineIx][colIx] {
						oneFlashed = true
						flashes++
						if i < 100 {
							flashesInFirst100++
						}

						flashed[lineIx][colIx] = true
						for neighbourLineIx := lineIx - 1; neighbourLineIx <= lineIx + 1; neighbourLineIx++ {
							for neighbourColIx := colIx - 1; neighbourColIx <= colIx + 1; neighbourColIx++ {
								if (neighbourLineIx != lineIx || neighbourColIx != colIx) && neighbourColIx != 0 && neighbourColIx < len(input[0]) - 1 && neighbourLineIx != 0 && neighbourLineIx < len(input) - 1 {
									input[neighbourLineIx][neighbourColIx] += 1
								}
							}
						}
					}
				}
			}
		}

		if flashes - oldFlashes == 100 {
			return flashesInFirst100, i + 1
		}

		for lineIx := 1; lineIx < len(input) - 1; lineIx++ {
			for colIx := 1; colIx < len(input[lineIx]) - 1; colIx++ {
				if flashed[lineIx][colIx] {
					input[lineIx][colIx] = 0
				}
			}
		}

		// for _, line := range input {
		// 	fmt.Println(line)
		// }

		// fmt.Println()
	}

	panic("No result")
}

func main() {
	dat, _ := os.ReadFile("input.txt")
	input := parseInput(string(dat))
	flashes, allFlash := process(input)

	fmt.Println("Day 9:")

	fmt.Printf("\tPart 1 result is %d\n", flashes)
	fmt.Printf("\tPart 2 result is %d\n", allFlash)
}
