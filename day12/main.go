package main

import (
	"fmt"
	"os"
	s "strings"
	"unicode"
)

type edge struct {
	from string
	to   string
}

type countSettings struct {
	graph            []edge
	paths            *[][]string
	currentNode      string
	visited          *[]string
	oneDoubleAllowed bool
	oneDouble        bool
	currentPath      *[]string
}

func parseInput(input string) (graph []edge) {
	lines := s.Split(input, "\n")
	graph = make([]edge, len(lines)*2)
	for ix, l := range lines {
		nodes := s.Split(l, "-")
		graph[ix*2] = edge{nodes[0], nodes[1]}
		graph[ix*2+1] = edge{nodes[1], nodes[0]}
	}

	return
}

func countAllPaths(graph []edge, oneDoubleAllowed bool) int {
	paths := make([][]string, 0)
	visited := make([]string, 0)
	currentPath := make([]string, 0)
	countPaths(countSettings{
		graph:            graph,
		paths:            &paths,
		currentNode:      "start",
		visited:          &visited,
		oneDouble:        false,
		oneDoubleAllowed: oneDoubleAllowed,
		currentPath:      &currentPath,
	})

	return len(paths)
}

func countPaths(s countSettings) {
	if s.currentNode == "end" {
		// Reached the end -> found a path
		*s.currentPath = append(*s.currentPath, s.currentNode)
		*s.paths = append(*s.paths, *s.currentPath)
		return
	}

	if isLower(s.currentNode) {
		// store visited small caves
		*s.visited = append(*s.visited, s.currentNode)
	}

	// Append node to current path
	*s.currentPath = append(*s.currentPath, s.currentNode)

	// Visit all connected caves
	for _, pathsFromCurrentNode := range s.graph {
		if pathsFromCurrentNode.from == s.currentNode {
			// Found a path from current node
			contained := false
			if isLower(pathsFromCurrentNode.to) {
				contained = contains(*s.visited, pathsFromCurrentNode.to)
				if isLower(pathsFromCurrentNode.to) && ((contained && (s.oneDouble || !s.oneDoubleAllowed)) || pathsFromCurrentNode.to == "start") {
					continue
				}
			}

			newPath := append([]string(nil), *s.currentPath...)
			newVisited := append([]string(nil), *s.visited...)
			countPaths(countSettings{
				graph:            s.graph,
				paths:            s.paths,
				currentNode:      pathsFromCurrentNode.to,
				visited:          &newVisited,
				oneDouble:        contained || s.oneDouble,
				oneDoubleAllowed: s.oneDoubleAllowed,
				currentPath:      &newPath,
			})
		}
	}
}

func contains(visisted []string, node string) bool {
	for _, n := range visisted {
		if n == node {
			return true
		}
	}

	return false
}

func isLower(s string) bool {
	for _, r := range s {
		if !unicode.IsLower(r) && unicode.IsLetter(r) {
			return false
		}
	}
	return true
}

func main() {
	dat, _ := os.ReadFile("input.txt")
	input := parseInput(string(dat))

	fmt.Println("Day 12:")

	fmt.Printf("\tPart 1 result is %d\n", countAllPaths(input, false))
	fmt.Printf("\tPart 2 result is %d\n", countAllPaths(input, true))
}
