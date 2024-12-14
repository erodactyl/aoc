package main

import (
	"bufio"
	"fmt"
	"os"
)

func main() {
	numbers := parseFile("input.txt")

	// Part 1
	// disk := createDisk(numbers)
	// rearranged := rearrangeDiskPart1(disk)
	// checksum := checksum(rearranged)
	// fmt.Println(checksum)

	// Part 2
	disk := rearrangeDiskPart2(numbers)
	checksum := checksum(disk)
	fmt.Println(checksum)
}

type slot struct {
	index  int
	length int
}

type file struct {
	index  int
	length int
	id     int
}

func findFilesAndEmptySlots(metadisk []int) ([]file, []slot) {
	files := []file{}
	slots := []slot{}
	index := 0
	fileId := 0

	for i := 0; i < len(metadisk); i++ {
		length := metadisk[i]
		if i%2 == 0 {
			file := file{
				index:  index,
				length: length,
				id:     fileId,
			}
			files = append(files, file)
			fileId++
		} else {
			slot := slot{
				index:  index,
				length: length,
			}
			slots = append(slots, slot)
		}

		index += length
	}

	return files, slots
}

func rearrangeDiskPart2(metadisk []int) []int {
	files, slots := findFilesAndEmptySlots(metadisk)
	right := len(files) - 1
	disk := createDisk(metadisk)

	for right > 0 {
		file := files[right]

		for left := 0; slots[left].index < file.index; left++ {
			slot := &slots[left]

			if slot.length >= file.length {
				// Write file to slot
				for i := slot.index; i < slot.index+file.length; i++ {
					disk[i] = file.id
				}

				// Remove original file
				for i := file.index; i < file.index+file.length; i++ {
					disk[i] = -1
				}

				slot.length = slot.length - file.length
				slot.index = slot.index + file.length

				break
			}
		}

		right--
	}

	return disk
}

func rearrangeDiskPart1(disk []int) []int {
	left := 0
	right := len(disk) - 1

	for left < right {
		if disk[left] != -1 {
			left++
		} else if disk[right] == -1 {
			right--
		} else {
			disk[left] = disk[right]
			disk[right] = -1
		}
	}

	return disk
}

func checksum(disk []int) int {
	sum := 0
	for i, num := range disk {
		if num != -1 {
			sum += i * num
		}
	}
	return sum
}

func createDisk(numbers []int) []int {
	isFile := true
	fileIndex := 0

	disk := []int{}

	for _, num := range numbers {
		var fill int
		if isFile {
			fill = fileIndex
			fileIndex++
		} else {
			fill = -1
		}
		isFile = !isFile

		for i := 0; i < num; i++ {
			disk = append(disk, fill)
		}

	}

	return disk
}

func parseFile(path string) []int {
	file, err := os.Open(path)
	if err != nil {
		panic("Can't read file")
	}
	defer file.Close()

	scanner := bufio.NewScanner(file)
	scanner.Scan()
	line := scanner.Text()

	numbers := make([]int, len(line))

	for i, char := range line {
		numbers[i] = int(char - '0')
	}

	return numbers
}
