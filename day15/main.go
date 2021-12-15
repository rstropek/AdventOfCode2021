package main

import (
	"fmt"
	"math"
	"os"
	"strconv"

	s "strings"

	rcd "github.com/RyanCarrier/dijkstra"
)

func parseInput(input string) [][]int {
	lines := s.Split(input, "\n")
	risks := make([][]int, len(lines))
	for lix, l := range lines {
		risks[lix] = make([]int, len(l))
		for cix, c := range l {
			v, _ := strconv.ParseInt(string(c), 10, 32)
			risks[lix][cix] = int(v)
		}
	}

	return risks
}

func duplicate(input [][]int) [][]int {
	result := make([][]int, len(input)*5)
	for lix := range result {
		result[lix] = make([]int, len(input[0])*5)
	}

	// Copy original
	for lix := range input {
		for cix := range input[lix] {
			result[lix][cix] = input[lix][cix]
		}
	}

	// First line
	for i := 1; i < 5; i++ {
		for lix := range input {
			for cix := range input[lix] {
				v := result[lix][cix+len(input[lix])*(i-1)] + 1
				if v > 9 {
					v = 1
				}
				result[lix][cix+len(input[lix])*i] = v
			}
		}
	}

	// Other lines
	for i := 1; i < 5; i++ {
		for lix := range input {
			for cix := range result[lix] {
				v := result[lix+len(input[lix])*(i-1)][cix] + 1
				if v > 9 {
					v = 1
				}
				result[lix+len(input[lix])*i][cix] = v
			}
		}
	}

	return result
}

type point struct {
	x int
	y int
}

func findElementWithMinDist(state [][]int, q [][]bool) point {
	minDist := math.MaxInt
	p := point{x: 0, y: 0}

	for lix, l := range q {
		for cix, c := range l {
			if !c && state[lix][cix] < minDist {
				minDist = state[lix][cix]
				p.x = cix
				p.y = lix
			}
		}
	}

	return p
}

func dijkstra(input [][]int) int {
	dist := make([][]int, len(input))
	q := make([][]bool, len(input))

	for lix := range input {
		dist[lix] = make([]int, len(input[lix]))
		q[lix] = make([]bool, len(input[lix]))
		for cix := range input[lix] {
			dist[lix][cix] = math.MaxInt
			q[lix][cix] = false
		}
	}

	dist[0][0] = 0

	for {
		p := findElementWithMinDist(dist, q)
		q[p.y][p.x] = true

		checks := []point{{x: 1, y: 0}, {x: 0, y: 1}, {x: -1, y: 0}, {x: 0, y: -1}}
		for _, check := range checks {
			ptc := point{x: p.x + check.x, y: p.y + check.y}
			if ptc.x < 0 || ptc.y < 0 || ptc.x >= len(input[0]) || ptc.y >= len(input) {
				continue
			}

			alt := dist[p.y][p.x] + input[ptc.y][ptc.x]
			if alt < dist[ptc.y][ptc.x] {
				dist[ptc.y][ptc.x] = alt
			}

			if ptc.x == len(input[0])-1 && ptc.y == len(input)-1 {
				return dist[len(input)-1][len(input[0])-1]
			}
		}
	}
}

func solveWithReadyMade(input [][]int) int {
	graph := rcd.NewGraph()
	for lix := range input {
		for cix := range input[0] {
			graph.AddVertex(lix*1000 + cix)
		}
	}

	for lix := range input {
		for cix := range input[0] {
			checks := []point{{x: 1, y: 0}, {x: 0, y: 1}, {x: -1, y: 0}, {x: 0, y: -1}}
			for _, check := range checks {
				ptc := point{x: cix + check.x, y: lix + check.y}
				if ptc.x < 0 || ptc.y < 0 || ptc.x >= len(input[0]) || ptc.y >= len(input) {
					continue
				}

				graph.AddArc(lix*1000+cix, ptc.y*1000+ptc.x, int64(input[ptc.y][ptc.x]))
			}
		}
	}

	best, _ := graph.Shortest(0, (len(input)-1)*1000+(len(input[0])-1))

	return int(best.Distance)
}

func main() {
	dat, _ := os.ReadFile("input.txt")

	fmt.Println("Day 15:")

	input := parseInput(string(dat))
	q := dijkstra(input)
	fmt.Printf("\tPart 1 result is %d\n", q)

	input2 := parseInput(string(dat))
	duplicatedInput := duplicate(input2)
	q2 := dijkstra(duplicatedInput)
	fmt.Printf("\tPart 2 result is: %d\n", q2)
}
