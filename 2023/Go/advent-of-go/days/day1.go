package days

import (
	"advent-of-go/get_input"
	"fmt"
	"regexp"
	"strconv"
	"strings"
)

func Day1() {
	content := get_input.GetInput(get_input.AoCDate{Year: 2023, Day: 1})
	sum_part1 := 0
	sum_part2 := 0
	re := regexp.MustCompile("[0-9]")
	for _, line := range strings.Split(strings.TrimSuffix(content, "\n"), "\n") {
		digits := re.FindAllString(line, -1)
		first, err := strconv.Atoi(digits[0])
		last, err := strconv.Atoi(digits[len(digits)-1])

		if err != nil {
			fmt.Println(err)
		} else {
			sum_part1 += first*10 + last
		}

	}
	fmt.Println("Day 1 - Part 1", sum_part1)

	fmt.Println("Day 1 - Part 2", sum_part2)
}
