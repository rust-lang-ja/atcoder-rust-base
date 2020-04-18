from argparse import ArgumentParser
from subprocess import Popen, PIPE
from typing import List, Any


def main() -> None:
    parser = ArgumentParser()
    parser.add_argument('exe')
    exe = parser.parse_args().exe

    # Sample
    judge(exe, ['Male', 'Female', 'Vacant'])


def judge(exe: str, seats: List[str]) -> None:
    with Popen([exe], stdin=PIPE, stdout=PIPE) as proc:
        def write(content: Any) -> None:
            proc.stdin.write(f'{content}\n'.encode())
            proc.stdin.flush()

        write(len(seats))

        for _ in range(20):
            seat = seats[int(proc.stdout.readline().decode())]
            write(seat)
            if seat == 'Vacant':
                break
        else:
            raise Exception('run out')


if __name__ == '__main__':
    main()
