package main

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

const testInputSmall string = `6,10
0,14
9,10
0,3
10,4
4,11
6,0
6,12
4,1
0,13
10,12
3,4
3,0
8,4
1,10
2,14
8,10
9,0

fold along y=7
fold along x=5`

func TestParseInput(t *testing.T) {
	paper := parseInput(testInputSmall)
	assert.Equal(t, 18, len(paper.points))
	assert.Equal(t, 6, paper.points[0].x)
	assert.Equal(t, 10, paper.points[0].y)
	assert.Equal(t, 2, len(paper.folds))
	assert.Equal(t, "y", paper.folds[0].axis)
	assert.Equal(t, 7, paper.folds[0].ix)
}

func TestFolding(t *testing.T) {
	paper := parseInput(testInputSmall)
	count, _ := doFolding(paper, true)
	assert.Equal(t, 17, count)
}
