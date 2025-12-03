package main

import (
	"strconv"
	"strings"
)

func main() {

}

func step_one(input string) int {
	count0 := 0
	pos := 50
	for _, line := range strings.Split(input, "\n") {
		if line == "" {
			continue
		}
		direction := line[0]
		distance := line[1:]
		distanceInt, _ := strconv.Atoi(distance)
		if direction == 'L' {
			pos = (pos - distanceInt) % 100
			if pos < 0 {
				pos += 100
			}
		} else {
			pos = (pos + distanceInt) % 100
		}
		if pos == 0 {
			count0++
		}
	}

	return count0
}

func step_two(input string) int {
	pos := 50
	count0 := 0
	for _, line := range strings.Split(input, "\n") {
		if line == "" {
			continue
		}
		direction := line[0]
		distance := line[1:]
		distanceInt, _ := strconv.Atoi(distance)

		// Add complete rotations
		count0 += distanceInt / 100

		rem := distanceInt % 100

		if direction == 'L' {
			for i := 0; i < rem; i++ {
				pos = pos - 1
				if pos < 0 {
					pos += 100
				}
				if pos == 0 {
					count0++
				}
			}
		} else {
			for i := 0; i < rem; i++ {
				pos = (pos + 1) % 100
				if pos == 0 {
					count0++
				}
			}
		}
	}
	return count0
}
