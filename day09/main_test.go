package main

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

const testInput string = `2199943210
3987894921
9856789892
8767896789
9899965678`

func TestParse(t *testing.T) {
	s := parseInput(testInput)
	assert.Equal(t, 5, len(s.values))
	assert.Equal(t, 10, len(s.values[0]))
	assert.Equal(t, 2, s.values[0][0])
	assert.Equal(t, 8, s.values[len(s.values)-1][len(s.values[0])-1])
}

func TestIsMinimum(t *testing.T) {
	s := parseInput(testInput)
	assert.True(t, s.isMinimum(1, 0))
	assert.True(t, s.isMinimum(9, 0))
	assert.True(t, s.isMinimum(2, 2))
	assert.True(t, s.isMinimum(6, 4))
}

func TestIsNoMinimum(t *testing.T) {
	s := parseInput(testInput)
	assert.False(t, s.isMinimum(2, 0))
	assert.False(t, s.isMinimum(8, 0))
	assert.False(t, s.isMinimum(3, 2))
	assert.False(t, s.isMinimum(5, 4))
}

func TestRiskLevels(t *testing.T) {
	s := parseInput(testInput)
	assert.Equal(t, 15, s.calculateTotalRisk())
}

func TestHandleBasin(t *testing.T) {
	pos := []struct {
		size int
		x    int
		y    int
	}{
		{3, 0, 0},
		{3, 1, 0},
		{3, 0, 1},
		{9, 6, 0},
		{14, 0, 3},
	}

	for _, p := range pos {
		s := parseInput(testInput)
		assert.Equal(t, p.size, s.calculateSizeAndFill(p.x, p.y))
	}
}

func TestSizes(t *testing.T) {
	s := parseInput(testInput)
	assert.Equal(t, 1134, s.calculateBasins())
}
