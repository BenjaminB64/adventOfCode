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
	expectedTest := 1227775554
	step_one_test := step_one(test)
	if step_one_test != expectedTest {
		t.Errorf("Step one test failed: expected %d, got %d", expectedTest, step_one_test)
	}

	expectedChall := 54234399924
	step_one_chall := step_one(chall)
	if step_one_chall != expectedChall {
		t.Errorf("Step one chall failed: expected %d, got %d", expectedChall, step_one_chall)
	}
}

func TestStepTwo(t *testing.T) {
	expectedTest := 4174379265
	step_two_test := step_two(test)
	if step_two_test != expectedTest {
		t.Errorf("Step two test failed: expected %d, got %d", expectedTest, step_two_test)
	}

	expectedChall := 4174379265
	step_two_chall := step_two(chall)
	if step_two_chall != expectedChall {
		t.Errorf("Step two chall failed: expected %d, got %d", expectedChall, step_two_chall)
	}
}

func TestCheckIfSequenceIsRepeated(t *testing.T) {
	if checkIfSequenceIsRepeated("123123", "123") {
		t.Errorf("expected true, got false")
	}
}
