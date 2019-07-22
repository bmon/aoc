package main

import (
	"bufio"
	"log"
	"os"
	"strconv"
	"strings"
)

type present struct {
	length int
	width  int
	height int
}

func minInt(vars ...int) int {
	min := vars[0]

	for _, i := range vars {
		if min > i {
			min = i
		}
	}

	return min
}

func maxInt(vars ...int) int {
	max := vars[0]
	for _, i := range vars {
		if max < i {
			max = i
		}
	}
	return max
}

func decodeToPresent(data string) present {
	dimensions := strings.Split(data, "x")
	if len(dimensions) != 3 {
		log.Fatalln("Error decoding dimensions, expected 3, got:", len(dimensions), data)
	}

	var length, width, height int
	var err error
	if length, err = strconv.Atoi(dimensions[0]); err != nil {
		log.Fatal(err)
	}
	if width, err = strconv.Atoi(dimensions[1]); err != nil {
		log.Fatal(err)
	}
	if height, err = strconv.Atoi(dimensions[2]); err != nil {
		log.Fatal(err)
	}
	return present{length, width, height}
}

func (p present) getWrappingPaper() int {
	lw := p.length * p.width
	wh := p.width * p.height
	lh := p.length * p.height

	smallest := minInt(lw, wh, lh)
	return 2*(lw+wh+lh) + smallest
}

func (p present) getRibbon() int {
	perim := 2 * ((p.length + p.width + p.height) - maxInt(p.length, p.width, p.height))
	vol := p.width * p.length * p.height
	return perim + vol
}

func main() {
	input, err := os.Open(os.Args[1])
	if err != nil {
		log.Fatal(err)
	}

	scanner := bufio.NewScanner(input)
	paper := 0
	ribbon := 0
	for scanner.Scan() {
		p := decodeToPresent(scanner.Text())
		paper += p.getWrappingPaper()
		ribbon += p.getRibbon()
	}
	log.Println("total paper:", paper)
	log.Println("total ribbon:", ribbon)
}
