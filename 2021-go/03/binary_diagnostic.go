package main

import (
	"bufio"
	"fmt"
	"os"
	"strconv"
)

type SolveError struct {
	Message string
}

func (e *SolveError) Error() string {
	return e.Message
}

func ReadLines(path string) ([]string, error) {
	lines := make([]string, 0)

	file, err := os.Open(path)
	if err != nil {
		return lines, err
	}
	defer file.Close()

	scanner := bufio.NewScanner(file)

	for scanner.Scan() {
		lines = append(lines, scanner.Text())
	}

	if err := scanner.Err(); err != nil {
		return lines, err
	}
	return lines, nil
}

func ParseLines(lines []string) ([]int64, error) {
	vals := make([]int64, 0)
	for _, line := range lines {
		val, err := strconv.ParseInt(line, 2, 16)
		if err != nil {
			return vals, err
		}
		vals = append(vals, val)
	}
	return vals, nil
}

const datumSize int = 12

func CountBits(vals []int64) []int {
	bitCounts := make([]int, datumSize)
	for _, val := range vals {
		for i := 0; i < datumSize; i++ {
			bit := (val >> i) & 1
			if bit == 1 {
				bitCounts[i]++
			}
		}
	}
	return bitCounts
}

func Solve1(vals []int64) (int, error) {
	bitCounts := CountBits(vals)

	var gamma, epsilon int
	for i := 0; i < datumSize; i++ {
		if bitCounts[i] > len(vals)-bitCounts[i] {
			gamma |= (1 << i)
		}
	}
	epsilon = ^gamma & 0xFFF

	return gamma * epsilon, nil
}

func Filter(nums []int64, predicate func(int64) bool) []int64 {
	res := make([]int64, 0)

	for _, num := range nums {
		if predicate(num) {
			res = append(res, num)
		}
	}

	return res
}

func OxygenGeneratorRating(vals []int64) int64 {
	i := datumSize
	for len(vals) > 1 {
		bitCounts := CountBits(vals)
		msbCount := bitCounts[i-1]
		var msb int64 = 0
		if msbCount >= len(vals)-msbCount {
			msb = 1
		}
		vals = Filter(vals, func(val int64) bool {
			return ((val >> (i - 1)) & 1) == msb
		})
		i--
	}
	return vals[0]
}

func CO2ScubberRating(vals []int64) int64 {
	i := datumSize
	for len(vals) > 1 {
		bitCounts := CountBits(vals)
		msbCount := bitCounts[i-1]
		var msb int64 = 0
		if msbCount < len(vals)-msbCount {
			msb = 1
		}
		vals = Filter(vals, func(val int64) bool {
			return ((val >> (i - 1)) & 1) == msb
		})
		i--
	}
	return vals[0]
}

func Solve2(vals []int64) (int, error) {
	oxygenRating := (int)(OxygenGeneratorRating(vals))
	co2Rating := (int)(CO2ScubberRating(vals))
	return oxygenRating * co2Rating, nil
}

func main() {
	const path string = "input.txt"

	lines, err := ReadLines(path)
	if err != nil {
		fmt.Printf("Scann error: %v", err)
		return
	}

	vals, err := ParseLines(lines)
	if err != nil {
		fmt.Printf("Parse error: %v", err)
		return
	}

	soln1, err := Solve1(vals)
	if err != nil {
		fmt.Printf("Solve error: %v", err)
		return
	}

	fmt.Println("Solution (1):", soln1)

	soln2, err := Solve2(vals)
	if err != nil {
		fmt.Printf("Solve error: %v", err)
		return
	}

	fmt.Println("Solution (2):", soln2)
}
