# python version 3.10.8: can't refer to Self type in classes
from __future__ import annotations
import unittest


Coord = tuple[int, int]


def parse_input(fname: str) -> list[tuple[int, int, int, int]]:
    with open(fname) as file:
        return list(map(parse_line, file))


def parse_line(line: str) -> tuple[int, int, int, int]:
    splitted = line.split(" ")
    x_sen = int(splitted[2][2:-1])
    y_sen = int(splitted[3][2:-1])
    x_b = int(splitted[8][2:-1])
    y_b = int(splitted[9][2:])
    return (x_sen, y_sen, x_b, y_b)


#   01234
# 0 S****
# 1     *
# 2     E <- distance = 6
def manhattanDist(a: Coord, b: Coord) -> int:
    return abs(a[0] - b[0]) + abs(a[1] - b[1])


# given a sensor, its radius in which no sensors exist(the known sensor will have
# to be dealt with outside the function), and a row number, gives the start and
# end values of x where no sensor exists
def rangeEmptyOnLine(sensor: Coord, radius: int, row: int) -> tuple[int, int]:
    assert abs(sensor[1] - row) <= radius
    dist_to_row = abs(sensor[1] - row)
    dist_from_border = radius - dist_to_row  # 0 means on the border

    start = sensor[0] - dist_from_border
    end = sensor[0] + dist_from_border

    return (start, end)


def solve1(fname: str, row_inspect: int) -> int:
    report = parse_input(fname)
    row_empty: set[int] = set()

    for (xs, ys, xb, yb) in report:
        radius = manhattanDist((xs, ys), (xb, yb))
        if abs(ys - row_inspect) > radius:
            continue

        start, end = rangeEmptyOnLine((xs, ys), radius, row_inspect)

        for i in range(start, end + 1):
            row_empty.add(i)

        if yb == row_inspect:
            row_empty.discard(xb)

    return len(row_empty)


class Range:
    def __init__(self, start, end):
        self.start = start
        self.end = end

    def __repr__(self) -> str:
        return f"Range({self.start}, {self.end})"

    def merge(self, other: Range) -> bool:
        if (other.start <= self.end and other.end > self.start) or (
            other.end >= self.start and other.start < self.end
        ):

            self.end = max(self.end, other.end)
            self.start = min(self.start, other.start)
            return True

        return False


class Sensor:
    def __init__(self, x, y, xb, yb):
        self.x = x
        self.y = y
        self.xb = xb
        self.yb = yb

    def radius(self) -> int:
        return manhattanDist((self.x, self.y), (self.xb, self.yb))

    def getColsCoverRange(self, row: int) -> Range:
        rad = self.radius()
        if abs(self.y - row) > rad:
            return Range(-1, -1)
        return Range(*rangeEmptyOnLine((self.x, self.y), rad, row))


def tuningFreq(x: int, y: int) -> int:
    return 4000000 * x + y


def solve2(fname: str, size_limit: int) -> int:
    report = parse_input(fname)
    sensors = list(map(lambda coords: Sensor(*coords), report))

    for j in range(0, size_limit):
        emptyRangeMap = map(lambda s: s.getColsCoverRange(j), sensors)
        emptyRanges: list[Range] = list(
            filter(lambda coords: coords.start >= 0 or coords.end >= 0, emptyRangeMap)
        )
        emptyRanges.sort(key=lambda rg: rg.start)

        rg = emptyRanges[0]
        for other in emptyRanges[1:]:
            if rg.end > size_limit:
                break

            if rg.merge(other):
                continue

            sus_from = rg.end + 1
            sus_till = other.start - 1
            assert sus_from == sus_till
            return tuningFreq(sus_from, j)

    return -1


class TestSum(unittest.TestCase):
    def testParse(self):
        expected = [
            (2, 18, -2, 15),
            (9, 16, 10, 16),
            (13, 2, 15, 3),
            (12, 14, 10, 16),
            (10, 20, 10, 16),
            (14, 17, 10, 16),
            (8, 7, 2, 10),
            (2, 0, 2, 10),
            (0, 11, 2, 10),
            (20, 14, 25, 17),
            (17, 20, 21, 22),
            (16, 7, 15, 3),
            (14, 3, 15, 3),
            (20, 1, 15, 3),
        ]
        self.assertEqual(parse_input("test_input.txt"), expected, "Failed parse input")

    def testSolve1(self):
        ans = solve1("test_input.txt", 10)
        self.assertEqual(ans, 26, "")

    def testRange(self):
        tests: list[tuple] = [  # merges, no subsets of one another
            ((Range(0, 5), Range(5, 10)), {"start": 0, "end": 10, "ret": True}),
            ((Range(5, 10), Range(0, 5)), {"start": 0, "end": 10, "ret": True}),
            ((Range(5, 10), Range(0, 5)), {"start": 0, "end": 10, "ret": True}),
            # merges, subset of one another
            ((Range(0, 10), Range(5, 7)), {"start": 0, "end": 10, "ret": True}),
            ((Range(5, 7), Range(0, 10)), {"start": 0, "end": 10, "ret": True}),
            ((Range(0, 10), Range(0, 10)), {"start": 0, "end": 10, "ret": True}),
            # no merge, disjunctive
            ((Range(0, 5), Range(6, 10)), {"start": 0, "end": 5, "ret": False}),
            ((Range(6, 10), Range(0, 5)), {"start": 6, "end": 10, "ret": False}),
        ]

        for i, t in enumerate(tests, 1):
            r = t[0][0]
            res = r.merge(t[0][1])
            self.assertEqual(
                res, t[1]["ret"], f"test #{i}: got Range({r.start}, {r.end})"
            )
            self.assertEqual(r.start, t[1]["start"], f"test #{i}")
            self.assertEqual(r.end, t[1]["end"], f"test #{i}")

    def testSensor(self):
        class Test:
            def __init__(self, inp, out):
                self.inp = inp
                self.out = out

        tests = [
            # ((Sensort(x, y, xb, yb), row_inspect), (start, end))
            Test((Sensor(8, 7, 2, 10), 7), (-1, 17)),
            Test((Sensor(8, 7, 2, 10), 0), (6, 10)),
            Test((Sensor(8, 7, 2, 10), 16), (8, 8)),
            Test((Sensor(8, 7, 2, 10), 17), (-1, -1)),
        ]

        for t in tests:
            ans = t.inp[0].getColsCoverRange(t.inp[1])
            self.assertEqual(ans.start, t.out[0])
            self.assertEqual(ans.end, t.out[1])

    def testSolve2(self):
        ans = solve2("test_input.txt", 20)
        self.assertEqual(ans, 56000011)


def main():
    ans = solve1("input.txt", 2000000)
    print(ans)
    ans = solve2("input.txt", 4000000)
    print(ans)


if __name__ == "__main__":
    # unittest.main()
    main()
