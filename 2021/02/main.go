package main

import (
	"fmt"
	"log"
	"strconv"
	"strings"

	"github.com/echojc/aocutil"
)

func main() {
	data, err := aocutil.NewInputFromFile("session_id")
	if err != nil {
		log.Fatal(err)
	}

	lines, err := data.Strings(2021, 2)
	if err != nil {
		log.Fatal(err)
	}

	q2(lines)
}

func q2(instructions []string) {
	fmt.Println(instructions[:10])
	var hpos, depth, aim int
	for i := range instructions {
		cmd, valStr, _ := strings.Cut(instructions[i], " ")
		val, _ := strconv.Atoi(valStr)
		switch cmd {
		case "forward":
			hpos += val
			depth += val * aim
		case "down":
			aim += val
		case "up":
			aim -= val
		}
	}
	fmt.Printf("hpos: %d depth: %d product: %d\n", hpos, depth, hpos*depth)
}
