package main

import (
	"bufio"
	"fmt"
	"os"
	"strconv"
	"strings"
)

type SolveError struct {
	Message string
}

func (e *SolveError) Error() string {
	return e.Message
}

type Stringer interface {
	String() string
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

type Line struct {
	x1 int
	y1 int
	x2 int
	y2 int
}

func (line Line) String() string {
	return fmt.Sprintf("%v,%v -> %v,%v", line.x1, line.y1, line.x2, line.y2)
}

func ParseLines(lines []string) ([]Line, error) {
	vals := make([]Line, 0)

	for _, line := range lines {
		tokens := strings.Split(line, " ")
		pointStr1 := strings.Split(tokens[0], ",")
		pointStr2 := strings.Split(tokens[2], ",")
		x1, err := strconv.Atoi(pointStr1[0])
		if err != nil {
			return vals, err
		}
		y1, err := strconv.Atoi(pointStr1[1])
		if err != nil {
			return vals, err
		}
		x2, err := strconv.Atoi(pointStr2[0])
		if err != nil {
			return vals, err
		}
		y2, err := strconv.Atoi(pointStr2[1])
		if err != nil {
			return vals, err
		}
		val := Line{x1, y1, x2, y2}
		vals = append(vals, val)
	}

	return vals, nil
}

func Abs(x int) int {
	if x < 0 {
		return -x
	}
	return x
}

func Min(x, y int) int {
	if x < y {
		return x
	}
	return y
}

func Max(x, y int) int {
	if x > y {
		return x
	}
	return y
}

func Sgn(x int) int {
	if x < 0 {
		return -1
	} else if x > 0 {
		return 1
	}
	return 0
}

func Solve1(vals []Line) (int, error) {
	var grid [1000][1000]int
	for _, val := range vals {
		if val.x1 != val.x2 && val.y1 != val.y2 {
			continue
		}
		var lo, hi int
		if val.x1 != val.x2 {
			lo = Min(val.x1, val.x2)
			hi = Max(val.x1, val.x2)
			for i := lo; i <= hi; i++ {
				grid[val.y1][i]++
			}
		} else {
			lo = Min(val.y1, val.y2)
			hi = Max(val.y1, val.y2)
			for i := lo; i <= hi; i++ {
				grid[i][val.x1]++
			}
		}
	}

	var dangerCount int
	for y, row := range grid {
		for x := range row {
			if grid[y][x] > 1 {
				dangerCount++
			}
		}
	}

	return dangerCount, nil
}

func Solve2(vals []Line) (int, error) {
	var grid [1000][1000]int
	for _, val := range vals {
		length := Max(Abs(val.x1-val.x2), Abs(val.y1-val.y2))
		xinc := Sgn(val.x2 - val.x1)
		yinc := Sgn(val.y2 - val.y1)
		x := val.x1
		y := val.y1
		for i := 0; i <= length; i++ {
			grid[y][x]++
			x += xinc
			y += yinc
		}
	}

	var dangerCount int
	for y, row := range grid {
		for x := range row {
			if grid[y][x] > 1 {
				dangerCount++
			}
		}
	}

	return dangerCount, nil
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
