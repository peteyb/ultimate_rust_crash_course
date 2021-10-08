const MY_CONST: i32 = 100;
use hello::{do_stuff_mutable_reference, do_stuff_owner, do_stuff_reference, greet};
use rand::prelude::*;
use std::collections::HashMap;
use std::fs::File;

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

    // collections
    let mut v: Vec<i32> = Vec::new();
    v.push(2);
    v.push(4);
    v.push(5);
    println!("v = {:?}", v);
    println!("val2 = {}", v[2]);
    let x = v.pop();
    println!("x = {:?} and {:?}", x, v);

    let v2 = vec![2, 4, 6];
    println!("v2 = {:?}", v2);

    let mut h: HashMap<u8, bool> = HashMap::new();
    h.insert(5, true);
    h.insert(6, false);
    let have_five = h.remove(&5).unwrap();
    println!("have five = {}", have_five);

    // enums
    let colour1 = Colour::Red;
    let colour2 = Colour::Green;
    let colour3 = Colour::Blue;
    println!("Colour 1 = {:?}", colour1);
    println!("Colour 2 = {:?}", colour2);
    println!("Colour 3 = {:?}", colour3);

    let item1 = DispenderItem::Place { x: 1, y: 2 };
    println!("item = {:?}", item1);

    if let DispenderItem::Place { x: 1, y: 2 } = item1 {
        println!("a: item is a dispenser");
    }

    match item1 {
        DispenderItem::Empty => {
            println!("b: item is an empty dispenser");
        }
        DispenderItem::Ammo(x) => {
            println!("b: item is ammo with value {}", x);
        }
        // DispenderItem::Things(x, y) => {
        //     println!("b: item is a Thing with values {} and {}", x, y);
        // }
        DispenderItem::Place { x, y } => {
            println!("b: item is a Place {} and {}", x, y);
        }
        _ => {
            println!("Default")
        }
    }

    let item2 = DispenderItem::Ammo(5);
    let item3 = DispenderItem::Empty;
    let item4 = DispenderItem::Things(String::from("test"), 5);
    println!("item2 = {:?}", item2);
    println!("item3 = {:?}", item3);
    println!("item4 = {:?}", item4);

    let my_match = match item2 {
        DispenderItem::Ammo(x) => x + 1,
        _ => 1,
    };
    println!("my_match = {:?}", my_match);

    // enum Option
    let mut option1: Option<i32> = None;
    println!("option1 = {:?}", option1);
    option1 = Some(5);
    println!("option1 = {:?}", option1);
    println!("option1 is_some = {:?}", option1.is_some());
    println!("option1 is_none = {:?}", option1.is_none());
    for i in option1 {
        println!("{}", i);
    }

    // enum Result
    let res = File::open("foo");
    if res.is_err() {
        println!("Could not open file")
    }
    if res.is_ok() {
        let f = res.unwrap();
        println!("{:?}", f);
    }

    let res2 = File::open("bar");
    match res2 {
        Ok(f) => {
            println!("file opened {:?}", f)
        }
        Err(e) => {
            println!("File open error {}", e)
        }
    }
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

// enum definitions
#[derive(Debug)]
enum Colour {
    Red,
    Green,
    Blue,
}

#[derive(Debug)]
enum DispenderItem {
    Empty,
    Ammo(u8),
    Things(String, i32),
    Place { x: i32, y: i32 },
}
