ORD_a = ord("a")
ORD_A = ord("A")
NUM_OF_TYPES = (ord("z") - ord("a") + 1) * 2


def priority(char: str):
    o = ord(char)
    if o >= ORD_a:
        return o - ORD_a + 1

    return o - ORD_A + 27


def main():
    total = 0
    with open("input.txt", "r") as fh:
        for line in fh.readlines():
            line = line.strip()
            comp1 = set(line[: len(line) // 2])
            comp2 = set(line[len(line) // 2 :])
            rep = comp1.intersection(comp2)

            for item in rep:
                total += priority(item)

    print(total)


def task2():
    total = 0
    with open("input.txt", "r") as fh:
        ln = 0
        try:
            while fh:
                m1 = set(fh.readline().strip())
                m2 = set(fh.readline().strip())
                m3 = set(fh.readline().strip())

                common = m1.intersection(m2).intersection(m3)
                total += priority(common.pop())
                ln += 3
        except KeyError:
            ...

    print(total)


if __name__ == "__main__":
    task2()
