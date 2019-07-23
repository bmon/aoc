package main

import (
	"testing"
)

var cases = []struct {
	secret string
	target int
}{
	{"abcdef", 609043},
	{"pqrstuv", 1048970},
}

func TestFindMd5Sum(t *testing.T) {
	for _, tc := range cases {
		num := findMd5Sum([]byte(tc.secret), "00000")
		if num != tc.target {
			t.Errorf("got: %d expected: %d", num, tc.target)
		}
	}
}
