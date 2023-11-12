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

type BingoCard struct {
	Board   [5][5]int
	Covered [5][5]bool
	Rows    [5]int
	Columns [5]int
}

const cardSize int = 5

func CreateBingoCard(cardStr []string) (BingoCard, error) {
	var card BingoCard
	for i := 0; i < cardSize; i++ {
		card.Rows[i] = 5
		card.Columns[i] = 5
	}
	for y, row := range cardStr {
		squares := strings.Fields(row)
		for x, square := range squares {
			squareVal, err := strconv.Atoi(square)
			if err != nil {
				return card, err
			}
			card.Board[y][x] = squareVal
		}
	}
	return card, nil
}

func (card *BingoCard) Reset() {
	for i := 0; i < cardSize; i++ {
		card.Rows[i] = 5
		card.Columns[i] = 5
	}
	card.Covered = [5][5]bool{}
}

func (card *BingoCard) CoverSquare(val int) bool {
	for y, row := range card.Board {
		for x, square := range row {
			if square == val {
				card.Covered[y][x] = true
				card.Rows[y]--
				card.Columns[x]--
				if card.Rows[y] == 0 || card.Columns[x] == 0 {
					return true
				}
			}
		}
	}
	return false
}

func (card BingoCard) CountUncovered() int {
	sum := 0
	for y, row := range card.Board {
		for x, _ := range row {
			if !card.Covered[y][x] {
				sum += card.Board[y][x]
			}
		}
	}
	return sum
}

func (card BingoCard) String() string {
	str := ""
	for i, row := range card.Board {
		str += fmt.Sprintf("%v", row)
		if i < len(card.Board)-1 {
			str += "\n"
		}
	}
	return str
}

func ParseLines(lines []string) ([]int, []BingoCard, error) {
	nums := make([]int, 0)
	cards := make([]BingoCard, 0)

	numsLine := strings.Split(lines[0], ",")
	for _, numStr := range numsLine {
		num, err := strconv.Atoi(numStr)
		if err != nil {
			return nums, cards, err
		}
		nums = append(nums, num)
	}

	cardStrs := make([][]string, 0)
	for _, line := range lines[1:] {
		if line == "" {
			cardStrs = append(cardStrs, make([]string, 0))
			continue
		}
		lastCard := &cardStrs[len(cardStrs)-1]
		*lastCard = append(*lastCard, line)
	}

	for _, cardStr := range cardStrs {
		card, err := CreateBingoCard(cardStr)
		if err != nil {
			return nums, cards, err
		}
		cards = append(cards, card)
	}

	return nums, cards, nil
}

func Solve1(nums []int, cards []BingoCard) (int, error) {
	for i := range cards {
		cards[i].Reset()
	}
	for _, num := range nums {
		for i := range cards {
			if cards[i].CoverSquare(num) {
				soln := cards[i].CountUncovered() * num
				return soln, nil
			}
		}
	}
	return 0, nil
}

func Solve2(nums []int, cards []BingoCard) (int, error) {
	for i := range cards {
		cards[i].Reset()
	}
	steps := make([]int, len(cards))
	last := make([]int, len(cards))
	for i := range cards {
		for j, num := range nums {
			if cards[i].CoverSquare(num) {
				steps[i] = j + 1
				last[i] = num
				break
			}
		}
	}

	max := 0
	argmax := 0
	for i, v := range steps {
		if v > max {
			max = v
			argmax = i
		}
	}

	soln := cards[argmax].CountUncovered() * last[argmax]
	return soln, nil
}

func main() {
	const path string = "input.txt"

	lines, err := ReadLines(path)
	if err != nil {
		fmt.Printf("Scann error: %v", err)
		return
	}

	nums, cards, err := ParseLines(lines)
	if err != nil {
		fmt.Printf("Parse error: %v", err)
		return
	}

	soln1, err := Solve1(nums, cards)
	if err != nil {
		fmt.Printf("Solve error: %v", err)
		return
	}

	fmt.Println("Solution (1):", soln1)

	soln2, err := Solve2(nums, cards)
	if err != nil {
		fmt.Printf("Solve error: %v", err)
		return
	}

	fmt.Println("Solution (2):", soln2)
}
