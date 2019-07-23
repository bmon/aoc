package main

import (
	"testing"
)

var cases = []struct {
	word string
	res  bool
}{
	{"ugknbfddgicrmopn", false},
	{"aaa", false},
	{"jchzalrnumimnmhp", true},
	{"haegwjzuvuyypxyu", true},
	{"dvszwmarrgswjxmb", true},
}

func TestIsNaughty(t *testing.T) {
	for _, tc := range cases {
		res := isNaughty(tc.word)
		if res != tc.res {
			t.Errorf("%q got: %v expected: %v", tc.word, res, tc.res)
		}
	}
}

var cases2 = []struct {
	word string
	res  bool
}{
	{"qjhvhtzxzqqjkmpb", false},
	{"xxyxx", false},
	{"uurcxstgmygtbstg", true},
	{"ieodomkazucvgmuy", true},
}

func TestIsNaughty2(t *testing.T) {
	for _, tc := range cases2 {
		res := isNaughty2(tc.word)
		if res != tc.res {
			t.Errorf("%q got: %v expected: %v", tc.word, res, tc.res)
		}
	}
}
