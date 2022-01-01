package main

import (
	"fmt"
	"log"

	"github.com/echojc/aocutil"
)

func main() {
	data, err := aocutil.NewInputFromFile("session_id")
	if err != nil {
		log.Fatal(err)
	}

	nums, err := data.Ints(2021, 1)
	if err != nil {
		log.Fatal(err)
	}
	fmt.Println(nums[:10])
	manipPart2(nums)
	fmt.Println(nums[:10])
	var last int
	increasing := 0
	for i, num := range nums {
		if i > 0 && num > last {
			increasing++
		}
		last = num
	}

	fmt.Printf("%d nums, %d increasing\n", len(nums), increasing)
}

func manipPart2(nums []int) {
	for i := range nums {
		if i+2 >= len(nums) {
			nums[i] = 0
		} else {
			nums[i] += nums[i+1] + nums[i+2]
		}
	}
}
