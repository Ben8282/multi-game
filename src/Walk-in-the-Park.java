import java.util.Scanner;

class WalkInThePark {
  public static void main(String[] args) {
  Scanner user = new Scanner (System.in);
  System.out.println("Hello! Welcome to A Normal Day In A Normal Park!"); // 1 pg.
  System.out.print("Please enter your name: "); // 1 pg.
  String Username = new String(); // 1 pg.
  Username = user.nextLine(); // 1 pg.
  System.out.println("Hi " + Username + ", hope ur doing good..."); // 2 pg.
  System.out.print("So, tell me... why are you on your computer?"); // 2 pg.
  System.out.println("                                "); // 2 pg.
  System.out.print("                                "); // 2 pg.
  System.out.println("                               "); // 2 pg.
  System.out.println("Option 1. Well, I am really bored..."); // 2 pg.
  System.out.print("Option 2. Because without a computer, I can't play this can I?");// 2 pg.
  System.out.println("                                ");// 2 pg.
  System.out.println("Option 3. Because I want to see what the heck this is!");// 2 pg.
  System.out.println("Option 4. Why are you asking me?");// 2 pg.
  
  
  
  int option1;
  option1 = user.nextInt();
  user.nextLine();
  if (option1 <= 0) {
  System.out.println("Please input an option to continue..."); // 3 pg. alt 0...
  user.nextLine();
  
  }else if (option1 <= 1) { // 2pg.
  System.out.println("That explains it."); // 3 pg. alt 1
  System.out.print("When there are so many stimuli in today's world, we tend to nod off now and then."); // 3 pg. alt 1
  System.out.println(""); // 3 pg. alt 1
  System.out.println("Now, I have a special trick up my sleeves."); // 3 pg. alt 1
  System.out.println("Do you want to hear it?"); // 3 pg. alt 1
  System.out.print("yes/no?"); // 3 pg. alt 1
  String choice = user.nextLine(); // 3 pg. alt 1
  
  if (choice.equalsIgnoreCase("yes")) { // 4 pg. alt yes
    System.out.println(""); // 4 pg. alt yes
    System.out.println(""); // 4 pg. alt yes
    System.out.println("Bordem breeds bravery... as they always say..."); // 4 pg. alt yes
    System.out.println("But...bravery often ends up with someone getting hurt!"); // 4 pg. alt yes
    System.out.print("Is that right?"); // 4 pg. alt yes
    System.out.println(""); // 4 pg. alt yes
    System.out.println(""); // 4 pg. alt yes
    System.out.println("Option 1. Yes, that's why I'm trying not to be bored!"); // 4 pg. alt yes
    System.out.println("Option 2. Yes, I used to be an adventurer like you, but then I took an arrow in the knee..."); // 4 pg. alt yes
    System.out.println("Option 3. Well, not really if your careful."); // 4 pg. alt yes
    System.out.print("Option 4. Never, I am the bravest man in all the lands! I never get hurt!"); // 4 pg. alt yes
    
  int option2;
  option2 = user.nextInt();
  if (option1 <= 0) {
    System.out.println("Please input an option to continue..."); //
    
  }else if (option1 <= 1) {
    System.out.println("Great answer! We'll get you suited write away!");
    System.out.println("");
    System.out.println("You seem like an adventurous fella!");
    System.out.println("");
    System.out.println("Get ready for a great time!");
    System.out.println("");
    System.out.println("Know I want you to imagine that you are in the gates of a park...");
    System.out.println("");
    Scanner scanner1 = new Scanner(System.in); //      !!!!!!!!!!
    System.out.println("Press ENTER to enter the park"); //      !!!!!!!!!!
     String User = scanner1.nextLine(); //      !!!!!!!!!!
  System.out.println("");
    System.out.print("You boldly enter the hustle and bustle of the park.");
    
    
  }else if (option1 <= 1) {
    System.out.println("This is option 1");  
    
  }else if (option1 <= 2) {
  System.out.println("This is option 2");
  
  }else if (option1 <=3) {
    System.out.println("3");
    
  }else if (option1 <= 4) {
    System.out.println("4");
    }
    
  }
  
  
  
  
  // // // // // // // // // // // // // // // // // // // // // // // // // // //  Option 2 for question 1
  
  
  
  }else if (option1 <= 2) {
  System.out.println("opt 2 q0");
  
  }else if (option1 <=3) {
    System.out.println("opt 3 q0");
    
  }else if (option1 <= 4) {
    System.out.println("opt 4 q0");
    }
  }
}
