package main

import (
	"bufio"
	"log"
	"os"
	"regexp"
)

func main() {
	input, err := os.Open(os.Args[1])
	if err != nil {
		log.Fatal(err)
	}

	nice := 0
	nice2 := 0
	scanner := bufio.NewScanner(input)
	for scanner.Scan() {
		word := scanner.Text()

		if !isNaughty(word) {
			nice++
		}
		if !isNaughty2(word) {
			nice2++
		}
	}
	log.Println("nice:", nice)
	log.Println("nice2:", nice2)
}

var vowels = regexp.MustCompile("[aeiou]")
var bads = regexp.MustCompile("ab|cd|pq|xy")

func isNaughty(word string) bool {
	nvowels := vowels.FindAllStringIndex(word, -1)
	if len(nvowels) < 3 {
		return true
	}
	nbads := bads.FindAllStringIndex(word, -1)
	if len(nbads) > 0 {
		return true
	}
	dupes := 0
	runes := []rune(word)
	for i, c := range runes[:len(runes)-1] {
		if c == runes[i+1] {
			dupes++
		}
	}
	return dupes == 0
}

func isNaughty2(word string) bool {
	runes := []rune(word)
	if len(runes) < 4 {
		return true
	}

	repeatGaps := 0
	for i, c := range runes[:len(runes)-2] {
		if c == runes[i+2] {
			repeatGaps++
		}
	}

	doubleRepeats := 0
	for i := range runes[:len(runes)-2] {
		sub := runes[i+2:]
		for j := range sub[:len(sub)-1] {
			if runes[i] == sub[j] && runes[i+1] == sub[j+1] {
				doubleRepeats++
			}
		}
	}

	return repeatGaps <= 0 || doubleRepeats <= 0
}
