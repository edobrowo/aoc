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

func ParseLines(lines []string) ([]int, error) {
	vals := make([]int, 0)
	for _, line := range lines {
		num, err := strconv.Atoi(line)
		if err != nil {
			return vals, err
		}
		vals = append(vals, num)
	}
	return vals, nil
}

func Solve1(vals []int) (int, error) {
	if len(vals) < 1 {
		return 0, &SolveError{Message: "vals must have length of at least 1"}
	}

	prev := vals[0]
	count := 0

	for _, val := range vals[1:] {
		if val > prev {
			count++
		}
		prev = val
	}

	return count, nil
}

func PartialSums(vals []int) []int {
	sums := make([]int, 1)

	for i, v := range vals {
		sums = append(sums, sums[i]+v)
	}

	return sums
}

func Solve2(vals []int) (int, error) {
	if len(vals) < 3 {
		return 0, &SolveError{Message: "vals must have length of at least 1"}
	}

	sums := PartialSums(vals)
	count := 0

	for i := 4; i < len(sums); i++ {
		if sums[i]-sums[i-3] > sums[i-1]-sums[i-4] {
			count++
		}
	}

	return count, nil
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

	count1, err := Solve1(vals)
	if err != nil {
		fmt.Printf("Solve error: %v", err)
	}

	fmt.Println("Solution (1):", count1)

	count2, err := Solve2(vals)
	if err != nil {
		fmt.Printf("Solve error: %v", err)
	}

	fmt.Println("Solution (2):", count2)
}
