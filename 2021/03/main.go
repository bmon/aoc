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

	lines, err := data.Strings(2021, 3)
	if err != nil {
		log.Fatal(err)
	}

	q3(lines)
}

func q3(bits []string) {
	count1s := make([]int, len(bits[0]))
	fmt.Println(bits[:10])
	for _, num := range bits {
		for i, digit := range num {
			if digit == '1' {
				count1s[i]++
			}
		}
	}
	fmt.Println(count1s)
	mask := 0
	for i := range count1s {
		digit := 0
		if count1s[i] > len(bits)/2 {
			digit = 1
		}
		mask = mask<<1 | digit
	}
	gamma := mask
	epsilon := mask ^ 0b111111111111
	fmt.Printf("gamma: %012b (%d) eps: %012b (%d) product: %d\n", gamma, gamma, epsilon, epsilon, gamma*epsilon)
}
