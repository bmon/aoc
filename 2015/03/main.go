package main

import (
	"io/ioutil"
	"log"
	"os"
)

type vertex struct {
	x int
	y int
}

func (v *vertex) move(direction byte) {
	switch direction {
	case '>':
		v.x++
	case '<':
		v.x--
	case '^':
		v.y++
	case 'v':
		v.y--
	default:
		log.Fatal("Got bad direction:", string(direction))
	}
}

func main() {
	input, err := ioutil.ReadFile(os.Args[1])
	if err != nil {
		log.Fatal(err)
	}

	santaVert := &vertex{0, 0}
	roboVert := &vertex{0, 0}
	isRobo := false
	presents := map[vertex]int{*santaVert: 1}
	for _, dir := range input {
		vert := santaVert
		if isRobo {
			vert = roboVert
		}

		vert.move(dir)
		count, ok := presents[*vert]
		if !ok {
			count = 0
		}
		presents[*vert] = count + 1

		isRobo = !isRobo
	}
	log.Println("houses:", len(presents))
}
