fn checkGuess(guess: i32, secretNumber: i32) -> i32{
    if guess == secretNumber{
        0
    }
    else if guess > secretNumber {
        1
    }
    else {
        -1
    }
}

fn main() {
    let secretNumber: i32 = 42;
    let mut attempts: i32 = 0;

    let mut guess = 56;

    loop {
        attempts += 1;

        println!("Your guess was {}", guess);
        let result = checkGuess(guess, secretNumber);

        if result == 0 {
            println!("You guessed the number correct!");
            break;
        }
        else if result == 1 {
            println!("You guessed too high. Please try again.");
            guess -= 1;
        }
        else{
            println!("You guessed to low. Please try again.");
            guess += 1;
        }
    }

    println!("It took you {} guesses", attempts)
}