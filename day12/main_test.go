package main

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

const testInputSmall string = `start-A
start-b
A-c
A-b
b-d
A-end
b-end`

const testInputMedium string = `dc-end
HN-start
start-kj
dc-start
dc-HN
LN-dc
HN-end
kj-sa
kj-HN
kj-dc`

const testInputLarge string = `fs-end
he-DX
fs-he
start-DX
pj-DX
end-zg
zg-sl
zg-pj
pj-he
RW-he
fs-DX
pj-RW
zg-RW
start-pj
he-WI
zg-he
pj-fs
start-RW`

func TestParseInput(t *testing.T) {
	input := parseInput(testInputSmall)
	assert.Equal(t, 14, len(input))
	assert.Equal(t, "start", input[0].from)
	assert.Equal(t, "A", input[0].to)
	assert.Equal(t, "A", input[1].from)
	assert.Equal(t, "start", input[1].to)
}

func TestCountPaths1(t *testing.T) {
	input := parseInput(testInputSmall)
	assert.Equal(t, 10, countAllPaths(input, false))
}

func TestCountPaths1_2(t *testing.T) {
	input := parseInput(testInputSmall)
	assert.Equal(t, 36, countAllPaths(input, true))
}

func TestCountPaths2(t *testing.T) {
	input := parseInput(testInputMedium)
	assert.Equal(t, 19, countAllPaths(input, false))
}

func TestCountPaths2_2(t *testing.T) {
	input := parseInput(testInputMedium)
	assert.Equal(t, 103, countAllPaths(input, true))
}

func TestCountPaths3(t *testing.T) {
	input := parseInput(testInputLarge)
	assert.Equal(t, 226, countAllPaths(input, false))
}

func TestCountPaths3_2(t *testing.T) {
	input := parseInput(testInputLarge)
	assert.Equal(t, 3509, countAllPaths(input, true))
}
