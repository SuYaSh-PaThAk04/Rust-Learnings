use std::{cmp::Ordering, io};
use rand::{Rng, RngExt};
fn main() {
    println!("Welcome to the game!");
    let secret_number = rand::rng().random_range(1..=100);
  loop {
      
  
    println!("Please input your guess");
   
    let mut guess = String :: new();

    io::stdin()
    .read_line(&mut guess)
    .expect("Failed to read your guess");
    println!("Your guess was {guess}");
   let guess : u32 = match  guess.trim().parse(){
    Ok(num)=> num,
    Err(_)=> continue,
   };
    match guess.cmp(&secret_number){
        Ordering::Less =>println!("The guess was less than expected number"),
        Ordering::Greater => println!("The guess was greater then expected"),
        Ordering::Equal => {println!(" YAYY YOU FIND YOUR NUMBERRR !!!!!");
    break;}
    }
    } 
}
