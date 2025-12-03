package main

import (
	"log/slog"
	"sort"
	"strconv"
	"strings"
)

func main() {

}

func step_one(input string) int {
	lines := strings.Split(input, "\n")
	sum := 0
	for _, line := range lines {
		joltage := findLargestJoltage(line)
		sum += joltage
		slog.Debug("Joltage", "joltage", joltage)
		slog.Debug("Sum", "sum", sum)
	}
	return sum
}

func step_two(input string) int {
	lines := strings.Split(input, "\n")
	sum := 0
	for _, line := range lines {
		joltage := findLargestJoltageTwo(line)
		sum += joltage
		slog.Debug("Joltage", "joltage", joltage)
		slog.Debug("Sum", "sum", sum)
	}
	return sum
}

func findLargestJoltage(line string) int {
	firstJoltage := '0'
	secondJoltage := '0'
	shortestLine := ""
	counted := map[byte]int{}
	slog.Debug("Finding largest joltage for line", "line", line)
	for i := 0; i < len(line); i++ {
		counted[line[i]]++
		shortestLine += string(line[i])
	}

	for i := 0; i < len(shortestLine); i++ {
		if rune(shortestLine[i]) > firstJoltage && i < len(shortestLine)-1 {
			firstJoltage = rune(shortestLine[i])
			secondJoltage = '0'
			continue
		}
		if rune(shortestLine[i]) > secondJoltage {
			secondJoltage = rune(shortestLine[i])
			continue
		}
	}
	slog.Debug("First joltage", "firstJoltage", firstJoltage)
	slog.Debug("Second joltage", "secondJoltage", secondJoltage)
	return int(firstJoltage-48)*10 + int(secondJoltage-48)
}

func findLargestJoltageTwo(line string) int {
	shortestLine := getMaxWithMinLength(line, 12)
	slog.Debug("Shortest line", "line", line, "length", 12, "shortestLine", shortestLine)
	joltage, _ := strconv.Atoi(shortestLine)
	return joltage
}

func findShortestLine(line string, length int) string {
	newLine := line
	for i := 0; i < 9; i++ {
		newLine = subtractOneAll(line, i)
		if len(newLine) == length {
			return newLine
		}
		if len(newLine) < length {
			return line
		}
		line = newLine
	}
	slog.Debug("No shortest line found", "line", line, "length", length)
	return line
}

func subtractOneAll(line string, index int) string {
	lineBytes := []byte(line)
	removed := 0
	for i := 0; i < len(line); i++ {
		if line[i] == byte('0'+index) {
			lineBytes = append(lineBytes[:i-removed], lineBytes[i+1-removed:]...)
			removed++
			if len(lineBytes) == 12 {

				return string(lineBytes)
			}
		}
	}

	return string(lineBytes)
}

func keepLength(line string, length int) string {
	lineBytes := []rune(line)
	candidates := [][]rune{}
	max := '0'

	for i := 0; i < len(lineBytes)-length; i++ {
		if lineBytes[i] > max {
			max = lineBytes[i]
			candidates = [][]rune{[]rune(findShortestLine(string(lineBytes[i:]), length))}
			continue
		}
		if lineBytes[i] == max {
			candidates = append(candidates, []rune(findShortestLine(string(lineBytes[i:]), length)))
		}
	}

	sort.Slice(candidates, func(i, j int) bool {
		iInt, _ := strconv.Atoi(string(candidates[i]))
		jInt, _ := strconv.Atoi(string(candidates[j]))
		return iInt > jInt
	})

	return string(candidates[0])
}

func getMaxWithMinLength(line string, length int) string {
	lineBytes := []rune(line)
	max := '0'
	pos := 0
	for i := 0; i <= len(lineBytes)-length; i++ {
		if lineBytes[i] > max {
			max = lineBytes[i]
			pos = i
		}
	}

	if length <= 1 {
		return string(lineBytes[pos])
	}

	if pos+length == len(lineBytes) {
		return string(lineBytes[pos:])
	}

	res := string(string(lineBytes[pos]) + getMaxWithMinLength(string(lineBytes[pos+1:]), length-1))

	return res
}
