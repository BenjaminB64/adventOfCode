package main

import (
	"fmt"
	"math/big"
	"strconv"
	"strings"
)

func main() {

}

type Operator uint8

const (
	OperatorPlus Operator = iota
	OperatorMult Operator = iota
)

func step_one(input string) *big.Int {
	lines := strings.Split(input, "\n")
	operators := []Operator{}
	line := lines[len(lines)-1]
	lineOperators := strings.Split(line, " ")
	for _, operator := range lineOperators {
		if operator == "" {
			continue
		}
		if operator == "+" {
			operators = append(operators, OperatorPlus)
			continue
		}
		operators = append(operators, OperatorMult)
	}

	columns := [][]uint64{}
	for i, line := range lines {
		if i == len(lines)-1 {
			continue
		}
		parts := strings.Split(line, " ")
		iColumn := 0
		for _, part := range parts {
			if part == "" {
				continue
			}
			num, _ := strconv.ParseUint(part, 10, 64)
			if len(columns) <= iColumn {
				columns = append(columns, []uint64{})
			}
			columns[iColumn] = append(columns[iColumn], num)
			iColumn++
		}
	}
	res := big.NewInt(0)
	for iColumn, column := range columns {
		columnRes := big.NewInt(0)
		for i, num := range column {
			if i == 0 {
				columnRes.SetUint64(num)
				continue
			}
			if operators[iColumn] == OperatorPlus {
				columnRes.Add(columnRes, big.NewInt(int64(num)))
				continue
			}
			columnRes.Mul(columnRes, big.NewInt(int64(num)))
		}
		res.Add(res, columnRes)
	}

	return res
}

func step_two(input string) *big.Int {
	lines := strings.Split(input, "\n")

	maxLength := 0
	for _, line := range lines {
		if len(line) > maxLength {
			maxLength = len(line)
		}
	}

	table := [][]rune{}
	for i, line := range lines {
		if len(table) <= i {
			table = append(table, make([]rune, maxLength))
		}

		for j, r := range line {
			table[i][j] = r
		}
	}
	res := big.NewInt(0)
	actualSign := OperatorPlus

	var columnNumbers []uint64 = nil
	lastRow := len(table) - 1
	for iColumn := 0; iColumn < maxLength; iColumn++ {
		lastRowRune := table[lastRow][iColumn]
		if lastRowRune != ' ' {
			if columnNumbers != nil {
				res.Add(res, calculateColumn(columnNumbers, actualSign))
				columnNumbers = []uint64{}
			}
			if table[lastRow][iColumn] == '+' {
				actualSign = OperatorPlus
			} else {
				actualSign = OperatorMult
			}
		}

		numberStr := ""
		for iRow := 0; iRow < len(table)-1; iRow++ {
			if table[iRow][iColumn] == ' ' {
				continue
			}
			numberStr += string(table[iRow][iColumn])
		}
		if numberStr == "" {
			continue
		}
		fmt.Println("numberStr", numberStr)
		num, _ := strconv.ParseUint(numberStr, 10, 64)
		columnNumbers = append(columnNumbers, num)
	}

	if columnNumbers != nil {
		res.Add(res, calculateColumn(columnNumbers, actualSign))
	}

	return res
}

func calculateColumn(columnNumbers []uint64, actualSign Operator) *big.Int {
	fmt.Println("columnNumbers", columnNumbers)
	fmt.Println("actualSign", actualSign)
	res := big.NewInt(0)
	for i, num := range columnNumbers {
		if i == 0 {
			res.SetUint64(num)
			continue
		}
		if actualSign == OperatorPlus {
			res.Add(res, big.NewInt(int64(num)))
		} else {
			res.Mul(res, big.NewInt(int64(num)))
		}
	}
	return res
}
