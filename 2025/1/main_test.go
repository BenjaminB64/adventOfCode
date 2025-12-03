package main

import (
	_ "embed"
	"testing"
)

//go:embed inputs/input.txt
var chall string

//go:embed inputs/test.txt
var test string

func TestStepOne(t *testing.T) {
	step_one_test := step_one(test)
	if step_one_test != 3 {
		t.Errorf("Step one failed: expected 3, got %d", step_one_test)
	}
	step_one_chall := step_one(chall)
	if step_one_chall != 1092 {
		t.Errorf("Step one failed: expected 3, got %d", step_one_chall)
	}
}

func TestStepTwo(t *testing.T) {
	step_two_test := step_two(test)
	if step_two_test != 6 {
		t.Errorf("Step two failed: expected 6, got %d", step_two_test)
	}
	step_two_chall := step_two(chall)
	if step_two_chall != 6616 {
		t.Errorf("Step two failed: expected 3, got %d", step_two_chall)
	}
}
