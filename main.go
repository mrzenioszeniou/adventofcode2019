package main

import (
	"fmt"
	"os"
)

func main() {

	if len(os.Args) != 2 {
		print_usage()
		return
	}


	var part1, part2 int
	switch os.Args[1] {
		case "1":
			part1, part2 = Day1()
		default:
		  fmt.Println("No solution available for day " + os.Args[1])
		  return
	}
	
  fmt.Printf("Part 1: %d\nPart 2: %d\n", part1, part2)
}

func print_usage() {
	fmt.Println(`Usage: adventofcode2019 DAY
	  DAY - The day for which you want the solution (int)`)
}
