use std::io;
use rand::Rng;
use std::cmp::Ordering;


struct Rectangle {
    width: u32,
    height: u32,
    name: String,
}

impl Rectangle {
    fn max(&self, other: Self) -> Self {
        let w = self.width.max(other.width);
        let h = self.height.max(other.height);
        Rectangle {
            width: w,
            height: h,
            name: String::from("max")
        }
    }
    fn set_to_max(&mut self, other: Rectangle) {
        *self = self.max(other)
    }
}


fn main() {
    println!("Guess the number");
    
    let secret_number = rand::thread_rng().gen_range(1..=100);
    
    
    
    loop {
        println!("Please input your guess");
        
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
        
        println!("You guessed: {}", guess);
        
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small"),
            Ordering::Greater => println!("Too big"),
            Ordering::Equal => {
                println!("You win");
                break;
            }
        }
    }
}
