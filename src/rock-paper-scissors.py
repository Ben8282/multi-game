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
        
    players = input("enter your answer")
    print ("you picked ",players)
    print ("I picked ",Bots)

    if Bots == players or Bots == "rock" and players == "r" or Bots == "paper" and players == "p" or Bots == "scissors" and players == "s":
        print ("tie")
        print ("do you want to play again? 1 for yes, 2 for no")
        play = int(input())

    if Bots == "rock":
        if players == "p" or players == "paper":
            print("you win")
            print ("do you want to play again? 1 for yes, 2 for no")
            play = int(input())
        if players == "s" or players == "scissors":
            print("I win!")
            print ("do you want to play again? 1 for yes, 2 for no")
            play = int(input())
        else :
            print("error or you made a mistake")

    if Bots == "paper":
        if players == "s" or players == "scissors":
            print("you win")
            print ("do you want to play again? 1 for yes, 2 for no")
            play = int(input())
        if players == "r" or players == "rock":
            print("I win!")
            print ("do you want to play again? 1 for yes, 2 for no")
            play = int(input())
        else :
            print("error or you made a mistake")
    

    if Bots == "scissors":
        if players == "r" or players == "rock":
            print("you win")
            print ("do you want to play again? 1 for yes, 2 for no")
            play = int(input())
        if players == "p" or players == "paper":
            print("I win!")
            print ("do you want to play again? 1 for yes, 2 for no")
            play = int(input())
        else:
            print("error or you made a mistake")
  
