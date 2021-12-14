package main

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

const testInput string = `NNCB

CH -> B
HH -> N
CB -> H
NH -> C
HB -> C
HC -> B
HN -> C
NN -> C
BH -> H
NC -> B
NB -> B
BN -> B
BB -> N
BC -> B
CC -> N
CN -> C`

func TestParseInput(t *testing.T) {
	input := parseInput(testInput)

	assert.Equal(t, 16, len(input.rules))
	assert.Equal(t, 3, len(input.pairs))
	assert.Equal(t, "B", input.rules["CH"])
	assert.Equal(t, int64(1), input.pairs["NN"])
	assert.Equal(t, int64(2), input.counts["N"])
	assert.Equal(t, int64(1), input.counts["C"])
	assert.Equal(t, int64(1), input.counts["B"])
}

func TestInsert1(t *testing.T) {
	input := parseInput(testInput)
	input = insertMultiple(input, 1)
	assert.Equal(t, int64(2), input.counts["B"])
	assert.Equal(t, int64(1), input.pairs["NC"])
	assert.Equal(t, int64(0), input.pairs["CB"])
}

func TestInsert2(t *testing.T) {
	input := parseInput(testInput)
	input = insertMultiple(input, 2)
	assert.Equal(t, int64(6), input.counts["B"])
	assert.Equal(t, int64(2), input.pairs["CB"])
	assert.Equal(t, int64(0), input.pairs["HB"])
}

func TestInsert3(t *testing.T) {
	input := parseInput(testInput)
	input = insertMultiple(input, 3)
	assert.Equal(t, int64(11), input.counts["B"])
	assert.Equal(t, int64(0), input.pairs["CB"])
	assert.Equal(t, int64(3), input.pairs["HB"])
}

func TestInsert4(t *testing.T) {
	input := parseInput(testInput)
	input = insertMultiple(input, 4)
	assert.Equal(t, int64(5), input.pairs["CB"])
	assert.Equal(t, int64(23), input.counts["B"])
}

func TestInsert10(t *testing.T) {
	input := parseInput(testInput)
	input = insertMultiple(input, 10)
	assert.Equal(t, int64(1749), input.counts["B"])
	assert.Equal(t, int64(298), input.counts["C"])
	assert.Equal(t, int64(1588), findResult(input.counts))
}

func TestInsert40(t *testing.T) {
	input := parseInput(testInput)
	input = insertMultiple(input, 40)
	assert.Equal(t, int64(2192039569602), input.counts["B"])
	assert.Equal(t, int64(3849876073), input.counts["H"])
	assert.Equal(t, int64(2188189693529), findResult(input.counts))
}