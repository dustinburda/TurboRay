#![feature(generic_const_exprs)]

use std::io;
use rand::Rng;
use std::fmt;


mod vec;
mod sphere;
mod ray;
// // mod sphere;
// use vec::Vec;
// // use sphere::Sphere;

struct Person {
    age: u8,
    name: String
}

impl Person {
    pub fn new(age: u8, name: String) -> Person {
        Person { age: age, name: name }
    }
}

impl fmt::Display for Person {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "age: {},  name: {}", self.age, self.name)
    }
}


pub fn main() {

    let dustin: Person = Person::new (24, String::from("Dustin Burda"));

    println!("{}", dustin);

    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    println!("The secret number is {secret_number}");

    println!("Please input your guess!");

    let mut guess = String::new();

    io::stdin()
    .read_line(&mut guess)
    .expect("Failed to read line");

    
}

