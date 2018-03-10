# from random import randint

def mult(a, b, n=10):
    al = a[1]
    au = a[0]
    bl = b[1]
    bu = b[0]

    part_1 = (al * bl) + ((au * bl) * (n))
    part_2 = (al * bu) + ((au * bu) * (n))

    return (part_1) + (part_2 * n)

def test(num, c):
    a = num[0]
    b = num[1]
    return (b*c) + ((a*c) * (10**1))

def test_the_test():
    for a in range(10):
        for b in range(10):
            for c in range(10):
                num = b + (a * 10)
                if (num * c) != test((a,b), c):
                    print("Oh no!! {},{} * {} failed!".format(a, b, c))
    print("Done")

def test_the_mult():
    for a in range(10):
        for b in range(10):
            for c in range(10):
                for d in range(10):
                    numA = b + (a * 10)
                    numB = d + (c * 10)
                    if (numA * numB) != mult((a,b), (c,d)):
                        print("Oh no!! {},{} * {},{} failed!".format(a, b, c, d))
    print("Done")

test_the_test()
test_the_mult()


