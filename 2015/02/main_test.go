package main

import "testing"

var paperTests = []struct {
	input  string
	output int
}{
	{"2x3x4", 58},
	{"1x1x10", 43},
}

func TestPresentPaper(t *testing.T) {
	for _, tc := range paperTests {
		present := decodeToPresent(tc.input)
		if tc.output != present.getWrappingPaper() {
			t.Errorf("Got: %d, expected: %d", present.getWrappingPaper(), tc.output)
		}
	}
}

var ribbonTests = []struct {
	input  string
	output int
}{
	{"2x3x4", 34},
	{"1x1x10", 14},
}

func TestPresentRibbon(t *testing.T) {
	for _, tc := range ribbonTests {
		present := decodeToPresent(tc.input)
		if tc.output != present.getRibbon() {
			t.Errorf("Got: %d, expected: %d", present.getRibbon(), tc.output)
		}
	}
}
