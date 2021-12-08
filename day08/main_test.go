package main

import (
	"os"
	"testing"

	"github.com/stretchr/testify/assert"
)

func TestParseSingleLine(t *testing.T) {
	const input string = "acedgfb cdfbe gcdfa fbcad dab cefabd cdfgeb eafb cagedb ab | cdfeb fcadb cdfeb cdbaf"
	result := parseInput(input)
	assert.Equal(t, 1, len(result))
	assert.Equal(t, 10, len(result[0].part1))
	assert.Equal(t, 4, len(result[0].part2))
	assert.Equal(t, "acedgfb", result[0].part1[0])
	assert.Equal(t, "cdbaf", result[0].part2[len(result[0].part2)-1])
}

func TestParseMultiLine(t *testing.T) {
	const input string = `be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe
edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc`
	result := parseInput(input)
	assert.Equal(t, 2, len(result))
	assert.Equal(t, 10, len(result[1].part1))
	assert.Equal(t, 4, len(result[1].part2))
}

func TestCountPart1(t *testing.T) {
	dat, _ := os.ReadFile("test_input_1.txt")
	assert.Equal(t, 26, countPart1(parseInput(string(dat))))
}

func TestContainsAll(t *testing.T) {
	assert.True(t, containsAll("asdf", "as"))
	assert.True(t, containsAll("asdf", "a"))
	assert.False(t, containsAll("adf", "as"))
	assert.False(t, containsAll("yxcv", "as"))
}

func TestBuildDict(t *testing.T) {
	const input string = "acedgfb cdfbe gcdfa fbcad dab cefabd cdfgeb eafb cagedb ab | cdfeb fcadb cdfeb cdbaf"
	result := parseInput(input)
	assert.Equal(t, "cagedb", result[0].dict[0])
	assert.Equal(t, "ab", result[0].dict[1])
	assert.Equal(t, "gcdfa", result[0].dict[2])
	assert.Equal(t, "fbcad", result[0].dict[3])
	assert.Equal(t, "eafb", result[0].dict[4])
	assert.Equal(t, "cdfbe", result[0].dict[5])
	assert.Equal(t, "cdfgeb", result[0].dict[6])
	assert.Equal(t, "dab", result[0].dict[7])
	assert.Equal(t, "acedgfb", result[0].dict[8])
	assert.Equal(t, "cefabd", result[0].dict[9])
}

func TestTranslate(t *testing.T) {
	const input string = "acedgfb cdfbe gcdfa fbcad dab cefabd cdfgeb eafb cagedb ab | cdfeb fcadb cdfeb cdbaf"
	result := parseInput(input)

	assert.Equal(t, 5353, translate(result[0].part2, result[0].dict))
}

func TestTranslatePart2(t *testing.T) {
	dat, _ := os.ReadFile("test_input_1.txt")
	result := parseInput(string(dat))
	assert.Equal(t, 61229, translateAll(result))
}

func TestTranslate2(t *testing.T) {
	const input string = "be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe"
	result := parseInput(input)

	assert.Equal(t, 8394, translate(result[0].part2, result[0].dict))
}
