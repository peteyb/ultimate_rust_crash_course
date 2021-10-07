const MY_CONST: i32 = 100;
use hello::greet;
use rand::prelude::*;

fn main() {
    let mut rng = thread_rng();
    let rng_val: f64 = rng.gen();
    println!("Range val = {}", rng_val);
    
    greet();

    println!("Hello, world!");

    let bunnies = 2;
    let mut bunnies2: i32 = 4;
    let (bunnies3, carrots) = (8, 50);

    let bunnies_text = format!("{} bunnies", bunnies);
    println!("{}", bunnies_text);
    println!("{}", format!("{} bunnies2", bunnies2));
    println!("{}", format!("{} bunnies3 with {} carrots", bunnies3, carrots));

    bunnies2 = 45;
    println!("{} bunnies2 with MY_CONST={}", bunnies2, MY_CONST);

    let x = 5;
    {
        let y = 99;
        println!("x{} is in scope, y{} is in scope", x, y);
    }
    {
        let x = 99;
        println!("x{} has value", x);
    }
    println!("x{} has value", x);

    // let enimgma: i32;
    // println!("enigma has value {}", enimgma);
}
