package main

import (
	_ "embed"
	"log/slog"
	"math/big"
	"testing"

	testloghandler "github.com/BenjaminB64/adventOfCode/2025/3/testLogHandler"
)

//go:embed inputs/input.txt
var chall string

//go:embed inputs/test.txt
var test string

func TestStepOne(t *testing.T) {
	handler := &testloghandler.TestLogHandler{T: t, Level: slog.LevelInfo}
	slog.SetDefault(slog.New(handler))
	expectedTest := 4277556
	step_one_test := step_one(test)
	if step_one_test.Cmp(big.NewInt(int64(expectedTest))) != 0 {
		t.Errorf("Step one test failed: expected %d, got %d", expectedTest, step_one_test)
	}

	expectedChall := 4951502530386
	step_one_chall := step_one(chall)
	if step_one_chall.Cmp(big.NewInt(int64(expectedChall))) != 0 {
		t.Errorf("Step one chall failed: expected %d, got %d", expectedChall, step_one_chall)
	}
}

func TestStepTwo(t *testing.T) {
	handler := &testloghandler.TestLogHandler{T: t, Level: slog.LevelInfo}
	slog.SetDefault(slog.New(handler))
	expectedTest := 3263827
	step_two_test := step_two(test)
	if step_two_test.Cmp(big.NewInt(int64(expectedTest))) != 0 {
		t.Errorf("Step two test failed: expected %d, got %d", expectedTest, step_two_test)
	}

	expectedChall := 8486156119946
	step_two_chall := step_two(chall)
	if step_two_chall.Cmp(big.NewInt(int64(expectedChall))) != 0 {
		t.Errorf("Step two chall failed: expected %d, got %d", expectedChall, step_two_chall)
	}
}
