pub fn inspect(s: &str) {
    if s.ends_with('s') {
        println!("plural")
    } else {
        println!("singular")
    }
}

pub fn change(s: &mut String) {
    s.push('s')
}

pub fn eat(s: String) -> bool {
    if s.starts_with('b') && s.contains('a') {
        true
    } else {
        false
    }
}

pub fn add(x: &i32, y: &i32) -> i32 {
    *x + *y
}
