import random
from unicodedata import name

def wordle():

    with open("src/words.txt", "r") as file:
        word_list = file.read().splitlines()

    secret_word = random.choice(word_list)

    if name == "flowery" or name == "Flowery" or name == "flowey" or name == "Flowey":
        print(f"Hello, {name}! You will get deltarune and undertale words.")
        secret_word = random.choice(["deltarune", "undertale", "sans", "papyrus", "toriel", "asriel", "chara", "frisk", "gaster", "mettaton", "alphys", "undyne", "temmie", "flowey", "asgore", "napstablook", "monsterkid", "burgerpants", "doggo", "muffet", "jerry", "madjick", "tsundereplane", "bratty", "catty", "royalguard", "snowdin", "waterfall", "hotland", "core", "ralsei", "kris", "susie", "noelle", "dess", "jevil", "queen", "lancer", "rudinn", "seam", "catti", "gerson", "jockington", "jockingtonthe3rd", "jockingtonthe3rd'sfriend", "jockingtonthe3rd'sfriend'sfriend","gaster"])
    
    attempts = 6  
    print("Welcome to Wordle!")
    print(f"You have {attempts} attempts to guess the word.")
    print(f"Please enter a {len(secret_word)}-letter word.")
    
    while attempts > 0:
        guess = input("Enter your guess: ").lower()
        
        if len(guess) != len(secret_word):
            print(f"Please enter a {len(secret_word)}-letter word.")
            continue
        
        if guess == secret_word:
            print("Congratulations! You've guessed the word correctly!")
            return
        
        feedback = []
        for i in range(len(secret_word)):
            if guess[i] == secret_word[i]:
                feedback.append(guess[i].upper())  
            elif guess[i] in secret_word:
                feedback.append(guess[i])  
            else:
                feedback.append("_")  
        
        print("Feedback: " + " ".join(feedback))
        attempts -= 1
        print(f"You have {attempts} attempts left.")
    
    print(f"Sorry, you've run out of attempts. The secret word was '{secret_word}'.")

print("when you enter an answer it will be lowercase if it is correct but in the wrong spot, if it is correct it will be uppercase")

name = input("Enter your name: ")

wordle() 