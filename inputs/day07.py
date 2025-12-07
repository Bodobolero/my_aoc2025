# copyright Frank Neumann 2025
import time

with open("input07.txt") as f:
    input = f.readlines()
# start measurement after reading input because in Rust we read input at compile time
start=time.perf_counter()
input = [[y for y in x.strip()] for x in input]
ways = [[0] * len(input[0]) for x in input]

def move_beam(ys, xs):
    if xs < 0 or xs >= len(input[0]):
        return 0
    if ys == len(input):
        return 1
    if input[ys][xs] == '.':
        input[ys][xs] = '|'
        w = move_beam(ys+1, xs)
        ways[ys][xs] = w
        return w
    elif input[ys][xs] == '^':
        w1 = move_beam(ys+1, xs-1)
        w2 = move_beam(ys+1, xs+1)
        return w1 + w2
    elif input[ys][xs] == '|':
        return ways[ys][xs]

number_paths = move_beam(1, input[0].index("S"))
end=time.perf_counter()
print (f"Time: {end-start:.6f} seconds")
print(f"Number paths: {number_paths}")