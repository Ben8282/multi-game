import random

def gamble():
    print("Welcome to the gambling game!")
    balance = 100 
    while True:
        print(f"Your current balance is: ${balance}")
        bet = input(f"{name}, enter your bet amount (or type 'exit' to quit): ")
        if bet.lower() == 'exit':
            print("Thanks for playing! Goodbye.")
            break
        try:
            bet = int(bet)
            if bet <= 0 or bet > balance:
                print("Invalid bet amount. Please enter a positive number within your balance.")
                continue
        except ValueError:
            print("Please enter a valid number.")
            continue
        
        outcome = random.randint(1,1000)
        if name == "pro gambler" or name == "Pro Gambler" or name == "Pro gambler" or name == "pro Gambler":
            outcome = 1000
        if outcome <= 500:
            balance += bet
            print(f"You won! Your new balance is: ${balance}")
        if outcome >= 500 and outcome <= 950:
            balance -= bet
            print(f"You lost! Your new balance is: ${balance}")
        if outcome <= 950 and outcome >= 995:
            balance += bet * 10
            print(f"Jackpot! You won ${bet * 10}! Your new balance is: ${balance}")
        if outcome == 1000:
            balance += bet * 777
            print(f"Jackpot 777! You won ${bet * 777}! Your new balance is: ${balance}")
        
        if balance <= 0:
            print("You have run out of money! Game over.")
            break
name = input("What's your name? ")
print(f"Welcome, {name}!")
gamble()
