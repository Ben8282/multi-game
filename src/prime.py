import prime_check
print("welcome to the prime number checker")
print("Enter a number to check if it is prime or not")
while True:
    num = input("Enter a number: ")
    try:
        num = int(num)
        if num > 0:
            break
        else:
            print("please enter a valid positive integer")
    except ValueError:
        print("please enter a valid positive integer") 
result = prime_check.prime_check(num)
if result == True:
    print(num,"is a prime number")
else:
    print(num,"is not a prime number")
