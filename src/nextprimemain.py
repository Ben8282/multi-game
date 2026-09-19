import next_prime
print("welcome to the next prime number checker")
print("Enter a number to find the next prime after it")
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
result = next_prime.next_prime(num)
print("The next prime number after", num, "is", result)