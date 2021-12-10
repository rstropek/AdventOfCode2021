package main

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

const testInput string = `[({(<(())[]>[[{[]{<()<>>
[(()[<>])]({[<{<<[]>>(
{([(<{}[<>[]}>{[]{[(<()>
(((({<>}<{<{<>}{[]{[]{}
[[<[([]))<([[{}[[()]]]
[{[{({}]{}}([{[{{{}}([]
{<[[]]>}<{[{[{[]{()[[[]
[<(<(<(<{}))><([]([]()
<{([([[(<>()){}]>(<<{{
<{([{{}}[<[[[<>{}]]]>[]]`

func TestProcessLegal(t *testing.T) {
	legal := []string{
		"()",
		"[]",
		"{}",
		"<>",
		"([])",
		"{()()()}",
		"<([{}])>",
		"[<>({}){}[([])<>]]",
		"(((((((((())))))))))",
	}
	for _, l := range legal {
		endIx, err := process(l, 0)
		assert.Equal(t, len(l), endIx)
		assert.Nil(t, err)
	}
}

func TestProcessIncomplete(t *testing.T) {
	incompletes := []string{
		"(",
		"[",
		"{",
		"<",
		"([]",
		"{()(",
		"<([{}",
		"[<>({}){}[([])<>",
		"(((((((((()))))",
	}
	for _, l := range incompletes {
		_, err := process(l, 0)
		_, ok := err.(*IncompleteError)
		assert.True(t, ok, l)
	}
}

func TestCorrupted(t *testing.T) {
	corrupted := []struct {
		testString  string
		invalidChar byte
	}{
		{"(]", ']'},
		{"(((()))}", '}'},
		{"<([]){()}[{}])", ')'},
	}
	for _, l := range corrupted {
		_, err := process(l.testString, 0)
		e, ok := err.(*SyntaxError)
		assert.True(t, ok)
		assert.Equal(t, l.invalidChar, e.invalidClosing)
	}
}

func TestMissing(t *testing.T) {
	missing := []struct {
		testString string
		missing    string
		points     int
	}{
		{"[({(<(())[]>[[{[]{<()<>>", "}}]])})]", 288957},
		{"[(()[<>])]({[<{<<[]>>(", ")}>]})", 5566},
		{"(((({<>}<{<{<>}{[]{[]{}", "}}>}>))))", 1480781},
		{"{<[[]]>}<{[{[{[]{()[[[]", "]]}}]}]}>", 995444},
		{"<{([{{}}[<[[[<>{}]]]>[]]", "])}>", 294},
	}
	for _, l := range missing {
		_, err := process(l.testString, 0)
		e, ok := err.(*IncompleteError)
		assert.True(t, ok)
		assert.Equal(t, l.missing, e.missing)
		assert.Equal(t, l.points, e.Points())
	}
}

func Test1(t *testing.T) {
	input := parseInput(testInput)
	assert.Equal(t, 26397, processInput1(input))
}

func Test2(t *testing.T) {
	input := parseInput(testInput)
	assert.Equal(t, 288957, processInput2(input))
}
