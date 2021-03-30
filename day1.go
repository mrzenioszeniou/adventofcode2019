package main

import "fmt"
import "os"
import "strconv"


func Day1() (int, int) {

	data := ReadLines("input/day1.txt")

	total := 0

	for _, weightStr := range data {
		weight, err := strconv.Atoi(weightStr)
		if err != nil {
			fmt.Println(err)
			os.Exit(1)
		}
		total += calcFuel(weight)
	}

	return total,-1
}

func calcFuel(weight int) int {
	return weight / 3 - 2
}
