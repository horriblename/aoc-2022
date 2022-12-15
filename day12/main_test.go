package main

import (
	"reflect"
	"testing"
)

var gNeighborsTests = []struct {
	in  Coord
	out []Coord
}{
	{
		in: Coord{2, 2},
		out: []Coord{
			{1, 2},
			{2, 1},
			{2, 3},
		},
	},
	{
		in: Coord{1, 4},
		out: []Coord{
			{0, 4},
			{1, 3},
		},
	},
}

func TestNeighbors(t *testing.T) {
	grid, _, _ := example_grid()

	for _, test := range gNeighborsTests {
		ns := valid_neighbors(test.in, &grid)
		if !reflect.DeepEqual(ns, test.out) {
			t.Errorf("got %v", ns)
		}

	}
}

func TestParseInput(t *testing.T) {
	_, start, end := parse_input("test_input.txt")
	start_expected := Coord{0, 0}
	end_expected := Coord{5, 2}

	_ = Grid{
		[]uint8{0, 0, 1, 16, 15, 14, 13, 12},
		[]uint8{0, 1, 2, 17, 24, 23, 23, 11},
		[]uint8{0, 2, 2, 18, 25, 25, 23, 10},
		[]uint8{0, 2, 2, 19, 20, 2, 1, 22, 9},
		[]uint8{0, 1, 3, 4, 5, 6, 7, 8},
	}

	// how does DeepEqual work?

	// if !reflect.DeepEqual(grid, expected) {
	// 	t.Errorf("grid is wrong, got %v", grid)
	// }

	if !reflect.DeepEqual(start, Coord{0, 0}) {
		t.Errorf("start: expected %v, got %v", start_expected, start)
	}

	if !reflect.DeepEqual(end, Coord{5, 2}) {
		t.Errorf("end: expected %v, got %v", end_expected, end)
	}
}

func example_grid() (Grid, Coord, Coord) {
	return parse_input("test_input.txt")
}

func TestSolve1(t *testing.T) {
	grid, start, end := example_grid()
	ans := solve1(&grid, start, end)
	if ans != 31 {
		t.Errorf("expected 31, got %d", ans)
	}
}

func TestSolve2(t *testing.T) {
	grid, _, end := example_grid()
	ans := solve2(&grid, end)
	if ans != 29 {
		t.Errorf("expected 29, got %d", ans)
	}
}
