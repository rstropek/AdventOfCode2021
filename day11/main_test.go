package main

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

const testInput string = `5483143223
2745854711
5264556173
6141336146
6357385478
4167524645
2176841721
6882881134
4846848554
5283751526`

func TestParseInput(t *testing.T) {
	input := parseInput(testInput)
	assert.Equal(t, 12, len(input))
	assert.Equal(t, 12, len(input[0]))

	// Test presence of ghost cells
	for _, v := range input[0] {
		assert.Equal(t, byte(0), v)
	}

	for _, v := range input[len(input) - 1] {
		assert.Equal(t, byte(0), v)
	}

	for _, l := range input {
		assert.Equal(t, byte(0), l[0])
		assert.Equal(t, byte(0), l[len(l) - 1])
	}
}

func Test1(t *testing.T) {
	input := parseInput(testInput)
	flashes, _ := process(input)
	assert.Equal(t, 1656, flashes)
}

func Test2(t *testing.T) {
	input := parseInput(testInput)
	_, allFlashes := process(input)
	assert.Equal(t, 195, allFlashes)
}