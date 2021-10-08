const MY_CONST: i32 = 100;
use hello::{do_stuff_mutable_reference, do_stuff_owner, do_stuff_reference, greet};
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
    println!(
        "{}",
        format!("{} bunnies3 with {} carrots", bunnies3, carrots)
    );

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

    // strings, owners and references
    let mut s1 = String::from("abc");
    let s2 = String::from("abc");
    let s3 = s1.clone();
    println!("s3 = {}", s3);
    do_stuff_reference(&s1);
    println!("{}", s1);
    do_stuff_owner(s2);
    do_stuff_mutable_reference(&mut s1);
    // println!("{}", s2);
    // structs
    let mut fox = RedFox::new();
    let life_left = fox.life;
    println!("fox life = {}", life_left);
    fox.enemy = false;
    println!("fox life = {}", fox.some_method());
    println!("is fox moved... {}", fox.enemy);

    fox.run();
    print_noise(fox);
}

struct RedFox {
    enemy: bool,
    life: u8,
}

impl RedFox {
    fn new() -> Self {
        Self {
            enemy: true,
            life: 70,
        }
    }

    fn some_method(&self) -> bool {
        self.enemy
    }
}

impl Noisy for RedFox {
    fn get_noise(&self) -> &str {
        "Meow!"
    }
}

trait Noisy {
    fn get_noise(&self) -> &str;
}

// T variable for any object implementing the type
fn print_noise<T: Noisy>(item: T) {
    println!("{}", item.get_noise());
}

// default implementation of trait method
trait Run {
    fn run(&self) {
        println!("Run forest, run")
    }
}
impl Run for RedFox {}
