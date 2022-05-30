use rand::Rng;

fn main() {
    println!("Guess the number!");
    let generated_secret_number: u32 = random_num_gen();
    // println!("The secrent number is {}", generated_secret_number);

    loop {
        println!("Please input your guess.");

        let mut guess = String::new();

        std::io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        // trim() trims spaces & newline char
        // parse() with turbofish syntax; it parses the input to u32
        let guess = match guess.trim().parse::<u32>() {
            Ok(num) => num,
            Err(_) => continue, // _ is a catch-all value
        };

        println!("You guessed: {}", guess);

        // compare values
        match guess.cmp(&generated_secret_number) {
            std::cmp::Ordering::Less => println!("Too small"),
            std::cmp::Ordering::Greater => println!("Too big"),
            std::cmp::Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}

fn random_num_gen() -> u32 {
    // generate a random number between 0 and 1
    let mut secret_number: f32 = rand::thread_rng().gen();
    // multiply secret_float_number to fit into the wanted range (1,101)
    secret_number = secret_number * 100.0;

    // shadow secret_number to u32
    let secret_number: u32 = secret_number.round() as u32;

    return secret_number;
}
