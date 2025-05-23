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

type Direction int

const (
	Forward Direction = 0
	Up      Direction = 1
	Down    Direction = 2
)

func ToDirection(s string) Direction {
	dir := Forward
	switch s {
	case "forward":
		dir = Forward
	case "up":
		dir = Up
	case "down":
		dir = Down
	default:
		dir = Forward
	}
	return dir
}

type Command struct {
	Dir       Direction
	Magnitude int
}

type Stringer interface {
	String() string
}

func (command Command) String() string {
	return fmt.Sprintf("(%v, %v)", command.Dir, command.Magnitude)
}

func ParseLines(lines []string) ([]Command, error) {
	vals := make([]Command, 0)
	for _, line := range lines {
		tokens := strings.Split(line, " ")
		dir := ToDirection(tokens[0])
		magn, err := strconv.Atoi(tokens[1])
		if err != nil {
			return vals, err
		}
		vals = append(vals, Command{Dir: dir, Magnitude: magn})
	}
	return vals, nil
}

func Solve1(commands []Command) (int, error) {
	dmap := map[Direction]int{Forward: 0, Up: -1, Down: 1}
	hmap := map[Direction]int{Forward: 1, Up: 0, Down: 0}
	depth, horizontal := 0, 0
	for _, command := range commands {
		dmult, ok := dmap[command.Dir]
		if !ok {
			return 0, &SolveError{Message: "Invalid direction"}
		}
		hmult, ok := hmap[command.Dir]
		if !ok {
			return 0, &SolveError{Message: "Invalid direction"}
		}
		depth += dmult * command.Magnitude
		horizontal += hmult * command.Magnitude
	}
	return depth * horizontal, nil
}

type Position struct {
	Horizontal int
	Depth      int
	Aim        int
}

func Solve2(commands []Command) (int, error) {
	amap := map[Direction]int{Forward: 0, Up: -1, Down: 1}
	posn := Position{0, 0, 0}
	for _, command := range commands {
		amult, ok := amap[command.Dir]
		if !ok {
			return 0, &SolveError{Message: "Invalid direction"}
		}
		posn.Aim += amult * command.Magnitude
		if command.Dir == Forward {
			posn.Horizontal += command.Magnitude
			posn.Depth += posn.Aim * command.Magnitude
		}
	}
	return posn.Horizontal * posn.Depth, nil
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
