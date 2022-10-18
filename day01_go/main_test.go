package main

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func TestParseSingleLine(t *testing.T) {
	const input string = "1\n2\n3"
	result := parseInput(input)
	assert.Equal(t, result, []int{1, 2, 3})
}

func TestCountIncreases(t *testing.T) {
	input := []int{199, 200, 208, 210, 200, 207, 240, 269, 260, 263}
	result := countIncreases(input)
	assert.Equal(t, result, 7)
}

func TestCountIncreases2(t *testing.T) {
	input := []int{199, 200, 208, 210, 200, 207, 240, 269, 260, 263}
	result := countIncreases2(input)
	assert.Equal(t, result, 5)
}
