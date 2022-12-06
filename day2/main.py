ORD_A = ord("A")
ORD_X = ord("X")


def score(op: int, me: int):
    s = me + 1

    # draw
    if op == me:
        return s + 3

    # win
    if (me - op) % 3 == 1:
        return s + 6

    # lose
    return s


# part 2 of the question
def score2(op: int, res: int):
    if res == 0:
        s = (op - 1) % 3 + 1
    elif res == 1:
        s = op + 1
    else:
        s = (op + 1) % 3 + 1
    return res * 3 + s


def main():
    total_score = 0
    with open("input.txt", "r") as fh:
        for line in fh.readlines():
            op = ord(line[0]) - ORD_A
            me = ord(line[2]) - ORD_X

            # switch between `score` and `score2` depending on which part
            # you're doing
            total_score += score2(op, me)

    print(total_score)


if __name__ == "__main__":
    main()
