from math import *

nan = float('NaN')
inf = float('Inf')

def eq(left, right):
    return left == right

def ne(left, right):
    return left != right

def lt(left, right):
    return left < right

def le(left, right):
    return left <= right

def gt(left, right):
    return left > right

def ge(left, right):
    return left >= right

func_map = {
    "==": eq,
    "!=": ne,
    "< ": lt,
    "<=": le,
    "> ": gt,
    ">=": ge,
}

compare = [2.0, nan, -nan, inf, -inf]
values = [nan, -nan, inf, -inf]

for i in range(len(values)):
    for j in range(len(compare)):
        for op in func_map:
            print("%g %s %g -> %s" %
                    (values[i], op, compare[j], func_map[op](values[i], compare[j])))

        print("")
