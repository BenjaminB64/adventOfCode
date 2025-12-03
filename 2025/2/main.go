package main

import (
	"strconv"
	"strings"
)

func main() {

}

func isInvalidOne(id string) bool {
	if len(id)%2 != 0 {
		return false
	}
	startPos := len(id) / 2
	for i := 0; i < startPos; i++ {
		if id[i] != id[startPos+i] {
			return false
		}
	}
	return true
}

func step_one(input string) int {
	return common(input, isInvalidOne)
}

func step_two(input string) int {
	return common(input, isInvalidTwo)
}

func isInvalidTwo(id string) bool {
	sequence := ""
	for i := 1; i < len(id); i++ {
		sequence = id[0:i]
		if checkIfSequenceIsRepeated(id, sequence) {
			return true
		}
	}
	return false
}

func checkIfSequenceIsRepeated(id string, sequence string) bool {
	for i := len(sequence); i < len(id); i += len(sequence) {
		if i+len(sequence) > len(id) {
			return false
		}
		if id[i:i+len(sequence)] != sequence {
			return false
		}
	}
	return true
}

func common(input string, isInvalid func(string) bool) int {
	lines := strings.Split(input, ",")
	sum := 0
	for _, line := range lines {
		parts := strings.Split(line, "-")
		first := parts[0]
		last := parts[1]

		firstInt, _ := strconv.Atoi(first)
		lastInt, _ := strconv.Atoi(last)

		for i := firstInt; i <= lastInt; i++ {
			if isInvalid(strconv.Itoa(i)) {
				sum += i
			}
		}
	}
	return sum
}
