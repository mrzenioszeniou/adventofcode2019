package main

import "fmt"
import "os"
import "strconv"


func Day1() (int, int) {

	lines := ReadLines("input/day1.txt")

	data := []int {}
	for _, weightStr := range lines {
		weight, err := strconv.Atoi(weightStr)
		if err != nil {
			fmt.Println(err)
			os.Exit(1)
		}
		data = append(data, weight)		
	}

	day1 := 0
	for _, weight := range data {
		day1 += calcFuel(weight)
	}

	day2 := 0
	for _, weight := range data {
		last := calcFuel(weight)
		fuel := last

		for {
			lastFuel := calcFuel(last)
			if lastFuel < 0 {
				break
			} else {
				fuel += lastFuel
				last = lastFuel
			}
		}

		day2 += fuel	
	} 

	return day1,day2
}

func calcFuel(weight int) int {
	return weight / 3 - 2
}
