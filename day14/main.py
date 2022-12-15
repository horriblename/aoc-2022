import numpy as np

TYPE = int


class Grid:
    AIR = 0
    ROCK = 1
    SAND = 2

    def __init__(self) -> None:
        # self.x_left = 0
        # self.x_right = 1
        # self.y_height = 1
        self.right_grid = np.array([[self.AIR]]).astype(TYPE)
        self.left_grid = np.array([[]]).astype(TYPE)

    def grid_height(self) -> int:
        return len(self.right_grid)

    def left_width(self) -> int:
        return self.left_grid.shape[1]

    def right_width(self) -> int:
        return self.right_grid.shape[1]

    def grid_width(self) -> int:
        return self.left_width() + self.right_width()

    def get(self, x: int, y: int) -> TYPE:
        if x < 500:
            return self.left_grid[y][499 - x]
        return self.right_grid[y][x - 500]

    def set(self, x: int, y: int, val: TYPE):
        if x < 500:
            self.left_grid[y][499 - x] = val
            return
        self.right_grid[y][x - 500] = val
        return

    def maybe_extend_width(self, x_lo: int, x_hi, y: int) -> bool:
        size = 500 - x_lo
        if x_lo < 500 and self.left_width() < size:
            self.left_grid = np.pad(self.left_grid, (size, self.grid_height()))

        size = x_hi - 499
        if x_hi > 500 and self.right_width() < size:
            self.right_grid = np.pad(self.right_grid, (size, self.grid_height()))

        if y > self.grid_height():
            self.left_grid = np.pad(self.left_grid, ())


def buildGrid(fname: str):
    x_left = 0
    x_right = 1  # x=500
    y_height = 1

    grid = np.ndarray

    with open(fname) as file:
        for line in file.readlines():
            vertices = line.split(" -> ")


def main():
    buildGrid("input.txt")
    pass


# Tests


def test():
    pass


if __name__ == "__main__":
    test()
    main()
