package main

import (
	"fmt"
	"os"
	s "strings"
)

func parseInput(input string) (board [][]byte) {
	lines := s.Split(input, "\n")
	board = make([][]byte, len(lines)+2)

	// Add ghost row
	board[0] = make([]byte, len(lines[0])+2)
	for lineIx, line := range lines {
		boardLine := make([]byte, 12)
		boardLine[0] = 0 // Add ghost cell
		for i := 0; i < len(line); i++ {
			boardLine[i+1] = line[i] - byte('0')
		}

		boardLine[len(boardLine)-1] = 0 // Add ghost cell
		board[lineIx+1] = boardLine
	}

	// Add ghost row
	board[len(board)-1] = make([]byte, len(lines[len(lines)-1])+2)
	return
}

func process(input [][]byte) (int, int) {
	flashes := 0           // Number of total flashes
	flashesInFirst100 := 0 // Number of total flashes in the first 100 passes
	for i := 0; true; i++ {
		// First, the energy level of each octopus increases by 1.
		for lineIx := range input {
			for colIx := range input[lineIx] {
				input[lineIx][colIx] += 1
			}
		}

		// Create helper array to store which flashed during current pass
		flashed := make([][]bool, len(input))
		for lineIx := range input {
			flashed[lineIx] = make([]bool, len(input[lineIx]))
			// No need to initialize fields, automatically set to false
		}

		// Remember current number of flashes to be able to find out
		// if all flashed during current pass
		oldFlashes := flashes

		// Indicates whether one flashed during current pass
		oneFlashed := true
		for oneFlashed {
			oneFlashed = false

			for lineIx := range input {
				if lineIx == 0 || lineIx == len(input)-1 {
					// Ghost cells
					continue
				}

				for colIx := range input[lineIx] {
					if colIx == 0 || colIx == len(input[lineIx])-1 {
						// Ghost cells
						continue
					}

					// Any octopus with an energy level greater than 9 flashes.
					// An octopus can only flash at most once per step.
					if input[lineIx][colIx] > 9 && !flashed[lineIx][colIx] {
						oneFlashed = true
						flashes++
						if i < 100 {
							flashesInFirst100++
						}

						flashed[lineIx][colIx] = true

						// [A flash] increases the energy level of all adjacent octopuses by 1,
						// including octopuses that are diagonally adjacent.
						for nLineIx := lineIx - 1; nLineIx <= lineIx+1; nLineIx++ {
							for nColIx := colIx - 1; nColIx <= colIx+1; nColIx++ {
								input[nLineIx][nColIx] += 1
							}
						}
					}
				}
			}
		}

		if flashes-oldFlashes == 100 {
			// All flashed during current pass, found both solutions.
			// Assumption: "all flashed" happens after the first 100 passes.
			//             Given for AoC puzzle.
			return flashesInFirst100, i + 1
		}

		// Any octopus that flashed during this step has its energy level set to 0.
		for lineIx := range input {
			for colIx := range input[lineIx] {
				if flashed[lineIx][colIx] {
					input[lineIx][colIx] = 0
				}
			}
		}
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
