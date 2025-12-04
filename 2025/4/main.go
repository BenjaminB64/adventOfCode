package main

import (
	"fmt"
	"slices"
	"strings"
)

func main() {

}

func step_one(input string) int {
	lines := strings.Split(input, "\n")
	plan := [][]string{}
	for i, line := range lines {
		linePlan := []string{}
		for j := 0; j < len(line); j++ {
			linePlan = append(linePlan, string(lines[i][j]))
		}
		plan = append(plan, linePlan)
	}

	accessibleRollPapersCoordinates := [][]int{}
	accessibleRollPapers := 0
	for y := 0; y < len(plan); y++ {
		for x := 0; x < len(plan[y]); x++ {
			if plan[y][x] != "@" {
				continue
			}
			// check 8 adjacent positions
			adjacentPositions := [][]int{
				{y - 1, x - 1}, {y - 1, x}, {y - 1, x + 1},
				{y, x - 1}, {y, x + 1},
				{y + 1, x - 1}, {y + 1, x}, {y + 1, x + 1},
			}
			adjacentRollPapers := 0
			for _, adjacentPosition := range adjacentPositions {
				if adjacentPosition[0] < 0 || adjacentPosition[0] >= len(plan) || adjacentPosition[1] < 0 || adjacentPosition[1] >= len(plan[adjacentPosition[0]]) {
					continue
				}
				if plan[adjacentPosition[0]][adjacentPosition[1]] == "@" {
					adjacentRollPapers++
				}
			}
			if adjacentRollPapers < 4 {
				accessibleRollPapersCoordinates = append(accessibleRollPapersCoordinates, []int{y, x})
				accessibleRollPapers++
			}
		}
	}

	// print plan with accessible roll papers coordinates
	for y := 0; y < len(plan); y++ {
		for x := 0; x < len(plan[y]); x++ {
			if slices.ContainsFunc(accessibleRollPapersCoordinates, func(coordinate []int) bool {
				return coordinate[0] == y && coordinate[1] == x
			}) {
				fmt.Print("x")
			} else {
				fmt.Print(plan[y][x])
			}
		}
		fmt.Println()
	}
	return accessibleRollPapers
}

func step_two(input string) int {
	lines := strings.Split(input, "\n")
	plan := [][]string{}
	for i, line := range lines {
		linePlan := []string{}
		for j := 0; j < len(line); j++ {
			linePlan = append(linePlan, string(lines[i][j]))
		}
		plan = append(plan, linePlan)
	}

	totalAccessibleRollPapers := 0

	for {
		accessibleRollPapersCoordinates := [][]int{}
		accessibleRollPapers := 0
		for y := 0; y < len(plan); y++ {
			for x := 0; x < len(plan[y]); x++ {
				if plan[y][x] != "@" {
					continue
				}
				// check 8 adjacent positions
				adjacentPositions := [][]int{
					{y - 1, x - 1}, {y - 1, x}, {y - 1, x + 1},
					{y, x - 1}, {y, x + 1},
					{y + 1, x - 1}, {y + 1, x}, {y + 1, x + 1},
				}
				adjacentRollPapers := 0
				for _, adjacentPosition := range adjacentPositions {
					if adjacentPosition[0] < 0 || adjacentPosition[0] >= len(plan) || adjacentPosition[1] < 0 || adjacentPosition[1] >= len(plan[adjacentPosition[0]]) {
						continue
					}
					if plan[adjacentPosition[0]][adjacentPosition[1]] == "@" {
						adjacentRollPapers++
					}
				}
				if adjacentRollPapers < 4 {
					accessibleRollPapersCoordinates = append(accessibleRollPapersCoordinates, []int{y, x})
					accessibleRollPapers++
				}
			}
		}

		// print plan with accessible roll papers coordinates
		for y := 0; y < len(plan); y++ {
			for x := 0; x < len(plan[y]); x++ {
				if slices.ContainsFunc(accessibleRollPapersCoordinates, func(coordinate []int) bool {
					return coordinate[0] == y && coordinate[1] == x
				}) {
					//fmt.Print("x")
					plan[y][x] = "."
				} else {
					//fmt.Print(plan[y][x])
				}
			}
			//fmt.Println()
		}
		totalAccessibleRollPapers += accessibleRollPapers
		if accessibleRollPapers == 0 {
			break
		}
	}
	return totalAccessibleRollPapers
}
