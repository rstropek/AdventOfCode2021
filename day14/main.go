package main

import (
	"fmt"
	"math"
	"os"
	s "strings"
)

type state struct {
	pairs  map[string]int64
	rules  map[string]string
	counts map[string]int64
}

func parseInput(input string) (st state) {
	st = state{pairs: make(map[string]int64), counts: make(map[string]int64)}
	segments := s.Split(input, "\n\n")
	template := segments[0]
	st.counts[string(template[0])] = 1
	for ix := 1; ix < len(template); ix++ {
		pair := template[(ix - 1):(ix + 1)]
		if cnt, prs := st.pairs[pair]; prs {
			st.pairs[pair] = cnt + 1
		} else {
			st.pairs[pair] = 1
		}

		if cnt, prs := st.counts[string(template[ix])]; prs {
			st.counts[string(template[ix])] = cnt + 1
		} else {
			st.counts[string(template[ix])] = 1
		}
	}

	ruleStrings := s.Split(segments[1], "\n")
	st.rules = make(map[string]string)
	for _, ruleString := range ruleStrings {
		ruleStrings := s.Split(ruleString, " -> ")
		st.rules[ruleStrings[0]] = ruleStrings[1]
	}

	return
}

func insertMultiple(input state, count int64) state {
	for i := int64(0); i < count; i++ {
		input = insert(input)
	}

	return input
}

func mapInc(target map[string]int64, key string, incBy int64) int64 {
	if cnt, tprs := target[key]; tprs {
		target[key] = cnt + incBy
		return cnt + incBy
	}

	target[key] = incBy
	return incBy
}

func insert(input state) state {
	result := make(map[string]int64)

	for pair, cnt := range input.pairs {
		ins := input.rules[pair]
		target1 := string(pair[0]) + ins
		mapInc(result, target1, cnt)
		
		target2 := ins + string(pair[1])
		mapInc(result, target2, cnt)
		mapInc(input.counts, ins, cnt)
	}

	input.pairs = result
	return input
}

func findResult(input map[string]int64) (result int64) {
	max := int64(0)
	low := int64(math.MaxInt64)
	for _, v := range input {
		if v > max {
			max = v
		} 

		if v < low {
			low = v
		}
	}

	return max - low
}

func main() {
	dat, _ := os.ReadFile("input.txt")
	
	fmt.Println("Day 14:")
	
	input := parseInput(string(dat))
	input = insertMultiple(input, 10)
	fmt.Printf("\tPart 1 result is %d\n", findResult(input.counts))

	input = parseInput(string(dat))
	input = insertMultiple(input, 40)
	fmt.Printf("\tPart 2 result is: %d\n", findResult(input.counts))
}
