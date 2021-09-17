//Let Rust know we are using 'rand' crate
//enables to call anything placing ::rand
extern crate rand;

//Module that contains Read and Write traits
//including the ability to accept user input.
use std::io; 

//This type is an enum with Less, Greater,
//and Equal variants.
use std::cmp::Ordering;

//Enables methods for generating random numbers. 
use rand::Rng;

fn main() {

    //Printing a prompt stating what the game is
    //and requesting input from the user.
    println!("Guess the number!");
    
    //rand::thread_rng is the random number generator
    //seeded by the operating system.
    //gen_range is a method called to the funcion
    //previously defined by use rand::Rng statement.
    //This method takes two numbers as arguments
    //and generates the random number.
    //It is inclusive in the lower bound
    //and exclusive in the uppper bound.
    let secret_number = rand::thread_rng().gen_range(1, 101);

    //println!("The secret number is: {}", secret_number);

    //Inserting a loop to give the user more chances
    //to guess.
    loop {
        println!("Please input your guess.");

        //let creates a variable named guess to store
        //the user input.
        //mut makes a mutable variable.
        //String::new is a function that returns a
        //new instance of a 'String'
        //'String' is a type provided by the standard library.
        //:: syntax in ::new indicates that new is an
        //associated function of the 'String' type.
        //it creates a new empty string.
        let mut guess = String::new();

        //stdin is a type that represents a handle
        //to the standard input of the terminal.    
        //read_line is a method that gets the input
        //from the user from the standard input handle.
        //We are passing the &mut guess argument.
        //read_line takes what the user types into the
        //standard input and place that into a string
        //using that string as an argument and it needs
        //to be mutable.
        //& indicates that this argument is a reference
        //making data available without needing to
        //copy that data into the memory multiple times.
        io::stdin().read_line(&mut guess)

            //read_line returns a value io::Result
            //'Result' type is an enumeration which is
            //type that can have a fixed set of values.
            //For 'Result' the variants are 'Ok' and 'Err'.
            //This to encode error handling information.
            //As any other type it has methods defined on it.
            //expect is a method for io::Result.
            //If the instance for io:Result is 'Err' expect
            //will cause the program to crash and display
            //the message passed as an argument.
            .expect("Failed to read line");

        //Here let guess declares a new variable that replaces
        //guess string variable declared before
        //by means of calling the trim method which
        //removes any blank spaces at the end or beginning
        //of the string and the parse method parsing the string
        //into some kind of number in this case a u32 integer.
        let guess: u32 = match guess.trim().parse(){
            //.expect("Please type a number!");

            //Adding match to handle error instead
            //of just crashing the program.
            //_ is a catchall value matching all
            //Err values to execute continue i.e.
            //go to the next loop.
            Ok(num) => num,
            Err(_) => continue,
        };
        //Prints the string we saved the user's input in.
        //{} is a placeholder that will print guess value.
        println!("You guessed: {}", guess);

        //cmp method compares 2 values guess and the secret number.
        //match expression is made up of 'arms'. An arm consist
        //of a pattern and the code that should be run if the
        //value given to the beginning of the match expression
        //fits the arm's pattern.
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                //Exit the loop when the guess is correct.
                break;
            }
        }
    }
}
