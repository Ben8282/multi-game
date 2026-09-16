import math
def next_prime(num):
    answer = num
    is_prime = False
    while is_prime == False:
        answer = answer + 1
        is_prime = True
        if answer == 1:
            is_prime = False
            continue
        sqrtnum = int(math.sqrt(answer))
        for i in range(2, sqrtnum + 1):
            if answer % i == 0:
                is_prime = False
                continue
        if is_prime == True:
            break
        else:
            continue
    return answer