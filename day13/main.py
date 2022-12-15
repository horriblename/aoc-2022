import re
import functools
from typing import List

Item = int | List["Item"]


class Tokenizer:
    def __init__(self, text: str) -> None:
        self.text = text
        self.index = 0
        self.reNum = re.compile(r"[0-9]+")

    def peek(self, chars: int = 1) -> str:
        return self.text[self.index : self.index + chars]

    def next(self, chars: int = 1) -> str:
        ret = self.text[self.index : self.index + chars]
        self.index += chars
        return ret

    def eatNum(self) -> str:
        match = self.reNum.search(self.text, self.index)
        if match is None or match.start() > self.index:
            return ""
        self.index = match.end()
        return self.text[match.start() : match.end()]


def parseList(tokens: Tokenizer) -> Item:
    ret = []
    tokens.next()  # eat '['

    while True:
        if tokens.peek() == "[":
            ret.append(parseList(tokens))
        elif tokens.peek().isdigit():
            ret.append(int(tokens.eatNum()))

        if tokens.peek() == "]":
            break

        tokens.next()  # eat comma

    tokens.next()  # eat '['

    return ret


def parse(text: str) -> Item:
    text = text.strip()
    if text == "":
        raise Exception("cannot parse empty text")
    tokens = Tokenizer(text)
    return parseList(tokens)


# -1 means left < right
def compare(left: Item, right: Item) -> int:
    if type(left) is int and type(right) is int:
        if left == right:
            return 0
        if left < right:
            return -1
        return 1

    if type(left) is int and type(right) is not int:
        return compare([left], right)

    if type(left) is not int and type(right) is int:
        return compare(left, [right])

    if type(left) is not list:
        raise Exception("Unreachable")
    if type(right) is not list:
        raise Exception("Unreachable")

    if len(left) == 0 and len(right) > 0:
        return -1

    if len(right) == 0 and len(left) > 0:
        return 1

    if 0 == len(left) == len(right):
        return 0

    compare_first = compare(left[0], right[0])
    if compare_first != 0:
        return compare_first

    return compare(left[1:], right[1:])


# in-place quicksort, because why not
def quickSort(items: list[Item]):
    qs(items, 0, len(items) - 1)


# lo and hi are inclusive
def qs(items: list[Item], lo: int, hi: int):
    if lo == hi:
        return

    mid = partition(items, lo, hi)
    qs(items, lo, mid - 1)
    qs(items, mid + 1, hi)


def partition(items: list[Item], lo: int, hi: int) -> int:
    pivot = items[hi]
    idx_hi = hi

    for idx_lo in range(lo, hi):
        if idx_lo >= idx_hi:
            break

        if compare(items[idx_lo], pivot) >= 0:
            idx_hi -= 1
            items[idx_hi], items[idx_lo] = items[idx_lo], items[idx_hi]

    idx_hi += 1
    items[idx_hi], items[hi] = items[hi], items[idx_hi]

    return idx_hi


def solve1(fname: str) -> int:
    count = 0

    with open(fname, "r") as file:
        pair_index = 1
        while True:
            left = parse(file.readline())
            right = parse(file.readline())

            if compare(left, right) == -1:
                count += pair_index

            if not file.readline():
                break

            pair_index += 1

    return count


def get_signals(fname: str) -> list[Item]:
    items: list[Item] = []
    with open(fname, "r") as file:
        while True:
            items.append(parse(file.readline()))
            items.append(parse(file.readline()))

            if not file.readline():
                break

    return items


def solve2(fname: str) -> int:
    items = get_signals(fname)
    items.append([[2]])
    items.append([[6]])

    items.sort(key=functools.cmp_to_key(compare))

    divider_idx = [0, 0]

    for i, item in enumerate(items, start=1):
        if item == [[2]]:
            divider_idx[0] = i
        elif divider_idx[0] != 0 and item == [[6]]:
            divider_idx[1] = i
            break

    return divider_idx[0] * divider_idx[1]


def main():
    x = solve1("input.txt")
    print(x)
    x = solve2("input.txt")
    print(x)


def testParseList():
    test_results = [
        [1, 1, 3, 1, 1],
        [1, 1, 5, 1, 1],
        [[1], [2, 3, 4]],
        [[1], 4],
        [9],
        [[8, 7, 6]],
        [[4, 4], 4, 4],
        [[4, 4], 4, 4, 4],
        [7, 7, 7, 7],
        [7, 7, 7],
        [],
        [3],
        [[[]]],
        [[]],
        [1, [2, [3, [4, [5, 6, 7]]]], 8, 9],
        [1, [2, [3, [4, [5, 6, 0]]]], 8, 9],
    ]

    with open("test_input.txt") as file:
        for res in test_results:
            line = file.readline()
            if line.strip() == "":
                line = file.readline()
            x = parse(line)
            assert x == res, f"expected {res}, got {x}"

    print("parse: ok")


def testSolve1():
    x = solve1("test_input.txt")
    assert x == 13, f"expected 13 got {x}"
    print("solve1: ok")


def testSort():
    x = get_signals("test_input.txt")
    x.append([[2]])
    x.append([[6]])
    x.sort(key=functools.cmp_to_key(compare))
    expected = [
        [],
        [[]],
        [[[]]],
        [1, 1, 3, 1, 1],
        [1, 1, 5, 1, 1],
        [[1], [2, 3, 4]],
        [1, [2, [3, [4, [5, 6, 0]]]], 8, 9],
        [1, [2, [3, [4, [5, 6, 7]]]], 8, 9],
        [[1], 4],
        [[2]],
        [3],
        [[4, 4], 4, 4],
        [[4, 4], 4, 4, 4],
        [[6]],
        [7, 7, 7],
        [7, 7, 7, 7],
        [[8, 7, 6]],
        [9],
    ]

    assert x == expected, "sorting is wrong"
    print("sort: ok")


def testSolve2():
    x = solve2("test_input.txt")
    assert x == 140, f"expected 140, got {x}"
    print("solve2: ok")


def tests():
    testParseList()
    testSolve1()
    testSort()
    testSolve2()


if __name__ == "__main__":
    tests()

    main()
