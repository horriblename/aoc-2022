package main

import (
	"bufio"
	"fmt"
	"log"
	"os"
)

type Grid = [][]uint8
type Coord = [2]int

func parse_input(fname string) (grid Grid, start, end Coord) {
	file, err := os.Open(fname)
	if err != nil {
		log.Fatal(err)
	}
	defer file.Close()

	scanner := bufio.NewScanner(file)
	grid = make(Grid, 0)
	var line_num int = 0

	for scanner.Scan() {
		line := scanner.Text()
		grid = append(grid, make([]uint8, len(line)))
		row := grid[len(grid)-1]

		for i, c := range line {
			if c == 'S' {
				row[i] = 0
				start = Coord{i, line_num}
				continue
			}
			if c == 'E' {
				row[i] = 'z' - 'a'
				end = Coord{i, line_num}
				continue
			}
			row[i] = uint8(c) - 'a'
		}

		line_num++
	}

	return grid, start, end
}

func valid_neighbors(coord Coord, grid *Grid) []Coord {
	neighbors := make([]Coord, 0)
	x := coord[0]
	y := coord[1]
	grid_width := len((*grid)[0])
	grid_height := len(*grid)

	if x > 0 && (*grid)[y][x-1] <= (*grid)[y][x]+1 {
		neighbors = append(neighbors, Coord{x - 1, y})
	}
	if x < grid_width-1 && (*grid)[y][x+1] <= (*grid)[y][x]+1 {
		neighbors = append(neighbors, Coord{x + 1, y})
	}
	if y > 0 && (*grid)[y-1][x] <= (*grid)[y][x]+1 {
		neighbors = append(neighbors, Coord{x, y - 1})
	}
	if y < grid_height-1 && (*grid)[y+1][x] <= (*grid)[y][x]+1 {
		neighbors = append(neighbors, Coord{x, y + 1})
	}
	return neighbors
}

func q_dequeue(q *[]Coord) Coord {
	x := (*q)[0]
	*q = (*q)[1:]
	return x
}

func solve1(grid *Grid, start, end Coord) int {
	queue := make([]Coord, 0, 1)
	queue = append(queue, start)

	shortest_dist := make([][]int, len(*grid))
	visited := make([][]bool, len(*grid))

	for j := range shortest_dist {
		dist_row := make([]int, len((*grid)[0]))
		visit_row := make([]bool, len((*grid)[0]))

		for i := range dist_row {
			dist_row[i] = int(^uint(0) >> 1) // Max integer value
			visit_row[i] = false
		}
		shortest_dist[j] = dist_row
		visited[j] = visit_row
	}
	shortest_dist[start[1]][start[0]] = 0
	visited[start[1]][start[0]] = true

	for len(queue) != 0 { // TODO
		visit := q_dequeue(&queue)
		neighbors := valid_neighbors(visit, grid)
		for _, neigh := range neighbors {
			if !visited[neigh[1]][neigh[0]] {
				queue = append(queue, neigh)
				visited[neigh[1]][neigh[0]] = true
			}
		}

		visitee_dist := shortest_dist[visit[1]][visit[0]]

		for _, neigh := range neighbors {
			shortest_dist[neigh[1]][neigh[0]] = min(
				shortest_dist[neigh[1]][neigh[0]],
				visitee_dist+1,
			)
		}
	}

	return shortest_dist[end[1]][end[0]]
}

func solve2(grid *Grid, end Coord) (shortest int) {
	shortest = int(^uint(0) >> 1)
	for y, row := range *grid {
		for x, cell := range row {
			if cell == 0 {
				shortest = min(
					shortest,
					solve1(grid, Coord{x, y}, end),
				)
			}
		}
	}

	return shortest
}

func min(a, b int) int {
	if a < b {
		return a
	}
	return b
}

func main() {
	grid, start, end := parse_input("input.txt")

	ans := solve1(&grid, start, end)
	fmt.Printf("%d", ans)

	ans = solve2(&grid, end)
	fmt.Printf("%d", ans)
}
