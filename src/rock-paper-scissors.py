import random

print("hi welcome to rock paper scissors you can enter r, p, s or rock, paper, scissors")

play = 1

while play == 1:

    Bot = (random.randint (1, 3))

    if Bot == 1:
        Bots = ("rock")
    if Bot == 2:
        Bots = ("paper")
    if Bot == 3:
        Bots = ("scissors")
    while True:    
        players = input("enter your answer: ")
        players = players.lower()
        if players == "rock" or players == "r" or players == "paper" or players == "p" or players == "scissors" or players == "s":
            break
        else:
            print("make sure to type r, p, s or rock, paper, scissors")
            continue
    print ("you picked",players)
    print ("I picked",Bots)

    if Bots == players or Bots == "rock" and players == "r" or Bots == "rock" and players == "rock" or Bots == "paper" and players == "p" or Bots == "paper" and players == "paper" or Bots == "scissors" and players == "s" or Bots == "scissors" and players == "scissors":
        print ("tie")
        print ("do you want to play again? 1 for yes, 2 for no")
        while True:
            play = input()
            try:
                play = int(play)
                if play != 1 and play != 2:
                    print("please make sure you input 1 or 2")
                else:
                    break
            except ValueError: 
                print("please make sure you input 1 or 2")
    if Bots == "rock":
        if players == "p" and players == "paper":
            print("you win")
            print ("do you want to play again? 1 for yes, 2 for no")
            while True:
                play = input()
                try:
                    play = int(play)
                    if play != 1 and play != 2:
                        print("please make sure you input 1 or 2")
                    else:
                        break
                except ValueError: 
                    print("please make sure you input 1 or 2")
        if players == "s" or players == "scissors":
            print("I win!")
            print ("do you want to play again? 1 for yes, 2 for no")
            while True:
                play = input()
                try:
                    play = int(play)
                    if play != 1 and play != 2:
                        print("please make sure you input 1 or 2")
                    else:
                        break
                except ValueError: 
                    print("please make sure you input 1 or 2")
    if Bots == "paper":
        if players == "s" or players == "scissors":
            print("you win")
            print ("do you want to play again? 1 for yes, 2 for no")
            while True:
                play = input()
                try:
                    play = int(play)
                    if play != 1 and play != 2:
                        print("please make sure you input 1 or 2")
                    else:
                        break
                except ValueError: 
                    print("please make sure you input 1 or 2")
        if players == "r" or players == "rock":
            print("I win!")
            print ("do you want to play again? 1 for yes, 2 for no")
            while True:
                play = input()
                try:
                    play = int(play)
                    if play != 1 and play != 2:
                        print("please make sure you input 1 or 2")
                    else:
                        break
                except ValueError: 
                    print("please make sure you input 1 or 2")
    
    if Bots == "scissors":
        if players == "r" or players == "rock":
            print("you win")
            print ("do you want to play again? 1 for yes, 2 for no")
            while True:
                play = input()
                try:
                    play = int(play)
                    if play != 1 and play != 2:
                        print("please make sure you input 1 or 2")
                    else:
                        break
                except ValueError: 
                    print("please make sure you input 1 or 2")
        if players == "p" or players == "paper":
            print("I win!")
            print ("do you want to play again? 1 for yes, 2 for no")
            while True:
                play = input()
                try:
                    play = int(play)
                    if play != 1 and play != 2:
                        print("please make sure you input 1 or 2")
                    else:
                        break
                except ValueError: 
                    print("please make sure you input 1 or 2")
                    continue
        

  
