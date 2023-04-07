#! /usr/bin/python
# -*- coding: utf-8 -*-

import math


def calc(base, p=0.05, t=1):

    for x in range(t):
        base = base * (1 + p)
        print(x, base)
    return base


def calcV1(base, b=10000, p=0.05):
    r = base * b

    print(base, r)
    for x in range(10000):
        base = base * (1 + p)
        if base > r:
            break
    return base, x


# print(calc(7000, 0.05, 100))

print(calcV1(100000, 100, 0.15))

# print(math.pow(2, 10))
