package main

import (
	"bytes"
	"fmt"
	"os"
	"strconv"
	s "strings"
)

type point struct {
	x int
	y int
}

type fold struct {
	axis string
	ix   int
}

type paper struct {
	points []point
	folds  []fold
	width  int
	height int
}

func parseInput(input string) paper {
	width := 0
	height := 0
	segments := s.Split(input, "\n\n")
	coordsLines := s.Split(segments[0], "\n")
	points := make([]point, len(coordsLines))
	for cix, coordsStrings := range coordsLines {
		coordStrings := s.Split(coordsStrings, ",")
		x, _ := strconv.Atoi(string(coordStrings[0]))
		if (x + 1) > width {
			width = x + 1
		}

		y, _ := strconv.Atoi(string(coordStrings[1]))
		if (y + 1) > height {
			height = y + 1
		}

		points[cix] = point{x, y}
	}

	foldsStrings := s.Split(segments[1], "\n")
	folds := make([]fold, len(foldsStrings))
	for fix, foldString := range foldsStrings {
		foldString = foldString[len("fold along "):]
		foldStrings := s.Split(foldString, "=")
		ix, _ := strconv.Atoi(string(foldStrings[1]))
		folds[fix] = fold{foldStrings[0], ix}
	}

	return paper{points, folds, width, height}
}

func doFolding(p paper, singleRun bool) (sum int, output string) {
	content := make([][]bool, p.height)
	for line := 0; line < p.height; line++ {
		content[line] = make([]bool, p.width)
	}

	for _, point := range p.points {
		content[point.y][point.x] = true
	}

	for _, fold := range p.folds {
		if fold.axis == "y" {
			newContent := make([][]bool, fold.ix)

			// Copy upper half
			for l := range newContent {
				newContent[l] = content[l]
			}

			targetLine := len(newContent) - 1
			for l := len(newContent) + 1; l < len(content); l++ {
				for ix, c := range content[l] {
					newContent[targetLine][ix] = newContent[targetLine][ix] || c
				}

				targetLine--
			}

			content = newContent
		} else {
			newContent := make([][]bool, len(content))

			// Copy left half
			for ix := range newContent {
				newContent[ix] = make([]bool, fold.ix)
				for c := 0; c < len(newContent[ix]); c++ {
					newContent[ix][c] = content[ix][c]
				}
			}

			for ix, l := range newContent {
				targetCol := len(newContent[0]) - 1
				for c := len(newContent[0]) + 1; c < len(content[0]); c++ {
					l[targetCol] = l[targetCol] || content[ix][c]
					targetCol--
				}

			}

			content = newContent
		}

		if singleRun {
			break
		}
	}

	sum = 0
	for _, l := range content {
		for _, c := range l {
			if c {
				sum++
			}
		}
	}

	var outputBuffer bytes.Buffer
	for _, line := range content {
		for _, col := range line {
			if col {
				outputBuffer.WriteString("#")
				} else {
					outputBuffer.WriteString(".")
			}
		}
		outputBuffer.WriteRune('\n')
	}

	output = outputBuffer.String()

	return
}

func main() {
	dat, _ := os.ReadFile("input.txt")
	input := parseInput(string(dat))
	
	fmt.Println("Day 13:")
	
	count, _ := doFolding(input, true)
	fmt.Printf("\tPart 1 result is %d\n", count)

	_, output := doFolding(input, false)
	fmt.Printf("\tPart 2 result is:\n%s", output)
}
