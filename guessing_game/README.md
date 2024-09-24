# Guessing Game

We’ll implement a classic beginner programming problem: a guessing game. Here’s how it works:

1. the program will generate a random integer between 1 and 100.
2. It will then prompt the player to enter a guess.
3. After a guess is entered, the program will indicate whether the guess is too low or too high.
4. If the guess is correct, the game will print a congratulatory message and exit.

## Processing a Guess

The first part of the guessing game program will ask for user input, process that input, and check that the input is in the expected form. To start, we’ll allow the player to input a guess.

To obtain user input and then print the result as output, we need to bring the io **input/output** library into scope. The io library comes from the standard library, known as **std**
