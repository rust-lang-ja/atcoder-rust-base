import itertools
import random
import string
from argparse import ArgumentParser
from subprocess import Popen, PIPE
from typing import List


def main() -> None:
    parser = ArgumentParser()
    parser.add_argument('bin')

    binary = parser.parse_args().bin

    for _ in range(100):
        judge(binary, ''.join(random.sample(string.ascii_uppercase, 26)))

    for balls in itertools.permutations(string.ascii_uppercase[:5]):
        judge(binary, ''.join(balls))


def judge(binary: str, balls: str) -> None:
    n = len(balls)
    q = 7 if n == 5 else 100

    with Popen([binary], stdin=PIPE, stdout=PIPE) as proc:
        def read_words() -> List[str]:
            return proc.stdout.readline().decode('utf-8').split()

        def on_query(c1: str, c2: str) -> None:
            reply = '<' if balls.index(c1) < balls.index(c2) else '>'
            proc.stdin.write(f'{reply}\n'.encode('utf-8'))
            proc.stdin.flush()

        def on_answer(ans: str) -> None:
            if ans != balls:
                raise Exception('wrong answer')

        proc.stdin.write(f'{n} {q}\n'.encode('utf-8'))
        proc.stdin.flush()

        for _ in range(q):
            words = read_words()
            if len(words) == 3 and words[0] == '?':
                on_query(words[1], words[2])
            elif len(words) == 2 and words[0] == '!':
                return on_answer(words[1])
            else:
                raise Exception('invalid')
        else:
            words = read_words()
            if len(words) == 2 and words[0] == '!':
                return on_answer(words[1])
            raise Exception('answer me')


if __name__ == '__main__':
    main()
