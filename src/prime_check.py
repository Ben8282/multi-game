import math
def prime_check(num):
    if num == 1:
        return False
    sqrtnum = int(math.sqrt(num))
    for i in range(2, sqrtnum + 1):
        if num % i == 0:
            return False
    return True