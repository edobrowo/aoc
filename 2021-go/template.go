package main

import (
	"bufio"
	"fmt"
	"os"
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

	return vals, nil
}

func Solve1(vals []int) (int, error) {
	return 0, nil
}

func Solve2(vals []int) (int, error) {
	return 0, nil
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
