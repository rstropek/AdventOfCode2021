package main

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

const testInput string = `1163751742
1381373672
2136511328
3694931569
7463417111
1319128137
1359912421
3125421639
1293138521
2311944581`

func TestParseInput(t *testing.T) {
	input := parseInput(testInput)
	assert.Equal(t, 10, len(input))
	assert.Equal(t, 1, input[0][0])
	assert.Equal(t, 1, input[len(input)-1][len(input[9])-1])
}

func TestDijkstra(t *testing.T) {
	input := parseInput(testInput)
	q := dijkstra(input)
	assert.Equal(t, 40, q)
	assert.Equal(t, 40, solveWithReadyMade(input))
}

const testSmallInput string = `159
951
159`

func TestDuplicate(t * testing.T) {
	input := parseInput(testSmallInput)
	duplicated := duplicate(input)

	assert.Equal(t, len(input) * 5, len(duplicated))
	assert.Equal(t, len(input[0]) * 5, len(duplicated[0]))
	assert.Equal(t, 2, duplicated[0][3])
	assert.Equal(t, 6, duplicated[0][4])
	assert.Equal(t, 1, duplicated[0][5])
	assert.Equal(t, 2, duplicated[3][0])
	assert.Equal(t, 1, duplicated[4][0])
	assert.Equal(t, 2, duplicated[5][0])
	assert.Equal(t, 2, duplicated[5][0])
}

func TestDijkstra2(t *testing.T) {
	input := parseInput(testInput)
	duplicatedInput := duplicate(input)
	q := dijkstra(duplicatedInput)
	assert.Equal(t, 315, q)
}