package main

import (
	"io/ioutil"
	"log"
	"os"
)

func main() {
	input, err := ioutil.ReadFile(os.Args[1])
	if err != nil {
		log.Fatal(err)
	}
	floor := 0
	for i, ch := range input {
		switch ch {
		case '(':
			floor++
		case ')':
			floor--
		}
		if floor == -1 {
			log.Println("-1st floor:", i+1)
		}
	}
	log.Println("floor:", floor)
}
