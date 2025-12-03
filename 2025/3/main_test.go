package main

import (
	_ "embed"
	"log/slog"
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
	expectedTest := 357
	step_one_test := step_one(test)
	if step_one_test != expectedTest {
		t.Errorf("Step one test failed: expected %d, got %d", expectedTest, step_one_test)
	}

	expectedChall := 17346
	step_one_chall := step_one(chall)
	if step_one_chall != expectedChall {
		t.Errorf("Step one chall failed: expected %d, got %d", expectedChall, step_one_chall)
	}
}

func TestStepTwo(t *testing.T) {
	handler := &testloghandler.TestLogHandler{T: t, Level: slog.LevelInfo}
	slog.SetDefault(slog.New(handler))
	expectedTest := 3121910778619
	step_two_test := step_two(test)
	if step_two_test != expectedTest {
		t.Errorf("Step two test failed: expected %d, got %d", expectedTest, step_two_test)
	}

	expectedChall := 172981362045136
	step_two_chall := step_two(chall)
	if step_two_chall != expectedChall {
		t.Errorf("Step two chall failed: expected %d, got %d", expectedChall, step_two_chall)
	}
}

func TestFindShortestLine(t *testing.T) {
	handler := &testloghandler.TestLogHandler{T: t, Level: slog.LevelDebug}
	slog.SetDefault(slog.New(handler))
	expectedTest := "811111111119"
	shortestLine := getMaxWithMinLength("811111111111119", 12)
	if shortestLine != expectedTest {
		t.Errorf("Find shortest line test failed: expected %s, got %s", expectedTest, shortestLine)
	}

	expectedChall := "434234234278"
	shortestLine = getMaxWithMinLength("234234234234278", 12)
	if shortestLine != expectedChall {
		t.Errorf("Find shortest line chall failed: expected %s, got %s", expectedChall, shortestLine)
	}
	expectedChall = "987654321111"
	shortestLine = getMaxWithMinLength("987654321111111", 12)
	if shortestLine != expectedChall {
		t.Errorf("Find shortest line chall failed: expected %s, got %s", expectedChall, shortestLine)
	}

	expectedChall = "888911112111"
	shortestLine = getMaxWithMinLength("818181911112111", 12)
	if shortestLine != expectedChall {
		t.Errorf("Find shortest line chall failed: expected %s, got %s", expectedChall, shortestLine)
	}

	expectedChall = "999893222423"
	shortestLine = getMaxWithMinLength("2215452689925244273244333436189317446384838478525478824435233342352236255624326767355438753493222423", 12)
	if shortestLine != expectedChall {
		t.Errorf("Find shortest line chall failed: expected %s, got %s", expectedChall, shortestLine)
	}
}
