package main

import (
	"bufio"
	"fmt"
	"os"
)

func main() {
	matrix := parse("input.txt")
	reachableScoreSum, distinctScoreSum := traverse(matrix)

	fmt.Println(reachableScoreSum)
	fmt.Println(distinctScoreSum)
}

func traverse(matrix [][]int) (int, int) {
	// Part 1
	reachableScoreSum := 0

	// Part 2
	distinctScoreSum := 0

	for row := 0; row < len(matrix); row++ {
		for col := 0; col < len(matrix[row]); col++ {
			if matrix[row][col] == 0 {
				reachableScore, distinctScore := countTrailheadScore(matrix, row, col)
				reachableScoreSum += reachableScore
				distinctScoreSum += distinctScore

			}
		}
	}

	return reachableScoreSum, distinctScoreSum
}

type pos struct {
	row int
	col int
}

func countTrailheadScore(matrix [][]int, row int, col int) (int, int) {
	// Part 1
	endPositionSet := map[pos]bool{}

	// Part 2
	distinctTrailHeads := 0

	var lookFor func(int, int, int)

	lookFor = func(row int, col int, next int) {
		if next == 10 {
			endPositionSet[pos{row: row, col: col}] = true
			distinctTrailHeads++
			return
		}

		if row > 0 && matrix[row-1][col] == next {
			lookFor(row-1, col, next+1)
		}
		if row < len(matrix)-1 && matrix[row+1][col] == next {
			lookFor(row+1, col, next+1)
		}
		if col > 0 && matrix[row][col-1] == next {
			lookFor(row, col-1, next+1)
		}
		if col < len(matrix[row])-1 && matrix[row][col+1] == next {
			lookFor(row, col+1, next+1)
		}
	}

	lookFor(row, col, 1)

	return len(endPositionSet), distinctTrailHeads
}

func parse(path string) [][]int {
	blob, err := os.Open(path)
	defer blob.Close()

	if err != nil {
		panic("Can't open path")
	}

	scanner := bufio.NewScanner(blob)

	matrix := [][]int{}

	for scanner.Scan() {
		line := scanner.Text()

		numbers := make([]int, len(line))
		for i, char := range line {
			numbers[i] = int(char - '0')
		}

		matrix = append(matrix, numbers)
	}

	return matrix
}
